use deck::Deck;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Ship<'a> {
    pub name: String,
    pub current_deck: usize,
    pub decks: Vec<Deck<'a>>
}

impl<'a> Ship<'a> {
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
