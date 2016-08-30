#![feature(question_mark)]

extern crate rustc_serialize;

use rustc_serialize::json;
use std::fs::File;
use std::io::{Error, ErrorKind, Result, Read, Write};

pub mod block;
pub mod deck;
pub mod ship;

pub fn load(path: &str) -> Result<ship::Ship> {
    let mut file = File::open(path)?;

    let mut string = String::new();
    file.read_to_string(&mut string)?;

    let ship: ship::Ship = json::decode(&string).map_err(|err| Error::new(ErrorKind::Other, format!("{}", err)))?;
    Ok(ship)
}

pub fn save(path: &str, ship: &ship::Ship) -> Result<()> {
    let mut file = File::create(path)?;

    let encoder = json::as_pretty_json(ship);
    write!(file, "{}", encoder)?;
    Ok(())
}
