use std::fmt;
use std::path::PathBuf;
use std::result;

type Result<'a, T> = result::Result<T, DirVisitError<'a>>;

#[derive(Debug)]
enum DirVisitError<'a> {
    NotExists{path: &'a PathBuf}
}

impl<'a> fmt::Display for DirVisitError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DirVisitError::NotExists{path} =>
                write!(f, "Directory does not exists: {}", path.to_str().unwrap())
        }
    }
}


pub struct DirectoryVisitor {
    dir_list: Vec<PathBuf>,
    omit_hidden: bool
}

impl DirectoryVisitor {
    pub fn new(path: &PathBuf, omit_hidden: bool) -> Option<Self> {
        match Self::read_dir(path, true){
            Ok(vec) => Some(DirectoryVisitor{
                dir_list: vec,
                omit_hidden: omit_hidden
            }),
            Err(_) => {
                None
            }
        }
    }

    fn read_dir(path: &PathBuf, omit_hidden: bool) -> Result<Vec<PathBuf>> {
        match path.read_dir() {
            Err(_) => {
                return Err(DirVisitError::NotExists{path: path});
            }
            Ok(dir) => {
                return Ok(dir.filter(|d| d.is_ok())
                          .map(|d| d.unwrap())
                          .filter(|d| d.file_type().unwrap().is_dir())
                          .filter(|d| !(omit_hidden && d.path()
                                        .components().last().unwrap()
                                        .as_os_str().to_str().unwrap()
                                        .starts_with(".")))
                          .map(|d| d.path())
                          .collect());
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

        match Self::read_dir(&next, self.omit_hidden) {
            Ok(vec) => {
                self.dir_list.extend(vec.into_iter());
                Some(next)
            },
            Err(_) => {
                None
            }
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_visits_empty_dir() {
        let dirs = DirectoryVisitor::new(&PathBuf::from("./src"), true);
        let expected: Vec<PathBuf> = vec![];
        let actual: Vec<PathBuf> = dirs.unwrap().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_visits_existing_dir() {
        let dirs = DirectoryVisitor::new(&PathBuf::from("./target"), true);
        let expected = vec![
            PathBuf::from("./target/debug"),
            PathBuf::from("./target/debug/native"),
            PathBuf::from("./target/debug/incremental"),
            PathBuf::from("./target/debug/examples"),
            PathBuf::from("./target/debug/deps"),
            PathBuf::from("./target/debug/build")];
        let actual: Vec<PathBuf> = dirs.unwrap().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn it_panics_on_not_existing_dir() {
        let dirs = DirectoryVisitor::new(&PathBuf::from("./NOTEXISTING"), true);
        let _: Vec<PathBuf> = dirs.expect("Panic on not existing").collect();
    }
}
