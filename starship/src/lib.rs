#![feature(question_mark)]

extern crate rustc_serialize;

use rustc_serialize::json::{self, DecoderError};
use std::fs::File;
use std::io::Read;

pub mod block;
pub mod deck;
pub mod ship;

pub fn load(path: &str) -> Result<ship::Ship, DecoderError> {
    let mut file = File::open(path).map_err(|err| DecoderError::ApplicationError(format!("{}", err)))?;

    let mut string = String::new();
    file.read_to_string(&mut string).map_err(|err| DecoderError::ApplicationError(format!("{}", err)))?;

    let ship: ship::Ship = json::decode(&string)?;
    Ok(ship)
}
