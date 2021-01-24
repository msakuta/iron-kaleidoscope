extern crate rustc_serialize;
extern crate docopt;

extern crate iron_kaleidoscope;

use structopt::StructOpt;

use iron_kaleidoscope::driver::{main_loop,
                                Tokens
};

use docopt::Docopt;

const USAGE: &'static str = "
Usage: iron_kaleidoscope [(-l | -p | -i)]

Options:
    -l  Run only lexer and show its output.
    -p  Run only parser and show its output.
    -i  Run only IR builder and show its output.
";

#[derive(Debug, StructOpt)]
#[structopt(name = "Args")]
struct Args {
    #[structopt(short = "l", long)]
    flag_l: bool,
    #[structopt(short = "p", long)]
    flag_p: bool,
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
