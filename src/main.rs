extern crate orbclient;
extern crate orbfont;
extern crate orbimage;
extern crate starship;

use orbclient::{Color, EventOption, Window, K_UP, K_DOWN};
use orbfont::Font;
use orbimage::Image;

use std::collections::BTreeMap;
use std::fs;

fn main(){
    let mut window = Window::new(100, 100, 640, 480, "Frontier").unwrap();
    let font = Font::from_path("res/FiraMono-Regular.ttf").unwrap();

    let mut ship = starship::load("res/ship.json").unwrap();

    let mut block_kinds: BTreeMap<String, Image> = BTreeMap::new();
    for entry_result in fs::read_dir("res/blocks/").unwrap() {
        let entry = entry_result.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let mut image_path = path.clone();
            image_path.push("image.png");
            if image_path.is_file() {
                block_kinds.insert(entry.file_name().into_string().unwrap(), Image::from_path(&image_path).unwrap());
            }
        }
    }

    assert!(ship.decks.len() > 0);

    println!("{:#?}", ship);

    let mut deck_i = 0;
    let mut mouse_down = false;

    'events: loop {
        ship.update();

        let deck = &ship.decks[deck_i];

        let window_w = window.width();
        //let window_h = window.height();

        window.set(Color::rgb(255, 255, 255));

        let title = font.render(&format!("{} - {} - {}", ship.name, deck_i, deck.name), 24.0);
        let title_x = (window_w - title.width()) as i32/2;
        title.draw(&mut window, title_x, 0, Color::rgb(0, 0, 0));

        window.rect(0, 26, window_w, 2, Color::rgb(0, 0, 0));

        for block in deck.blocks.iter() {
            let x = block.x as i32 * 32;
            let y = block.y as i32 * 32 + 32;
            if let Some(image) = block_kinds.get(&block.kind) {
                image.draw(&mut window, x, y);
            } else {
                window.rect(x, y, 32, 32, Color::rgb(128, 128, 128));
                font.render(&block.kind, 16.0).draw(&mut window, x, y, Color::rgb(0, 0, 0));
            }

            if block.kind == "Tank" {
                if let Some(resource) = block.data.get("resource") {
                    let text = font.render(resource, 16.0);
                    let text_x = x + (32 - text.width()) as i32/2;
                    let text_y = y + (32 - text.height()) as i32/2;
                    text.draw(&mut window, text_x, text_y, Color::rgb(0, 0, 0));
                }
            }
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
                EventOption::Mouse(mouse_event) => if mouse_event.left_button {
                    if ! mouse_down {
                        println!("Click {}, {}", mouse_event.x, mouse_event.y);
                        for block in deck.blocks.iter() {
                            let x = block.x as i32 * 32;
                            let y = block.y as i32 * 32 + 32;
                            if mouse_event.x >= x && mouse_event.x < x + 32 && mouse_event.y >= y && mouse_event.y < y + 32 {
                                println!("    {:?}", block);
                            }
                        }
                    }
                    mouse_down = true;
                } else {
                    mouse_down = false;
                },
                EventOption::Quit(_quit_event) => break 'events,
                _ => ()
            }
        }
    }
}
