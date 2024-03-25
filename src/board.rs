use bevy::prelude::*;

use crate::colors::COLORS;

const TILE_SIZE: f32 = 30.0;
const TILE_SPACER: f32 = 0.0;

// should the board be a resource or a component?
#[derive(Resource, Debug)]
struct Board {
    // the board will be a square, (e.g. if size is 5 then the board is 5^2 or 25 tiles)
    size: usize,
    // @info: size * tile size -> the pixel quantity to be used when rendering the board
    physical_size: f32,
}

impl Board {
    fn new(size: usize) -> Self {
        let physical_size: f32 = size as f32 * TILE_SIZE;
        Self {
            size,
            physical_size,
        }
    }

    // @todo: implement function to turn a board's cell position into the physical rendered board.
    // Since the original board sizing uses an int 20 for example, we need to multiply that with
    // the tile size to get the physical size of the board to be rendered in pixels
    fn translate_to_physical(&self, pos: usize) -> f32 {
        let offset = -&self.physical_size / 2.0 + 0.5 * TILE_SIZE;
        offset + pos as f32 * TILE_SIZE
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::new(20))
            .add_systems(Startup, load_board);
    }
}

fn load_board(mut commands: Commands, board: Res<Board>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: COLORS.board,
            custom_size: Some(Vec2::new(board.physical_size, board.physical_size)),
            ..Default::default()
        },
        ..Default::default()
    });

    for x in 0..board.size {
        for y in 0..board.size {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: if (x + y) % 2 == 0 {
                        COLORS.tile_placeholder
                    } else {
                        COLORS.tile_placeholder_dark
                    },
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    board.translate_to_physical(x),
                    board.translate_to_physical(y),
                    0.0,
                ),
                ..default()
            });
        }
    }
}
