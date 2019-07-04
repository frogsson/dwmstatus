use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use std::thread::sleep;

#[macro_use]
extern crate serde_derive;
extern crate dirs;
extern crate reqwest;
extern crate serde_json;
extern crate toml;

mod cpu;
mod datetime;
mod mem;
mod net;
mod weather;
mod bat;

#[derive(Debug, PartialEq, Clone)]
struct Modules {
    time: Option<datetime::Time>,
    weather: Option<weather::Weather>,
    net: Option<net::Net>,
    cpu: Option<cpu::Cpu>,
    mem: Option<mem::Mem>,
    bat: Option<bat::Battery>,
}

impl Modules {
    fn init(config: Config, s: &str) -> Result<Modules, Box<dyn std::error::Error>> {
        let time = if s.contains("{datetime}") {
            Some(datetime::Time::init())
        } else {
            None
        };

        let weather = if s.contains("weather") {
            let url = config.format_url()?;
            Some(weather::Weather::init(url))
        } else {
            None
        };

        let net = if s.contains("{download}")
                  || s.contains("{upload}") {
            let interface = config.get_net_interface()?;
            Some(net::Net::init(interface))
        } else {
            None
        };

        let cpu = if s.contains("{cpu}") {
            Some(cpu::Cpu::init())
        } else {
            None
        };

        let mem = if s.contains("{memory}") {
            Some(mem::Mem::init())
        } else {
            None
        };

        let bat = if s.contains("{bat}") {
            Some(bat::Battery::init())
        } else {
            None
        };

        let m = Modules {
            time,
            weather,
            net,
            cpu,
            mem,
            bat,
        };

        Ok(m)
    }

    fn update_time(&mut self) -> String {
        match self.time {
            Some(ref mut v) => {
                v.update();
                v.output()
            },
            None => String::from("N/A"),
        }
    }

    fn update_net(&mut self) {
        if let Some(ref mut v) = self.net {
            v.update();
        }
    }

    fn net_dl(&mut self) -> String {
        match self.net {
            Some(ref mut v) => {
                match v.dl_output() {
                    Some(s) => s.to_string(),
                    None => String::from("N/A"),
                }
            },
            None => String::from("N/A"),
        }
    }

    fn net_up(&mut self) -> String {
        match self.net {
            Some(ref mut v) => {
                match v.up_output() {
                    Some(s) => s.to_string(),
                    None => String::from("N/A"),
                }
            },
            None => String::from("N/A"),
        }
    }

    fn update_weather(&mut self) -> String {
        match self.weather {
            Some(ref mut v) => {
                v.update();
                match v.output() {
                    Some(s) => s,
                    None => String::from("N/A"),
                }
            },
            None => String::from("N/A"),
        }
    }

    fn update_cpu(&mut self) -> String {
        match self.cpu {
            Some(ref mut v) => {
                v.update();
                match v.output() {
                    Some(s) => s,
                    None => String::from("N/A"),
                }
            },
            None => String::from("N/A"),
        }
    }

    fn update_mem(&mut self) -> String {
        match self.mem {
            Some(ref mut v) => {
                v.update();
                match v.output() {
                    Some(s) => s,
                    None => String::from("N/A"),
                }
            },
            None => String::from("N/A"),
        }
    }

    fn update_bat(&mut self) -> String {
        match self.bat {
            Some(ref mut v) => {
                v.update();
                match v.output() {
                    Some(s) => s,
                    None => String::from("N/A"),
                }
            },
            None => String::from("N/A"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    format: Option<String>,
    weather_apikey: Option<String>,
    weather_city: Option<String>,
    net_interface: Option<String>,
    update_interval: Option<f32>,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let path = get_config_path()?;
        let config_raw = std::fs::read_to_string(path)?;
        let config = toml::from_str(config_raw.as_str())?;
        Ok(config)
    }

    pub fn default() -> Config {
        Config {
            format: Some("{datetime}".to_string()),
            weather_apikey: None,
            weather_city: None,
            net_interface: None,
            update_interval: None,
        }
    }

    pub fn update_interval(&self) -> Duration {
        Duration::from_millis((self.update_interval.unwrap_or(1.0) * 1000.0) as u64)
    }

    fn format_url(&self) -> Result<String, &'static str> {
        let apikey = match &self.weather_apikey {
            Some(s) => s,
            None => return Err("`weather` module requires `weather_api` to be set in config.toml"),
        };

        let city = match &self.weather_city {
            Some(s) => s,
            None => return Err("`weather` module requires `weather_city` to be set in config.toml"),
        };

        Ok(format!(
            "https://api.openweathermap.org/data/2.5/weather?id={}&units=metric&appid={}",
            city, apikey
        ))
    }

    fn get_net_interface(&self) -> Result<String, &'static str> {
        match &self.net_interface {
            Some(e) => Ok(e.to_string()),
            None => Err("`net` module requires `net_interface` to be set in config.toml"),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let format = match &config.format {
        Some(v) => v.to_string(),
        None => return Err("`format` not found in config.toml".into()),
    };
    let update_interval = config.update_interval();
    let mut modules = Modules::init(config, &format)?;

    loop {
        let output = update(format.clone(), &mut modules);
        call(&output)?;
        sleep(update_interval);
    }
}

fn update(mut s: String, m: &mut Modules) -> String {
    if s.contains("{datetime}") {
        let t = m.update_time();
        s = s.replace("{datetime}", &t);
    };

    if s.contains("{weather}") {
        let t = m.update_weather();
        s = s.replace("{weather}", &t)
    };

    if s.contains("{download}") || s.contains("{upload}") {
        m.update_net();
        s = s.replace("{upload}", &m.net_up());
        s = s.replace("{download}", &m.net_dl());
    };

    if s.contains("{cpu}") {
        let t = m.update_cpu();
        s = s.replace("{cpu}", &t)
    };

    if s.contains("{memory}") {
        let t = m.update_mem();
        s = s.replace("{memory}", &t)
    };

    if s.contains("{bat}") {
        let t = m.update_bat();
        s = s.replace("{bat}", &t)
    };

    s
}

pub fn get_config_path() -> Result<PathBuf, &'static str> {
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".config/rustystatus/config.toml");
            Ok(path)
        }
        None => Err("missing home directory definition `$HOME`"),
    }
}

pub fn call(out: &str) -> Result<(), std::io::Error> {
    println!("{}", out);
    std::process::Command::new("xsetroot")
        .arg("-name")
        .arg(out)
        .output()?;
    Ok(())
}
