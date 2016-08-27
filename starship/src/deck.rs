use std::collections::BTreeMap;
use std::cmp::min;

use block::Block;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Deck {
    pub name: String,
    pub blocks: Vec<Block>
}

impl Deck {
    pub fn update(&mut self) {
        let mut tanks = vec![];
        //let mut conduits = vec![];

        // Find tanks and establish initial resource availability
        for (i, block) in self.blocks.iter().enumerate() {
            if block.kind == "Tank" {
                tanks.push((i, Tank::new(block.x, block.y, &block.data)));
            }
        }

        // Use conduits to transfer resource availability. Loop until no changes detected
        let mut changed = true;
        while changed {
            changed = false;
            for mut block in self.blocks.iter_mut() {
                if block.kind == "Conduit" {
                    let mut conduit = Conduit::new(block.x, block.y, &block.data);
                    for &mut (_tank_i, ref mut tank) in tanks.iter_mut() {
                        if adjacent(block.x, block.y, tank.x, tank.y) {
                            match tank.resource.as_str() {
                                "air" => if tank.current > 0 && conduit.air < conduit.rate {
                                    let amount = min(tank.current, conduit.rate - conduit.air);
                                    tank.current -= amount;
                                    conduit.air += amount;
                                    changed = true;
                                },
                                "electricity" => if tank.current > 0 && conduit.electricity < conduit.rate {
                                    let amount = min(tank.current, conduit.rate - conduit.electricity);
                                    tank.current -= amount;
                                    conduit.electricity += amount;
                                    changed = true;
                                },
                                "fuel" => if tank.current > 0 && conduit.fuel < conduit.rate {
                                    let amount = min(tank.current, conduit.rate - conduit.fuel);
                                    tank.current -= amount;
                                    conduit.fuel += amount;
                                    changed = true;
                                },
                                "water" => if tank.current > 0 && conduit.water < conduit.rate {
                                    let amount = min(tank.current, conduit.rate - conduit.water);
                                    tank.current -= amount;
                                    conduit.water += amount;
                                    changed = true;
                                },
                                _ => ()
                            }
                        }
                    }
                    conduit.save(&mut block.data);
                }
            }
        }

        // If resource availability touches a vacuum, consume that resource

        // Update the original tanks
        for (tank_i, tank) in tanks {
            tank.save(&mut self.blocks[tank_i].data);
        }

        //println!("{:#?}", tanks);
    }
}

fn adjacent(x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    return (x1 == x2 || x1 + 1 == x2 || x1 == x2 + 1)
        && (y1 == y2 || y1 + 1 == y2 || y1 == y2 + 1);
}

#[derive(Debug)]
struct Conduit {
    x: usize,
    y: usize,
    rate: usize,
    air: usize,
    electricity: usize,
    fuel: usize,
    water: usize,
}

impl Conduit {
    fn new(x: usize, y: usize, data: &BTreeMap<String, String>) -> Conduit {
        Conduit {
            x: x,
            y: y,
            rate: data.get("rate").map_or("", |s| &s).parse::<usize>().unwrap_or(0),
            air: data.get("air").map_or("", |s| &s).parse::<usize>().unwrap_or(0),
            electricity: data.get("electricity").map_or("", |s| &s).parse::<usize>().unwrap_or(0),
            fuel: data.get("fuel").map_or("", |s| &s).parse::<usize>().unwrap_or(0),
            water: data.get("water").map_or("", |s| &s).parse::<usize>().unwrap_or(0),
        }
    }

    fn save(self, data: &mut BTreeMap<String, String>) {
        data.insert("rate".to_string(), format!("{}", self.rate));
        data.insert("air".to_string(), format!("{}", self.air));
        data.insert("electricity".to_string(), format!("{}", self.electricity));
        data.insert("fuel".to_string(), format!("{}", self.fuel));
        data.insert("water".to_string(), format!("{}", self.water));
    }
}

#[derive(Debug)]
struct Tank {
    x: usize,
    y: usize,
    resource: String,
    rate: usize,
    current: usize,
    total: usize
}

impl Tank {
    fn new(x: usize, y: usize, data: &BTreeMap<String, String>) -> Tank {
        Tank {
            x: x,
            y: y,
            resource: data.get("resource").map_or("", |s| &s).to_string(),
            rate: data.get("rate").map_or("", |s| &s).parse::<usize>().unwrap_or(0),
            current: data.get("current").map_or("", |s| &s).parse::<usize>().unwrap_or(0),
            total: data.get("total").map_or("", |s| &s).parse::<usize>().unwrap_or(0),
        }
    }

    fn save(self, data: &mut BTreeMap<String, String>) {
        data.insert("resource".to_string(), format!("{}", self.resource));
        data.insert("rate".to_string(), format!("{}", self.rate));
        data.insert("current".to_string(), format!("{}", self.current));
        data.insert("total".to_string(), format!("{}", self.total));
    }
}
