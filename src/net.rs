use std::time::Instant;
use neterror::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Net {
    val: String,
    recv: f32,
    tran: f32,
    recv_stack: Vec<f32>,
    tran_stack: Vec<f32>,
    net_time: Instant,
    interface: String,
}

impl Net {
    pub fn init(i: String) -> Net {
        Net {
            val: String::new(),
            recv: 0.0,
            tran: 0.0,
            recv_stack: vec![0.0, 0.0, 0.0],
            tran_stack: vec![0.0, 0.0, 0.0],
            net_time: Instant::now(),
            interface: i,
        }
    }

    pub fn update(&mut self) {
        match read_net_proc(&self.interface) {
            Ok(i) => {
                let seconds_passed = self.net_time.elapsed().as_secs() * 1_000_000;

                if let Some(x) = i.get(0) {
                    self.recv_stack.remove(0);
                    self.recv_stack
                        .push((x - self.recv) / seconds_passed as f32);
                    self.recv = *x;
                }
                let recv = transfer_speed_as_mb(&self.recv_stack);

                if let Some(y) = i.get(8) {
                    self.tran_stack.remove(0);
                    self.tran_stack
                        .push((y - self.tran) / seconds_passed as f32);
                    self.tran = *y;
                }
                let tran = transfer_speed_as_mb(&self.tran_stack);

                self.val = format!("\u{e061}{:.2} MB/s \u{e060}{:.2} MB/s", recv, tran);
                self.net_time = Instant::now();
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                self.val = "".to_string();
            },
        }
    }

    pub fn output(&self) -> String {
        self.val.to_string()
    }
}

pub fn read_net_proc(interface: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let net_info = std::fs::read_to_string("/proc/net/dev")?;

    let vals: Vec<_> = net_info
        .split('\n')
        .filter(|s| s.contains(interface))
        .collect::<String>()
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<f32>().ok())
        .collect();

    if vals.is_empty() {
        Err(NetError::EmptyVec.into())
    } else {
        Ok(vals)
    }
}

fn transfer_speed_as_mb(v: &[f32]) -> f32 {
    let sum: f32 = v.iter().sum();
    let len: f32 = v.len() as f32;
    sum / len
}

mod neterror {
    use std::fmt;

    #[derive(Debug)]
    pub enum NetError {
        EmptyVec,
    }

    impl std::error::Error for NetError {
        fn description(&self) -> &str {
            match *self {
                NetError::EmptyVec => "vector is empty",
            }
        }
    }

    impl fmt::Display for NetError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                NetError::EmptyVec => f.write_str("vec is empty"),
            }
        }
    }
}
