use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct ResolutionSettings {
    pub large: Vec2,
    pub medium: Vec2,
    pub small: Vec2,
}

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        });
    }
}
