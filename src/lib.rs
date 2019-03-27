// extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate reqwest;
extern crate dirs;

mod datetime;
mod net;
mod cpu;
mod mem;
mod weather;

#[derive(Debug, PartialEq)]
pub enum ModuleName {
    Time,
    Weather,
    Net,
    Cpu,
    Mem,
}

#[derive(Debug)]
pub struct Modules {
    pub time: datetime::Time,
    pub net: net::Net,
    pub cpu: cpu::Cpu,
    pub mem: mem::Mem,
    pub weather: weather::Weather,
}

impl Modules {
    pub fn init(cfg: toml::Value, contains_weather: bool) -> Modules {
        let url = if contains_weather {
            let apikey = match cfg["weather_apikey"].as_str() {
                Some(s) => s,
                None => {
                    eprintln!("weather_apikey not found for weather module in output");
                    std::process::exit(0x0100);
                }
            };

            let city = match cfg["weather_city"].as_str() {
                Some(s) => s,
                None => {
                    eprintln!("weather_city not found for weather module in output");
                    std::process::exit(0x0100);
                }
            };

            format_url(city, apikey)
        } else {
            String::new()
        };

        Modules {
            time: datetime::Time::init(),
            net: net::Net::init(),
            cpu: cpu::Cpu::init(),
            mem: mem::Mem::init(),
            weather: weather::Weather::init(url),
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

pub fn read_config() -> toml::Value {
    let config_path = match dirs::home_dir() {
        Some(mut path) => {
            path.push(".config/rustystatus/config.toml");
            path
        },
        None => {
            eprintln!("Error: Missing home directory definition `$HOME`");
            std::process::exit(0x0100);
        },
    };

    let config_str = match std::fs::read_to_string(&config_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: `{}` {}", config_path.to_str().unwrap_or("$HOME"), e);
            std::process::exit(0x0100);
        },
    };

    let config: toml::Value = toml::from_str(&config_str).expect("HELLO");

    config
}

fn format_url(city: &str, apikey: &str) -> String {
    format!("https://api.openweathermap.org/data/2.5/weather?id={}&units=metric&appid={}", city, apikey)
}

pub fn parse_output_order(m_order: Option<&Vec<toml::Value>>) -> Vec<ModuleName> {
    let mut order = Vec::new();
    if let Some(ord) = m_order {
        for m in ord {
            match m.as_str() {
                Some("time") => order.push(ModuleName::Time),
                Some("netspeed") => order.push(ModuleName::Net),
                Some("cpu") => order.push(ModuleName::Cpu),
                Some("memory") => order.push(ModuleName::Mem),
                Some("weather") => order.push(ModuleName::Weather),
                Some(e) => eprintln!("{} - Not a Module", e),
                _ => (),
            }
        }
    }

    order
}
