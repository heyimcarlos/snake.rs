use bevy::prelude::*;

use crate::colors::COLORS;

pub const TILE_SIZE: f32 = 30.0;

#[derive(Resource, Debug)]
pub struct Board {
    pub size: i32,
    //  NOTE: size * tile size -> the pixel quantity to be used when rendering the board
    pub physical_size: f32,
}

impl Board {
    fn new(size: i32) -> Self {
        let physical_size: f32 = size as f32 * TILE_SIZE;
        Self {
            size,
            physical_size,
        }
    }

    //  TODO: implement function to turn a board's cell position into the physical rendered board. Since the original board sizing uses an int 20 for example, we need to multiply that with the tile size to get the physical size of the board to be rendered in pixels
    pub fn position_translate(&self, pos: i32) -> f32 {
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
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLORS.board,
                custom_size: Some(Vec2::new(board.physical_size, board.physical_size)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for x in 0..board.size {
                for y in 0..board.size {
                    builder.spawn(SpriteBundle {
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
                            board.position_translate(x),
                            board.position_translate(y),
                            0.0,
                        ),
                        ..default()
                    });
                }
            }
        });
}
