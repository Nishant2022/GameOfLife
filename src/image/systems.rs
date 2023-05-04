use bevy::{prelude::*, render::texture::ImageSampler, input::mouse::MouseWheel};

use crate::game::{resources::Board, systems::get_index};

use super::{resources::{ImageHandle, ScaleFactor}, components::MainCamera};

const IMAGE_WIDTH: f32 = 240.0;
const IMAGE_HEIGHT: f32 = 135.0;

pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    windows: Query<&Window>) {

    let window = windows.get_single().unwrap();
    let window_width: f32 = window.resolution.width();
    let window_height: f32 = window.resolution.height();

    let scale_factor:f32 = (window_width / IMAGE_WIDTH).max(window_height / IMAGE_HEIGHT);
    commands.insert_resource(ScaleFactor{scale: scale_factor});


    commands.spawn((Camera2dBundle::default(), MainCamera));

    let handle: Handle<Image> = asset_server.load("images/blank.png");
    commands.insert_resource(ImageHandle{handle: handle.clone()});
    
    commands.spawn(SpriteBundle {
        texture: handle,
        transform: Transform {
            scale: Vec3 { x: scale_factor, y: scale_factor, z: 1.0 },
            ..default()
        },
        ..default()
    });
}

pub fn edit_image(handle : Res<ImageHandle>, mut images: ResMut<Assets<Image>>, game: Res<Board>) {

    let mut board = &game.grid_1;
    if !game.grid_flag {
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
        if image.data[get_index(x, y, width, 4)] == 255 {
            image.data[get_index(x, y, width, 4)] = 0;
            image.data[get_index(x, y, width, 4) + 1] = 0;
            image.data[get_index(x, y, width, 4) + 2] = 255;
        }
        else {
            image.data[get_index(x, y, width, 4) + 2] = image.data[get_index(x, y, width, 4) + 2].saturating_sub(2);
        }
    }

}

pub fn camera_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    let mut projection = camera.single_mut();

    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if ev.y < 0.0 {
                    projection.scale *= 1.25 * -ev.y;
                }
                else {
                    projection.scale *= 0.75 * ev.y;
                }
            }
            MouseScrollUnit::Pixel => {
                println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
        }
    }

    projection.scale = projection.scale.clamp(0.05, 1.0);
}