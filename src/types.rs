#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub id: u32,
    pub name: String,
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn new(id: u32, name: String, x: f64, y: f64) -> Self {
        Point { id, name, x, y, }
    }
}