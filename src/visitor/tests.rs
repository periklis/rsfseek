use super::*;

#[test]
fn it_visits_path() {
    let res = visit_path("./src/");

    assert_eq!(Ok("Finished"), res);
}

#[test]
fn it_visits_unknown_path() {
    let res = visit_path("./unknown");

    assert_eq!(Err("Path does not exist"), res);
}

#[test]
fn it_visits_file_path() {
    let res = visit_path("./src/main.rs");

    assert_eq!(Err("Given path is not a directory"), res);
}
