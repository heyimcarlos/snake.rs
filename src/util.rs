use rand::{self, Rng};

use crate::snake::Position;

pub fn snake_starting_position(board_size: usize) -> Vec<Position> {
    let mut rng = rand::thread_rng();
    let start_x = rng.gen_range(5..board_size - 5);
    let start_y = rng.gen_range(5..board_size - 5);

    vec![
        Position {
            x: start_x as u8,
            y: start_y as u8,
        },
        Position {
            x: start_x as u8,
            y: start_y as u8 - 1,
        },
        Position {
            x: start_x as u8,
            y: start_y as u8 - 2,
        },
    ]
}
