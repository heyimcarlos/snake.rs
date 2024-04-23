use bevy::prelude::*;

pub struct ScorePlugin;

#[derive(Resource, Debug)]
pub struct Score {
    pub score: i32,
    pub highest_score: i32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            score: 0,
            highest_score: 0,
        }
    }
}

impl Score {
    pub fn game_over(&mut self) {
        self.highest_score = self.score;
        self.score = 0;
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
