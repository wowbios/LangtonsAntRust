mod point;
mod ant;
mod size;
mod strategy;

use self::point::Point;
use self::ant::Ant;
use self::size::Size;
use self::strategy::RlAntStrategy;

use nannou::prelude::*;

struct Model<'a> {
    ant: Ant<'a>,
    field: Vec<Vec<i32>>,
    events: Vec<(Point, i32)>
}

impl<'a> Model<'a> {
    fn new() -> Model<'a>
    {
        let size = Size{width:200, height:200};
        let field = vec![vec![0; 200]; 200];
        let ant = Ant::new(
            size,
            Point::new(50, 50),
            &RlAntStrategy{});
        let events = Vec::new();
        
        Model 
        {
            ant,
            field,
            events
        }
    }

    fn go(&mut self)
    {
        let prev_pos = Point::new(self.ant.position.x, self.ant.position.y);
        let a = &mut self.field[prev_pos.x as usize];
        let prev_state = &a[prev_pos.y as usize];
        let new_state = self.ant.go(*prev_state);
        a[prev_pos.y as usize] = new_state;

        self.events.push((prev_pos, new_state));
    }
}

fn model<'a>(_app: &App) -> Model<'a> {
    Model::new()
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.events.clear();
    _model.go();
}

fn view(_app: &App, _model: &Model, frame: Frame){
    let draw = _app.draw();
    const SCALE: f32 = 4.0;
    const OFFSET: f32 = -200.0;
    // draw.background().color(BLACK);
    for e in &_model.events
    {
        let color = if e.1 == 1 {WHITE} else {BLACK};
        draw.ellipse()
        .color(color)
        .w(SCALE)
        .h(SCALE)
        .x_y(
            e.0.x as f32 * SCALE + OFFSET,
            e.0.y as f32 * SCALE + OFFSET);
    }
    

    draw.to_frame(_app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
