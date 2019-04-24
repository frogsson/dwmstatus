extern crate rustystatus;

use rustystatus::Unwrap;
use rustystatus::{call, get_config_path};
use rustystatus::{Config, Module};
use std::error::Error;
use std::thread::sleep;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from(get_config_path()?).unwrap_or_default();

    // let separator = config.separator();
    let mut order = config.order()?;
    // let last_module = order.last().expect("no modules in output_order");

    loop {
        let mut output = String::new();

        for module in order.iter_mut() {
            match module.clone() {
                Module::Time(ref mut m) => {
                    m.update();
                    output.push_str(&m.output());
                },
                Module::Weather(ref mut m) => {
                    m.update();
                    output.push_str(&m.output());
                },
                Module::Net(ref mut m) => {
                    m.update();
                    output.push_str(&m.output());
                },
                Module::Cpu(ref mut m) => {
                    m.update();
                    output.push_str(&m.output());
                },
                Module::Mem(ref mut m) => {
                    m.update();
                    output.push_str(&m.output());
                },
            }
            output.push_str(&config.separator());
        }

        println!("{:?}", order);

        call(&output);
        sleep(config.update_interval());
    }
}
