use crate::cell_matrix::{Cell, Map};
use crate::corridor_tree::{remove_node, WrappedCorridorNode};
use crate::direction::Direction;
use crate::room::Room;

#[derive(Clone, Copy, PartialEq)]
pub struct Connection {
    pub x: u16,
    pub y: u16,
    pub id: usize,
    pub score: f32,
    pub direction: Direction,
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Print the map beautifully
        return write!(
            f,
            "x: {}, y: {}, id: {}, score: {}, direction: {}",
            self.x, self.y, self.id, self.score, self.direction
        );
    }
}

#[derive(Clone)]
pub struct Section {
    id: usize,
    pub connections: Vec<Connection>,
}

pub trait Sectionable {
    fn get_section(&self) -> &Section;
    fn get_section_mut(&mut self) -> &mut Section;
}

impl Section {
    pub fn new(id: usize) -> Self {
        return Section {
            id,
            connections: vec![],
        };
    }
    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    pub fn get_id(&self) -> usize {
        return self.id;
    }
    pub fn add_connection(&mut self, x: u16, y: u16, id: usize, score: f32, direction: Direction) {
        self.connections.push(Connection {
            x,
            y,
            id,
            score,
            direction,
        });
    }
    pub fn get_connections(&self) -> &Vec<Connection> {
        return &self.connections;
    }
}

impl PartialEq for Section {
    fn eq(&self, other: &Section) -> bool {
        self.id == other.id
    }
}

pub struct SectionMerger {
    map: Map,
    margins: (u8, u8),
    corridor_size: (u8, u8),
}

impl SectionMerger {
    pub fn new(map: Map, margins: (u8, u8), corridor_size: (u8, u8)) -> Self {
        return SectionMerger {
            map,
            margins,
            corridor_size,
        };
    }
    pub fn generate(mut self) -> Map {
        // First build the connection matrix
        for (cell, x, y) in self.map.iter_enumerate() {
            if cell.is_rock() {
                // there's enough room here
                let left = self
                    .map
                    .get_section(x as i32 - self.corridor_size.0 as i32, y.into());
                let right = self
                    .map
                    .get_section(x as i32 + (self.margins.0) as i32, y.into());
                if left != None
                    && right != None
                    && left != right
                    && self.is_section_with_margin(
                        x as i32 - self.corridor_size.0 as i32,
                        y.into(),
                        left.unwrap(),
                    )
                    && self.is_section_with_margin(
                        x as i32 + (self.margins.0) as i32,
                        y.into(),
                        right.unwrap(),
                    )
                {
                    // we know that these ID's are correct currently,
                    let left_id = left.unwrap().get_id();
                    let right_id = right.unwrap().get_id();
                    // There is a horizontal connection
                    let left_score =
                        self.score_pos(x as i32 - self.corridor_size.0 as i32, y.into(), false);
                    let right_score =
                        self.score_pos(x as i32 + self.margins.0 as i32, y.into(), false);
                    self.map
                        .get_section_mut(x as i32 - self.corridor_size.0 as i32, y.into())
                        .unwrap()
                        .add_connection(x, y, right_id, left_score.min(right_score), Direction::E);
                    self.map
                        .get_section_mut(x as i32 + self.margins.0 as i32, y.into())
                        .unwrap()
                        .add_connection(x, y, left_id, left_score.min(right_score), Direction::W);
                    self.map.set(x, y, Cell::Rock);
                }
                let top = self
                    .map
                    .get_section(x.into(), y as i32 - self.corridor_size.1 as i32 as i32);
                let bottom = self
                    .map
                    .get_section(x.into(), y as i32 + (self.margins.1) as i32);
                if top != None
                    && bottom != None
                    && top != bottom
                    && self.is_section_with_margin(
                        x.into(),
                        y as i32 - self.corridor_size.1 as i32,
                        top.unwrap(),
                    )
                    && self.is_section_with_margin(
                        x.into(),
                        y as i32 + (self.margins.1) as i32,
                        bottom.unwrap(),
                    )
                {
                    // There is a vertical connection
                    // we know that these ID's are correct currently,
                    let top_id = top.unwrap().get_id();
                    let bottom_id = bottom.unwrap().get_id();

                    let top_score =
                        self.score_pos(x.into(), y as i32 - self.corridor_size.1 as i32, true);
                    let bottom_score =
                        self.score_pos(x.into(), y as i32 + self.margins.1 as i32, true);
                    self.map
                        .get_section_mut(x.into(), y as i32 - self.corridor_size.1 as i32)
                        .unwrap()
                        .add_connection(x, y, bottom_id, top_score.min(bottom_score), Direction::S);
                    self.map
                        .get_section_mut(x.into(), y as i32 + self.margins.1 as i32)
                        .unwrap()
                        .add_connection(x, y, top_id, top_score.min(bottom_score), Direction::N);
                    self.map.set(x, y, Cell::Rock);
                }
            }
        }
        // Go through and mark all section as the same section and throw away
        // unconnected sections
        let unused_sections = self.find_unused_sections();
        // Prune corridor tree
        for root_node in self.map.corridor_tree.clone() {
            self.iterate_node(&root_node, 100);
        }
        return self.map;
    }

    fn find_unused_sections(&mut self) -> Vec<Section> {
        let first_section = &self.map.section_vec[0];
        let id = first_section.get_id();
        let connections = self.map.get_best_connections(&first_section);
        self.iterate_connections(&connections, id);
        // Check for unconnected sections
        let mut unconnected_vec = vec![];
        for section in &self.map.section_vec {
            if section.get_id() != id {
                unconnected_vec.push(section.clone());
            }
        }
        return unconnected_vec;
    }

    fn iterate_connections(&mut self, connections: &Vec<Connection>, id: usize) {
        for connection in connections {
            match connection.direction {
                Direction::N | Direction::S => {
                    self.map.set_rect(
                        Cell::Connection,
                        connection.x,
                        connection.y,
                        self.corridor_size.0 as u16,
                        self.margins.1 as u16,
                    );
                }
                Direction::W | Direction::E => {
                    self.map.set_rect(
                        Cell::Connection,
                        connection.x,
                        connection.y,
                        self.margins.0 as u16,
                        self.corridor_size.1 as u16,
                    );
                }
            }
            if self.map.get_connection_section(&connection).get_id() != id {
                self.map.get_connection_section_mut(&connection).set_id(id);
                let connections = self
                    .map
                    .get_best_connections(self.map.get_connection_section(connection));
                self.iterate_connections(&connections, id)
            }
        }
    }

    fn score_pos(&self, x: i32, y: i32, horizontal: bool) -> f32 {
        let position = if horizontal { x } else { y };
        return match self.map.get(x, y) {
            Cell::Room(idx) => self.score_room_pos(&self.map.get_room(*idx), position, horizontal),
            Cell::Corridor(_) => 1f32,
            _ => 0f32,
        };
    }

    fn is_section_with_margin(&self, x: i32, y: i32, section: &Section) -> bool {
        match self.map.rect_is(
            x,
            y,
            self.corridor_size.0 as u16,
            self.corridor_size.1 as u16,
            |c| match self.map.get_cell_section(c) {
                Some(s) if s != section => Some(false),
                Some(s) if s == section => None,
                _ => Some(false),
            },
        ) {
            Some(x) => return x,
            None => {}
        }
        return true;
    }

    fn score_room_pos(&self, room: &Room, position: i32, horizontal: bool) -> f32 {
        if horizontal {
            (1f32
                - (position as f32 + (self.corridor_size.0 as f32 / 2f32)
                    - (room.x as f32 + (room.width as f32 / 2f32)))
                    .abs()
                    / (room.width as f32 / 2f32))
                .max(0f32)
        } else {
            (1f32
                - (position as f32 + (self.corridor_size.1 as f32 / 2f32)
                    - (room.y as f32 + (room.height as f32 / 2f32)))
                    .abs()
                    / (room.height as f32 / 2f32))
                .max(0f32)
        }
    }

    fn iterate_node(&mut self, node: &WrappedCorridorNode, count: u32) {
        let children = &node.borrow().children.clone();
        if node.borrow().children.len() > 1 {
            // There's a branching in the tree
            // Mark this as a branch.
            for child in children {
                self.iterate_node(&child, 0);
            }
        }
        if node.borrow().children.len() == 1 {
            // it's a continuation of the branch
            self.iterate_node(&children[0], count + 1)
        }
        if node.borrow().children.len() == 0 {
            // it's a leaf
            if count < 10 {
                // Check if there's any connections surrounding it
                match self.map.rect_border_is(
                    node.borrow().x as i32 - 1,
                    node.borrow().y as i32 - 1,
                    self.corridor_size.0 as u16 + 2,
                    self.corridor_size.1 as u16 + 2,
                    |c| match c {
                        Cell::Connection => Some(true),
                        _ => None,
                    },
                ) {
                    Some(_) => {}
                    _ => {
                        // There is no connection surrounding it
                        match &node.borrow().parent {
                            Some(parent) => {
                                match (
                                    parent.borrow().x as i32 - (node.borrow().x as i32),
                                    parent.borrow().y as i32 - (node.borrow().y as i32),
                                ) {
                                    (x, _) if (x < 0) => {
                                        // parent is to the left
                                        self.map.set_rect(
                                            Cell::Wall,
                                            node.borrow().x + 1,
                                            node.borrow().y,
                                            1,
                                            1, // self.corridor_size.1 as u16,
                                        );
                                    }
                                    (x, _) if (x > 0) => {
                                        // parent is to the right
                                        self.map.set_rect(
                                            Cell::Wall,
                                            node.borrow().x,
                                            node.borrow().y,
                                            1,
                                            1, // self.corridor_size.1 as u16,
                                        );
                                    }
                                    (_, y) if (y < 0) => {
                                        // parent is to the top
                                        self.map.set_rect(
                                            Cell::Wall,
                                            node.borrow().x,
                                            node.borrow().y + 1,
                                            1, // self.corridor_size.0 as u16,
                                            1,
                                        );
                                    }
                                    _ => {
                                        // parent is at the bottom
                                        self.map.set_rect(
                                            Cell::Wall,
                                            node.borrow().x,
                                            node.borrow().y,
                                            1, // self.corridor_size.0 as u16,
                                            1,
                                        );
                                    }
                                }
                                remove_node(node);
                            }
                            None => {}
                        }
                    }
                }
            }
        }
    }
}
