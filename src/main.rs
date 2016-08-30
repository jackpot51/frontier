extern crate orbclient;
extern crate orbfont;
extern crate orbimage;
extern crate starship;

use orbclient::{Color, EventOption, Window, K_UP, K_DOWN, K_DEL, K_ESC};
use orbfont::Font;
use orbimage::Image;

use starship::block::{Block, BlockResource};

use std::collections::BTreeMap;
use std::cmp::max;
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
    let mut dragging = None;
    let mut editing = None;

    while running.load(Ordering::SeqCst) {
        {
            let mut ship = ship_lock.lock().unwrap();
            let name = ship.name.clone();
            let deck_len = ship.decks.len();
            let deck = &mut ship.decks[deck_i];

            if redraw.load(Ordering::SeqCst) {
                redraw.store(false, Ordering::SeqCst);

                let window_w = window.width();
                //let window_h = window.height();

                window.set(Color::rgb(255, 255, 255));

                let title = font.render(&format!("{} - {} - {}", name, deck_i, deck.name), 24.0);
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
                        if let Some((name, resource)) = block.resources.iter().next() {
                            let text = font.render(name, 16.0);
                            text.draw(&mut window, x, y, Color::rgb(0, 0, 0));

                            let text = font.render(&format!("{}", resource.amount as u32), 16.0);
                            text.draw(&mut window, x, y + 16, Color::rgb(0, 0, 0));
                        }
                    } else if block.kind == "Vent" {
                        if let Some(resource) = block.resources.get("air") {
                            let text = font.render(&format!("{}", resource.amount as u32), 16.0);
                            text.draw(&mut window, x, y, Color::rgb(0, 0, 0));
                        }
                        if let Some(resource) = block.resources.get("free_air") {
                            let text = font.render(&format!("{}", resource.amount as u32), 16.0);
                            text.draw(&mut window, x, y + 16, Color::rgb(0, 0, 0));
                        }
                    } else if block.kind == "Deck" {
                        if let Some(resource) = block.resources.get("free_air") {
                            let text = font.render(&format!("{}", resource.amount as u32), 16.0);
                            text.draw(&mut window, x, y + 16, Color::rgb(0, 0, 0));
                        }
                    }
                }

                if let Some(i) = dragging {
                    if let Some(block) = deck.blocks.get(i) {
                        let x = block.x as i32 * 32;
                        let y = block.y as i32 * 32 + 32;

                        window.rect(x, y, 32, 2, Color::rgb(255, 0, 0));
                        window.rect(x, y, 2, 32, Color::rgb(255, 0, 0));
                        window.rect(x, y + 30, 32, 2, Color::rgb(255, 0, 0));
                        window.rect(x + 30, y, 2, 32, Color::rgb(255, 0, 0));
                    }
                }

                if let Some((block_x, block_y)) = editing {
                    let x = block_x as i32 * 32;
                    let y = block_y as i32 * 32 + 32;

                    window.rect(x, y, 32, 2, Color::rgb(0, 0, 255));
                    window.rect(x, y, 2, 32, Color::rgb(0, 0, 255));
                    window.rect(x, y + 30, 32, 2, Color::rgb(0, 0, 255));
                    window.rect(x + 30, y, 2, 32, Color::rgb(0, 0, 255));
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
                                K_UP => if deck_i + 1 < deck_len {
                                    deck_i += 1;
                                    redraw.store(true, Ordering::SeqCst);
                                },
                                K_DOWN => if deck_i > 0 {
                                    deck_i -= 1;
                                    redraw.store(true, Ordering::SeqCst);
                                },
                                K_DEL => if let Some((block_x, block_y)) = editing.take() {
                                    let mut remove = None;
                                    for (i, block) in deck.blocks.iter().enumerate() {
                                        if block_x == block.x && block_y == block.y {
                                            remove = Some(i);
                                        }
                                    }
                                    if let Some(i) = remove {
                                        deck.blocks.remove(i);
                                    }
                                },
                                K_ESC => editing = None,
                                _ => {
                                    match key_event.character {
                                        'A' | 'a' => if let Some((block_x, block_y)) = editing.take() {
                                            let mut block = Block::new(block_x, block_y, "Tank".to_string());
                                            block.resources.insert("air".into(), BlockResource { amount: 100.0, capacity: 100.0 });
                                            deck.blocks.push(block);
                                        },
                                        'E' | 'e' => if let Some((block_x, block_y)) = editing.take() {
                                            let mut block = Block::new(block_x, block_y, "Tank".to_string());
                                            block.resources.insert("electricity".into(), BlockResource { amount: 100.0, capacity: 100.0 });
                                            deck.blocks.push(block);
                                        },
                                        'F' | 'f' => if let Some((block_x, block_y)) = editing.take() {
                                            let mut block = Block::new(block_x, block_y, "Tank".to_string());
                                            block.resources.insert("fuel".into(), BlockResource { amount: 100.0, capacity: 100.0 });
                                            deck.blocks.push(block);
                                        },
                                        'W' | 'w' => if let Some((block_x, block_y)) = editing.take() {
                                            let mut block = Block::new(block_x, block_y, "Tank".to_string());
                                            block.resources.insert("water".into(), BlockResource { amount: 100.0, capacity: 100.0 });
                                            deck.blocks.push(block);
                                        },
                                        'C' | 'c' => if let Some((block_x, block_y)) = editing.take() {
                                            let mut block = Block::new(block_x, block_y, "Conduit".to_string());
                                            block.resources.insert("air".into(), BlockResource { amount: 0.0, capacity: 5.0 });
                                            block.resources.insert("electricity".into(), BlockResource { amount: 0.0, capacity: 5.0 });
                                            block.resources.insert("fuel".into(), BlockResource { amount: 0.0, capacity: 5.0 });
                                            block.resources.insert("water".into(), BlockResource { amount: 0.0, capacity: 5.0 });
                                            deck.blocks.push(block);
                                        },
                                        'D' | 'd' => if let Some((block_x, block_y)) = editing.take() {
                                            let mut block = Block::new(block_x, block_y, "Deck".to_string());
                                            block.resources.insert("free_air".into(), BlockResource { amount: 0.0, capacity: 5.0 });
                                            deck.blocks.push(block);
                                        },
                                        'H' | 'h' => if let Some((block_x, block_y)) = editing.take() {
                                            deck.blocks.push(Block::new(block_x, block_y, "Hull".to_string()));
                                        },
                                        'V' | 'v' => if let Some((block_x, block_y)) = editing.take() {
                                            let mut block = Block::new(block_x, block_y, "Vent".to_string());
                                            block.resources.insert("air".into(), BlockResource { amount: 0.0, capacity: 5.0 });
                                            block.resources.insert("free_air".into(), BlockResource { amount: 0.0, capacity: 5.0 });
                                            deck.blocks.push(block);
                                        },
                                        _ => ()
                                    }
                                }
                            }
                        },
                        EventOption::Mouse(mouse_event) => {
                            if mouse_event.left_button {
                                if let Some(i) = dragging {
                                    let x = max(mouse_event.x/32, 0) as usize;
                                    let y = max((mouse_event.y - 32)/32, 0) as usize;

                                    let block = &mut deck.blocks[i];
                                    if block.x != x || block.y != y {
                                        block.x = x;
                                        block.y = y;
                                        redraw.store(true, Ordering::SeqCst);
                                    }
                                } else {
                                    println!("Left {}, {}", mouse_event.x, mouse_event.y);

                                    for (i, block) in deck.blocks.iter().enumerate() {
                                        let x = block.x as i32 * 32;
                                        let y = block.y as i32 * 32 + 32;
                                        if mouse_event.x >= x && mouse_event.x < x + 32 && mouse_event.y >= y && mouse_event.y < y + 32 {
                                            dragging = Some(i);
                                            println!("    {:?}", block);
                                        }
                                    }
                                }
                            } else {
                                dragging = None;
                            }

                            if mouse_event.right_button {
                                let x = max(mouse_event.x/32, 0) as usize;
                                let y = max((mouse_event.y - 32)/32, 0) as usize;

                                if editing != Some((x, y)) {
                                    println!("Right {}, {}", mouse_event.x, mouse_event.y);
                                    editing = Some((x, y));

                                    for block in deck.blocks.iter() {
                                        let x = block.x as i32 * 32;
                                        let y = block.y as i32 * 32 + 32;
                                        if mouse_event.x >= x && mouse_event.x < x + 32 && mouse_event.y >= y && mouse_event.y < y + 32 {
                                            println!("    {:?}", block);
                                        }
                                    }
                                }
                            }
                        },
                        EventOption::Quit(_quit_event) => running.store(false, Ordering::SeqCst),
                        _ => ()
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(10));
    }

    handle.join().unwrap();

    starship::save("res/ship_new.json", &*ship_lock.lock().unwrap()).unwrap();
}
