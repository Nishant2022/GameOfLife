use bevy::prelude::*;

use self::systems::{game_step, setup, place_cells, GameState};

pub mod systems;
pub mod resources;

pub struct GameOfLife;

impl Plugin for GameOfLife {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_startup_system(setup)
            .add_system(game_step.run_if(in_state(GameState::Running)))
            .add_system(place_cells)
            ;
    }
}