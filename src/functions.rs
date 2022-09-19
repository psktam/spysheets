use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;

use serde_yaml::{Mapping, Value};

use crate::tables::{CellValue, Table};

pub trait Function {
    fn call(&self, table: &Table, input_selection: &Vec<(u64, u64)>) -> Option<HashMap<(u64, u64), CellValue>>;
}


pub struct RawInput {
    values: HashMap<(u64, u64), CellValue>
}

impl RawInput {
    pub fn from_mapping(data: &Value) -> RawInput {        
        let mut results = HashMap::new();

        let rows = data.as_sequence().unwrap();

        for (row_idx, row) in rows.iter().enumerate() {
            let row_data = row.as_sequence().unwrap();
            for (col_idx, value) in row_data.iter().enumerate() {
                let as_cell_value = match value {
                    Value::String(x) => CellValue::Str(x.to_string()),
                    Value::Number(x) => {
                        if x.is_i64() { CellValue::Int(x.as_i64().unwrap()) }
                        else { CellValue::Float(x.as_f64().unwrap()) }
                    },
                    _ => { panic!("Unrecognized data type!"); }
                };

                results.insert((row_idx as u64, col_idx as u64), as_cell_value);
            }
        }

        RawInput { values: results }
    }
}

impl Function for RawInput {
    fn call(&self, _table: &Table, _input_selection: &Vec<(u64, u64)>) -> Option<HashMap<(u64, u64), CellValue>> {
        Some(self.values.clone())
    }
}


pub struct Average {}

impl Function for Average {
    fn call(&self, table: &Table, input_selection: &Vec<(u64, u64)>) -> Option<HashMap<(u64, u64), CellValue>> {
        let mut running_sum = 0.0;

        for key in input_selection.iter() {
            match table.values[key] {
                CellValue::Float(x) => { running_sum += x; },
                CellValue::Int(x) => { running_sum += x as f64; },
                CellValue::Str(_) => return None
            }
        }

        let average = running_sum / (input_selection.len() as f64);
        let mut result = HashMap::new();

        result.insert((0, 0), CellValue::Float(average));

        Some(result)
    }
}


pub fn function_factory(config: &Mapping) -> Option<Rc<dyn Function>> {
    let func_type = config["type"].as_str().unwrap();

    match func_type {
        "RawInput" => Some(Rc::new(RawInput::from_mapping(&config["values"]))),
        "Average" => Some(Rc::new(Average {} )),
        _ => None
    }
}