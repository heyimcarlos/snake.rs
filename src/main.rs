mod board;
mod colors;
mod map;

use bevy::{prelude::*, render::color};
use board::BoardPlugin;
use map::MapPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.52, 0.73, 0.17)))
        .add_plugins(DefaultPlugins)
        // .add_plugins(MapPlugin)
        .add_plugins(BoardPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (bevy::window::close_on_esc, update_snake_movement).chain(),
        )
        .run();
}

#[derive(Component)]
pub struct Snake {
    pub velocity: Velocity,
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

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // the default projection is 1000 far, -1000 near
    commands.spawn(Camera2dBundle { ..default() });

    // snake (player)
    commands.spawn((
        SpriteBundle {
            // the position of an entity
            transform: Transform {
                translation: Vec3::new(100.0, 20.0, 0.0),
                rotation: default(),
                // the x value or width of the snake, should be 100% of the cell's width (I think)
                // the y value which represents the length of the scale, should increase after
                // every eat
                scale: Vec3::new(0.1, 1.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: color::Color::LIME_GREEN,
                custom_size: Some(Vec2::new(10.0, 20.0)),
                ..default()
            },
            // texture: asset_server.load("./"),
            ..default()
        },
        Snake {
            velocity: Velocity::new(Vec2::new(1., 0.)),
        },
    ));

    // center dot for reference
    commands.spawn((SpriteBundle {
        // the position of an entity
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: default(),
            // the x value or width of the snake, should be 100% of the cell's width (I think)
            // the y value which represents the length of the scale, should increase after
            // every eat
            scale: Vec3::new(0.1, 0.1, 0.),
            ..default()
        },
        sprite: Sprite {
            color: color::Color::WHITE,
            custom_size: Some(Vec2::new(10.0, 20.0)),
            ..default()
        },
        // texture: asset_server.load("./"),
        ..default()
    },));
}

const SNAKE_SPEED: f32 = 5.0;

fn update_snake_movement(
    mut query: Query<(&mut Transform), With<Snake>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
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

    // if keyboard_input.pressed(KeyCode::ArrowLeft) {
    //     velocity.value = Vec2::new(-1., 0.);
    // } else if keyboard_input.pressed(KeyCode::ArrowRight) {
    //     velocity.value = Vec2::new(1., 0.);
    // }
    // println!("before y: {}", transform.translation.y);
    let new_y = transform.translation.y + direction * SNAKE_SPEED * time.delta_seconds();
    transform.translation.y = new_y;
    // println!("y: {}", transform.translation.y);
}
