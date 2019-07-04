#[derive(Debug, PartialEq, Clone)]
pub struct Battery {
    val: Option<String>,
}

impl Battery {
    pub fn init() -> Battery {
        Battery { val: None }
    }

    pub fn update(&mut self) {
        match std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
            Ok(s) => self.val = Some(s),
            Err(e) => {
                self.val = None;
                println!("Error: {}", e);
            },
        }
    }

    pub fn output(&self) -> Option<String> {
        match &self.val {
            Some(val) => Some(val.to_string()),
            _ => None,
        }

    }
}
