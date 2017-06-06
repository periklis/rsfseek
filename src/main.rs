#[macro_use] extern crate prettytable;

mod help;
mod visitor;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        argc @ 3 ... 5 => {
            let argv = if argc == 3 { [1,0] } else { [1,3] };
            let mut path = String::from("");
            let mut limit = 100;
            let path_param = "--path";
            let limit_param = "--limit";

            for i in &argv {
                match args[*i].parse::<String>() {
                    Ok(ref param) if param == path_param => {
                        path = args[i+1].parse::<String>().unwrap();
                    },
                    Ok(ref param) if param == limit_param => {
                        limit = args[i+1].parse::<usize>().unwrap();
                    },
                    Ok(_) | Err(_) => {}
                }
            }

            visitor::run(&path, limit);
        }
        1 ... 2 | _ => {
            help::usage();
        }
    }
}
