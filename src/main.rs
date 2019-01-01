extern crate chrono;
extern crate reqwest;

use chrono::{Local};
use reqwest::{get};
use std::process::Command;
use std::string::String;
use std::thread::{sleep};
use std::time::{Duration, SystemTime};

fn main() {
    let update_weather = Duration::from_secs(3600);
    let mut last_weather_update = SystemTime::now();
    let mut weather = String::from(get_weather());

    loop {
        let mut output = String::new();

        if last_weather_update.elapsed().unwrap() >= update_weather {
            weather = get_weather();
            last_weather_update = SystemTime::now()
        }

        output.push_str(&weather);
        output.push_str(&format!(" {}", get_timedate()));
        call(output);
        sleep(Duration::from_secs(60));
    }
}

fn call(out: String) {
    Command::new("xsetroot")
        .arg("-name")
        .arg(out)
        .output()
        .expect("something happened");
}

fn get_timedate() -> String {
    let current_time = Local::now().format("%A %b %Y-%m-%d %H:%M").to_string();
    let out = String::from(
        format!("\u{e225}{}", current_time));

    out
}

fn get_weather() -> String {
    // wttr.in/:help
    // wttr.in/CITY?T0

    let mut req = get("http://wttr.in/Orebro?t0").expect("404?");
    let mut body = String::new();

    if req.status().as_u16() == 200 {
        body.push_str(&req.text().expect("error >.<"));
    } else {
        return "".to_string()
    }

    let presection: Vec<&str> = body.split("<pre>").collect();
    let mut weather = String::from("\u{e01d}");
    let mut firstval = false;

    for (num, line) in presection[1].split("\n").enumerate() {
        if num == 3 {
            let mut weathervec: Vec<&str> = line.split(">").collect();
            let current_weather = weathervec.pop().unwrap().trim();
            weather.push_str(current_weather);
        }

        if num == 4 {
            for val in line.split("<span") {
                let celvec = val.split(">").collect::<Vec<&str>>();

                if celvec.len() >= 2 {
                    let cel = celvec[1].split("<")
                        .collect::<Vec<&str>>()[0]
                        .to_string().parse::<u32>();
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
        }
    }
    weather
}
