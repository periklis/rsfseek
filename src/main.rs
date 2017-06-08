#[macro_use] extern crate prettytable;

mod help;
mod collector;
mod visitor;

use collector::TopFilesCollector;
use visitor::DirectoryVisitor;

use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

use prettytable::Table;

type ResVec = Vec<(PathBuf, u64)>;

pub fn run(root: &str, limit: usize) {
    println!("Scanning: {} for top {} biggest files", root, limit);

    let mut vec = ResVec::with_capacity(limit);

    for _ in 0..limit-1 {
        vec.push((PathBuf::from(""), 0));
    }

    let root_dir = PathBuf::from(root);
    let results = Arc::new(Mutex::new(vec));
    let (tx, rx) = channel::<PathBuf>();

    let visitor = thread::spawn(move || {
        let visitor = DirectoryVisitor::new(&root_dir, true);

        for dir in visitor.expect("Visiting directory failed") {
            tx.send(dir).expect("Failed sending path buffer");
        }
    });

    let file_results = results.clone();
    let collector = thread::spawn(move || {
        let mut collector = TopFilesCollector::new(&file_results);

        for next in rx {
            collector.collect(&next);
        }
    });

    visitor.join().expect("Failed joining visitor thread");
    collector.join().expect("Failed joining collector thread");

    let mut table = Table::new();
    table.add_row(row!["Filename", "Size (Bytes)"]);

    for item in results.lock().unwrap().iter() {
        table.add_row(row![item.0.to_str().unwrap(), r->item.1]);
    }

    table.printstd();
}

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

            run(&path, limit);
        }
        1 ... 2 | _ => {
            help::usage();
        }
    }
}
