extern crate chrono;
extern crate reqwest;

use reqwest::{get};
use chrono::{Local};
use std::string::String;
use std::result::Result;
use std::error::Error;
use std::process::Command;

// wttr.in
const CITY: &str = "Orebro";

pub fn call(out: String) {
    println!("{}", out);
    Command::new("xsetroot")
        .arg("-name")
        .arg(out)
        .output()
        .expect("something happened");
}

pub fn get_time() -> String {
    let current_time = Local::now().format("\u{e225}%A %b %Y-%m-%d %H:%M").to_string();
    current_time
}

pub fn get_weather() -> Result<String, Box<dyn Error>> {
    // wttr.in/:help
    // wttr.in/CITY?T0

    let url = format!("http://wttr.in/{}?t0", CITY);
    let mut body = String::new();
    let mut weather = String::from("\u{e01d}");
    let mut firstval = false;

    body.push_str(&get(&url)?.text()?);

    let body: Vec<&str> = body.split("<pre>").collect();
    let body: Vec<_> = body[1].split("\n").collect();

    if body.len() <= 5 {
        return Err(Box::from("ree"))
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
    Ok(weather)
}
