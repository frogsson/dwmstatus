extern crate rustystatus;

use rustystatus::Config;
use rustystatus::{run, call};

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::thread::sleep(std::time::Duration::from_secs(5));
        Config::default()
    });

    if let Err(e) = run(config) {
        eprintln!("Error: {}", e);
        call("");
        std::process::exit(1);
    };

}
