use std::env;
use std::process;

use mini_grep::{run, Config};

fn main() {
    // let args: Vec<_> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1)
    });
    // println!("{:?}",config);

    if let Err(e) = run(config) {
        eprintln!("Application error:{e}");
        process::exit(1)
    };
}
