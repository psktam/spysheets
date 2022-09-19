use std::collections::HashMap;
use std::env;
use std::path::Path;

pub mod core;
use crate::core::{file_operations, tables, operations};


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = Path::new(&args[1]);

    let operations = file_operations::load_file(file_path);
    let mut table = tables::Table {
        values: HashMap::new(),
        op_map: HashMap::new()
    };

    for op in operations.iter() {
        operations::apply_operation(&mut table, op);
    }


    println!("Final Table");
    println!("====================");
    println!("{:?}", &table);
}
