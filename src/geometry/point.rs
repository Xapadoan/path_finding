#[derive(Clone, Debug)]
pub struct Point(u8, u8);

impl Point {
    pub fn new(x: u8, y: u8) -> Self {
        Self(x,  y)
    }

    pub fn x(&self) -> u8 {
        self.0
    }

    pub fn y(&self) -> u8 {
        self.1
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

pub trait CollectionOfPoints {
    fn points(&self) -> Vec<&Point>;
}
