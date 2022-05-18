use std::fmt::{Display, Error, Formatter};

pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(format!("({}, {})", &self.x, &self.y).as_str())?;
        Ok(())
    }
}