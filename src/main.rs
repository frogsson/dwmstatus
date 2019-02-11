extern crate rustystatus;

use std::string::String;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::fs;

/*
https://home.openweathermap.org

API KEY
$HOME/.config/rustystatus/apikey
*/

fn main() {
    let api_key = fs::read_to_string("/home/kim/.config/rustystatus/apikey").unwrap();
    let mut lastweatherupdate = SystemTime::now();
    let mut weather = match rustystatus::get_weather(&api_key) {
        Ok(s) => s,
        Err(e) => { 
            println!("{:?}", e);
            "".to_string()
        },

    };
    let five_min = Duration::from_secs(300);
    let five_sec = Duration::from_secs(5);

    loop {
        let mut output = String::new();

        if lastweatherupdate.elapsed().unwrap() >= five_min {
            weather = match rustystatus::get_weather(&api_key) {
                Ok(s) => s,
                Err(_) => "".to_string(),
            };
            lastweatherupdate = SystemTime::now();
        }

        output.push_str(&format!("{}", weather));
        output.push_str(&format!(" {}", rustystatus::get_time()));
        rustystatus::call(output);
        sleep(five_sec);
    }
}
