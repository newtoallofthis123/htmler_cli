use std::fs::File;
use std::io::{Read, Error};

pub fn check_file_exits(filename: &str) -> bool {
    let f = File::open(filename);
    match f {
        Ok(_) => return true,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        },
    };
}


pub fn read_file(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(error) => Err(error),
    }
}