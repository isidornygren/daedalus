#[derive(Clone, PartialEq, Copy)]
pub enum Cell {
    Rock(bool, bool), // Rocks are the ceiling of the map
    SolidRock,        // Solid rock are unbreakable rocks, e.g outside of the map
    Wall,             // horizontal, vertical
    // Room contains the index of the room in the room vector.
    Room(usize),
    Corridor(usize),
    Perimeter(u8), // how close it is to a wall
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
            Cell::Rock(_, _) => true,
            _ => false,
        }
    }
}

pub struct CellMatrix {
    pub cell_vector: Vec<Cell>,
    pub width: u16,
    pub height: u16,
    last_section: u16,
}

const EMPTY_CELL: Cell = Cell::SolidRock;

impl CellMatrix {
    pub fn new(width: u16, height: u16, filler_cell: Cell) -> Self {
        return CellMatrix {
            width,
            height,
            cell_vector: vec![filler_cell; ((width as u32) * (height as u32)) as usize],
            last_section: 0,
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
    pub fn new_section(&mut self) -> u16 {
        self.last_section += 1;
        return self.last_section.clone();
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
    pub fn get_rect(&self, x: i32, y: i32, width: u16, height: u16) -> CellMatrix {
        let mut cell_matrix_rect = CellMatrix::new(width, height, Cell::SolidRock);
        for pos_y in y..(y + height as i32) {
            for pos_x in x..(x + width as i32) {
                let cell = self.get(pos_x, pos_y).clone();
                cell_matrix_rect.set((pos_x - x) as u16, (pos_y - y) as u16, cell)
            }
        }
        return cell_matrix_rect;
    }

    pub fn rect_is<F, T>(&self, x: i32, y: i32, width: u16, height: u16, func: F) -> Option<T>
    where
        F: Fn(&Cell) -> Option<T>,
    {
        for pos_y in y..(y + height as i32) {
            for pos_x in x..(x + width as i32) {
                match func(&self.get(pos_x, pos_y)) {
                    Some(x) => return Some(x),
                    _ => {}
                }
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

impl std::fmt::Display for CellMatrix {
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
