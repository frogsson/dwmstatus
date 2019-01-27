extern crate chrono;
extern crate reqwest;

use chrono::{Local};
use reqwest::{get};
use std::process::Command;
use std::string::String;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

const CITY: &str = "Orebro";

fn main() {
    let mut output = String::new();
    let mut lastweatherupdate = SystemTime::now();
    let halfhour = Duration::from_secs(1800);

    let mut weather = String::from(get_weather());

    loop {
        output.clear();

        if lastweatherupdate.elapsed().unwrap() >= halfhour {
            weather = get_weather();
            lastweatherupdate = SystemTime::now();
        }

        output.push_str(&format!("{}", weather));
        output.push_str(&format!(" {}", get_time()));
        call(&output);
        sleep(Duration::from_secs(5));
    }
}

fn call(out: &str) {
    Command::new("xsetroot")
        .arg("-name")
        .arg(out)
        .output()
        .expect("something happened");
}

fn get_time() -> String {
    let current_time = Local::now().format("\u{e225}%A %b %Y-%m-%d %H:%M").to_string();
    current_time
}

fn get_weather() -> String {
    // wttr.in/:help
    // wttr.in/CITY?T0

    let url = format!("http://wttr.in/{}?t0", CITY);
    let mut body = String::new();

    match get(&url) {
        Ok(mut b) => body.push_str(&b.text().expect("error pushing body")),
        Err(_) => return "".to_string(),
    };

    let body: Vec<&str> = body.split("<pre>").collect();

    let mut weather = String::from("\u{e01d}");
    let mut firstval = false;

    let body: Vec<_> = body[1].split("\n").collect();

    if body.len() <= 5 {
        return "".to_string()
    }

    let mut weathervec: Vec<&str> = body[3].split(">").collect();
    let current_weather = weathervec.pop().unwrap().trim();
    weather.push_str(current_weather);

    for val in body[4].split("<span") {
        let celvec = val.split(">").collect::<Vec<&str>>();

        if celvec.len() >= 2 {
            let cel = celvec[1].split("<")
                .collect::<Vec<&str>>()[0]
                .to_string().parse::<i8>();
            match cel {
                Ok(celu) => { 
                    if !firstval {
                        weather.push_str(&format!(" {}°C", celu));
                        firstval = true;
                    } else {
                        weather.push_str(&format!(" to {}°C", celu));
                        break
                    }
                },
                _ => (),
            };
        }
    }
    weather
}
