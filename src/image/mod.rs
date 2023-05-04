use bevy::prelude::*;

use self::systems::{setup, edit_image};

mod systems;
mod resources;

pub struct PixelImage;

impl Plugin for PixelImage {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(edit_image);
    }
}