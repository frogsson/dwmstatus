use std::time::Instant;

#[derive(Debug)]
pub struct Net {
    val: String,
    recv: f32,
    tran: f32,
    recv_stack: Vec<f32>,
    tran_stack: Vec<f32>,
    net_time: Instant,
}

impl Net {
    pub fn init() -> Net {
        Net {
            val: String::new(),
            recv: 0.0,
            tran: 0.0,
            recv_stack: vec![0.0, 0.0, 0.0],
            tran_stack: vec![0.0, 0.0, 0.0],
            net_time: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        if let Some(i) = read_net_proc() {
            let seconds_passed = self.net_time.elapsed().as_secs() * 1_000_000;

            if let Some(x) = i.get(0) {
                self.recv_stack.remove(0);
                self.recv_stack.push((x - self.recv) / seconds_passed as f32);
                self.recv = *x;
            }
            let recv = transfer_speed_as_mb(&self.recv_stack);

            if let Some(y) = i.get(8) {
                self.tran_stack.remove(0);
                self.tran_stack.push((y - self.tran) / seconds_passed as f32);
                self.tran = *y;
            }
            let tran = transfer_speed_as_mb(&self.tran_stack);

            self.val = format!("\u{e061}{:.2} MB/s \u{e060}{:.2} MB/s", recv, tran);
            self.net_time = Instant::now();
        } else {
            self.val = "".to_string();
        }
    }

    pub fn output(&self) -> String {
        self.val.to_string()
    }
}

pub fn read_net_proc() -> Option<Vec<f32>> {
    let net_info = match std::fs::read_to_string("/proc/net/dev") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("`/proc/net/dev` {}", e);
            return None
        }
    };

    let vals: Vec<_> = net_info.split('\n')
        .filter(|s| s.contains("eno1"))
        .collect::<String>()
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<f32>().ok())
        .collect();

    if vals.is_empty() { return None }

    Some(vals)
}

fn transfer_speed_as_mb(v: &[f32]) -> f32 {
    let sum: f32 = v.iter().sum();
    let len: f32 = v.len() as f32;
    sum / len
}
