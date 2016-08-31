use std::borrow::Cow;

use block::Block;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Deck<'a> {
    pub name: String,
    pub blocks: Vec<Block<'a>>
}

#[derive(Debug)]
struct Node<'a> {
    i: usize,
    x: usize,
    y: usize,
    resource: Cow<'a, str>,
    amount: f64,
    capacity: f64,
}

impl<'a> Node<'a> {
    fn connected(&self, other: &Node) -> bool {
        return (self.x == other.x && self.y == other.y)      // same block
            || (self.x + 1 == other.x && self.y == other.y)  // to the left
            || (self.x == other.x + 1 && self.y == other.y)  // to the right
            || (self.x == other.x && self.y + 1 == other.y)  // above
            || (self.x == other.x && self.y == other.y + 1); // below
    }

    fn pressure(&self) -> f64 {
        return self.amount / self.capacity;
    }
}

#[derive(Debug)]
struct Change {
    i: usize,
    j: usize,
    pressure: f64
}

impl<'a> Deck<'a> {
    /// # Update the deck
    /// - First, identify resource movement using the following algorithm, repeated until complete:
    ///   - Fill conduits from connected tanks until rate is fulfilled or tanks are drained
    ///   - Drain conduits into nearby consumers, such as air ducts
    ///   - Drain air ducts into rooms, until hitting hulls or force fields
    /// - Next, identify sensor triggers
    ///   - Any sensors that detect low presure will send an alert on the conduits
    ///   - That alert will propogate to nearby computer consoles
    pub fn update(&mut self) -> bool {
        let mut redraw = false;

        let mut nodes = vec![];

        // Create nodes from blocks
        for (i, mut block) in self.blocks.iter_mut().enumerate() {
            // Vents transfer air to a room
            if block.kind == "Vent" {
                if block.resources.contains_key("air") && block.resources.contains_key("free_air") {
                    let mut air = block.resources["air"].amount;
                    let mut free_air = block.resources["free_air"].amount;
                    let capacity = block.resources["free_air"].capacity;
                    if air > 0.0 && free_air < capacity {
                        let amount = air.min(capacity - free_air);
                        air -= amount;
                        free_air += amount;

                        if let Some(mut resource) = block.resources.get_mut("air") {
                            resource.amount = air;
                        }

                        if let Some(mut resource) = block.resources.get_mut("free_air") {
                            resource.amount = free_air;
                        }
                    }
                }
            }

            for (name, resource) in block.resources.iter() {
                nodes.push(Node {
                    i: i,
                    x: block.x,
                    y: block.y,
                    resource: name.clone(),
                    amount: resource.amount,
                    capacity: resource.capacity
                });
            }
        }

        // Create change list
        let mut changes = vec![];
        for i in 0 .. nodes.len() {
            // Calculate changes for this node
            let a = &nodes[i];
            let mut node_changes = vec![];
            //let mut total = 0.0;
            for j in 0 .. nodes.len() {
                if j != i {
                    let b = &nodes[j];
                    if a.connected(b) && a.resource == b.resource && a.pressure() > b.pressure() {
                        //println!("{}, {} > {}, {}. {} > {}. {} > {}", a.x, a.y, b.x, b.y, i, j, a.pressure(), b.pressure());
                        let difference = (a.pressure() - b.pressure())/2.0;
                        node_changes.push((j, difference));
                        //total += difference;
                    }
                }
            }

            //let normalize = a.pressure()/total;

            // Normalize changes, so that two competing draws on one resource are shared
            for (j, pressure) in node_changes {
                changes.push(Change {
                    i: i,
                    j: j,
                    pressure: pressure
                });
            }
        }

        // Apply changes
        for change in changes {
            redraw = true;

            let amount = change.pressure.max(0.0).min(1.0) * nodes[change.i].amount.min(nodes[change.j].capacity - nodes[change.j].amount);
            nodes[change.i].amount -= amount;
            nodes[change.j].amount += amount;
        }

        // Update blocks from nodes
        for mut node in nodes {
            //If free air, check for vacuums
            if node.resource == "free_air" && node.amount > 0.0 {
                let ox = node.x as isize - 1;
                let oy = node.y as isize - 1;

                let mut neighbors: [[usize; 3]; 3] = [
                    [0, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0]
                ];

                for block in self.blocks.iter() {
                    let i = block.x as isize - ox;
                    let j = block.y as isize - oy;
                    if i >= 0 && i < 3 && j >= 0 && j < 3 {
                        neighbors[j as usize][i as usize] += 1;
                    }
                }

                let mut vacuums = 0.0;
                for (y, neighbor_row) in neighbors.iter().enumerate() {
                    for (x, &neighbor) in neighbor_row.iter().enumerate() {
                        if neighbor == 0 {
                            vacuums += 1.0/((x as f64 - 1.0).powf(2.0) + (y as f64 - 1.0).powf(2.0)).sqrt();
                        }
                    }
                }

                if vacuums > 0.0 {
                    let decay = 1.0 - vacuums/100.0;
                    //println!("{}, {}: {} vacuums, {} amount, {} decay", node.x, node.y, vacuums, node.amount, decay);
                    node.amount *= decay;
                }
            }

            //Update block
            if let Some(mut resource) = self.blocks[node.i].resources.get_mut(&node.resource) {
                resource.amount = node.amount;
            }
        }

        redraw
    }
}
