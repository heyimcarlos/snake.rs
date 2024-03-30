use bevy::prelude::*;

use crate::{snake::Position, state::GameState};

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), despawn_all_entities);
    }
}

fn despawn_all_entities(mut commands: Commands, query: Query<Entity, With<Position>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
