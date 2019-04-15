// extern crate serde;
extern crate dirs;
extern crate reqwest;
extern crate serde_json;
extern crate toml;

mod cpu;
mod datetime;
mod mem;
mod net;
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
    pub fn init(cfg: toml::Value, order: &[ModuleName]) -> Modules {
        let url = if order.contains(&ModuleName::Weather) {
            let apikey = cfg
                .get("weather_apikey")
                .and_then(toml::Value::as_str)
                .unwrap_or_else(|| {
                    eprintln!(
                        "Error: `weather` module requires `weather_api` to be set in config.toml"
                    );
                    std::process::exit(0x0100);
                });

            let city = cfg
                .get("weather_city")
                .and_then(toml::Value::as_str)
                .unwrap_or_else(|| {
                    eprintln!(
                        "Error: `weather` module requires `weather_city` to be set in config.toml"
                    );
                    std::process::exit(0x0100);
                });

            format_url(city, apikey)
        } else {
            String::new()
        };

        let net_interface = if order.contains(&ModuleName::Net) {
            cfg.get("net_interface")
                .and_then(toml::Value::as_str)
                .unwrap_or_else(|| {
                    eprintln!(
                        "Error: `net` module requires `net_interface` to be set in config.toml"
                    );
                    std::process::exit(0x0100);
                })
                .to_string()
        } else {
            String::new()
        };

        Modules {
            time: datetime::Time::init(),
            net: net::Net::init(net_interface),
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
        }
        None => {
            eprintln!("Error: Missing home directory definition `$HOME`");
            std::process::exit(0x0100);
        }
    };

    let config_str = match std::fs::read_to_string(&config_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: `{}` {}", config_path.to_str().unwrap_or("$HOME"), e);
            std::process::exit(0x0100);
        }
    };

    let config: toml::Value = match toml::from_str(&config_str) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: hello {}", e);
            std::process::exit(0x0100);
        }
    };

    config
}

fn format_url(city: &str, apikey: &str) -> String {
    format!(
        "https://api.openweathermap.org/data/2.5/weather?id={}&units=metric&appid={}",
        city, apikey
    )
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
