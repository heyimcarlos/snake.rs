use bevy::prelude::*;

use crate::{
    asset_loader::{ImageAssets, SpritePart},
    board::{Board, TILE_SIZE},
    schedule::InGameSet,
    snake::{Direction, Position, SnakeDirectionQueue, SnakeHead, SnakeSegment},
    state::GameState,
    util::food_position,
};

#[derive(Component, Debug)]
pub struct Food;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::BeforeGame), spawn_food)
            .add_systems(
                Update,
                (handle_eat_food, apply_eat_food)
                    .chain()
                    .in_set(InGameSet::EntityUpdates),
            )
            .add_event::<FoodEvent>();
    }
}

#[derive(Event, Debug)]
struct FoodEvent {
    entity: Entity,
}

impl FoodEvent {
    fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

pub fn spawn_food(mut commands: Commands, board: Res<Board>, assets: Res<ImageAssets>) {
    let food_pos = Position::new(board.size / 2 + 5, board.size / 2);
    commands.spawn((
        SpriteSheetBundle {
            atlas: TextureAtlas {
                layout: assets.sprite_sheet_layout.clone(),
                index: SpritePart::Apple as usize,
            },
            texture: assets.sprite_sheet.clone(),
            transform: Transform::from_xyz(
                board.position_translate(food_pos.x),
                board.position_translate(food_pos.y),
                1.0,
            ),
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            ..Default::default()
        },
        Food,
        Position::from(food_pos),
    ));
}

fn handle_eat_food(
    mut food_event_write: EventWriter<FoodEvent>,
    snake_head_query: Query<&Transform, With<SnakeHead>>,
    food_query: Query<(&Transform, Entity), With<Food>>,
) {
    let Ok(head_transform) = snake_head_query.get_single() else {
        return;
    };

    let Ok((food_tranform, food)) = food_query.get_single() else {
        return;
    };

    if head_transform.translation == food_tranform.translation {
        food_event_write.send(FoodEvent::new(food));
    }
}

fn apply_eat_food(
    mut commands: Commands,
    mut food_event_reader: EventReader<FoodEvent>,
    snake_body_query: Query<(&mut Position, &SnakeSegment), Without<SnakeHead>>,
    board: Res<Board>,
    assets: Res<ImageAssets>,
    mut snake_direction_queue: ResMut<SnakeDirectionQueue>,
) {
    for &FoodEvent { entity } in food_event_reader.read() {
        // @info: food eaten, despawn food
        commands.entity(entity).despawn();
        let tail_direction = snake_direction_queue.directions.back().unwrap().clone();
        snake_direction_queue.directions.push_back(tail_direction);

        let Some((&tail_pos, _)) = snake_body_query.iter().last() else {
            return;
        };
        commands.spawn((
            SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: assets.sprite_sheet_layout.clone(),
                    index: match tail_direction {
                        Direction::Up => SpritePart::TailUp as usize,
                        Direction::Down => SpritePart::TailDown as usize,
                        Direction::Left => SpritePart::TailLeft as usize,
                        Direction::Right => SpritePart::TailRight as usize,
                    },
                },
                texture: assets.sprite_sheet.clone(),
                transform: Transform::from_xyz(
                    board.position_translate(tail_pos.x),
                    board.position_translate(tail_pos.y),
                    10.0,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                ..default()
            },
            SnakeSegment,
            Position::from(tail_pos),
        ));

        let food_pos = food_position(board.size);
        commands.spawn((
            SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: assets.sprite_sheet_layout.clone(),
                    index: SpritePart::Apple as usize,
                },
                texture: assets.sprite_sheet.clone(),
                transform: Transform::from_xyz(
                    board.position_translate(food_pos.x),
                    board.position_translate(food_pos.y),
                    1.0,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                ..Default::default()
            },
            Food,
            Position::from(food_pos),
        ));
    }
}
