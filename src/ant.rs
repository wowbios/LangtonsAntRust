use crate::point::Point;
use crate::size::Size;
use crate::strategy::Strategy;

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Ant<TStrategy: Strategy> {
    field_size: Size,
    pub position: Point,
    direction: Direction,
    strategy: TStrategy
}

impl<TStrategy: Strategy> Ant<TStrategy> {
    pub fn new(field_size: Size, position: Point, strategy: TStrategy) -> Ant<TStrategy> {
        Ant {
            field_size,
            position,
            strategy,
            direction: Direction::Up,
        }
    }

    pub fn go(&mut self, prev_state: i32) -> i32 {
        let (new_state, steps) = self.strategy.go(prev_state);
        for step in steps {
            if step { self.turn_right(); } else { self.turn_left(); }
        }
        self.move_forward();
        new_state
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

impl<'a, TStrategy: Strategy> std::fmt::Display for Ant<TStrategy> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(format!("Pos: {}", &self.position).as_str())?;
        Ok(())
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            Direction::Up => "UP",
            Direction::Down => "DOWN",
            Direction::Left => "LEFT",
            Direction::Right => "Right",
        })?;
        Ok(())
    }
}