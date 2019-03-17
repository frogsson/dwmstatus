extern crate chrono;
extern crate reqwest;
extern crate serde_json;
extern crate dirs;

use std::string::String;
use std::result::Result;
use std::time::{Duration, Instant};

/*
https://home.openweathermap.org
https://api.openweathermap.org/data/2.5/weather?q={CITY_ID}&appid={API_KEY}

API KEY
$HOME/.config/rustystatus/apikey

add crate failure
https://rust-lang-nursery.github.io/cli-wg/tutorial/errors.html
*/

pub struct Modules {
    time: String,
    memory: String,
    weather: Weather,
    net: Net,
    cpu: Cpu,
    last_update: Instant,
}

struct Weather {
    url: String,
    output: String,
    five_min: Duration,
}

struct Net {
    output: String,
    recv: f64,
    tran: f64,
    recv_stack: Vec<f64>,
    tran_stack: Vec<f64>,
    net_time: Instant,
}

struct Cpu {
    output: String,
    system: i32,
    last_sum: i32,
}

impl Modules {
    pub fn default() -> Self {
        let n = Net {
            output: String::new(),
            recv: 0.0,
            tran: 0.0,
            recv_stack: vec![0.0, 0.0, 0.0],
            tran_stack: vec![0.0, 0.0, 0.0],
            net_time: Instant::now(),
        };

        let c = Cpu {
            output: String::new(),
            system: 0,
            last_sum: 0,
        };

        let u = format_url();

        let w = Weather {
            url: u.clone(),
            output: get_weather(&u).unwrap_or_default(),
            five_min: Duration::from_secs(300),
        };

        Self {
            weather: w,
            memory: String::new(),
            time: String::new(),
            last_update: Instant::now(),
            net: n,
            cpu: c,
        }
    }

    pub fn output(&self) -> String {
        format!("{} {} {} {} {}", self.net.output, self.memory, self.cpu.output, self.weather.output, self.time)
    }

    pub fn update_time(&mut self) {
        self.time = get_time();
    }

    pub fn update_weather(&mut self) {
        if self.last_update.elapsed() >= self.weather.five_min {
            self.weather.output = get_weather(&self.weather.url).unwrap_or_default();
            self.last_update = Instant::now();
        }
    }

    pub fn update_net(&mut self) {
        if let Some(n) = read_net_proc() {
            let seconds_passed = self.net.net_time.elapsed().as_secs() * 1_000_000;

            let x = n[0]; 
            self.net.recv_stack.remove(0);
            self.net.recv_stack.push((x - self.net.recv) / seconds_passed as f64);
            self.net.recv = x;
            let recv = transfer_speed_as_mb(&self.net.recv_stack);

            let y = n[8]; 
            self.net.tran_stack.remove(0);
            self.net.tran_stack.push((y - self.net.tran) / seconds_passed as f64);
            self.net.tran = y;
            let tran = transfer_speed_as_mb(&self.net.tran_stack);

            self.net.output = format!("\u{e061}{:.2} MB/s \u{e060}{:.2} MB/s", recv, tran);
            self.net.net_time = Instant::now();
        } else {
            self.net.output = "".to_string();
        }
    }

    pub fn update_cpu(&mut self) {
        //      user    nice   system  idle      iowait irq   softirq  steal  guest  guest_nice
        // cpu  74608   2520   24433   1117073   6176   4054  0        0      0      0

        // explanation for this shit
        // https://www.idnt.net/en-GB/kb/941772
        if let Some(cpu) = read_cpu_proc() {
            let cpu_sum: i32 = cpu.iter().sum();

            let s = cpu[3];

            let cpu_delta = cpu_sum - self.cpu.last_sum;
            let cpu_idle = s - self.cpu.system;
            let cpu_used = cpu_delta - cpu_idle;
            let cpu_usage = 100 * cpu_used / cpu_delta;

            self.cpu.system = s;
            self.cpu.last_sum = cpu_sum;

            self.cpu.output = format!("\u{e223}{:02}%", cpu_usage);
        } else {
            self.cpu.output = "".to_string();
        }
    }

    pub fn update_memory(&mut self) {
        self.memory = read_memory_proc().unwrap_or_default();
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
            eprintln!("Error: `{}` {}", apikey_path.to_str().unwrap_or("$HOME"), e);
            std::process::exit(0x0100);
        },
    };

    format!("https://api.openweathermap.org/data/2.5/weather?id=2686657&units=metric&appid={}", apikey)
}

fn fetch_json(u: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    Ok(reqwest::get(u)?.json()?)
}

fn get_weather(u: &str) -> Option<String> {
    /* JSON FORMAT
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

    let json: serde_json::Value = match fetch_json(u) {
        Ok(j) => j,
        Err(e) => {
            println!("{}", e);
            return None
        },
    };

    let degrees_cel: Option<i8> = json.pointer("/main/temp")
        .and_then(|n| n.as_f64().and_then(|f| Some(f.round() as i8)));

    let weather: Option<String> = json.pointer("/weather/0/description")
        .and_then(|s| s.as_str().and_then(|ss| {
            let mut x = ss.trim_matches('"').chars();
            x.next().and_then(|f| Some(f.to_uppercase().collect::<String>() + x.as_str()))
        }));

    let mut weather_str = String::new();
    if let (Some(x), Some(y)) = (weather, degrees_cel) {
        weather_str.push_str(&format!("\u{e01d}{} {}°C", x, y));
    }

    Some(weather_str)
}

pub fn read_net_proc() -> Option<Vec<f64>> {
    let net_info = match std::fs::read_to_string("/proc/net/dev") {
        Ok(s) => s,
        Err(e) => {
            println!("`/proc/net/dev` {}", e);
            return None
        }
    };

    let vals: Vec<_> = net_info.split('\n')
        .filter(|s| s.contains("eno1"))
        .collect::<String>()
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();

    if vals.is_empty() { return None }

    Some(vals)
}

fn transfer_speed_as_mb(v: &[f64]) -> f64 {
    let sum: f64 = v.iter().sum();
    let len: f64 = v.len() as f64;
    sum / len
}

fn read_cpu_proc() -> Option<Vec<i32>> {
    let cpu_proc = match std::fs::read_to_string("/proc/stat") {
        Ok(s) => s,
        Err(e) => {
            println!("`/proc/stat` {}", e);
            return None
        }
    };

    let cpu = cpu_proc.split('\n')
        .collect::<Vec<_>>()[0]
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    Some(cpu)
}

fn read_memory_proc() -> Option<String> {
    let cpu_proc = match std::fs::read_to_string("/proc/meminfo") {
        Ok(s) => s,
        Err(e) => {
            println!("`/proc/meminfo` {}", e);
            return None
        }
    };

    let v: Vec<_> = cpu_proc.split('\n')
        .filter(|s| s.contains("MemTotal") || s.contains("MemAvailable"))
        .map(|s| {
            let t: f32 = s.split_whitespace()
                .filter_map(|ss| ss.parse::<f32>().ok())
                .collect::<Vec<f32>>()[0];
            t
        })
        .collect();
    
    if v.len() < 2 { return None }

    // memory total = v[0]
    // memory available = v[1]
    let used_memory_perc = 100.0 - ((v[1] / v[0]) * 100.0);

    Some(format!("\u{e021}{:02}%", used_memory_perc.round()))
}
