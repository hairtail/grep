use std::process;
use grep::Config;
use grep::run;

fn main() {
    let config = Config::new(std::env::args()).unwrap_or_else(|err|{
        println!("Problem fails parsing arguments, {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error, {}", e);
        process::exit(1);
    }
}
