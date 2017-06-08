use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

type ResVec = Vec<(PathBuf, u64)>;

pub struct TopFilesCollector<'a> {
    results: &'a Arc<Mutex<ResVec>>
}

impl<'a> TopFilesCollector<'a> {
    pub fn new(target: &'a Arc<Mutex<ResVec>>) -> Box<Self> {
        Box::new(TopFilesCollector{results: target})
    }

    pub fn collect(&mut self, path: &PathBuf) {
        for entry in path.read_dir().expect("Failed reading directory") {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    let metadata = fs::metadata(entry.path())
                        .expect("Failed fetching metadata");
                    let filesize = metadata.len();
                    let mut vec = self.results.lock().unwrap();

                    vec.sort_by(|l, r| {
                        return r.1.cmp(&l.1);
                    });

                    for i in vec.iter_mut() {
                        if filesize >= i.1 {
                            *i = (PathBuf::from(entry.path()), filesize);
                            break;
                        }
                    }
                }
            }
        }
    }
}
