use bevy::prelude::*;

#[derive(Resource)]
pub struct ImageHandle {
    pub handle: Handle<Image>
}

#[derive(Resource)]
pub struct ScaleFactor {
    pub scale: f32
}