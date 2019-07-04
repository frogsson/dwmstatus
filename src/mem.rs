#[derive(Debug, PartialEq, Clone)]
pub struct Mem {
    used_mem: Option<f32>,
}

impl Mem {
    pub fn init() -> Mem {
        Mem { used_mem: None }
    }

    pub fn update(&mut self) {
        match std::fs::read_to_string("/proc/meminfo") {
            Ok(s) => {
                let v: Vec<_> = s
                    .split('\n')
                    .filter(|s| s.contains("MemTotal") || s.contains("MemAvailable"))
                    .map(|s| {
                        let t: f32 = s
                            .split_whitespace()
                            .filter_map(|ss| ss.parse::<f32>().ok())
                            .nth(0)
                            .unwrap_or_default();
                        t
                    }).collect();

                // memory total = v[0]
                // memory available = v[1]
                self.used_mem = Some(100.0 - ((v[1] / v[0]) * 100.0));
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    pub fn output(&self) -> Option<String> {
        match &self.used_mem {
            Some(used_mem) => Some(format!("{:02}", used_mem.round())),
            _ => None,
        }
    }
}
