extern crate chrono;
extern crate reqwest;
extern crate serde_json;

use chrono::Local;
use std::string::String;
use std::result::Result;
use std::error::Error;
use std::process::Command;

// wttr.in
// api.openweathermap.org/data/2.5/weather?id=217279
// https://api.openweathermap.org/data/2.5/weather?q=London,uk&appid=YOUR_API_KEY
// 2686657

const CITY_ID: &str = "2686657";

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

pub fn get_weather(api_key: &str) -> Result<String, Box<dyn Error>> {
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
    let json_str = &reqwest::get(url)?.text()?; // reqwest.get()?.json()? is not able to parse for some reason 
    println!("{}", json_str);
    let vals: serde_json::Value = serde_json::from_str(json_str)?;
    let degrees_cel = &vals["main"]["temp"];
    let weather_status = &vals["weather"][0]["description"].to_string();
    let weather_status = weather_status.trim_matches('"');

    weather.push_str(&format!("{} {}°C", weather_status, degrees_cel));

    Ok(weather)
}


// pub fn get_weather() -> Result<String, Box<dyn Error>> {
//     // wttr.in/:help
//     // wttr.in/CITY?T0
// 
//     let url = format!("http://wttr.in/{}?t0", CITY);
//     let mut body = String::new();
//     let mut weather = String::from("\u{e01d}");
//     let mut firstval = false;
// 
//     body.push_str(&get(&url)?.text()?);
// 
//     let body: Vec<&str> = body.split("<pre>").collect();
//     let body: Vec<_> = body[1].split("\n").collect();
// 
//     if body.len() <= 5 {
//         return Err(Box::from("ree"))
//     }
// 
//     let mut weathervec: Vec<&str> = body[3].split(">").collect();
//     let current_weather = weathervec.pop().unwrap().trim();
//     weather.push_str(current_weather);
// 
//     for val in body[4].split("<span") {
//         let celvec = val.split(">").collect::<Vec<&str>>();
// 
//         if celvec.len() >= 2 {
//             let cel = celvec[1].split("<")
//                 .collect::<Vec<&str>>()[0]
//                 .to_string().parse::<i8>();
//             match cel {
//                 Ok(celu) => { 
//                     if !firstval {
//                         weather.push_str(&format!(" {}°C", celu));
//                         firstval = true;
//                     } else {
//                         weather.push_str(&format!(" to {}°C", celu));
//                         break
//                     }
//                 },
//                 _ => (),
//             };
//         }
//     }
//     Ok(weather)
// }
