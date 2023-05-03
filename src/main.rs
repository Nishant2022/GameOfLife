use bevy::prelude::*;
use image::PixelImage;

mod image;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelImage)
        .run();
}