use bevy::prelude::*;

use crate::{
    asset_loader::ImageAssets,
    board::{Board, TILE_SIZE},
    schedule::InGameSet,
    state::GameState,
    util::{get_sprite_index, snake_starting_position},
};

#[derive(Component, Debug)]
pub struct SnakeDirection {
    value: Direction,
    next_value: Direction,
}

impl Default for SnakeDirection {
    fn default() -> Self {
        SnakeDirection {
            value: Direction::default(),
            next_value: Direction::default(),
        }
    }
}

#[derive(Component, Debug)]
pub struct SnakeHead;

#[derive(Component, Debug)]
pub struct SnakeSegment;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Resource, Debug, Default)]
pub struct MovementTimer {
    timer: Timer,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::BeforeGame), spawn_snake)
            .insert_resource(MovementTimer {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            })
            .add_systems(Update, snake_movement_controls.in_set(InGameSet::UserInput))
            .add_systems(
                Update,
                snake_position_update.in_set(InGameSet::EntityUpdates),
            );
    }
}

fn spawn_snake(mut commands: Commands, board: Res<Board>, assets: Res<ImageAssets>) {
    let start_pos = snake_starting_position(board.size);
    let head_texture = TextureAtlas {
        layout: assets.sprite_sheet_layout.clone(),
        index: get_sprite_index(0, 4, 5),
    };

    // load snake head
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                board.position_translate(start_pos[0].x.into()),
                board.position_translate(start_pos[0].y.into()),
                2.0,
            ),
            texture: assets.sprite_sheet.clone(),
            sprite: Sprite {
                // color: Color::BLUE,
                custom_size: Some(Vec2::new(TILE_SIZE - 3., TILE_SIZE - 3.)),
                ..Default::default()
            },
            ..default()
        },
        TextureAtlas::from(head_texture),
        SnakeHead,
        SnakeSegment,
        Position::from(start_pos[0]),
        SnakeDirection::default(),
    ));

    // TextureAtlas::from(texture_atlas).index,
    // load snake tail
    start_pos[1..].iter().for_each(|segment| {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    board.position_translate(segment.x.into()),
                    board.position_translate(segment.y.into()),
                    10.0,
                ),
                texture: assets.sprite_sheet.clone(),
                sprite: Sprite {
                    // color: Color::BLUE,
                    custom_size: Some(Vec2::new(TILE_SIZE - 3., TILE_SIZE - 3.)),
                    ..default()
                },
                ..default()
            },
            SnakeSegment,
            Position::new(segment.x, segment.y),
        ));
    });
}

fn snake_position_update(
    board: Res<Board>,
    mut query: Query<(&mut Transform, &Position), With<SnakeSegment>>,
) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = Vec3::new(
            board.position_translate(pos.x.into()),
            board.position_translate(pos.y.into()),
            1.,
        )
    }
}

// @todo: do I rename this to update direction?
fn snake_movement_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut movement_timer: ResMut<MovementTimer>,
    mut snake_head_query: Query<(&mut SnakeDirection, &mut Position), With<SnakeHead>>,
    mut snake_body_query: Query<(&mut Position, &SnakeSegment), Without<SnakeHead>>,
) {
    let Ok((mut snake_direction, mut head_pos)) = snake_head_query.get_single_mut() else {
        return;
    };

    // so now we have a next direction value, this value will be useful if we want to conditionally
    // check it.
    let new_direction = if keyboard_input.pressed(KeyCode::ArrowUp)
        && snake_direction.value != Direction::Down
    {
        Direction::Up
    } else if keyboard_input.pressed(KeyCode::ArrowDown) && snake_direction.value != Direction::Up {
        Direction::Down
    } else if keyboard_input.pressed(KeyCode::ArrowLeft)
        && snake_direction.value != Direction::Right
    {
        Direction::Left
    } else if keyboard_input.pressed(KeyCode::ArrowRight)
        && snake_direction.value != Direction::Left
    {
        Direction::Right
    } else {
        snake_direction.value
    };

    snake_direction.next_value = match snake_direction.next_value {
        Direction::Up => {}
        Direction::Down => {}
        Direction::Right => {}
        Direction::Left => {}
    };

    // @todo: right now if the user quickly presses a new direction and immediatedly presses the
    // direction in which the snake is moving, the direction that's pressed first will be ignored.

    // update head position based on direction
    snake_direction.value = new_direction;

    // movement_timer.timer.tick(time.delta());
    // if !movement_timer.timer.just_finished() {
    //     return;
    // }

    // store previous head position, before updating it;
    let mut prev_pos = head_pos.clone();

    // @todo: have the applying of the new direction on its own system, it should also hold the
    // time delay.
    // @todo: have a next direction value and match that to check two inputs don't make he snake
    // die.
    // match snake_direction.value {
    //     Direction::Up => head_pos.y += 1,
    //     Direction::Down => head_pos.y -= 1,
    //     Direction::Left => head_pos.x -= 1,
    //     Direction::Right => head_pos.x += 1,
    // }
    //
    // for (mut segment_pos, _) in snake_body_query.iter_mut() {
    //     let temp = *segment_pos;
    //     *segment_pos = prev_pos;
    //     prev_pos = temp;
    // }
}

// @todo: thsi actually effectuates the changes one the positioning of the snake.
// so name it update_position?
fn new_system(mut movement_timer: ResMut<MovementTimer>, time: Res<Time>) {
    movement_timer.timer.tick(time.delta());
    if !movement_timer.timer.just_finished() {
        return;
    }

    // @todo: have the applying of the new direction on its own system, it should also hold the
    // time delay.
    // @todo: have a next direction value and match that to check two inputs don't make he snake
    // die.
    match snake_direction.value {
        Direction::Up => head_pos.y += 1,
        Direction::Down => head_pos.y -= 1,
        Direction::Left => head_pos.x -= 1,
        Direction::Right => head_pos.x += 1,
    }

    for (mut segment_pos, _) in snake_body_query.iter_mut() {
        let temp = *segment_pos;
        *segment_pos = prev_pos;
        prev_pos = temp;
    }
}
