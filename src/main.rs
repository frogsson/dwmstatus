extern crate rustystatus;

use std::thread::sleep;
use std::time::Duration;

/*
https://home.openweathermap.org

API KEY
$HOME/.config/rustystatus/apikey
*/

fn main() {
    let mut modules = rustystatus::Modules::new();
    let five_sec = Duration::from_secs(5);

    loop {
        modules.update();

        rustystatus::call(modules.output());
        sleep(five_sec);
    }
}
