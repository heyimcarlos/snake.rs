use bevy::prelude::*;

use crate::state::GameState;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    DespawnEntities,
    UserInput,
    PositionUpdates,
    CollisionDetection,
    EntityUpdates,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::DespawnEntities,
                InGameSet::UserInput,
                InGameSet::PositionUpdates,
                InGameSet::CollisionDetection,
                InGameSet::EntityUpdates,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}
