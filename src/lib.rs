/*!

clap-help is an alternate help printer for applications using clap.

clap-help displays arguments in a more readable format, in a width-aware table and lets you customize the content and style.

Minimal usage:

1. disable the standard clap help printer with `disable_help_flag = true`
2. add your own help flag
3. call `clap_help::print_help` in the handler for your help flag

```rust
use clap::{CommandFactory, Parser, ValueEnum};
use clap_help::Printer;

#[derive(Parser, Debug)]
#[command(name="my_prog", author, version, about, disable_help_flag = true)]
struct Args {

    /// Print help
    #[arg(long)]
    help: bool,

    // other arguments
}

fn main() {
    let args = Args::parse();
    if args.help {
        Printer::new(Args::command()).print_help();
        return;
    }

    // rest of the program
}

```

The examples directory shows how to customize the help.

*/

mod printer;

pub use printer::*;
