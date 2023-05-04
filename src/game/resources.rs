use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub grid_1: Vec<bool>,
    pub grid_2: Vec<bool>,
    pub grid_flag: bool
}