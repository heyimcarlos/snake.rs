use rand::{self, Rng};

use crate::snake::Position;

pub fn snake_starting_position(board_size: i32) -> Vec<Position> {
    let mut rng = rand::thread_rng();
    let start_x = rng.gen_range(5..board_size - 5);
    let start_y = rng.gen_range(5..board_size - 5);

    vec![
        Position {
            x: start_x,
            y: start_y,
        },
        Position {
            x: start_x - 1,
            y: start_y,
        },
        Position {
            x: start_x - 2,
            y: start_y,
        },
    ]
}

pub fn food_position(board_size: i32) -> Position {
    let mut rng = rand::thread_rng();
    Position {
        x: rng.gen_range(5..board_size - 5),
        y: rng.gen_range(5..board_size - 5),
    }
}
