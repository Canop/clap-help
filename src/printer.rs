use {
    clap::{
        Command,
    },
    std::fmt::Write,
    termimad::{
        minimad::OwningTemplateExpander,
        MadSkin,
    },
};

/// An object which you can configure to print the
/// help of a command
pub struct Printer {
    skin: MadSkin,
    expander: OwningTemplateExpander<'static>,
    introduction: &'static str,
    show_author: bool,
}

pub static TEMPLATE_TITLE: &str = "# **${name}** v${version}";
pub static TEMPLATE_AUTHOR: &str = "

*by* ${author}
";
pub static TEMPLATE_USAGE: &str = "
**Usage: ** `${name} [options]${positional-args}`
";
pub static TEMPLATE_POSITIONALS: &str = "

${positional-lines
* `${key}` : ${help}
}
";
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

impl Printer {
    pub fn new(cmd: Command) -> Self {
        let expander = Self::make_expander(&cmd);
        Self {
            skin: Self::make_skin(),
            expander,
            introduction: "",
            show_author: false,
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
    /// Give a mutable reference to the current skin
    /// (by default the automatically selected one)
    /// so that it can be modified
    pub fn skin_mut(&mut self) -> &mut MadSkin {
        &mut self.skin
    }
    /// Set the introduction text, interpreted as Markdown
    pub fn with_introduction(mut self, intro: &'static str) -> Self {
        self.introduction = intro;
        self
    }
    /// Set whether to display the application author (default is true)
    pub fn show_author(mut self, b: bool) -> Self {
        self.show_author = b;
        self
    }
    fn make_expander<'c>(cmd: &'c Command) -> OwningTemplateExpander<'static> {
        let mut expander = OwningTemplateExpander::new();
        expander.set_default("");
        let name = cmd.get_bin_name()
            .unwrap_or_else(|| cmd.get_name());
        expander.set("name", name);
        if let Some(author) = cmd.get_author() {
            expander.set("author", author);
        }
        if let Some(version) = cmd.get_version() {
            expander.set("version", version);
        }
        let options = cmd
            .get_arguments()
            .filter(|a|
                a.get_short().is_some() || a.get_long().is_some()
            );
        for arg in options {
            let sub = expander.sub("option-lines");
            if let Some(short) = arg.get_short() {
                sub.set("short", format!("-{short}"));
            }
            if let Some(long) = arg.get_long() {
                sub.set("long", format!("--{long}"));
            }
            if let Some(help) = arg.get_help() {
                sub.set("help", help);
            }
            if arg.get_action().takes_values() {
                if let Some(name) = arg.get_value_names().and_then(|arr| arr.get(0)) {
                    sub.set("value", name);
                };
            }
            let mut possible_values = arg.get_possible_values();
            if !possible_values.is_empty() {
                let possible_values: Vec<String> = possible_values
                    .drain(..)
                    .map(|v| format!("`{}`", v.get_name()))
                    .collect();
                expander
                    .sub("option-lines")
                    .set_md(
                        "possible_values",
                        format!(" Possible values: [{}]", possible_values.join(", ")),
                    );
            }
            if let Some(default) = arg.get_default_values().get(0) {
                expander
                    .sub("option-lines")
                    .set_md(
                        "default",
                        format!(" Default: `{}`", default.to_string_lossy()),
                    );
            }
        }
        let mut args = String::new();
        for arg in cmd.get_positionals() {
            let Some(key) = arg.get_value_names().and_then(|arr| arr.get(0)) else {
                continue;
            };
            if arg.is_required_set() {
                let _ = write!(&mut args, " {}", key);
            } else {
                let _ = write!(&mut args, " [{}]", key);
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
    pub fn expander_mut(&mut self) -> &mut OwningTemplateExpander<'static> {
        &mut self.expander
    }
    pub fn print_template(&self, template: &str) {
        self.skin.print_owning_expander_md(&self.expander, template);
    }
    /// Print the whole help: title, author, introduction, usage, and options
    ///
    /// To print only some parts, or to use custom templates, use `print_template`
    /// instead.
    pub fn print_help(&self) {
        self.print_template(TEMPLATE_TITLE);
        if self.show_author {
            self.print_template(TEMPLATE_AUTHOR);
        }
        self.skin.print_text(self.introduction);
        self.print_template(TEMPLATE_USAGE);
        self.print_template(TEMPLATE_POSITIONALS);
        self.print_template(TEMPLATE_OPTIONS);
    }
}
