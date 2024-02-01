use rl::{run, Config};
use std::{env, ffi::OsString, process};

fn main() {
    let args: Vec<OsString> = env::args_os().collect();
    dbg!(args);

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let cfg = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(&cfg) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
