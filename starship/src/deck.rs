use block::Block;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Deck {
    pub name: String,
    pub blocks: Vec<Block>
}

#[derive(Debug)]
struct Node {
    i: usize,
    x: usize,
    y: usize,
    resource: String,
    amount: f32,
    capacity: f32,
}

impl Node {
    fn connected(&self, other: &Node) -> bool {
        return (self.x == other.x && self.y == other.y)      // same block
            || (self.x + 1 == other.x && self.y == other.y)  // to the left
            || (self.x == other.x + 1 && self.y == other.y)  // to the right
            || (self.x == other.x && self.y + 1 == other.y)  // above
            || (self.x == other.x && self.y == other.y + 1); // below
    }

    fn pressure(&self) -> f32 {
        return self.amount / self.capacity;
    }
}

#[derive(Debug)]
struct Change {
    i: usize,
    j: usize,
    pressure: f32
}

impl Deck {
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

        let resources = ["air", "electricity", "fuel", "water"];
        let mut nodes = vec![];

        // Create nodes from blocks
        for (i, mut block) in self.blocks.iter_mut().enumerate() {
            match block.kind.as_str() {
                // Find tanks and establish initial resource availability
                "Tank" =>  {
                    nodes.push(Node {
                        i: i,
                        x: block.x,
                        y: block.y,
                        resource: block.data.get("resource").map_or("", |s| &s).to_string(),
                        amount: block.data.get("amount").map_or("", |s| &s).parse::<f32>().unwrap_or(0.0),
                        capacity: block.data.get("capacity").map_or("", |s| &s).parse::<f32>().unwrap_or(0.0)
                    });
                },
                // Conduits transfer resources of all types, up to a certain rate
                "Conduit" => for resource in resources.iter() {
                    nodes.push(Node {
                        i: i,
                        x: block.x,
                        y: block.y,
                        resource: resource.to_string(),
                        amount: block.data.get(*resource).map_or("", |s| &s).parse::<f32>().unwrap_or(0.0),
                        capacity: block.data.get("capacity").map_or("", |s| &s).parse::<f32>().unwrap_or(0.0)
                    });
                },
                // Vents transfer air to a room
                "Vent" => {
                    let mut air = block.data.get("air").map_or("", |s| &s).parse::<f32>().unwrap_or(0.0);
                    let mut free_air = block.data.get("free_air").map_or("", |s| &s).parse::<f32>().unwrap_or(0.0);
                    let capacity = block.data.get("capacity").map_or("", |s| &s).parse::<f32>().unwrap_or(0.0);

                    if air > 0.0 && free_air < capacity {
                        let amount = air.min(capacity - free_air);
                        air -= amount;
                        free_air += amount;

                        block.data.insert("air".to_string(), format!("{}", air));
                        block.data.insert("free_air".to_string(), format!("{}", free_air));
                    }

                    nodes.push(Node {
                        i: i,
                        x: block.x,
                        y: block.y,
                        resource: "air".to_string(),
                        amount: air,
                        capacity: capacity
                    });

                    nodes.push(Node {
                        i: i,
                        x: block.x,
                        y: block.y,
                        resource: "free_air".to_string(),
                        amount: free_air,
                        capacity: capacity
                    });
                },
                // Floors require air
                "Floor" => {
                    nodes.push(Node {
                        i: i,
                        x: block.x,
                        y: block.y,
                        resource: "free_air".to_string(),
                        amount: block.data.get("free_air").map_or("", |s| &s).parse::<f32>().unwrap_or(0.0),
                        capacity: block.data.get("capacity").map_or("", |s| &s).parse::<f32>().unwrap_or(0.0)
                    });
                }
                _ => (),
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
        for node in nodes {
            let block = &mut self.blocks[node.i];
            match block.kind.as_str() {
                // Find tanks and establish initial resource availability
                "Tank" =>  {
                    block.data.insert("amount".to_string(), format!("{}", node.amount));
                },
                // Conduits transfer resources of all types, up to a certain rate
                "Conduit" | "Vent" | "Floor" => {
                    block.data.insert(node.resource, format!("{}", node.amount));
                }
                _ => (),
            }
        }

        redraw
    }
}
