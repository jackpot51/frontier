use std::collections::BTreeMap;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Block {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
    pub kind: String,
    pub data: BTreeMap<String, String>
}
