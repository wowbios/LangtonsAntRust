pub trait Strategy {
    fn go(&self, prev_state: i32) -> (i32, Vec<bool>);
}

pub struct RlAntStrategy {
}

impl Strategy for RlAntStrategy {
    fn go(&self, prev_state: i32) -> (i32, Vec<bool>) {
        match prev_state {
            0 => {
                (1, vec![true])
            }
            1 => {
                (0, vec![false])
            }
            _ => panic!("shit happened"),
        }
    }
}