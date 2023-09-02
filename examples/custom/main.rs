use {
    clap::{CommandFactory, Parser, ValueEnum},
    clap_help::Printer,
    termimad::ansi,
};

static INTRO: &str = "

Compute `height x width`
More info at *https://dystroy.org*
";

pub static TEMPLATE_OPTIONS: &str = "

**Options:**
|:-:|:-:|:-|
|short|long|what it does|
|:-:|:-|:-|
${option-lines
|*${short}*|*${long}*|${help}${possible_values}${default}|
}
|-
";

/// Application launch arguments
#[derive(Parser, Debug)]
#[command(name = "custom", author, version, about, disable_help_flag = true)]
struct Args {
    /// Print help
    #[arg(long)]
    help: bool,

    /// Height, that is the distance between bottom and top
    #[arg(short, long, default_value = "9")]
    height: u16,

    /// Width, from there, to there, eg `4` or `5`
    #[arg(short, long, default_value = "3")]
    width: u16,

    /// Kill all birds to improve computation
    #[arg(short, long)]
    kill_birds: bool,

    /// Computation strategy
    #[arg(long, default_value = "fast")]
    strategy: Strategy,

    /// Bird separator
    #[arg(short, long, value_name = "SEP")]
    separator: Option<String>,

    /// Root Directory
    pub root: Option<std::path::PathBuf>,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Strategy {
    Fast,
    Precise,
}

fn main() {
    let args = Args::parse();

    if args.help {
        let mut printer = Printer::new(Args::command())
            .without("author")
            .with("introduction", INTRO)
            .with("options", TEMPLATE_OPTIONS);
        let skin = printer.skin_mut();
        skin.headers[0].compound_style.set_fg(ansi(202));
        skin.bold.set_fg(ansi(202));
        skin.italic = termimad::CompoundStyle::with_fg(ansi(45));
        skin.inline_code = termimad::CompoundStyle::with_fg(ansi(223));
        skin.table_border_chars = termimad::ROUNDED_TABLE_BORDER_CHARS;
        printer.print_help();
        return;
    }

    let (w, h) = (args.width, args.height);
    println!("Computation strategy: {:?}", args.strategy);
    println!("{w} x {h} = {}", w * h);
}
