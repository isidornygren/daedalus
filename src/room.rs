use crate::sections::Section;

pub struct Room {
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,
    pub section: Section,
}

impl Room {
    pub fn collides_with(&self, room: &Room, margins: (u8, u8)) -> bool {
        return (self.x as i32 - margins.0 as i32) < (room.x + room.width) as i32
            && room.x < self.x + self.width + margins.0 as u16
            && (self.y as i32 - margins.1 as i32) < (room.y + room.height) as i32
            && room.y < self.y + self.height + margins.1 as u16;
    }
}

pub struct Corridor {
    pub section: Section,
}
