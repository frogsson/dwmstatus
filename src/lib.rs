use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

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

#[derive(Debug, PartialEq, Clone)]
pub enum Module {
    Time(datetime::Time),
    Weather(weather::Weather),
    Net(net::Net),
    Cpu(cpu::Cpu),
    Mem(mem::Mem),
}

#[derive(Deserialize, Debug)]
pub struct Config {
    output_separator: Option<String>,
    output_order: Option<Vec<String>>,
    weather_apikey: Option<String>,
    weather_city: Option<String>,
    net_interface: Option<String>,
    update_interval: Option<f32>,
}

impl Config {
    pub fn from(path: PathBuf) -> Result<Config, Box<dyn Error>> {
        let config = std::fs::read_to_string(path)?;
        let config = toml::from_str(config.as_str())?;
        Ok(config)
    }

    fn default() -> Config {
        Config {
            output_separator: Some(" ".to_string()),
            output_order: Some(vec!["time".to_string()]),
            weather_apikey: None,
            weather_city: None,
            net_interface: None,
            update_interval: None,
        }
    }

    pub fn update_interval(&self) -> Duration {
        Duration::from_millis((self.update_interval.unwrap_or(1.0) * 1000.0) as u64)
    }

    pub fn separator(&self) -> &str {
        match &self.output_separator {
            Some(e) => e,
            None => " ",
        }
    }

    pub fn order(&self) -> Result<Vec<Module>, Box<dyn Error>> {
        let v = match &self.output_order {
            Some(e) => e,
            None => {
                eprintln!("Could not parse output_order");
                return Ok(vec![Module::Time(datetime::Time::init())]);
            }
        };

        let mut vm = Vec::new();
        for module in v.iter() {
            match module.as_str() {
                "time" => vm.push(Module::Time(datetime::Time::init())),
                "netspeed" => vm.push(Module::Net(net::Net::init(Config::get_net_interface(
                    &self.net_interface,
                )?))),
                "cpu" => vm.push(Module::Cpu(cpu::Cpu::init())),
                "memory" => vm.push(Module::Mem(mem::Mem::init())),
                "weather" => vm.push(Module::Weather(weather::Weather::init(Config::format_url(
                    &self.weather_apikey,
                    &self.weather_city,
                )?))),
                invalid_module => {
                    eprintln!("{:?} - Not a valid Module", invalid_module);
                }
            }
        }

        Ok(vm)
    }

    fn format_url(apikey: &Option<String>, city: &Option<String>) -> Result<String, &'static str> {
        let apikey = match apikey {
            Some(s) => s,
            None => {
                return Err(
                    "Error: `weather` module requires `weather_api` to be set in config.toml",
                )
            }
        };

        let city = match city {
            Some(s) => s,
            None => {
                return Err(
                    "Error: `weather` module requires `weather_city` to be set in config.toml",
                )
            }
        };

        Ok(format!(
            "https://api.openweathermap.org/data/2.5/weather?id={}&units=metric&appid={}",
            city, apikey
        ))
    }

    fn get_net_interface(interface: &Option<String>) -> Result<String, &'static str> {
        match interface {
            Some(e) => Ok(e.to_string()),
            None => Err("Error: `net` module requires `net_interface` to be set in config.toml"),
        }
    }
}

pub fn get_config_path() -> Result<PathBuf, &'static str> {
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".config/rustystatus/config.toml");
            Ok(path)
        }
        None => Err("Error: Missing home directory definition `$HOME`"),
    }
}

pub trait Unwrap {
    fn unwrap_or_default(self) -> Config;
}

impl Unwrap for Result<Config, Box<dyn Error>> {
    fn unwrap_or_default(self) -> Config {
        match self {
            Ok(c) => c,
            Err(_) => Config::default(),
        }
    }
}

pub fn call(out: &str) {
    println!("{}", out);
    std::process::Command::new("xsetroot")
        .arg("-name")
        .arg(out)
        .output()
        .expect("something happened");
}

pub fn update_and_output(module: &mut Module, sep: &str, last_m: bool) -> String {
    let mut output: String;
    match module {
        Module::Time(ref mut m) => {
            m.update();
            output = m.output();
        }
        Module::Weather(ref mut m) => {
            m.update();
            output = m.output();
        }
        Module::Net(ref mut m) => {
            m.update();
            output = m.output();
        }
        Module::Cpu(ref mut m) => {
            m.update();
            output = m.output();
        }
        Module::Mem(ref mut m) => {
            m.update();
            output = m.output();
        }
    }

    if !last_m {
        output.push_str(sep);
    }

    output
}

pub fn last_item(num: usize, len: usize) -> bool {
    num >= len
}
