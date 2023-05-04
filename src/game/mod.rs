use bevy::prelude::*;

use self::systems::{game_step, setup};

pub mod systems;
pub mod resources;

pub struct GameOfLife;

impl Plugin for GameOfLife {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(game_step)
            ;
    }
}