use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Copy, Clone, Debug, Default, RustcDecodable, RustcEncodable)]
pub struct BlockResource {
    pub amount: f32,
    pub capacity: f32
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Block<'a> {
    pub x: usize,
    pub y: usize,
    pub kind: String,
    pub resources: BTreeMap<Cow<'a, str>, BlockResource>
}

impl<'a> Block<'a> {
    pub fn new(x: usize, y: usize, kind: String) -> Block<'a> {
        Block {
            x: x,
            y: y,
            kind: kind,
            resources: BTreeMap::new()
        }
    }
}
