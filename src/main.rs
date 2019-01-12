extern crate chrono;
extern crate reqwest;

use chrono::{Local};
use reqwest::{get};
use std::process::Command;
use std::string::String;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

const CITY: &str = "Orebro";

fn main() {
    let time = Arc::new(Mutex::new(String::new()));
    let weather = Arc::new(Mutex::new(String::new()));
    let mut output = String::new();

    let timecpy = time.clone();
    thread::spawn(move || {
        get_time(timecpy);
    });

    let weathercpy = weather.clone();
    thread::spawn(move || {
        get_weather(weathercpy);
    });

    // give threads some initial time
    thread::sleep(Duration::from_secs(2));
    loop {
        {
            let t = time.lock().unwrap();
            let w = weather.lock().unwrap();
            output.clear();
            output.push_str(&w);
            output.push_str(&format!(" {}", t));
        }

        call(&output);
        thread::sleep(Duration::from_secs(60));
    }
}

fn call(out: &str) {
    Command::new("xsetroot")
        .arg("-name")
        .arg(out)
        .output()
        .expect("something happened");
}

fn get_time(time: Arc<Mutex<String>>) {
    loop {
        {
            let current_time = Local::now().format("%A %b %Y-%m-%d %H:%M").to_string();
            let mut time = time.lock().unwrap();
            time.clear();
            time.push_str(&format!("\u{e225}{}", current_time));
        }
        thread::sleep(Duration::from_secs(60));
    }
}

fn get_weather(wttr: Arc<Mutex<String>>) {
    // wttr.in/:help
    // wttr.in/CITY?T0

    loop {
        {
            let url = format!("http://wttr.in/{}?t0", CITY);
            let mut req = get(&url).expect("404?");
            let mut body = String::new();

            if req.status().as_u16() == 200 {
                body.push_str(&req.text().expect("error >.<"));
            } else {
                continue;
            }

            let body: Vec<&str> = body.split("<pre>").collect();

            let mut weather = String::from("\u{e01d}");
            let mut firstval = false;

            let body: Vec<_> = body[1].split("\n").collect();

            if body.len() <= 5 {
                // return "".to_string()
                continue;
            }

            let mut weathervec: Vec<&str> = body[3].split(">").collect();
            let current_weather = weathervec.pop().unwrap().trim();
            weather.push_str(current_weather);

            for val in body[4].split("<span") {
                let celvec = val.split(">").collect::<Vec<&str>>();

                if celvec.len() >= 2 {
                    // println!("{}", celvec[1].split("<").collect::<Vec<&str>>()[0]);
                    let cel = celvec[1].split("<")
                        .collect::<Vec<&str>>()[0]
                        .to_string().parse::<i8>();
                    // println!("{:?}", cel);
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
            let mut wttr = wttr.lock().unwrap();
            wttr.clear();
            wttr.push_str(&weather);
        }
        thread::sleep(Duration::from_secs(1800));
    }
}
