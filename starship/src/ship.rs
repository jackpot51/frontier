use deck::Deck;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Ship {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub decks: Vec<Deck>
}

impl Ship {
    pub fn update(&mut self) -> bool {
        let mut redraw = false;
        for deck in self.decks.iter_mut() {
            if deck.update() {
                redraw = true;
            }
        }
        redraw
    }
}
