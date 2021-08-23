use ggez::{
    self,
    input::keyboard::{self, KeyCode},
    nalgebra as na, Context,
};
use std::time::SystemTime;

use crate::constants;

pub fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

pub fn is_hit(
    invader_x: f32,
    invader_y: f32,
    invader_w: f32,
    invader_h: f32,
    fire_x: f32,
    fire_y: f32,
    fire_w: f32,
) -> bool {
    return fire_x < invader_x + invader_w
        && fire_x + fire_w > invader_x
        && fire_y < invader_y
        && fire_y > invader_y - invader_h;
}

pub fn get_hits(
    fire_positions: &mut std::vec::Vec<na::Point2<f32>>,
    invader_positions: &mut std::vec::Vec<na::Point2<f32>>,
    score: &mut i32,
) -> (Vec<na::Point<f32, na::U2>>, Vec<na::Point<f32, na::U2>>) {
    let mut hit_fire = std::vec::Vec::new();
    let mut hit_invader = std::vec::Vec::new();
    for fire_index in 0..fire_positions.len() {
        let fire = fire_positions[fire_index];

        for invader_index in 0..invader_positions.len() {
            let invader = invader_positions[invader_index];

            let is_hit = is_hit(
                invader.x,
                invader.y,
                constants::INVADER_SIZE,
                constants::INVADER_SIZE,
                fire.x,
                fire.y,
                constants::FIRE_WIDTH,
            );

            if is_hit {
                hit_fire.push(fire);
                hit_invader.push(invader);
                *score += 1;
            }
        }
    }
    (hit_fire, hit_invader)
}

pub fn set_controls(
    context: &mut Context,
    dt: f32,
    screend_width: f32,
    player_pos: &mut na::Point2<f32>,
) {
    if keyboard::is_key_pressed(context, KeyCode::Right) {
        player_pos.x += constants::PLAYER_SPEED * dt;
        clamp(
            &mut player_pos.x,
            0.0,
            screend_width - constants::PLAYER_WIDTH,
        )
    }
    if keyboard::is_key_pressed(context, KeyCode::Left) {
        player_pos.x -= constants::PLAYER_SPEED * dt;
        clamp(
            &mut player_pos.x,
            0.0,
            screend_width - constants::PLAYER_WIDTH,
        )
    }
}

pub fn get_current_time_as_millis()->u128{
return  SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
}

pub fn create_fires(last_fire_time:&mut u128, fire_positions: &mut std::vec::Vec<na::Point2<f32>>, player_pos: na::Point2<f32>){
        let now = get_current_time_as_millis();

        let time_past_since_last_fire = now - *last_fire_time;
        if time_past_since_last_fire > constants::FIRE_PADDING as u128 {
            fire_positions
                .push(na::Point2::<f32>::new(player_pos.x, player_pos.y));
            fire_positions.push(na::Point2::<f32>::new(
                player_pos.x + constants::PLAYER_WIDTH,
                player_pos.y,
            ));
            *last_fire_time = now;
        }
}