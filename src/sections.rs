use crate::cell_matrix::{Cell, CellMatrix};
use crate::map_generator::Map;
use crate::room::{Corridor, Room};

use rand::prelude::ThreadRng;
use rand::thread_rng;
use rand::Rng;

pub trait Sectionable {
    fn set_section(&mut self, section: u16);
    fn get_section(&self) -> u16;
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
                let right = self.get_section(x as i32 + self.margins.0 as i32, y.into());
                if left != 0
                    && right != 0
                    && left != right
                    && self.is_section_with_margin(
                        x as i32 - self.corridor_size.0 as i32,
                        y.into(),
                        left,
                    )
                    && self.is_section_with_margin(
                        x as i32 + self.margins.0 as i32,
                        y.into(),
                        right,
                    )
                {
                    // There is a horizontal connection
                    self.map.cell_matrix.set(x, y, Cell::Rock(true, false));
                }
                let top = self.get_section(x.into(), y as i32 - self.corridor_size.1 as i32);
                let bottom = self.get_section(x.into(), y as i32 + self.margins.1 as i32);
                if top != 0
                    && bottom != 0
                    && top != bottom
                    && self.is_section_with_margin(
                        x.into(),
                        y as i32 - self.corridor_size.1 as i32,
                        top,
                    )
                    && self.is_section_with_margin(
                        x.into(),
                        y as i32 + self.margins.1 as i32,
                        bottom,
                    )
                {
                    // There is a horizontal connection
                    self.map.cell_matrix.set(x, y, Cell::Rock(false, true));
                }
            }
        }
        return self.map;
    }
    fn is_section_with_margin(&self, x: i32, y: i32, section: u16) -> bool {
        match self.map.cell_matrix.rect_is(
            x,
            y,
            self.corridor_size.0 as u16,
            self.corridor_size.1 as u16,
            |c| {
                if self.get_cell_section(c) != section {
                    Some(false)
                } else {
                    None
                }
            },
        ) {
            Some(x) => return x,
            None => {}
        }
        return true;
    }
    fn is_rock_with_margin(&self, x: i32, y: i32) -> bool {
        match self.map.cell_matrix.rect_is(
            x,
            y,
            self.corridor_size.0 as u16,
            self.corridor_size.1 as u16,
            |c| if !c.is_rock() { Some(false) } else { None },
        ) {
            Some(x) => return x,
            None => {}
        }
        return true;
    }
    fn get_cell_section(&self, cell: &Cell) -> u16 {
        match cell {
            Cell::Room(idx) => {
                return self.map.rooms[*idx].section;
            }
            Cell::Corridor(idx) => {
                return self.map.corridors[*idx].section;
            }
            _ => return 0,
        };
    }
    fn get_section(&self, x: i32, y: i32) -> u16 {
        return self.get_cell_section(self.map.cell_matrix.get(x, y));
    }
}
