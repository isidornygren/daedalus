use crate::corridor_tree::WrappedCorridorNode;
use crate::direction::Direction;
use crate::room::{Corridor, Room};
use crate::sections::{Connection, Section};

#[derive(Clone, PartialEq, Copy)]
pub enum Cell {
    Rock,      // Rocks are the ceiling of the map
    SolidRock, // Solid rock are unbreakable rocks, e.g outside of the map
    Wall,      // horizontal, vertical
    // Room contains the index of the room in the room vector.
    Room(usize),
    Corridor(usize),
    Perimeter(u8), // how close it is to a wall
    Connection,
    Removed, // Debug cell
}

impl Cell {
    pub fn is_room(&self) -> bool {
        match self {
            Cell::Room(_) => true,
            _ => false,
        }
    }
    pub fn is_corridor(&self) -> bool {
        match self {
            Cell::Corridor(_) => true,
            _ => false,
        }
    }
    pub fn is_rock(&self) -> bool {
        match self {
            Cell::Rock => true,
            _ => false,
        }
    }
    pub fn is_walkable(&self) -> bool {
        match self {
            Cell::Room(_) => true,
            Cell::Corridor(_) => true,
            Cell::Connection => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Rock => write!(f, "Rock"),
            Cell::SolidRock => write!(f, "SolidRock"),
            Cell::Wall => write!(f, "Wall"),
            Cell::Room(_) => write!(f, "Room"),
            Cell::Corridor(_) => write!(f, "Corridor"),
            Cell::Perimeter(_) => write!(f, "Perimeter"),
            Cell::Connection => write!(f, "Connection"),
            Cell::Removed => write!(f, "Removed"),
        }
    }
}

pub struct Map {
    pub cell_vector: Vec<Cell>,
    pub width: u16,
    pub height: u16,
    room_vec: Vec<Room>,
    corridor_vec: Vec<Corridor>,
    pub corridor_tree: Vec<WrappedCorridorNode>,
    pub section_vec: Vec<Section>,
}

const EMPTY_CELL: Cell = Cell::SolidRock;

impl Map {
    pub fn new(width: u16, height: u16, filler_cell: Cell) -> Self {
        return Map {
            width,
            height,
            cell_vector: vec![filler_cell; ((width as u32) * (height as u32)) as usize],
            room_vec: vec![],
            corridor_vec: vec![],
            section_vec: vec![],
            corridor_tree: vec![],
        };
    }
    pub fn iter_enumerate(&self) -> Vec<(Cell, u16, u16)> {
        return self
            .cell_vector
            .iter()
            .enumerate()
            .map(|(i, c)| (c.clone(), i as u16 % self.width, i as u16 / self.width))
            .collect();
    }
    pub fn new_section(&mut self) -> usize {
        let index = self.section_vec.len();
        self.section_vec.push(Section::new(index));
        return index;
    }
    // Types are a bit higher for the get function
    // as we should be able to get negative values that will always return the
    // same value
    pub fn get(&self, x: i32, y: i32) -> &Cell {
        if x > (self.width as i32 - 1) || y > (self.height as i32 - 1) || x < 0 || y < 0 {
            return &EMPTY_CELL;
        }
        return &self.cell_vector[(y * (self.width as i32) + x) as usize];
    }
    pub fn get_best_connections(&self, section: &Section) -> Vec<Connection> {
        let mut best_connections: Vec<Connection> = vec![];
        let mut unsorted_connections = section.connections.clone();
        unsorted_connections.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        for connection in unsorted_connections.iter() {
            if !best_connections.iter().any(|&c| {
                self.get_connection_section(&c).get_id()
                    == self.get_connection_section(connection).get_id()
                    && c.direction == connection.direction
            }) {
                best_connections.push(connection.clone());
            }
        }
        return best_connections;
    }
    pub fn get_connection_section(&self, connection: &Connection) -> &Section {
        return &self.section_vec[connection.id];
    }
    pub fn get_connection_section_mut(&mut self, connection: &Connection) -> &mut Section {
        return &mut self.section_vec[connection.id];
    }
    pub fn get_cell_section(&self, cell: &Cell) -> Option<&Section> {
        match cell {
            Cell::Room(idx) => {
                let section_id = self.get_room(*idx).section_id;
                return Some(&self.section_vec[section_id]);
            }
            Cell::Corridor(idx) => {
                let section_id = self.get_corridor(*idx).section_id;
                return Some(&self.section_vec[section_id]);
            }
            _ => return None,
        };
    }
    pub fn get_section(&self, x: i32, y: i32) -> Option<&Section> {
        let cell = self.get(x, y);
        return self.get_cell_section(cell);
    }
    pub fn get_section_mut(&mut self, x: i32, y: i32) -> Option<(&mut Section)> {
        match self.get(x, y) {
            Cell::Room(idx) => {
                let section_id = self.get_room(*idx).section_id;
                return Some(&mut self.section_vec[section_id]);
            }
            Cell::Corridor(idx) => {
                let section_id = self.get_corridor(*idx).section_id;
                return Some(&mut self.section_vec[section_id]);
            }
            _ => return None,
        };
    }
    pub fn get_room(&self, idx: usize) -> &Room {
        return &self.room_vec[idx];
    }
    pub fn get_room_mut(&mut self, idx: usize) -> &mut Room {
        return &mut self.room_vec[idx];
    }
    pub fn get_corridor(&self, idx: usize) -> &Corridor {
        return &self.corridor_vec[idx];
    }
    pub fn get_corridor_mut(&mut self, idx: usize) -> &mut Corridor {
        return &mut self.corridor_vec[idx];
    }
    pub fn push_room(&mut self, room: Room) -> usize {
        self.room_vec.push(room);
        return self.room_vec.len() - 1;
    }
    pub fn add_corridor(&mut self) -> usize {
        let section_id = self.new_section();
        self.corridor_vec.push(Corridor { section_id });
        return self.corridor_vec.len() - 1;
    }
    pub fn iter_rooms(&self) -> std::slice::Iter<Room> {
        return self.room_vec.iter();
    }
    pub fn iter_corridors(&self) -> std::slice::Iter<Corridor> {
        return self.corridor_vec.iter();
    }
    pub fn get_rect(&self, x: i32, y: i32, width: u16, height: u16) -> Map {
        let mut cell_matrix_rect = Map::new(width, height, Cell::SolidRock);
        for pos_y in y..(y + height as i32) {
            for pos_x in x..(x + width as i32) {
                let cell = self.get(pos_x, pos_y).clone();
                cell_matrix_rect.set((pos_x - x) as u16, (pos_y - y) as u16, cell)
            }
        }
        return cell_matrix_rect;
    }

    pub fn rect_is<F>(&self, x: i32, y: i32, width: u16, height: u16, func: F) -> Option<Cell>
    where
        F: Fn(&Cell) -> bool,
    {
        for pos_y in y..(y + height as i32) {
            for pos_x in x..(x + width as i32) {
                if func(&self.get(pos_x, pos_y)) {
                    return Some(*self.get(pos_y, pos_y));
                }
            }
        }
        return None;
    }

    pub fn check_cells<F, T>(&self, coords: Vec<(i32, i32)>, func: F) -> Option<T>
    where
        F: Fn(&Cell) -> Option<T>,
    {
        for coord in coords {
            match func(&self.get(coord.0, coord.1)) {
                Some(a) => return Some(a),
                _ => {}
            }
        }
        return None;
    }

    pub fn rect_border_is<F, T>(
        &self,
        x: i32,
        y: i32,
        width: u16,
        height: u16,
        func: F,
    ) -> Option<T>
    where
        F: Fn(&Cell) -> Option<T>,
    {
        for pos_y in y..(y + height as i32) {
            match func(&self.get(x, pos_y)) {
                Some(a) => return Some(a),
                _ => {}
            }
            match func(&self.get(x + width as i32, pos_y)) {
                Some(a) => return Some(a),
                _ => {}
            }
        }
        for pos_x in x..(x + width as i32) {
            match func(&self.get(pos_x, y)) {
                Some(a) => return Some(a),
                _ => {}
            }
            match func(&self.get(pos_x, y + height as i32)) {
                Some(a) => return Some(a),
                _ => {}
            }
        }
        return None;
    }

    pub fn set_rect(&mut self, cell: Cell, x: u16, y: u16, width: u16, height: u16) {
        for pos_y in y..(y + height) {
            for pos_x in x..(x + width) {
                self.set(pos_x, pos_y, cell)
            }
        }
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        assert!(
            x < self.width,
            "x value not within width of matrix, {}>={}",
            x,
            self.width
        );
        assert!(
            y < self.height,
            "y value not within height of matrix, {}>={}",
            y,
            self.height
        );
        self.cell_vector[(y * self.width + x) as usize] = cell;
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Print the map beautifully
        let mut map_string = String::with_capacity(self.width as usize + self.height as usize);
        for (cell, x, y) in self.iter_enumerate() {
            if x == 0 && y > 0 {
                map_string.push('\n');
            }
            match cell {
                Cell::Room(_) => map_string.push('R'),
                Cell::Corridor(_) => map_string.push('C'),
                Cell::Wall => map_string.push('W'),
                _ => map_string.push(' '),
            };
        }
        return write!(f, "{}", map_string);
    }
}
