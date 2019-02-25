extern crate chrono;
extern crate reqwest;
extern crate serde_json;
extern crate dirs;

use std::string::String;
use std::result::Result;
use std::time::{Duration, SystemTime};

/*
https://home.openweathermap.org
https://api.openweathermap.org/data/2.5/weather?q={CITY_ID}&appid={API_KEY}

API KEY
$HOME/.config/rustystatus/apikey

add crate failure
https://rust-lang-nursery.github.io/cli-wg/tutorial/errors.html
*/

pub struct Modules {
    weather: String,
    last_update: Option<SystemTime>,
    time: String,
    five_min: Duration,
}

impl Modules {
    pub fn output(&self) -> String {
        format!("{} {}", self.weather, self.time)
    }

    pub fn new() -> Modules {
        Modules {
            weather: String::new(),
            time: String::new(),
            last_update: None,
            five_min: Duration::from_secs(300),
        }
    }

    pub fn update_time(&mut self) {
        self.time = get_time();
    }

    pub fn update_weather(&mut self, u: &String) {
        if let Some(e) = self.last_update {
            match e.elapsed() {
                Ok(s) => {
                    if s >= self.five_min {
                        self.weather = get_weather(u);
                        self.last_update = Some(SystemTime::now());
                    }
                },
                Err(_) => self.last_update = None,
            }
        } else {
            self.last_update = Some(SystemTime::now());
            self.weather = get_weather(u);
        }
    }
}

pub fn call(out: String) {
    println!("{}", out);
    std::process::Command::new("xsetroot")
        .arg("-name")
        .arg(out)
        .output()
        .expect("something happened");
}

fn get_time() -> String {
    chrono::Local::now()
        .format("\u{e225}%A %b %Y-%m-%d %H:%M")
        .to_string()
}

pub fn format_url() -> String {
    let apikey_path = match dirs::home_dir() {
        Some(mut path) => {
            path.push(".config/rustystatus/apikey");
            path
        },
        None => {
            eprintln!("Error: Missing home directory definition `$HOME`");
            std::process::exit(0x0100);
        },
    };

    let apikey = match std::fs::read_to_string(&apikey_path) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: `{}` {}", apikey_path.to_str().unwrap(), e);
            std::process::exit(0x0100);
        },
    };

    format!("https://api.openweathermap.org/data/2.5/weather?id=2686657&units=metric&appid={}", apikey)
}

fn get_weather(u: &String) -> String {
    let weather = match _get_weather(u) {
        Ok(s) => s,
        Err(_) => "".to_string(),
    };
    weather
}

fn _get_weather(u: &String) -> Result<String, Box<dyn std::error::Error>> {
    /* JSON_STR FORMAT
    {
        "base":"stations",
        "clouds":{"all":75},
        "cod":200,
        "coord":{"lat":59.27,"lon":15.21},
        "dt":1549862400,
        "id":2686657,
        "main":{"humidity":96,"pressure":992,"temp":274.15,"temp_max":274.15,"temp_min":274.15},
        "name":"Orebro",
        "sys":{"country":"SE","id":1777,"message":0.0036,"sunrise":1549867523,"sunset":1549899741,"type":1},
        "visibility":6000,
        "weather":[{"description":"mist","icon":"50n","id":701,"main":"Mist"}],
        "wind":{"deg":320,"speed":1.5}
    }
    */

    let mut j: serde_json::Value = reqwest::get(u)?.json()?;

    let degrees_cel = j.pointer("/main/temp")
        .unwrap()
        .as_i64()
        .unwrap();

    let mut x = j.pointer_mut("/weather/0/description")
        .unwrap()
        .as_str()
        .unwrap()
        .trim_matches('"')
        .chars();

    let weather_description = match x.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + x.as_str(),
    };

    Ok(format!("\u{e01d}{} {}°C", weather_description, degrees_cel))
}
