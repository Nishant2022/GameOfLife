use bevy::{prelude::*, window::WindowResolution, render::texture::ImageSampler};
use game::GameOfLife;
use image::PixelImage;

mod image;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Nishant's Game of Life".to_string(),
                    resolution: WindowResolution::new(1280.0, 720.0),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin {
                default_sampler: ImageSampler::nearest_descriptor()
            })
        )
        .add_plugin(GameOfLife)
        .add_plugin(PixelImage)
        .run();
}