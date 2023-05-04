use bevy::prelude::*;

use self::systems::{setup, edit_image, camera_zoom, translate_camera, window_resize};

pub mod systems;
pub mod resources;
pub mod components;

pub struct PixelImage;

impl Plugin for PixelImage {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(camera_zoom)
            .add_system(translate_camera)
            .add_system(window_resize)
            .add_system(edit_image);
    }
}