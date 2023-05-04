use bevy::prelude::*;
use rand::{distributions::Bernoulli, prelude::Distribution};

use crate::image::{components::MainCamera, resources::ScaleFactor};

use super::resources::{Board, CurrentGameState};

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Running,
    Paused
}

pub fn setup(mut commands: Commands) {
    let width: usize = 240;
    let height: usize = 135;
    let size: usize = width * height;
    let mut grid_vec: Vec<bool> = Vec::with_capacity(size);
    let dist = Bernoulli::new(0.1).unwrap();
    let mut rng = rand::thread_rng();

    for _i in 0..size {
        grid_vec.push(dist.sample(&mut rng));
    }

    commands.insert_resource(Board {width: width, height: height, grid_1: grid_vec[0..size].to_vec(), grid_2: grid_vec, grid_flag: false});

    commands.insert_resource(CurrentGameState{game_state: GameState::Running});
}

pub fn game_step(mut game: ResMut<Board>) {
    let width: i32 = game.width as i32;
    let height: i32 = game.height as i32;

    if game.grid_flag {
        for x in 0..width{
            for y in 0..height {
                let mut count = 0;
                for i in -1..=1 {
                    for j in -1..=1 {
                        if game.grid_1[get_pos(x + i, y + j, width, height)] {
                            count += 1;
                        }
                    }
                }
                if game.grid_1[get_pos(x, y, width, height)] {
                    count -= 1;
                    if count == 2 || count == 3 {
                        game.grid_2[get_pos(x, y, width, height)] = true;
                    }
                    else {
                        game.grid_2[get_pos(x, y, width, height)] = false;
                    }
                }
                else {
                    if count == 3 {
                        game.grid_2[get_pos(x, y, width, height)] = true;
                    }
                    else {
                        game.grid_2[get_pos(x, y, width, height)] = false;
                    }
                }
            }
        }
        game.grid_flag = false;
    }
    else {
        for x in 0..width{
            for y in 0..height {
                let mut count = 0;
                for i in -1..=1 {
                    for j in -1..=1 {
                        if game.grid_2[get_pos(x + i, y + j, width, height)] {
                            count += 1;
                        }
                    }
                }
                if game.grid_2[get_pos(x, y, width, height)] {
                    count -= 1;
                    if count == 2 || count == 3 {
                        game.grid_1[get_pos(x, y, width, height)] = true;
                    }
                    else {
                        game.grid_1[get_pos(x, y, width, height)] = false;
                    }
                }
                else {
                    if count == 3 {
                        game.grid_1[get_pos(x, y, width, height)] = true;
                    }
                    else {
                        game.grid_1[get_pos(x, y, width, height)] = false;
                    }
                }
            }
        }
        game.grid_flag = true;
    }
}

pub fn get_index(x: usize, y: usize, width: usize, step: usize) -> usize {
    (y * width + x) * step
}

fn get_pos(x: i32, y: i32, width: i32, height: i32) -> usize {
    get_index(x.rem_euclid(width) as usize, y.rem_euclid(height) as usize, width as usize, 1)
}

pub fn place_cells(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    buttons: Res<Input<MouseButton>>,
    mut board: ResMut<Board>,
    scale_factor: Res<ScaleFactor>
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    let window = windows.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let x = (world_position.x / scale_factor.scale + 120.0) as i32;
        let y = (- world_position.y / scale_factor.scale + 67.5) as i32;
        
        if x < 0 || x > 239 || y < 0 || y > 134 {return}

        let index = get_index(x as usize, y as usize, 240, 1);

        if buttons.pressed(MouseButton::Left) {
            if board.grid_flag {
                board.grid_1[index] = true;
            }
            else {
                board.grid_2[index] = true;
            }
        }
        if buttons.pressed(MouseButton::Right) {
            if board.grid_flag {
                board.grid_1[index] = false;
            }
            else {
                board.grid_2[index] = false;
            }
        }
    }
}

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut current_state: ResMut<CurrentGameState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut board: ResMut<Board>
) {
    if keys.just_pressed(KeyCode::Space) {
        match current_state.game_state {
            GameState::Running => {
                next_state.set(GameState::Paused);
                current_state.game_state = GameState::Paused;
            },
            GameState::Paused => {
                next_state.set(GameState::Running);
                current_state.game_state = GameState::Running;
            },
        }
    }
    if keys.just_pressed(KeyCode::C) {
        board.grid_1.fill(false);
    }
}