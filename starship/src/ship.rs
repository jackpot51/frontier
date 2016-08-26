use deck::Deck;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Ship {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub decks: Vec<Deck>
}
