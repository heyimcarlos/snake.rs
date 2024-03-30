use bevy::prelude::*;

#[derive(Debug, States, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_systems(
            Update,
            (
                game_state_input_events,
                // transition_to_in_game.run_if(in_state(GameState::GameOver)),
            ),
        );
    }
}

fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            _ => (),
        }
    }
}

fn _transition_to_in_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}