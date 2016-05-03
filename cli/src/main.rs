extern crate getopts;

use getopts::Options;
use std::{env, process};
use std::io::{Write, stderr};

fn print_version() {
    println!("0.1.0");
}

fn print_usage(opt: Options) {
    let usage = "db";
    println!("{}", opt.usage(&usage));
}

fn write_stderr(err: &str) {
    stderr().write_fmt(format_args!("Error: {}\n", err)).unwrap();
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();

    opts.optopt("h", "host", "hostname of server, Default: localhost", "HOSTNAME");
    opts.optopt("p", "port", "port of server, Default: ", "PORT");

    opts.optflag("h", "help", "print the help menu");
    opts.optflag("v", "version", "print current version number");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            write_stderr(&format!("This error should not be happening. Please submit an issue \
                                   on github :)\n\"{}\"",
                                  f));
            process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        process::exit(0);
    }

    if matches.opt_present("v") {
        print_version();
        process::exit(0);
    }
}
