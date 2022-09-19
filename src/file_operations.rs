use std::{fs::File, io::Read};
use std::path::Path;
use std::vec::Vec;

use serde_yaml::{from_str, Mapping};

use crate::operations::{operation_factory, OpSpec};


pub fn load_file(file_path: &Path) -> Vec<OpSpec> {
    let display = file_path.display();

    let mut file = match File::open(file_path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => { print!("Done!"); },
    }

    let config:Mapping = from_str(&contents).unwrap();
    let operation_configs = config["operations"].as_sequence().unwrap();
    let mut result = Vec::new();

    for item in operation_configs.iter() {
        result.push(operation_factory(item.as_mapping().unwrap()));
    }
    result
}