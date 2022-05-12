use crate::ant::Ant;

// pub trait Strategy<'a, 'b>
// {
//     fn go(&self, ant: &'b mut Ant, state: &'b i32) -> i32;
// }

// pub struct RL{}
// impl<'a, 'b> Strategy<'a,'b> for RL
// {
    
// fn go(&self, ant: &'b mut Ant, state: &'b i32) -> i32 { 
//     let new_state;
//         match state {
//             0 => {
//                 ant.turn_right();
//                 new_state = 1;
//             }
//             1 => {
//                 ant.turn_left();
//                 new_state = 0;
//             },
//             _ => panic!("shit happened")
//         }
//         ant.move_forward();
//         new_state
//  }
// }

pub fn rl_ant(ant: &mut Ant, state: i32) -> i32
    {
        let new_state;
        match state {
            0 => {
                ant.turn_right();
                new_state = 1;
            }
            1 => {
                ant.turn_left();
                new_state = 0;
            },
            _ => panic!("shit happened")
        }
        ant.move_forward();
        new_state
    }