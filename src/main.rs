use bevy::prelude::*;
use game::GameOfLife;
use image::PixelImage;

mod image;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GameOfLife)
        .add_plugin(PixelImage)
        .run();
}