extern crate rustystatus;

use std::thread::sleep;
use std::time::Duration;
use rustystatus::{parse_output_order, call, read_config, Modules, ModuleName};

fn main() {
    let config = read_config();

    let one_sec = Duration::from_secs(1);
    let order = parse_output_order(config["output_order"].as_array());
    let separator = config["output_separator"].as_str().unwrap_or(" ").to_string();

    let mut modules = Modules::init(config, &order);
    let mut output = String::new();

    loop {
        output.clear();

        for module in &order {
            match module {
                ModuleName::Time => {
                    modules.time.update();
                    output.push_str(&modules.time.output())
                },
                ModuleName::Net => {
                    modules.net.update();
                    output.push_str(&modules.net.output())
                },
                ModuleName::Cpu => {
                    modules.cpu.update();
                    output.push_str(&modules.cpu.output())
                },
                ModuleName::Mem => {
                    modules.mem.update();
                    output.push_str(&modules.mem.output())
                },
                ModuleName::Weather => {
                    modules.weather.update();
                    output.push_str(&modules.weather.output())
                },
            };

            output.push_str(&separator);
        }

        call(&output);
        sleep(one_sec);
    }
}
