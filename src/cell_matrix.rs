#[derive(Clone, PartialEq, Copy)]
pub enum CellKind {
    Rock,      // Rocks are the ceiling of the map
    SolidRock, // Solid rock are unbreakable rocks, e.g outside of the map
    Wall,
    // Room contains the index of the room in the room vector.
    Room,
    Corridor,
    Perimeter(u8), // how close it is to a wall
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub kind: CellKind,
}

pub struct CellMatrix {
    pub cell_vector: Vec<Cell>,
    pub width: u16,
    pub height: u16,
}

const EMPTY_CELL: Cell = Cell {
    kind: CellKind::SolidRock,
};

impl CellMatrix {
    pub fn new(width: u16, height: u16, filler_kind: CellKind) -> Self {
        return CellMatrix {
            width,
            height,
            cell_vector: vec![
                Cell { kind: filler_kind };
                ((width as u32) * (height as u32)) as usize
            ],
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
        assert!(width > 0, "Width needs to be a positive integer");
        assert!(height > 0, "Height needs to be a positive integer");
        let mut cell_matrix_rect = CellMatrix::new(width, height, CellKind::SolidRock);
        for pos_y in y..(y + height as i32) {
            for pos_x in x..(x + width as i32) {
                let cell = self.get(pos_x, pos_y).clone();
                cell_matrix_rect.set((pos_x - x) as u16, (pos_y - y) as u16, cell)
            }
        }
        return cell_matrix_rect;
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
