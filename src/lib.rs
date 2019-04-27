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
pub enum Module {
    Time(datetime::Time),
    Weather(weather::Weather),
    Net(net::Net),
    Cpu(cpu::Cpu),
    Mem(mem::Mem),
    Bat(bat::Battery),
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
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let path = get_config_path()?;
        let config = std::fs::read_to_string(path)?;
        let config = toml::from_str(config.as_str())?;
        Ok(config)
    }

    pub fn default() -> Config {
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

    pub fn separator(&self) -> String {
        match &self.output_separator {
            Some(e) => e.to_string(),
            None => " ".to_string(),
        }
    }

    pub fn modules(&self) -> Result<Vec<Module>, Box<dyn Error>> {
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
                "time" => {
                    let time = datetime::Time::init();
                    vm.push(Module::Time(time));
                },
                "netspeed" => {
                    let interface = self.get_net_interface()?;
                    let net = net::Net::init(interface);
                    vm.push(Module::Net(net));
                },
                "cpu" => {
                    let cpu = cpu::Cpu::init();
                    vm.push(Module::Cpu(cpu));
                },
                "memory" => {
                    let mem = mem::Mem::init();
                    vm.push(Module::Mem(mem));
                },
                "weather" => {
                    let url = self.format_url()?;
                    let weather = weather::Weather::init(url);
                    vm.push(Module::Weather(weather));
                },
                "battery" => {
                    let bat = bat::Battery::init();
                    vm.push(Module::Bat(bat));
                },
                invalid_module => {
                    eprintln!("{:?} - Not a valid Module", invalid_module);
                }
            }
        }

        Ok(vm)
    }

    fn format_url(&self) -> Result<String, &'static str> {
        let apikey = match &self.weather_apikey {
            Some(s) => s,
            None => {
                return Err(
                    "`weather` module requires `weather_api` to be set in config.toml",
                )
            }
        };

        let city = match &self.weather_city {
            Some(s) => s,
            None => {
                return Err(
                    "`weather` module requires `weather_city` to be set in config.toml",
                )
            }
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
    let mut modules = config.modules()?;
    let update_interval = config.update_interval();
    let separator = config.separator();
    let len = modules.len() - 1;
    std::mem::drop(config);

    loop {
        let output: String = modules
            .iter_mut()
            .enumerate()
            .map(|module| update_and_output(module.1, &separator, last_item(module.0, len)))
            .collect();

        call(&output);
        sleep(update_interval);
    }
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
        Module::Bat(ref mut m) => {
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
