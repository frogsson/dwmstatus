use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Battery {
    val: String,
}

impl Battery {
    pub fn init() -> Battery {
        Battery { val: String::new() }
    }

    pub fn update(&mut self) {
        let new_val = read_capacity();
        match new_val {
            Ok(s) => self.val = s,
            Err(e) => println!("{}", e),
        }
    }

    pub fn output(&self) -> String {
        format!("\u{e03b}{}%", self.val)
    }
}

pub fn read_capacity() -> Result<String, Box<dyn Error + 'static>> {
    let r = std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity")?;
    Ok(r)
}
