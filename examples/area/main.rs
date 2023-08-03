use {
    clap::{CommandFactory, Parser, ValueEnum},
    clap_help::Printer,
};

static INTRO: &str = "
Compute `height x width`
*You can do it either precisely (enough) or fast (I mean not too slow)*.
";

/// Application launch arguments
#[derive(Parser, Debug)]
#[command(name="area", author, version, about, disable_help_flag = true)]
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
        Printer::new(Args::command())
            .with("introduction", INTRO)
            .without("author")
            .print_help();
        return;
    }

    let (w, h) = (args.width, args.height);
    println!("Computation strategy: {:?}", args.strategy);
    println!("{w} x {h} = {}", w*h);
}

