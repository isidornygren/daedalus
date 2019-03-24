use crate::sections::Sectionable;

pub struct Room {
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,
    pub section: u16,
}

impl Room {
    pub fn collides_with(&self, room: &Room, margins: (u8, u8)) -> bool {
        return (self.x as i32 - margins.0 as i32) < (room.x + room.width) as i32
            && room.x < self.x + self.width + margins.0 as u16
            && (self.y as i32 - margins.1 as i32) < (room.y + room.height) as i32
            && room.y < self.y + self.height + margins.1 as u16;
    }
}

impl Sectionable for Room {
    fn set_section(&mut self, section: u16) {
        self.section = section;
    }
    fn get_section(&self) -> u16 {
        return self.section;
    }
}

pub struct Corridor {
    pub section: u16,
}

impl Sectionable for Corridor {
    fn set_section(&mut self, section: u16) {
        self.section = section;
    }
    fn get_section(&self) -> u16 {
        return self.section;
    }
}
