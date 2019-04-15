#[derive(Debug)]
pub struct Mem {
    val: String,
}

impl Mem {
    pub fn init() -> Mem {
        Mem { val: String::new() }
    }

    pub fn update(&mut self) {
        if let Some(s) = read_memory_proc() {
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
                })
                .collect();

            // memory total = v[0]
            // memory available = v[1]
            let used_memory_perc = 100.0 - ((v[1] / v[0]) * 100.0);

            self.val = format!("\u{e021}{:02}%", used_memory_perc.round())
        } else {
            self.val = "".to_string()
        }
    }

    pub fn output(&self) -> String {
        self.val.to_string()
    }
}

fn read_memory_proc() -> Option<String> {
    match std::fs::read_to_string("/proc/meminfo") {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("`/proc/meminfo` {}", e);
            None
        }
    }
}
