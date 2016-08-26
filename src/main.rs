extern crate orbclient;
extern crate orbfont;
extern crate starship;

use orbclient::{Color, EventOption, Window, K_UP, K_DOWN};
use orbfont::Font;

fn main(){
    let mut window = Window::new(100, 100, 640, 480, "Frontier").unwrap();
    let font = Font::from_path("res/FiraMono-Regular.ttf").unwrap();

    let ship = starship::load("res/ship.json").unwrap();

    assert!(ship.decks.len() > 0);

    println!("{:#?}", ship);

    let mut deck_i = 0;

    'events: loop {
        let deck = &ship.decks[deck_i];

        let window_w = window.width();
        let window_h = window.height();

        window.set(Color::rgb(255, 255, 255));

        let title = font.render(&format!("{} - {} - {}", ship.name, deck_i, deck.name), 24.0);
        let title_x = (window_w - title.width()) as i32/2;
        title.draw(&mut window, title_x, 0, Color::rgb(0, 0, 0));

        window.rect(0, 26, window_w, 2, Color::rgb(0, 0, 0));

        for block in deck.blocks.iter() {
            let x = block.x as i32 * 32;
            let y = block.y as i32 * 32 + 32;
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
