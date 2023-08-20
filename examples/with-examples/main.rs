mod examples;

use {
    clap::{CommandFactory, Parser, ValueEnum},
    examples::*,
};

static INTRO_TEMPLATE: &str = "
Compute `height x width`
";

static EXAMPLES_TEMPLATE: &str = "
**Examples:**

${examples
**${example-number})** ${example-title}: `${example-cmd}`
${example-comments}
}
";

/// Application launch arguments
#[derive(Parser, Debug)]
#[command(name="withex", author, version, about, disable_help_flag = true)]
struct Args {

    /// Print help
    #[arg(long)]
    help: bool,

    /// Only use ASCII characters
    #[arg(long)]
    ascii: bool,

    /// Height, that is the distance between bottom and top
    #[arg(short, long, default_value = "9")]
    height: u16,

    /// Width, from there, to there, eg `4` or `5`
    #[arg(short, long, default_value = "3")]
    width: u16,

    /// Computation strategy
    #[arg(long, default_value = "fast")]
    strategy: Strategy,

    /// Root Directory
    pub root: Option<std::path::PathBuf>,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Strategy {
    Fast,
    Precise,
}


pub fn print_help() {
    let args = Args::parse();
    let mut printer = clap_help::Printer::new(Args::command())
        .with("introduction", INTRO_TEMPLATE)
        .without("author");
    if args.ascii {
        printer.skin_mut().limit_to_ascii();
    }
    printer.template_keys_mut().push("examples");
    printer.set_template("examples", EXAMPLES_TEMPLATE);
    for (i, example) in EXAMPLES.iter().enumerate() {
        printer
            .expander_mut()
            .sub("examples")
            .set("example-number", i + 1)
            .set("example-title", example.title)
            .set("example-cmd", example.cmd)
            .set_md("example-comments", example.comments);
    }
    printer.print_help();
}


fn main() {
    let args = Args::parse();

    if args.help {
        print_help();
        return;
    }

    let (w, h) = (args.width, args.height);
    println!("Computation strategy: {:?}", args.strategy);
    println!("{w} x {h} = {}", w*h);
}

