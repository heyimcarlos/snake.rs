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
fn spawn_snake(mut commands: Commands, board: Res<Board>, asset_server: Res<AssetServer>) {
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
                custom_size: Some(Vec2::new(10., 10.)),
                ..default()
            },
            ..default()
        },
        Snake {
            // velocity: Velocity::new(Vec2::new(1., 0.)),
        },
    ));
    // snake (player)
    commands.spawn((
        SceneBundle {
            // the position of an entity
            transform: Transform::from_xyz(
                // @todo: randomize the spawn of the snake
                board.position_translate(0),
                board.position_translate(19),
                10.0,
            ),
            scene: asset_server.load("Missile.glb#Scene0"),
            ..default()
        },
        Snake,
    ));
}

const SNAKE_SPEED: f32 = 30.0;
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

    // determine the direction of in which the snake head is pointing at;
    // based on this direction rotate the head and keep moving forward
    let direction: Vec3 = if keyboard_input.pressed(KeyCode::ArrowUp) {
        Vec3::new(0.0, 1.0, 0.0)
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        Vec3::new(0.0, -1.0, 0.0)
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        Vec3::new(-1.0, 0.0, 0.0)
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 0.0, 0.0)
    };

    transform.translation += direction * SNAKE_SPEED * time.delta_seconds();
}
