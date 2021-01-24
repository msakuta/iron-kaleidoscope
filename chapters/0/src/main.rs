mod driver;
mod lexer;

use structopt::StructOpt;

use crate::driver::{main_loop, Tokens};

#[derive(Debug, StructOpt)]
#[structopt(name = "Args")]
struct Args {
    /// Run only lexer and show its output
    #[structopt(short = "l", long)]
    flag_l: bool,

    /// Run only parser and show its output.
    #[structopt(short = "p", long)]
    flag_p: bool,

    /// Run only IR builder and show its output.
    #[structopt(short = "i", long)]
    flag_i: bool,
}

fn main() {
    let args: Args = Args::from_args();

    if args.flag_p || args.flag_i {
        unimplemented!();
    }
    let stage = Tokens;

    main_loop(stage);
}
