use bevy::prelude::*;

use crate::board::{Board, TILE_SIZE};

#[derive(Component, Debug)]
pub struct Snake;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_snake)
            .add_systems(Update, snake_movement_controls);
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

// @info: maybe I can create an enum with all 4 directions which are vec2 (1, 0) (0, 1) (-1, 0) (0, -1)

// Speed and direction
#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec2,
}

impl Velocity {
    fn new(value: Vec2) -> Self {
        Self { value }
    }

    fn default() -> Self {
        Self { value: Vec2::ZERO }
    }
}

// describes the rate of change of velocity over time. It tells how quickly the velocity is
// changing, not only in speed, but in direction.
// #[derive(Component)]
// struct Acceleration {
//     value: Vec2,
// }
//
// impl Acceleration {
//     fn new(value: Vec2) -> Self {
//         Self { value }
//     }
// }

fn spawn_snake(mut commands: Commands, board: Res<Board>) {
    // snake (player)
    commands.spawn((
        SpriteBundle {
            // the position of an entity
            transform: Transform::from_xyz(
                // @todo: randomize the spawn of the snake
                board.position_translate(0),
                board.position_translate(19),
                10.0,
            ),
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            // texture: asset_server.load("./"),
            ..default()
        },
        Snake {
            // velocity: Velocity::new(Vec2::new(1., 0.)),
        },
    ));
}

const SNAKE_SPEED: f32 = 5.0;
// @todo: The snake should always move at the same rate of speed.
// what changes is its  direction, but the rate of movement is the same for the whole gameplay

fn snake_movement_controls(
    mut query: Query<(&mut Transform), With<Snake>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    board: Res<Board>,
) {
    let Ok((mut transform)) = query.get_single_mut() else {
        return;
    };
    let mut direction: f32 = 0.;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += 1.;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction -= 1.;
    }

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x += -SNAKE_SPEED * board.physical_size * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += SNAKE_SPEED * TILE_SIZE * time.delta_seconds();
    }

    let new_y =
        transform.translation.y + direction * SNAKE_SPEED * TILE_SIZE * time.delta_seconds();
    transform.translation.y = new_y;
}
