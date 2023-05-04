use bevy::{prelude::*, input::mouse::MouseWheel, window::WindowResized};
use rand::prelude::*;

use crate::game::{resources::Board, systems::get_index};

use super::{resources::{ImageHandle, ScaleFactor, FadeColor}, components::{MainCamera, MainImage}};

pub const IMAGE_WIDTH: f32 = 240.0;
pub const IMAGE_HEIGHT: f32 = 135.0;

pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    windows: Query<&Window>) {

    let window = windows.get_single().unwrap();
    let window_width: f32 = window.resolution.width();
    let window_height: f32 = window.resolution.height();

    let scale_factor:f32 = (window_width / IMAGE_WIDTH).min(window_height / IMAGE_HEIGHT);
    commands.insert_resource(ScaleFactor{scale: scale_factor});


    commands.spawn((Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::BLACK)
        },
        ..default()
    }, MainCamera));

    let handle: Handle<Image> = asset_server.load("images/blank.png");
    commands.insert_resource(ImageHandle{handle: handle.clone()});
    
    commands.spawn((SpriteBundle {
        texture: handle,
        transform: Transform {
            scale: Vec3 { x: scale_factor, y: scale_factor, z: 1.0 },
            ..default()
        },
        ..default()
    }, MainImage));

    let mut rng = rand::thread_rng();
    let r: u8 = rng.gen();
    let g: u8 = rng.gen();
    let b: u8 = rng.gen();
    commands.insert_resource(FadeColor{r, g, b});
}

pub fn edit_image(
    handle : Res<ImageHandle>, 
    mut images: ResMut<Assets<Image>>, 
    game: Res<Board>,
    fade_color: Res<FadeColor>) {

    let mut board = &game.grid_1;
    if !game.grid_flag {
        board = &game.grid_2;
    }

    if let Some(image) = images.get_mut(&handle.handle){
        let width: usize = image.size().x as usize;
        let height: usize = image.size().y as usize;
        for row in 0..height {
            for col in 0..width {
                update_image_pixel(image, col, row, width, &board[get_index(col, row, width, 1)], &fade_color)
            }
        }
    }
}

fn update_image_pixel(image: &mut Image, x: usize, y: usize, width: usize, state: &bool, fade_color: &FadeColor) {

    if *state {
        image.data[get_index(x, y, width, 4)] = 255;
        image.data[get_index(x, y, width, 4) + 1] = 255;
        image.data[get_index(x, y, width, 4) + 2] = 255;
    }
    else {
        if image.data[get_index(x, y, width, 4)] == 255 {
            image.data[get_index(x, y, width, 4)] = fade_color.r;
            image.data[get_index(x, y, width, 4) + 1] = fade_color.g;
            image.data[get_index(x, y, width, 4) + 2] = fade_color.b;
        }
        else {
            image.data[get_index(x, y, width, 4)] = image.data[get_index(x, y, width, 4)].saturating_sub(2);
            image.data[get_index(x, y, width, 4) + 1] = image.data[get_index(x, y, width, 4) + 1].saturating_sub(2);
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

pub fn translate_camera(
    mut camera_transform: Query<&mut Transform, With<MainCamera>>,
    camera_scale: Query<&OrthographicProjection, With<MainCamera>>,
    keys: Res<Input<KeyCode>>,
    windows: Query<&Window>
) {
    let mut translation = camera_transform.single_mut();

    let scale = camera_scale.single().scale;
    let camera_top_left = camera_scale.single().area.min;
    let camera_bottom_right = camera_scale.single().area.max;

    let window_width: f32 = windows.get_single().unwrap().resolution.width();
    let window_height: f32 = windows.get_single().unwrap().resolution.height();

    let translation_amount = scale / 0.05;

    if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
        translation.translation.y += translation_amount;
    }
    if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
        translation.translation.x -= translation_amount;
    }
    if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
        translation.translation.y -= translation_amount;
    }
    if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
        translation.translation.x += translation_amount;
    }

    let x: f32 = translation.translation.x;
    let y: f32 = translation.translation.y;


    if -(window_width / 2.0) - camera_top_left.x <= (window_width / 2.0) - camera_bottom_right.x {
        translation.translation.x = x.clamp(-(window_width / 2.0) - camera_top_left.x, (window_width / 2.0) - camera_bottom_right.x);
    }
    if -(window_height / 2.0) - camera_top_left.y <= (window_height / 2.0) - camera_bottom_right.y {
        translation.translation.y = y.clamp(-(window_height / 2.0) - camera_top_left.y, (window_height / 2.0) - camera_bottom_right.y);
    }

}

pub fn window_resize(
    mut events: EventReader<WindowResized>,
    mut scale_factor: ResMut<ScaleFactor>,
    mut images: Query<&mut Transform, With<MainImage>>
) {
    for event in events.iter() {
        scale_factor.scale = (event.width / IMAGE_WIDTH).min(event.height / IMAGE_HEIGHT);
        images.single_mut().scale = Vec3 {x: scale_factor.scale, y: scale_factor.scale, z: 1.0};
    }
}