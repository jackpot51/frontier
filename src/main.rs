extern crate orbclient;
extern crate orbfont;
extern crate orbimage;
extern crate starship;

use orbclient::{Color, EventOption, Window, K_UP, K_DOWN, K_ESC};
use orbfont::Font;
use orbimage::Image;

use std::collections::BTreeMap;
use std::fs;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main(){
    let mut window = Window::new_flags(100, 100, 640, 480, "Frontier", true).unwrap();
    let font = Font::from_path("res/FiraMono-Regular.ttf").unwrap();

    let ship_lock = Arc::new(Mutex::new(starship::load("res/ship.json").unwrap()));

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

    assert!(ship_lock.lock().unwrap().decks.len() > 0);

    let running = Arc::new(AtomicBool::new(true));
    let redraw = Arc::new(AtomicBool::new(true));

    let running_update = running.clone();
    let redraw_update = redraw.clone();
    let ship_update = ship_lock.clone();
    let handle = thread::spawn(move || {
        while running_update.load(Ordering::SeqCst) {
            {
                let mut ship = ship_update.lock().unwrap();
                if ship.update() {
                    redraw_update.store(true, Ordering::SeqCst);
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    let mut deck_i = 0;
    let mut mouse_down = false;

    while running.load(Ordering::SeqCst) {
        {
            let ship = ship_lock.lock().unwrap();
            let deck = &ship.decks[deck_i];

            if redraw.load(Ordering::SeqCst) {
                redraw.store(false, Ordering::SeqCst);

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
                            text.draw(&mut window, x, y, Color::rgb(0, 0, 0));
                        }

                        if let Some(amount) = block.data.get("amount") {
                            let text = font.render(amount, 16.0);
                            text.draw(&mut window, x, y + 16, Color::rgb(0, 0, 0));
                        }
                    }
                }

                window.sync();
            }

            let mut evented = true;
            while evented {
                evented = false;

                for event in window.events() {
                    evented = true;

                    match event.to_option() {
                        EventOption::Key(key_event) => if key_event.pressed {
                            match key_event.scancode {
                                K_UP => if deck_i + 1 < ship.decks.len() {
                                    deck_i += 1;
                                },
                                K_DOWN => if deck_i > 0 {
                                    deck_i -= 1;
                                },
                                K_ESC => running.store(false, Ordering::SeqCst),
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
                        EventOption::Quit(_quit_event) => running.store(false, Ordering::SeqCst),
                        _ => ()
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(10));
    }

    handle.join();
}
