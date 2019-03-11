extern crate rand;
use rand::thread_rng;
use rand::Rng;

#[derive(Clone, Copy)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let formatted_string = match self {
            Direction::N => "North",
            Direction::E => "East",
            Direction::S => "South",
            Direction::W => "West",
        };
        write!(f, "{}", formatted_string)
    }
}

impl Direction {
    pub fn turn_clockwise(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
    pub fn rand() -> Direction {
        match thread_rng().gen_range(0, 4) {
            0 => Direction::N,
            1 => Direction::E,
            2 => Direction::S,
            _ => Direction::W,
        }
    }
}
