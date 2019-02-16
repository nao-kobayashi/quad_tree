#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub id: u32,
    pub name: String,
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(id: u32, name: String, x: i32, y: i32) -> Self {
        Point { id, name, x, y, }
    }
}