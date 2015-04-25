extern crate getopts;

mod check;
mod discover;
mod list;

use std::convert::AsRef;
use std::process::{exit};
use self::getopts::Options;

const EX_USAGE: i32 = 64;

fn print_usage(program: &str, opts: Options) {
    let banner = format!("Usage: {} [options] command [arguments]", program);
    print!("{}", opts.usage(&banner));
}

fn handle_usage_error(program: &str, opts: Options) -> ! {
    print_usage(&program, opts);
    exit(EX_USAGE);
}

pub fn run(args: Vec<String>) {
    let mut opts = Options::new();
    let program = args[0].clone();

    opts.optflag("h", "help", "show this message");
    let matches = match opts.parse(&args[1..]) {
        Ok(m)   => m,
        Err(_)  => handle_usage_error(&program, opts)
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let command = if !matches.free.is_empty() {
        matches.free[0].clone()
    }
    else {
        handle_usage_error(&program, opts);
    };

    match command.as_ref() {
        "check"     => check::run(),
        "discover"  => match matches.free.len() {
            2 => discover::run(&matches.free[1].clone()),
            _ => handle_usage_error(&program, opts)
        },
        "help"      => print_usage(&program, opts),
        "list"      => list::run(),
        _           => handle_usage_error(&program, opts)
    }
}
