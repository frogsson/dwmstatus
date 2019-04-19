#[derive(Debug)]
pub struct Cpu {
    val: String,
    system: i32,
    last_sum: i32,
}

impl Cpu {
    pub fn init() -> Cpu {
        Cpu {
            val: String::new(),
            system: 0,
            last_sum: 0,
        }
    }

    pub fn update(&mut self) -> &mut Self {
        //      user    nice   system  idle      iowait irq   softirq  steal  guest  guest_nice
        // cpu  74608   2520   24433   1117073   6176   4054  0        0      0      0

        // explanation for this shit
        // https://www.idnt.net/en-GB/kb/941772
        if let Some(cpu) = read_cpu_proc() {
            let cpu_sum: i32 = cpu.iter().sum();

            let s = cpu[3];

            let cpu_delta = cpu_sum - self.last_sum;
            let cpu_idle = s - self.system;
            let cpu_used = cpu_delta - cpu_idle;
            let cpu_usage = 100 * cpu_used / cpu_delta;

            self.system = s;
            self.last_sum = cpu_sum;

            self.val = format!("\u{e223}{:02}%", cpu_usage);
        } else {
            self.val = "".to_string();
        }

        self
    }

    pub fn output(&self) -> String {
        self.val.to_string()
    }
}

fn read_cpu_proc() -> Option<Vec<i32>> {
    let cpu_proc = match std::fs::read_to_string("/proc/stat") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("`/proc/stat` {}", e);
            return None;
        }
    };

    let cpu = cpu_proc
        .split('\n')
        .nth(0)
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    Some(cpu)
}
