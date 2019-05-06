use crate::cell_matrix::{Cell, Map};
use crate::corridor_tree::{remove_node, CorridorNode, WrappedCorridorNode};
use crate::room::Corridor;
use crate::sections::Section;

use rand::prelude::ThreadRng;
use rand::thread_rng;
use rand::Rng;

use crate::direction::Direction;

pub struct LabyrinthGenerator {
    map: Map,
    corridor_width: u8,
    corridor_height: u8,
    corridor_errantness: f32,
    margins: (u8, u8),
    rng: ThreadRng,
}

impl LabyrinthGenerator {
    pub fn new(
        map: Map,
        corridor_width: u8,
        corridor_height: u8,
        corridor_errantness: f32,
        margins: (u8, u8),
    ) -> LabyrinthGenerator {
        return LabyrinthGenerator {
            map,
            corridor_width,
            corridor_height,
            corridor_errantness,
            margins,
            rng: thread_rng(),
        };
    }
    pub fn generate(mut self) -> Map {
        // let mut corridor_vector: Vec<Corridor> = vec![];
        let mut root_nodes: Vec<WrappedCorridorNode> = vec![];
        'suitable: loop {
            match self.find_suitable_corridor_location() {
                Ok((x, y)) => {
                    let idx = self.map.add_corridor();
                    root_nodes.push(self.traverse_corridor(x, y, Direction::rand(), idx, None));
                }
                Err(_) => break 'suitable,
            };
        }
        // print tree
        /* for root_node in root_nodes.clone() {
            println!("--------------------------------");
            print!("({}, {})\n", root_node.borrow().x, root_node.borrow().y);
            self.print_tree_children(&root_node);
        }*/
        // Remove all corridors that are too small
        // self.prune_node_tree(root_nodes);
        self.map.corridor_tree = root_nodes;
        return self.map;
    }

    fn print_tree_children(&self, node: &WrappedCorridorNode) {
        for child in &node.borrow().children {
            println!("˅˅˅˅˅˅˅˅˅˅˅˅˅˅˅˅");
            print!("Child({}, {}) ", child.borrow().x, child.borrow().y);
            self.print_tree_children(child);
            println!("˄˄˄˄˄˄˄˄˄˄˄˄˄˄˄˄");
        }
    }

    fn find_suitable_corridor_location(&mut self) -> Result<(u16, u16), String> {
        let start_x = self.rng.gen_range(0, self.map.width);
        let start_y = self.rng.gen_range(0, self.map.height);

        for y in 0..(self.map.height - (1 + self.corridor_height) as u16) {
            for x in 0..(self.map.width - (1 + self.corridor_width) as u16) {
                let x_pos = (x + start_x) % self.map.width;
                let y_pos = (y + start_y) % self.map.height;

                if is_suitable_corridor_location(
                    &self.map,
                    x_pos as i32,
                    y_pos as i32,
                    self.corridor_width,
                    self.corridor_width,
                    (
                        self.margins.0,
                        self.margins.1,
                        self.margins.0,
                        self.margins.1,
                    ),
                ) {
                    return Ok((x_pos, y_pos));
                }
            }
        }
        return Err(String::from(
            "Could not find suitable corridor on cellmatrix",
        ));
    }

    fn add_corridor_piece(
        &mut self,
        cell: Cell,
        x: u16,
        y: u16,
        parent: Option<&WrappedCorridorNode>,
    ) -> WrappedCorridorNode {
        self.map.set_rect(
            cell,
            x,
            y,
            self.corridor_width as u16,
            self.corridor_height as u16,
        );
        return CorridorNode::new(parent, x, y);
    }
    /**
     * Recursive corridor logic
     */
    fn traverse_corridor(
        &mut self,
        x: u16,
        y: u16,
        // horizontal / vertical
        direction: Direction,
        corridor_index: usize,
        parent: Option<&WrappedCorridorNode>,
    ) -> WrappedCorridorNode {
        let mut direction = match thread_rng().gen::<f32>() {
            x if x > self.corridor_errantness => Direction::rand(),
            _ => direction,
        };
        // Start the labyrinth algorithm here
        // This just sets a corridor piece at the starting location
        let node = self.add_corridor_piece(Cell::Corridor(corridor_index), x, y, parent);

        let mut direction_pool = vec![Direction::E, Direction::N, Direction::S, Direction::W];
        for _ in 0..3 {
            match direction {
                Direction::N => {
                    if is_suitable_corridor_location(
                        &self.map,
                        x as i32,
                        y as i32 - 1,
                        self.corridor_width,
                        1,
                        (self.margins.0, self.margins.1, self.margins.0, 0),
                    ) {
                        self.traverse_corridor(
                            x,
                            y - 1,
                            direction.clone(),
                            corridor_index,
                            Some(&node),
                        );
                    }
                }
                Direction::E => {
                    let x_new = x + self.corridor_width as u16;
                    if is_suitable_corridor_location(
                        &self.map,
                        x_new as i32,
                        y as i32,
                        1,
                        self.corridor_height,
                        (0, self.margins.1, self.margins.0, self.margins.1),
                    ) {
                        self.traverse_corridor(
                            x + 1,
                            y,
                            direction.clone(),
                            corridor_index,
                            Some(&node),
                        );
                    }
                }
                Direction::S => {
                    let y_new = y + self.corridor_height as u16;
                    if is_suitable_corridor_location(
                        &self.map,
                        x as i32,
                        y_new as i32,
                        self.corridor_width,
                        1,
                        (self.margins.0, 0, self.margins.0, self.margins.1),
                    ) {
                        self.traverse_corridor(
                            x,
                            y + 1,
                            direction.clone(),
                            corridor_index,
                            Some(&node),
                        );
                    }
                }
                Direction::W => {
                    if is_suitable_corridor_location(
                        &self.map,
                        x as i32 - 1,
                        y as i32,
                        1,
                        self.corridor_height,
                        (self.margins.0, self.margins.1, 0, self.margins.1),
                    ) {
                        self.traverse_corridor(
                            x - 1,
                            y,
                            direction.clone(),
                            corridor_index,
                            Some(&node),
                        );
                    }
                }
            }
            let idx = direction_pool.iter().position(|d| *d == direction).unwrap();
            direction_pool.remove(idx);
            if direction_pool.len() == 1 {
                direction = direction_pool[0];
            } else {
                direction = direction_pool[self.rng.gen_range(0, direction_pool.len()) as usize];
            }
        }
        return node;
    }
}

fn is_suitable_corridor_location(
    map: &Map,
    x: i32,
    y: i32,
    width: u8,
    height: u8,
    // Margins doesn't check for position outside of map
    // left, top, right, bottom
    margins: (u8, u8, u8, u8),
) -> bool {
    let margin_rect = map.get_rect(
        x - margins.0 as i32,
        y - margins.1 as i32,
        width as u16 + (margins.2 + margins.0) as u16,
        height as u16 + (margins.3 + margins.1) as u16,
    );
    if margin_rect
        .cell_vector
        .iter()
        .any(|c| *c != Cell::SolidRock && !c.is_rock())
    {
        return false;
    }
    // Check the inner rectangle as well
    let cell_rect = map.get_rect(x, y, width as u16, height as u16);
    if cell_rect.cell_vector.iter().any(|c| !c.is_rock()) {
        return false;
    }
    return true;
}
