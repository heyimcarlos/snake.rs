use bevy::prelude::*;

pub struct ScorePlugin;

#[derive(Resource, Debug)]
pub struct Score {
    pub value: i32,
    pub previous: i32,
    pub highest: i32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            value: 0,
            previous: 0,
            highest: 0,
        }
    }
}

impl Score {
    pub fn game_over(&mut self) {
        if self.value > self.highest {
            self.highest = self.value;
        }
        self.value = 0;
    }
}

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
    }
}

// pub struct ScoreEvent {
//
// }

//  NOTE: Scoreboard
//  1. when the snake eats, increase the score
//  2. since it's a monolithic state, let's keep it as a resource?
