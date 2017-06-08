//use std::fmt;
use std::path::PathBuf;
//use std::result;

// type Result<T> = result::Result<T, VisitError>;

// #[derive(Debug, PartialEq)]
// pub enum VisitError {
//     NotExist,
//     NotDirectory,
//     ReadDirFailed
// }

// impl fmt::Display for VisitError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             VisitError::NotExist => write!(f, "Directory does not exist for path"),
//             VisitError::NotDirectory => write!(f, "Path is not a directory"),
//             VisitError::ReadDirFailed => write!(f, "Reading directory for path failed")
//         }
//     }
// }


pub struct DirectoryVisitor {
    dir_list: Vec<PathBuf>
}

impl DirectoryVisitor {
    pub fn new(path: &PathBuf) -> Self {
        DirectoryVisitor{
            dir_list: Self::read_dir(path)
        }
    }

    fn read_dir(path: &PathBuf) -> Vec<PathBuf> {
        match path.read_dir() {
            Err(_) => {
                return vec![];
            }
            Ok(dir) => {
                return dir.filter(|d| d.is_ok())
                    .map(|d| d.unwrap())
                    .filter(|d| d.file_type().unwrap().is_dir())
                    .map(|d| d.path())
                    .collect();
            }
        }
    }
}

impl Iterator for DirectoryVisitor {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        if self.dir_list.is_empty() {
            return None;
        }

        let next = self.dir_list.pop().unwrap();
        self.dir_list.extend(Self::read_dir(&next).into_iter());

        Some(next)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_visit_dir() {
        let dirs = DirectoryVisitor::new("/Users/periklis/Projects/github");
        let expected: Vec<PathBuf> = vec![];
        let actual: Vec<PathBuf> = dirs.collect();

        assert_eq!(expected, actual);
        // let v = DirectoryIter::new("/Users/periklis/Projects/github/");
        // let dirs = v.chi.collect::<Vec<String>>();
        // let expc: Vec<String> = vec![];

        // assert_eq!(expc, dirs);
    }
}
