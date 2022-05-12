mod point;
mod ant;
mod size;
mod strategy;

use point::Point;
use ant::Ant;
use size::Size;

fn main() {
    let size = Size{width:100, height:100};
    let mut ant = Ant::new(
        size,
        Point::new(50, 50),
        Box::new(strategy::rl_ant),
        0);
        
    for _ in 1..100
    {
        println!("ANT: {}", ant);
        ant.go();
    }
    println!("ANT: {}", ant);
}
