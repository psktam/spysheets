use std::collections::HashMap;
use std::rc::Rc;

use serde_yaml::Mapping;

use crate::core::coordinates::{Coordinate, coordinate_factory};
use crate::core::functions::{Function, function_factory};
use crate::core::selections::{Select, EmptySelection, selection_factory};
use crate::tables::{Table, CellValue};

pub type OpID = u64;


fn _get_corners(results: &HashMap<(u64, u64), CellValue>) -> ((u64, u64), (u64, u64)){
    let mut first_row = std::u64::MAX;
    let mut first_col = std::u64::MAX;
    let mut last_row: u64 = 0;
    let mut last_col: u64 = 0;

    for (row, col) in results.keys() {
        first_row = std::cmp::min(first_row, *row);
        first_col = std::cmp::min(first_col, *col);
        last_row = std::cmp::max(last_row, *row);
        last_col = std::cmp::max(last_col, *col);
    }

    ((first_row, first_col), (last_row, last_col))
}


pub struct OpSpec {
    pub id: u64,
    pub input_selection: Rc<dyn Select>,
    pub output_anchor: Rc<dyn Coordinate>,
    pub function: Rc<dyn Function>
}


pub fn operation_factory(config: &Mapping) -> OpSpec {
    let input_selection;
    if config.contains_key("input_selection") {
        input_selection = selection_factory(config["input_selection"].as_mapping().unwrap()).unwrap();
    }
    else {
        input_selection = Rc::new(EmptySelection {});
    }

    let function = function_factory(
        config["function"].as_mapping().unwrap()).unwrap();

    let output_anchor = coordinate_factory(
        config["output_anchor"].as_mapping().unwrap()).unwrap();

    OpSpec { 
        id: config["id"].as_u64().unwrap(),
        input_selection: input_selection, 
        output_anchor: output_anchor, 
        function: function
    }
}


pub fn apply_operation(table: &mut Table, spec: &OpSpec) {
    let inputs = spec.input_selection.select(table).unwrap();
    let results = spec.function.call(table, &inputs).unwrap();

    // First update the operation map with the region that this operation wrote to.
    table.op_map.insert(spec.id, _get_corners(&results));

    // Next calculate the coordinates of the anchor.
    let anchor_coordinates = spec.output_anchor.resolve(table).unwrap();

    // Finally, offset all of the results by the anchor.
    for (base_coord, val) in results.iter() {
        let new_row = base_coord.0 + anchor_coordinates.0;
        let new_col = base_coord.1 + anchor_coordinates.1;

        // For the time being, I guess cloning the results is the most expedient
        // thing to do, but the RIGHT thing to do is to figure out how to pass
        // ownership from the result hashmap to the table.values hashmap.
        table.values.insert((new_row, new_col), val.clone());
    }
}