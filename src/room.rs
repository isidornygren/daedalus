pub struct Room {
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,
}

impl Room {
    pub fn collides_with(&self, room: &Room, margins: (u8, u8, u8, u8)) -> bool {
        return (self.x as u32) < room.x as u32 + room.width as u32 + (margins.1 + margins.3) as u32
            && (self.x as i32 + self.width as i32 + (margins.0 + margins.2) as i32) > room.x as i32
            && (self.y as u32) < room.y as u32 + room.height as u32 + (margins.1 + margins.3) as u32
            && (self.y as i32 + self.height as i32 + (margins.0 + margins.3) as i32) > room.y as i32;
    }
}