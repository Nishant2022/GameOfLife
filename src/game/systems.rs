use bevy::prelude::*;
use rand::{distributions::Bernoulli, prelude::Distribution};

use super::resources::Board;


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
                    if count != 2 && count != 3 {
                        game.grid_2[get_pos(x, y, width, height)] = false;
                    }
                    else {
                        game.grid_2[get_pos(x, y, width, height)] = true;
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
                    if count != 2 && count != 3 {
                        game.grid_1[get_pos(x, y, width, height)] = false;
                    }
                    else {
                        game.grid_1[get_pos(x, y, width, height)] = true;
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