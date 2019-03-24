use crate::cell_matrix::Cell;
use crate::map_generator::Map;
use crate::room::Room;

pub struct Section {
    id: u16,
    connections: Vec<(u16, u16, u16, f32)>,
}

impl Section {
    pub fn new(id: u16) -> Self {
        return Section {
            id,
            connections: vec![],
        };
    }
    pub fn get_id(&self) -> u16 {
        return self.id;
    }
    pub fn set_id(&mut self, id: u16) {
        self.id = id;
    }
    pub fn add_connection(&mut self, x: u16, y: u16, id: u16, score: f32) {
        self.connections.push((x, y, id, score));
    }
    pub fn get_connections(&self) -> &Vec<(u16, u16, u16, f32)> {
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
        for (cell, x, y) in self.map.cell_matrix.iter_enumerate() {
            if cell.is_rock() {
                // there's enough room here
                let left = self.get_section(x as i32 - self.corridor_size.0 as i32, y.into());
                let right = self.get_section(x as i32 + (self.margins.0) as i32, y.into());
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
                    let left_id = left.unwrap().get_id();
                    let right_id = left.unwrap().get_id();
                    // There is a horizontal connection
                    let left_score =
                        self.score_pos(x as i32 - self.corridor_size.0 as i32, y.into(), false);
                    let right_score =
                        self.score_pos(x as i32 + self.margins.0 as i32, y.into(), false);
                    self.get_section_mut(x as i32 - self.corridor_size.0 as i32, y.into())
                        .unwrap()
                        .add_connection(x, y, right_id, left_score.min(right_score));
                    self.get_section_mut(x as i32 + self.margins.0 as i32, y.into())
                        .unwrap()
                        .add_connection(x, y, left_id, left_score.min(right_score));
                    self.map.cell_matrix.set(x, y, Cell::Rock(true, false));
                }
                let top = self.get_section(x.into(), y as i32 - self.corridor_size.1 as i32 as i32);
                let bottom = self.get_section(x.into(), y as i32 + (self.margins.1) as i32);
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
                    let top_id = top.unwrap().get_id();
                    let bottom_id = bottom.unwrap().get_id();

                    let top_score =
                        self.score_pos(x.into(), y as i32 - self.corridor_size.1 as i32, true);
                    let bottom_score =
                        self.score_pos(x.into(), y as i32 + self.margins.1 as i32, true);
                    self.get_section_mut(x.into(), y as i32 - self.corridor_size.1 as i32)
                        .unwrap()
                        .add_connection(x, y, bottom_id, top_score.min(bottom_score));
                    self.get_section_mut(x.into(), y as i32 + self.margins.1 as i32)
                        .unwrap()
                        .add_connection(x, y, top_id, top_score.min(bottom_score));
                    self.map.cell_matrix.set(x, y, Cell::Rock(false, true));
                }
            }
        }
        return self.map;
    }
    fn score_pos(&self, x: i32, y: i32, horizontal: bool) -> f32 {
        let position = if horizontal { x } else { y };
        return match self.map.cell_matrix.get(x, y) {
            Cell::Room(idx) => self.score_room_pos(&self.map.rooms[*idx], position, horizontal),
            Cell::Corridor(_) => 1f32,
            _ => 0f32,
        };
    }
    fn is_section_with_margin(&self, x: i32, y: i32, section: &Section) -> bool {
        match self.map.cell_matrix.rect_is(
            x,
            y,
            self.corridor_size.0 as u16,
            self.corridor_size.1 as u16,
            |c| match self.get_cell_section(c) {
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
    fn get_cell_section(&self, cell: &Cell) -> Option<&Section> {
        match cell {
            Cell::Room(idx) => {
                return Some(&self.map.rooms[*idx].section);
            }
            Cell::Corridor(idx) => {
                return Some(&self.map.corridors[*idx].section);
            }
            _ => return None,
        };
    }
    fn get_section_mut(&mut self, x: i32, y: i32) -> Option<&mut Section> {
        match self.map.cell_matrix.get(x, y) {
            Cell::Room(idx) => {
                return Some(&mut self.map.rooms[*idx].section);
            }
            Cell::Corridor(idx) => {
                return Some(&mut self.map.corridors[*idx].section);
            }
            _ => return None,
        };
    }
    fn get_section(&self, x: i32, y: i32) -> Option<&Section> {
        return self.get_cell_section(self.map.cell_matrix.get(x, y));
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
}
