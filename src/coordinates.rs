use std::rc::Rc;

use serde_yaml::Mapping;

use crate::operations::OpID;
use crate::tables::Table;


pub enum OrdType {
    Row,
    Col
}

pub trait Ordinate{ 
    fn resolve(&self, table: &Table) -> u64;
}


// Absolute ordinate. Simplest one, resolves to a constant number.
pub struct AbsOrdinate {
    val: u64
}

impl Ordinate for AbsOrdinate {
    fn resolve(&self, _table: &Table) -> u64 {
        self.val
    }
}


fn ordinate_factory(config: &Mapping) -> Option<Rc<dyn Ordinate>> {
    let ord_type = config["type"].as_str().unwrap();

    match ord_type {
        "AbsOrdinate" => {
            let val = config["val"].as_u64().unwrap();
            Some(Rc::new(AbsOrdinate {val: val}))
        }
        _ => None
    }
}


// Op-based ordinate. Resolves to a corner or an edge in a region where an 
// operation writes to.
pub enum Edge {
    First,
    Last
}


// After all, what is a coordinate but just a bunch of ordinates?
pub struct BaseCoordinate {
    row: Rc<dyn Ordinate>,
    col: Rc<dyn Ordinate>
}

pub trait Coordinate {
    fn resolve(&self, table: &Table) -> Option<(u64, u64)>;
}

impl Coordinate for BaseCoordinate {
    fn resolve(&self, table: &Table) -> Option<(u64, u64)> {
        Some((self.row.resolve(table), self.col.resolve(table)))
    }
}


pub enum Corner {
    TopLeft,
    TopRight,
    BotLeft,
    BotRight
}

pub struct OpCorner {
    op_id: OpID,
    corner: Corner
}

impl Coordinate for OpCorner {
    fn resolve(&self, table: &Table) -> Option<(u64, u64)> {
        let ((first_row, first_col), (last_row, last_col)) = table.op_map[&self.op_id];
        let result = match self.corner {
            Corner::TopLeft => Some((first_row, first_col)),
            Corner::TopRight => Some((first_row, last_col)),
            Corner::BotLeft => Some((last_row, first_col)),
            Corner::BotRight => Some((last_row, last_col))
        };

        result
    }
}

impl OpCorner {
    pub fn from_config(config: &Mapping) -> OpCorner {
        let op_id:OpID = config["op_id"].as_u64().unwrap();
        let corner = match config["corner"].as_str().unwrap() {
            "TopRight" => Corner::TopRight,
            "TopLeft" => Corner::TopLeft,
            "BotRight" => Corner::BotRight,
            "BotLeft" => Corner::BotLeft,
            _ => panic!("Unrecognized corner type: {}", config["corner"].as_str().unwrap())
        };

        OpCorner { op_id: op_id, corner: corner }
    }
}


pub struct OffsetCoordinate {
    internal_coordinate: Rc<dyn Coordinate>,
    row_offset: u64,
    col_offset: u64
}

impl Coordinate for OffsetCoordinate {
    fn resolve(&self, table: &Table) -> Option<(u64, u64)> {
        match self.internal_coordinate.resolve(table) {
            Some((base_x, base_y)) => Some((base_x + self.row_offset, base_y + self.col_offset)),
            None => None
        }
    }
}

impl OffsetCoordinate {
    pub fn from_config(config: &Mapping) -> OffsetCoordinate{
        let row_offset = config["row_offset"].as_u64().unwrap();
        let col_offset = config["col_offset"].as_u64().unwrap();
        let internal_coordinate = coordinate_factory(
            config["internal_coordinate"].as_mapping().unwrap()).unwrap();

        OffsetCoordinate {
            internal_coordinate: internal_coordinate,
            row_offset: row_offset,
            col_offset: col_offset
        }
    }
}


pub fn coordinate_factory(config: &Mapping) -> Option<Rc<dyn Coordinate>> {
    let coord_type = config["type"].as_str().unwrap();
    match coord_type {
        "BaseCoordinate" => {
            let row = ordinate_factory(config["row"].as_mapping().unwrap());
            let col = ordinate_factory(config["col"].as_mapping().unwrap());

            Some(Rc::new(BaseCoordinate {row: row.unwrap(), col: col.unwrap()}))
        },

        "OffsetCoordinate" => {
            Some(Rc::new(OffsetCoordinate::from_config(config)))
        },

        "OpCorner" => Some(Rc::new(OpCorner::from_config(config))),

        _ => None
    }
}