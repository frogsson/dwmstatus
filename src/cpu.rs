use cpuerror::*;
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Cpu {
    val: std::result::Result<i32, CpuError>,
    system: i32,
    last_sum: i32,
}

impl Cpu {
    pub fn init() -> Cpu {
        Cpu {
            val: Ok(0),
            system: 0,
            last_sum: 0,
        }
    }

    pub fn update(&mut self) {
        //      user    nice   system  idle      iowait irq   softirq  steal  guest  guest_nice
        // cpu  74608   2520   24433   1117073   6176   4054  0        0      0      0

        // explanation for this shit
        // https://www.idnt.net/en-GB/kb/941772
        match read_cpu_proc() {
            Ok(cpu) =>  {
                let cpu_sum: i32 = cpu.iter().sum();

                if let Some(s) = cpu.get(3) {
                    let cpu_delta = cpu_sum - self.last_sum;
                    let cpu_idle = s - self.system;
                    let cpu_used = cpu_delta - cpu_idle;
                    let cpu_usage = 100 * cpu_used / cpu_delta;

                    self.system = *s;
                    self.last_sum = cpu_sum;

                    self.val = Ok(cpu_usage);
                } else {
                    eprintln!("Error: could get value from `/proc/stat`");
                }
            },
            Err(e) => {
                eprintln!("Error: `/proc/stat` {}", e);
                self.val = Err(CpuError::Generic);
            }
        }
    }

    pub fn output(&self) -> Option<String> {
        match self.val {
            Ok(i) => Some(format!("{:02}", i)),
            Err(_) => None,
        }
    }
}

fn read_cpu_proc() -> Result<Vec<i32>, Box<dyn Error>> {
    let cpu = std::fs::read_to_string("/proc/stat")?
        .split('\n')
        .nth(0)
        .ok_or_else(|| CpuError::ReadProc)?
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    Ok(cpu)
}

mod cpuerror {
    use std::fmt;

    #[derive(Debug, PartialEq, Clone)]
    pub enum CpuError {
        ReadProc,
        Generic,
    }

    impl std::error::Error for CpuError {
        fn description(&self) -> &str {
            match *self {
                CpuError::ReadProc => "failed parsing `/proc/stat`",
                CpuError::Generic => "generic error",
            }
        }
    }

    impl fmt::Display for CpuError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                CpuError::ReadProc => f.write_str("failed parsing `/proc/stat`"),
                CpuError::Generic => f.write_str("generic error"),
            }
        }
    }
}
