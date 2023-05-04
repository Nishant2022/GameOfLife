use bevy::{prelude::*, render::texture::ImageSampler};

use crate::game::{resources::Board, systems::get_index};

use super::resources::ImageHandle;

const SCALE_FACTOR: f32 = 5.0;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let handle: Handle<Image> = asset_server.load("images/blank.png");
    commands.insert_resource(ImageHandle{handle: handle.clone()});
    commands.spawn(SpriteBundle {
        texture: handle,
        transform: Transform {
            scale: Vec3 { x: SCALE_FACTOR, y: SCALE_FACTOR, z: 1.0 },
            ..default()
        },
        ..default()
    });
}

pub fn edit_image(handle : Res<ImageHandle>, mut images: ResMut<Assets<Image>>, game: Res<Board>) {

    let mut board = &game.grid_1;
    if game.grid_flag {
        board = &game.grid_2;
    }

    if let Some(image) = images.get_mut(&handle.handle){
        image.sampler_descriptor = ImageSampler::nearest();
        let width: usize = image.size().x as usize;
        let height: usize = image.size().y as usize;
        for row in 0..height {
            for col in 0..width {
                update_image_pixel(image, col, row, width, &board[get_index(col, row, width, 1)])
            }
        }
    }
}

fn update_image_pixel(image: &mut Image, x: usize, y: usize, width: usize, state: &bool) {

    if *state {
        image.data[get_index(x, y, width, 4)] = 255;
        image.data[get_index(x, y, width, 4) + 1] = 255;
        image.data[get_index(x, y, width, 4) + 2] = 255;
    }
    else {
        image.data[get_index(x, y, width, 4)] = 0;
        image.data[get_index(x, y, width, 4) + 1] = 0;
        image.data[get_index(x, y, width, 4) + 2] = 0;
    }

}