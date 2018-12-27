#![feature(custom_attribute)]
#![feature(try_from)]
#![feature(get_type_id)]

pub mod interpreter;
pub mod model;
pub mod parsers;

pub use crate::parsers::parse;
pub use crate::interpreter::evaluate;
