extern crate rustystatus;

use rustystatus::Config;
use rustystatus::Unwrap;
use rustystatus::{call, get_config_path, last_item, update_and_output};
use std::error::Error;
use std::thread::sleep;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from(get_config_path()?).unwrap_or_default();

    let separator = config.separator();
    let mut order = config.order()?;
    let len = order.len() - 1;

    loop {
        let output = order
            .iter_mut()
            .enumerate()
            .map(|module| update_and_output(module.1, separator, last_item(module.0, len)))
            .collect::<String>();

        call(&output);
        sleep(config.update_interval());
    }
}
