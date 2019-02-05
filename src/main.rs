extern crate dwmstatus;

use std::string::String;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

// https://home.openweathermap.org

fn main() {
    let mut lastweatherupdate = SystemTime::now();
    let mut weather = match dwmstatus::get_weather() {
        Ok(s) => s,
        Err(_) => "".to_string()
    };
    let halfhour = Duration::from_secs(1800);

    loop {
        let mut output = String::new();

        if lastweatherupdate.elapsed().unwrap() >= halfhour {
            weather = match dwmstatus::get_weather() {
                Ok(s) => s,
                Err(_) => "".to_string(),
            };
            lastweatherupdate = SystemTime::now();
        }

        output.push_str(&format!("{}", weather));
        output.push_str(&format!(" {}", dwmstatus::get_time()));
        dwmstatus::call(output);
        sleep(Duration::from_secs(5));
    }
}
