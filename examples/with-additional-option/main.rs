use {
    clap::{CommandFactory, Parser},
};


/// Application launch arguments
#[derive(Parser, Debug)]
#[command(name = "wao", author, version, about, disable_help_flag = true)]
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

    /// Root Directory
    pub root: Option<std::path::PathBuf>,
}

pub fn print_help() {
    let mut printer = clap_help::Printer::new(Args::command())
        .without("author");
    printer
        .expander_mut()
        .sub("option-lines")
        .set("short", "-z")
        .set("long", "--zeta")
        .set("value", "ZETA")
        .set("help", "Set the index of the last letter of the greek alphabet");
    printer.print_help();
}

fn main() {
    let args = Args::parse();

    if args.help {
        print_help();
        return;
    }

    let (w, h) = (args.width, args.height);
    println!("{w} x {h} = {}", w * h);
}
