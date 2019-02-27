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
    net_vals: Net,
    net_print: String,
}

#[derive(Debug)]
pub struct Net {
    recv: f64,
    tran: f64,
    recv_stack: Vec<f64>,
    tran_stack: Vec<f64>,
    net_time: SystemTime,
    last_time: u64,
}

impl Modules {
    pub fn output(&self) -> String {
        format!("{} {} {}", self.net_print, self.weather, self.time)
    }

    pub fn new() -> Modules {
        let net = Net {
            recv: 0.0,
            tran: 0.0,
            recv_stack: vec![0.0, 0.0, 0.0],
            tran_stack: vec![0.0, 0.0, 0.0],
            net_time: SystemTime::now(),
            last_time: 5000000,
        };

        Modules {
            weather: String::new(),
            time: String::new(),
            last_update: None,
            five_min: Duration::from_secs(300),
            net_vals: net,
            net_print: String::new(),
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

    pub fn update_net(&mut self) {
        let mut n = parse_net_proc();

        let new_time = match self.net_vals.net_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        let half = (new_time - self.net_vals.last_time) * 1000000;
        self.net_vals.last_time = new_time;
        self.net_vals.net_time = SystemTime::now();

        let x = n.remove(0); 
        self.net_vals.recv_stack.remove(0);
        self.net_vals.recv_stack.push((x - self.net_vals.recv) / half as f64);
        self.net_vals.recv = x;
        let recv_sum: f64 = self.net_vals.recv_stack.iter().sum();
        let recv_len: f64 = self.net_vals.recv_stack.len() as f64;
        let recv = recv_sum / recv_len;

        let y = n.remove(7); 
        self.net_vals.tran_stack.remove(0);
        self.net_vals.tran_stack.push((y - self.net_vals.tran) / half as f64);
        self.net_vals.tran = y;
        let tran_sum: f64 = self.net_vals.tran_stack.iter().sum();
        let tran_len: f64 = self.net_vals.tran_stack.len() as f64;
        let tran = tran_sum / tran_len;

        println!("{:?}", self.net_vals);

        self.net_print = format!("\u{e061}{:.2} MB/s \u{e060}{:.2} MB/s", recv, tran);
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

    let json: serde_json::Value = reqwest::get(u)?.json()?;

    let mut degrees_cel: Option<i8> = None;
    if let Some(s) = json.pointer("/main/temp") {
        if let Some(val) = s.as_f64() {
            degrees_cel = Some(val.round() as i8);
        }     
    }

    let mut weather: Option<String> = None;
    if let Some(s) = json.pointer("/weather/0/description") {
        if let Some(val) = s.as_str() {
            let mut x = val.trim_matches('"').chars();
            if let Some(f) = x.next() {
                weather = Some(f.to_uppercase().collect::<String>() + x.as_str());
            }
        }
    }

    let mut weather_str = String::new();
    if let (Some(x), Some(y)) = (weather, degrees_cel) {
        weather_str.push_str(&format!("\u{e01d}{} {}°C", x, y));
    }

    Ok(weather_str)
}

pub fn parse_net_proc() -> Vec<f64> {
    let net_info = match std::fs::read_to_string("/proc/net/dev") {
        Ok(s) => s,
        _ => "".to_string(),
    };

    let mut n = String::new();
    for x in net_info.split("\n") {
        if x.contains("eno1") {
            n = x.to_string();
            break
        }
    }

    let mut vals: Vec<f64> = Vec::new();
    for x in n.trim().split_whitespace() {
        match x.parse::<f64>() {
            Ok(i) => vals.push(i),
            Err(_) => (),
        }
    }

    vals
}
