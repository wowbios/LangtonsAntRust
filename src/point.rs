pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(format!("({}, {})", &self.x, &self.y).as_str())?;
        std::result::Result::Ok(())
    }
}
