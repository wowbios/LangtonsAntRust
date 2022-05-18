mod ant;
mod point;
mod size;
mod strategy;

use self::ant::Ant;
use self::point::Point;
use self::size::Size;
use self::strategy::RlAntStrategy;

use nannou::prelude::*;
use crate::strategy::Strategy;

struct Model<TStrategy: Strategy> {
    ant: Ant<TStrategy>,
    field: Vec<Vec<i32>>,
    events: Vec<(Point, i32)>
}

impl<'a, TStrategy: Strategy> Model<TStrategy> {
    fn new(strategy: TStrategy) -> Model<TStrategy> {
        let size = Size {
            width: 200,
            height: 200
        };
        let field = vec![vec![0; 200]; 200];
        let ant = Ant::new(size, Point::new(50, 50), strategy);
        let events = Vec::new();

        Model { ant, field, events }
    }

    fn go(&mut self) {
        let prev_pos = Point::new(self.ant.position.x, self.ant.position.y);
        let position = &mut self.field[prev_pos.x as usize][prev_pos.y as usize];
        *position = self.ant.go(*position);
        self.events.push((prev_pos, *position));
    }
}

fn model(_app: &App) -> Model<RlAntStrategy> {
    Model::new(RlAntStrategy {})
}

fn update<TStrategy: Strategy>(_app: &App, _model: &mut Model<TStrategy>, _update: Update) {
    _model.events.clear();
    _model.go();
}

fn view<TStrategy: Strategy>(_app: &App, _model: &Model<TStrategy>, frame: Frame) {
    let draw = _app.draw();
    const SCALE: f32 = 4.0;
    const OFFSET: f32 = -200.0;
    // draw.background().color(BLACK);
    for e in &_model.events {
        let color = if e.1 == 1 { WHITE } else { BLACK };
        draw.ellipse()
            .color(color)
            .w(SCALE)
            .h(SCALE)
            .x_y(e.0.x as f32 * SCALE + OFFSET, e.0.y as f32 * SCALE + OFFSET);
    }

    draw.to_frame(_app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}