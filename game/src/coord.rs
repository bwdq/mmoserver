

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Coord {
    x: i32,
    y: i32,
}


impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}


pub struct Coord2d {
    x: i32,
    y: i32,
    z: Coord,
}

impl Coord2d {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            z: Coord::new(0,0),
        }
    }
}