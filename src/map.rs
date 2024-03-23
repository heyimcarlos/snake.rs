use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Clone, Default, Copy, Debug)]
enum Cell {
    #[default]
    Empty,
    SnakeSegment,
    Food,
}

// @info: the map is a good resource, because it's a singleton and is not dependent on any entity.
#[derive(Resource, Debug, Default)]
struct Map {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![Cell::Empty; width]; height],
            width,
            height,
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::new(32, 18))
            .add_systems(Startup, load_map);
    }
}
//  [
//      y1 [x1, x2]
//      y2 [x1, x2]
// ]
//
const CELL_WIDTH: f32 = 40.;
const CELL_HEIGHT: f32 = 40.;

// default window size is 1280 x 720
// x expands from (-640, 640)
// y expands from (-360, 360)
// top-left most cell should be (-640, 360) if every cell took one pixel

fn load_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Camera>,
) {
    // println!("MAP LOADING; width: {}; height: {}", map.width, map.height);
    let Ok(window) = window_query.get_single() else {
        return;
    };

    // let Ok(camera) = camera_query.get_single() else {
    //     return;
    // };
    // println!("{}", camera.world_to_viewport());
    println!("WINDOW: W:{}, H:{}", window.width(), window.height());
    for y in 0..map.height {
        for x in 0..map.width {
            let start_x =
                -window.width() / 2. + (window.width() - (map.width as f32 * CELL_WIDTH)) / 2.;
            let start_y =
                -window.height() / 2. + (window.height() - (map.height as f32 * CELL_HEIGHT)) / 2.;
            // (column - columns length / 2) * cell_width + cell_width / 2;
            // let x = (start_x as f32 - map.width as f32 / 2.) * CELL_WIDTH + (CELL_WIDTH / 2.);
            // let y = (start_y as f32 - map.height as f32 / 2.) * CELL_HEIGHT + (CELL_HEIGHT / 2.);
            // let pos_x = start_x + x as f32 * CELL_WIDTH + CELL_WIDTH / 2.;
            // let pos_y = start_y + y as f32 * CELL_HEIGHT + CELL_HEIGHT / 2.;
            let pos_x = (x as f32 - map.width as f32 / 2.) + 0.5;
            let pos_y = (y as f32 - map.height as f32 / 2.) + 0.5;
            match map.grid[y][x] {
                Cell::Empty => {
                    // println!("LOADING CELL");
                    commands.spawn(SpriteBundle {
                        transform: Transform::from_xyz(pos_x, pos_y, 0.),
                        sprite: Sprite {
                            color: Color::rgb(0.2, 0.3, 0.2),
                            custom_size: Some(Vec2::new(1., 1.)),
                            ..default()
                        },
                        ..default()
                    });
                }
                Cell::Food => {
                    commands.spawn(SpriteBundle {
                        transform: Transform::from_translation(Vec3::ZERO),
                        sprite: Sprite {
                            color: Color::BLUE,
                            custom_size: Some(Vec2::new(1., 1.)),
                            ..default()
                        },
                        ..default()
                    });
                }
                Cell::SnakeSegment => {
                    commands.spawn(SpriteBundle {
                        transform: Transform::from_translation(Vec3::ZERO),
                        sprite: Sprite {
                            color: Color::ORANGE_RED,
                            ..default()
                        },
                        ..default()
                    });
                }
            };
        }
    }
}
