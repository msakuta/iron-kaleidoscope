use crate::driver::{main_loop, IR, Tokens, AST};
use structopt::StructOpt;

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
    let args = Args::from_args();

    let stage = if args.flag_l {
        Tokens
    } else if args.flag_p {
        AST
    } else if args.flag_i {
        IR
    } else {
        IR
    };

    main_loop(stage);
}
