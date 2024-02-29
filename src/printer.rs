use {
    clap::Command,
    std::collections::HashMap,
    termimad::{
        minimad::{OwningTemplateExpander, TextTemplate},
        FmtText, MadSkin,
    },
};

/// Default template for the "title" section
pub static TEMPLATE_TITLE: &str = "# **${name}** ${version}";

/// Default template for the "author" section
pub static TEMPLATE_AUTHOR: &str = "
*by* ${author}
";

/// Default template for the "usage" section
pub static TEMPLATE_USAGE: &str = "
**Usage: ** `${name} [options]${positional-args}`
";

/// Default template for the "positionals" section
pub static TEMPLATE_POSITIONALS: &str = "
${positional-lines
* `${key}` : ${help}
}
";

/// Default template for the "options" section
pub static TEMPLATE_OPTIONS: &str = "
**Options:**
|:-:|:-:|:-:|:-|
|short|long|value|description|
|:-:|:-|:-:|:-|
${option-lines
|${short}|${long}|${value}|${help}${possible_values}${default}|
}
|-
";

/// Keys used to enable/disable/change templates
pub static TEMPLATES: &[&str] = &[
    "title",
    "author",
    "introduction",
    "usage",
    "positionals",
    "options",
    "bugs",
];

/// An object which you can configure to print the
/// help of a command
pub struct Printer<'t> {
    skin: MadSkin,
    expander: OwningTemplateExpander<'static>,
    template_keys: Vec<&'static str>,
    templates: HashMap<&'static str, &'t str>,
    pub full_width: bool,
    pub max_width: Option<usize>,
}

impl<'t> Printer<'t> {
    pub fn new(mut cmd: Command) -> Self {
        cmd.build();
        let expander = Self::make_expander(&cmd);
        let mut templates = HashMap::new();
        templates.insert("title", TEMPLATE_TITLE);
        templates.insert("author", TEMPLATE_AUTHOR);
        templates.insert("usage", TEMPLATE_USAGE);
        templates.insert("positionals", TEMPLATE_POSITIONALS);
        templates.insert("options", TEMPLATE_OPTIONS);
        Self {
            skin: Self::make_skin(),
            expander,
            templates,
            template_keys: TEMPLATES.to_vec(),
            full_width: false,
            max_width: None,
        }
    }
    /// Build a skin for the detected theme of the terminal
    /// (i.e. dark, light, or other)
    pub fn make_skin() -> MadSkin {
        match terminal_light::luma() {
            Ok(luma) if luma > 0.85 => MadSkin::default_light(),
            Ok(luma) if luma < 0.2 => MadSkin::default_dark(),
            _ => MadSkin::default(),
        }
    }
    /// Use the provided skin
    pub fn with_skin(mut self, skin: MadSkin) -> Self {
        self.skin = skin;
        self
    }
    /// Set a maximal width, so that the whole terminal width isn't used.
    ///
    /// This may make some long sentences easier to read on super wide
    /// terminals, especially when the whole text is short.
    /// Depending on your texts and parameters, you may set up a width
    /// of 100 or 150.
    pub fn with_max_width(mut self, w: usize) -> Self {
        self.max_width = Some(w);
        self
    }
    /// Give a mutable reference to the current skin
    /// (by default the automatically selected one)
    /// so that it can be modified
    pub fn skin_mut(&mut self) -> &mut MadSkin {
        &mut self.skin
    }
    /// Change a template
    pub fn set_template(&mut self, key: &'static str, template: &'t str) {
        self.templates.insert(key, template);
    }
    /// Change or add a template
    pub fn with(mut self, key: &'static str, template: &'t str) -> Self {
        self.set_template(key, template);
        self
    }
    /// Unset a template
    pub fn without(mut self, key: &'static str) -> Self {
        self.templates.remove(key);
        self
    }
    /// A mutable reference to the list of template keys, so that you can
    /// insert new keys, or change their order.
    /// Any key without matching template will just be ignored
    pub fn template_keys_mut(&mut self) -> &mut Vec<&'static str> {
        &mut self.template_keys
    }
    /// A mutable reference to the list of template keys, so that you can
    /// insert new keys, or change their order.
    /// Any key without matching template will just be ignored
    #[deprecated(since = "0.6.2", note = "use template_keys_mut instead")]
    pub fn template_order_mut(&mut self) -> &mut Vec<&'static str> {
        &mut self.template_keys
    }
    fn make_expander(cmd: &Command) -> OwningTemplateExpander<'static> {
        let mut expander = OwningTemplateExpander::new();
        expander.set_default("");
        let name = cmd.get_bin_name().unwrap_or_else(|| cmd.get_name());
        expander.set("name", name);
        if let Some(author) = cmd.get_author() {
            expander.set("author", author);
        }
        if let Some(version) = cmd.get_version() {
            expander.set("version", version);
        }
        let options = cmd
            .get_arguments()
            .filter(|a| a.get_short().is_some() || a.get_long().is_some());
        for arg in options {
            let sub = expander.sub("option-lines");
            if let Some(short) = arg.get_short() {
                sub.set("short", format!("-{short}"));
            }
            if let Some(long) = arg.get_long() {
                sub.set("long", format!("--{long}"));
            }
            if let Some(help) = arg.get_help() {
                sub.set_md("help", help.to_string());
            }
            if arg.get_action().takes_values() {
                if let Some(name) = arg.get_value_names().and_then(|arr| arr.first()) {
                    sub.set("value", name);
                };
            }
            let mut possible_values = arg.get_possible_values();
            if !possible_values.is_empty() {
                let possible_values: Vec<String> = possible_values
                    .drain(..)
                    .map(|v| format!("`{}`", v.get_name()))
                    .collect();
                expander.sub("option-lines").set_md(
                    "possible_values",
                    format!(" Possible values: [{}]", possible_values.join(", ")),
                );
                if let Some(default) = arg.get_default_values().first() {
                    expander.sub("option-lines").set_md(
                        "default",
                        format!(" Default: `{}`", default.to_string_lossy()),
                    );
                }
            }
        }
        let mut args = String::new();
        for arg in cmd.get_positionals() {
            let Some(key) = arg.get_value_names().and_then(|arr| arr.first()) else {
                continue;
            };
            args.push(' ');
            if !arg.is_required_set() {
                args.push('[');
            }
            if arg.is_last_set() {
                args.push_str("-- ");
            }
            args.push_str(key);
            if !arg.is_required_set() {
                args.push(']');
            }
            let sub = expander.sub("positional-lines");
            sub.set("key", key);
            if let Some(help) = arg.get_help() {
                sub.set("help", help);
            }
        }
        expander.set("positional-args", args);
        expander
    }
    /// Give you a mut reference to the expander, so that you can overload
    /// the variable of the expander used to fill the templates of the help,
    /// or add new variables for your own templates
    pub fn expander_mut(&mut self) -> &mut OwningTemplateExpander<'static> {
        &mut self.expander
    }
    /// Print the provided template with the printer's expander
    ///
    /// It's normally more convenient to change template_keys or some
    /// templates, unless you want none of the standard templates
    pub fn print_template(&self, template: &str) {
        self.skin.print_owning_expander_md(&self.expander, template);
    }
    /// Print all the templates, in order
    pub fn print_help(&self) {
        if self.full_width {
            self.print_help_full_width()
        } else {
            self.print_help_content_width()
        }
    }
    fn print_help_full_width(&self) {
        for key in &self.template_keys {
            if let Some(template) = self.templates.get(key) {
                self.print_template(template);
            }
        }
    }
    fn print_help_content_width(&self) {
        let (width, _) = termimad::terminal_size();
        let mut width = width as usize;
        if let Some(max_width) = self.max_width {
            width = width.min(max_width);
        }
        let mut texts: Vec<FmtText> = self
            .template_keys
            .iter()
            .filter_map(|key| self.templates.get(key))
            .map(|&template| {
                let template = TextTemplate::from(template);
                let text = self.expander.expand(&template);
                FmtText::from_text(&self.skin, text, Some(width))
            })
            .collect();
        let content_width = texts
            .iter()
            .fold(0, |cw, text| cw.max(text.content_width()));
        for text in &mut texts {
            text.set_rendering_width(content_width);
            println!("{}", text);
        }
    }
}
