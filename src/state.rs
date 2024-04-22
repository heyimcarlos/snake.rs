use bevy::prelude::*;

#[derive(Debug, States, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub enum GameState {
    Playing,
    #[default]
    NewGame,
    Paused,
    GameOver,
}

#[derive(Debug, States, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub enum MenuState {
    #[default]
    On,
    Off,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_state::<MenuState>()
            .add_systems(Update, state_input_events)
            .add_systems(OnEnter(GameState::GameOver), transition_to_new_game);
    }
}

fn state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    game_state: Res<State<GameState>>,
    menu_state: Res<State<MenuState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match game_state.get() {
            GameState::Playing => {
                next_state.set(GameState::Paused);
                next_menu_state.set(MenuState::On);
            }
            GameState::Paused => {
                next_state.set(GameState::Playing);
                next_menu_state.set(MenuState::Off);
            }
            _ => (),
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        match game_state.get() {
            GameState::GameOver => next_state.set(GameState::NewGame),
            _ => (),
        }
    }

    //  NOTE: If the menu is not showing and arrow keys are pressed start playing
    if menu_state.get() == &MenuState::Off
        && keyboard_input.any_just_pressed([
            KeyCode::ArrowUp,
            KeyCode::ArrowDown,
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
        ])
    {
        match game_state.get() {
            GameState::NewGame => next_state.set(GameState::Playing),
            _ => (),
        }
    }
}

fn transition_to_new_game(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    next_game_state.set(GameState::NewGame);
    next_menu_state.set(MenuState::On);
}
