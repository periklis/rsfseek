use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, channel};
use std::thread;

type ResVec = Vec<(String, u64)>;

fn collect_files(path: &str, results: &Arc<Mutex<ResVec>>) {
    let path = Path::new(path);

    for entry in path.read_dir().expect("Failed reading directory") {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                let metadata = fs::metadata(entry.path())
                    .expect("Failed fetching metadata");
                let filesize = metadata.len();
                let mut vec = results.lock().unwrap();

                vec.sort_by(|l, r| {
                    return r.1.cmp(&l.1);
                });

                for i in vec.iter_mut() {
                    if filesize >= i.1 {
                        *i = (String::from(entry.path().to_str().unwrap()), filesize);
                        break;
                    }
                }
            }
        }
    }
}

fn visit_dir(str: &str, tx: &Sender<String>) -> Result<&'static str, &'static str> {
    let path = Path::new(str);

    if !path.exists() {
       return Err("Path does not exist");
    }

    if !path.is_dir() {
        return Err("Given path is not a directory");
    }

    for entry in path.read_dir().expect("Failed reading directory") {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                let next_dir = String::from(entry.path().to_str().unwrap());
                tx.send(next_dir)
                    .expect("Failed to send to the channel");
                visit_dir(entry.path().to_str().unwrap(), tx)
                    .expect("Failed visitation");
            }
        }
    }

    Ok("Finished")
}

pub fn run(root: &str, limit: usize) {
    println!("Scanning: {} for top {} biggest files", root, limit);

    let mut vec = ResVec::with_capacity(limit);

    for _ in 0..limit-1 {
        vec.push(("".to_string(), 0));
    }

    let root_dir = String::from(root);
    let results = Arc::new(Mutex::new(vec));
    let (tx, rx) = channel::<String>();

    let visitor = thread::spawn(move || {
        visit_dir(&root_dir, &tx).expect("Failed to visit root");
    });

    let file_results = results.clone();
    let collector = thread::spawn(move || {
        for next in rx {
            collect_files(&next, &file_results);
        }
    });

    visitor.join().expect("Failed joining visitor thread");
    collector.join().expect("Failed joining collector thread");

    for item in results.lock().unwrap().iter() {
        println!("Filename: {} size: {}", item.0, item.1);
    }
}
