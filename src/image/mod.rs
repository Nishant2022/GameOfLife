use bevy::{prelude::*, render::texture::ImageSampler};

pub struct PixelImage;

impl Plugin for PixelImage {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(edit_image);
    }
}

const SCALE_FACTOR: f32 = 5.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn edit_image(handle : Res<ImageHandle>, mut images: ResMut<Assets<Image>>) {

    if let Some(image) = images.get_mut(&handle.handle) {
        image.sampler_descriptor = ImageSampler::nearest();
        let width: usize = image.size().x as usize;
        let height: usize = image.size().y as usize;
        for i in (0..(height * 4)).step_by(8) {
            for j in (0..(width * 4)).step_by(8) {
                update_image_pixel(image, j, i, width, Vec3 { x: 255.0, y: 255.0, z: 127.0 })
            }
        }
    }
}

#[derive(Resource)]
struct ImageHandle {
    handle: Handle<Image>
}

fn update_image_pixel(image: &mut Image, x: usize, y: usize, width: usize, color: Vec3) {

    image.data[y * width + x] = color.x as u8;
    image.data[y * width + x + 1] = color.y as u8;
    image.data[y * width + x + 2] = color.z as u8;
}