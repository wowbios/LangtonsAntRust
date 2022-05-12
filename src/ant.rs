use crate::point::Point;
use crate::size::Size;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}


pub struct Ant {
    field_size: Size,
    position: Point,
    direction: Direction,
    state: i32,
    strategy: Box<dyn Fn(&mut Ant, i32) -> i32>
}

impl Ant {
    pub fn new(
        field_size: Size,
        position: Point,
        strategy: Box<dyn Fn(&mut Ant, i32) -> i32>,
        initial_state: i32,
    ) -> Ant {
        Ant {
            field_size,
            position,
            strategy,
            direction: Direction::Up,
            state: initial_state,
        }
    }

    pub fn go(&mut self) {
        self.state = match self.state {
            0 => {
                self.turn_right();
                self.move_forward();
                1
            }
            1 => {
                self.turn_left();
                self.move_forward();                
                0
            },
            _ => panic!("shit happened")
        }
    }

    pub fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
    fn move_up(&mut self) {
        self.position = Point::new(
            (self.position.x + &self.field_size.height - 1) % &self.field_size.height,
            self.position.y,
        );
    }

    fn move_down(&mut self) {
        self.position = Point::new(
            (self.position.x + 1) % &self.field_size.height,
            self.position.y,
        );
    }

    fn move_left(&mut self) {
        self.position = Point::new(
            self.position.x,
            (self.position.y + self.field_size.width - 1) % self.field_size.width,
        );
    }

    fn move_right(&mut self) {
        self.position = Point::new(
            self.position.x,
            (self.position.y + 1) % self.field_size.width,
        );
    }
}

impl std::fmt::Display for Ant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(format!("Pos: {}", &self.position).as_str())?;
        std::result::Result::Ok(())
    }
}
