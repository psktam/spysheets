use std::collections::HashMap;


pub type OpID = u64;


#[derive(Clone, Debug)]
pub enum CellValue {
    Int(i64),
    Float(f64),
    Str(String)
}


#[derive(Debug)]
pub struct Table {
    pub values: HashMap<(u64, u64), CellValue>,
    // Map an op-id to an upper left and lower right corner.
    pub op_map: HashMap<OpID, ((u64, u64), (u64, u64))>
}