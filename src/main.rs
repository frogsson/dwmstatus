extern crate rustystatus;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let url = rustystatus::format_url();
    let mut modules = rustystatus::Modules::default();
    let one_sec = Duration::from_secs(1);

    loop {
        modules.update_time();
        modules.update_weather(&url);
        modules.update_net();
        modules.update_cpu();
        modules.update_memory();

        rustystatus::call(modules.output());
        sleep(one_sec);
    }
}
