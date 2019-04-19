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
enum ModuleName {
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

pub fn parse_output_order(output_order: Option<&toml::Value>) -> Vec<ModuleName> {
    output_order
        .and_then(toml::value::Value::as_array)
        .expect("Error: parsing `output_order` in config.toml")
        .iter()
        .filter_map(|module| match_order_input(module))
        .collect()
}

fn match_order_input(m: &toml::Value) -> Option<ModuleName> {
    match m.as_str() {
        Some("time") => Some(ModuleName::Time),
        Some("netspeed") => Some(ModuleName::Net),
        Some("cpu") => Some(ModuleName::Cpu),
        Some("memory") => Some(ModuleName::Mem),
        Some("weather") => Some(ModuleName::Weather),
        Some(e) => { eprintln!("{:?} - Not a valid Module", e); None },
        _ => None,
    }
}

pub fn match_module(m: &ModuleName, modules: &mut Modules) -> String {
    match m {
        ModuleName::Time => modules.time.update().output(),
        ModuleName::Net => modules.net.update().output(),
        ModuleName::Cpu => modules.cpu.update().output(),
        ModuleName::Mem => modules.mem.update().output(),
        ModuleName::Weather => modules.weather.update().output(),
    }
}
