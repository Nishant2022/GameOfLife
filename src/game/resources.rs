use bevy::prelude::Resource;

use super::systems::GameState;

#[derive(Resource)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub grid_1: Vec<bool>,
    pub grid_2: Vec<bool>,
    pub grid_flag: bool
}

#[derive(Resource)]
pub struct CurrentGameState {
    pub game_state: GameState
}