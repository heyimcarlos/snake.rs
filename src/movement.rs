use bevy::prelude::*;

use crate::{
    board::{Board, TILE_SIZE},
    snake::Snake,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(
    mut commands: Commands,
    // snake_body: Res<SnakeBody>,
    // mut head_query: Query<(&mut Transform, &Snake), With<SnakeDirection>>,
    // mut body_query: Query<(&mut Transform, &Snake, Entity), Without<SnakeDirection>>,
    board: Res<Board>,
) {
    // if let Ok((mut transform, _)) = head_query.get_single_mut() {
    //     let head = snake_body.segments.first().unwrap();
    //     transform.translation = Vec3::new(
    //         board.position_translate(head.x.into()),
    //         board.position_translate(head.y.into()),
    //         1.,
    //     )
    // }
    //
    // let mut idx = 0;
    // for (mut transform, _, _) in body_query.iter_mut() {
    //     if let Some(segment) = snake_body.segments.get(idx + 1) {
    //         transform.translation = Vec3::new(
    //             board.position_translate(segment.x.into()),
    //             board.position_translate(segment.y.into()),
    //             1.,
    //         );
    //         idx += 1;
    //     } else {
    //
    // else {
    //         commands.spawn((
    //             SpriteBundle {
    //                 transform: Transform::from_xyz(
    //                     // @todo: randomize the spawn of the snake
    //                     board.position_translate(segment.x.into()),
    //                     board.position_translate(segment.y.into()),
    //                     10.0,
    //                 ),
    //                 sprite: Sprite {
    //                     color: Color::DARK_GRAY,
    //                     custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
    //                     ..default()
    //                 },
    //                 ..default()
    //             },
    //             Snake,
    //         ));
    //     };
    // }
    // };
    // }
}
