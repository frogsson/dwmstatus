extern crate rustystatus;

use rustystatus::{call, match_module, parse_output_order, read_config, Modules};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let config = read_config();

    let update_interval = Duration::from_millis(
        config
            .get("update_interval")
            .and_then(|value| value.as_float().map(|f| f * 1000.0))
            .unwrap_or(1000.0) as u64,
    );

    let order = parse_output_order(config.get("output_order"));
    let separator = config["output_separator"]
        .as_str()
        .unwrap_or(" ")
        .to_string();

    let mut modules = Modules::init(config, &order);
    let last_module = order.last().expect("no modules in output_order");

    loop {
        let output: String = order.iter()
            .map(|m| {
                let mut module_output = match_module(m, &mut modules);
                if m != last_module {
                    module_output.push_str(&separator);
                }
                module_output
            }).collect();

        call(&output);
        sleep(update_interval);
    }
}
