mod help;
mod visitor;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            match args[1].parse::<String>() {
                Ok(param) => {
                    if param == "--path" {
                        visitor::run(&args[2], 10);
                    }
                }
                _ => println!("Unkonwn argument"),
            }
        }
        1 ... 2 | _ => {
            help::usage();
        }
    }
}
