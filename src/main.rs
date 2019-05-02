extern crate rustystatus;

use rustystatus::Config;
use rustystatus::{run, call};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        sleep(Duration::from_secs(5));
        Config::default()
    });

    if let Err(e) = run(config) {
        eprintln!("Error: {}", e);

        // reset xsetroot
        if let Err(e) = call("") {
            eprintln!("{:?}", e);
        }

        std::process::exit(1);
    };
}
