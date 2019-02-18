extern crate rustystatus;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut modules = rustystatus::Modules::new();
    let five_sec = Duration::from_secs(5);

    loop {
        modules.update();
        rustystatus::call(modules.output());
        sleep(five_sec);
    }
}
