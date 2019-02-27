extern crate rustystatus;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let url = rustystatus::format_url();
    let mut modules = rustystatus::Modules::new();
    let five_sec = Duration::from_secs(5);

    loop {
        modules.update_time();
        modules.update_weather(&url);
        modules.update_net();

        rustystatus::call(modules.output());
        sleep(five_sec);
    }
}
