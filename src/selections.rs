use std::rc::Rc;

use serde_yaml::{Mapping};

use crate::tables::Table;
use crate::coordinates::{Coordinate, coordinate_factory};

pub trait Select {
    fn select(&self, table: &Table) -> Option<Vec<(u64, u64)>>;
}


pub struct EmptySelection { }

impl Select for EmptySelection {
    fn select(&self, _table: &Table) -> Option<Vec<(u64, u64)>> {
        Some(Vec::new())
    }
}


struct BoxSelection {
    pub upper_left: Rc<dyn Coordinate>,
    pub bottom_right: Rc<dyn Coordinate>
}

impl Select for BoxSelection {
    fn select(&self, table: &Table) -> Option<Vec<(u64, u64)>> {
        let upper_left = self.upper_left.resolve(table);
        let lower_right = self.bottom_right.resolve(table);

        if (upper_left == None) || (lower_right == None) {
            None
        }
        else {
            let upper_left = upper_left.unwrap();
            let lower_right = lower_right.unwrap();
            let mut result = Vec::new();
            for row in upper_left.0..(lower_right.0 + 1){
                for col in upper_left.1..(lower_right.1 + 1){
                    result.push((row, col));
                }
            }
            Some(result)
        }
    }
}


pub fn selection_factory(config: &Mapping) -> Option<Rc<dyn Select>> {
    let sel_type = config["type"].as_str().unwrap();
    match sel_type {
        "BoxSelection" => {
            let upper_left = coordinate_factory(
                config["upper_left"].as_mapping().unwrap()).unwrap();
            let lower_right = coordinate_factory(
                config["lower_right"].as_mapping().unwrap()).unwrap();
            let selection = BoxSelection{ 
                upper_left: upper_left,
                bottom_right: lower_right
            };
            Some(Rc::new(selection))
        },
        _ => None
    }
}