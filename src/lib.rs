//! # chemistru-raw
//! 
//! Raw elemental data loaded from JSON, with no data cleansing

lazy_static::lazy_static! {
    pub static ref ELEMENTS: Vec<chemistru_elements::raw::RawElement> = serde_json::from_str(include_str!("../periodic-table-data/periodic-table.json")).unwrap();
}
