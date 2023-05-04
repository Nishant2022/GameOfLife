use bevy::prelude::*;

use self::systems::{setup, edit_image, camera_zoom};

pub mod systems;
pub mod resources;
pub mod components;

pub struct PixelImage;

impl Plugin for PixelImage {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(camera_zoom)
            .add_system(edit_image);
    }
}