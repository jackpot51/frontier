extern crate orbclient;
extern crate orbfont;
extern crate starship;

use orbclient::{Color, EventOption, Window, K_UP, K_DOWN};
use orbfont::Font;

fn main(){
    let mut window = Window::new(100, 100, 640, 480, "Frontier").unwrap();
    let font = Font::find(None, None, None).unwrap();

    let ship = starship::load("res/ship.json").unwrap();

    assert!(ship.decks.len() > 0);

    println!("{:#?}", ship);

    let mut deck_i = 0;

    'events: loop {
        let deck = &ship.decks[deck_i];

        window.set(Color::rgb(255, 255, 255));
        font.render(&format!("{} - {} - {}", ship.name, deck_i, deck.name), 16.0).draw(&mut window, 0, 0, Color::rgb(0, 0, 0));
        for block in deck.blocks.iter() {
            let x = block.x as i32 * 32;
            let y = block.y as i32 * 32 + 16;
            let w = block.w as u32 * 32;
            let h = block.h as u32 * 32;
            window.rect(x, y, w, h, Color::rgb(128, 128, 128));
            font.render(&block.kind, 16.0).draw(&mut window, x, y, Color::rgb(0, 0, 0));
        }
        window.sync();

        for event in window.events() {
            match event.to_option() {
                EventOption::Key(key_event) => if key_event.pressed {
                    match key_event.scancode {
                        K_UP => if deck_i + 1 < ship.decks.len() {
                            deck_i += 1;
                        },
                        K_DOWN => if deck_i > 0 {
                            deck_i -= 1;
                        },
                        _ => ()
                    }
                },
                EventOption::Quit(_quit_event) => break 'events,
                _ => ()
            }
        }
    }
}
