use block::Block;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Deck {
    pub name: String,
    pub blocks: Vec<Block>
}
