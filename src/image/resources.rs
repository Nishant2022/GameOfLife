use bevy::prelude::*;

#[derive(Resource)]
pub struct ImageHandle {
    pub handle: Handle<Image>
}

#[derive(Resource)]
pub struct ScaleFactor {
    pub scale: f32
}

#[derive(Resource)]
pub struct FadeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}