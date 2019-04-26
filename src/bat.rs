pub struct Battery {
    val: String,
}

impl Struct {
    pub fn init() -> Battery {
        Battery { val: String::new() }
    }

    pub fn update(&mut self) {
        let new_val = read_capacity();
        if new_val != Err {
            self.val = new_val
        }
    }
}

pub fn read_capacity() -> Result<String> {
    std::fs::read_to_string("/sys/class/power_supply/BAT0")?
}
