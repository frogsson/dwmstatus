extern crate rustystatus;

use rustystatus::{Config, Unwrap};
use rustystatus::{call, get_config_path, last_item, update_and_output};
use std::error::Error;
use std::thread::sleep;

fn main() -> Result<(), Box<dyn Error>> {
    let mut modules;
    let update_interval;
    let separator;
    let len;

    {
        let config = Config::from(get_config_path()?).unwrap_or_default();
        modules = config.modules()?;
        update_interval = config.update_interval();
        separator = config.separator();
        len = modules.len() - 1;
    }

    loop {
        let output: String = modules
            .iter_mut()
            .enumerate()
            .map(|module| update_and_output(module.1, &separator, last_item(module.0, len)))
            .collect();

        call(&output);
        sleep(update_interval);
    }
}
