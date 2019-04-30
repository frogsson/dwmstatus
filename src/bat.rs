#[derive(Debug, PartialEq, Clone)]
pub struct Battery {
    val: String,
}

impl Battery {
    pub fn init() -> Battery {
        Battery { val: String::new() }
    }

    pub fn update(&mut self) {
        match std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
            Ok(s) => self.val = s,
            Err(e) => println!("{}", e),
        }
    }

    pub fn output(&self) -> String {
        format!("\u{e03b}{}%", self.val)
    }
}
