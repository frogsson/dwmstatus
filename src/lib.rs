extern crate chrono;
extern crate reqwest;
extern crate serde_json;

use chrono::Local;
use std::string::String;
use std::result::Result;
use std::process::Command;

// api.openweathermap.org/data/2.5/weather?id=217279
// https://api.openweathermap.org/data/2.5/weather?q=London,uk&appid=YOUR_API_KEY
// Orebro = 2686657

const CITY_ID: &str = "2686657";

pub fn call(out: String) {
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

pub fn get_weather(api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
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

    let mut weather = String::from("\u{e01d}");

    let url = &format!("https://api.openweathermap.org/data/2.5/weather?id={}&units=metric&appid={}", CITY_ID, api_key);
    // reqwest.get()?.json()? is not able to parse for some reason 
    let json_str = &reqwest::get(url)?.text()?; 
    let vals: serde_json::Value = serde_json::from_str(json_str)?;

    let degrees_cel = &vals["main"]["temp"];

    let x = &vals["weather"][0]["description"].to_string();
    let mut x = x.trim_matches('"').chars();
    let weather_description = match x.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + x.as_str(),
    };

    weather.push_str(&format!("{} {}°C", weather_description, degrees_cel));

    Ok(weather)
}

