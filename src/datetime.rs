extern crate chrono;

#[derive(Debug)]
pub struct Time {
    val: String,
}

impl Time {
    pub fn init() -> Time {
        Time { val: String::new() }
    }

    pub fn update(&mut self) -> &mut Self {
        self.val = chrono::Local::now()
            .format("\u{e225}%A %b %Y-%m-%d %H:%M")
            .to_string();
        self
    }

    pub fn output(&self) -> String {
        self.val.to_string()
    }
}
