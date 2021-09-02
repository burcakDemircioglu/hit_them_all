use crate::utilities::constants;
use crate::utilities::utility;
use ggez::{self, graphics, nalgebra as na, Context};

pub fn update_invaders(
    context: &mut Context,
    invader_positions: &mut std::vec::Vec<na::Point2<f32>>,
    hit_invader: Vec<na::Point<f32, na::U2>>,
    life: &mut i32,
) {
    let (screend_width, screen_height) = graphics::drawable_size(context);

    let dt = ggez::timer::delta(context).as_secs_f32();

    for invader_pos in invader_positions {
        // Reset hit invaders
        if hit_invader.contains(invader_pos) {
            *invader_pos = utility::get_init_invader_pos(screend_width);
        }

        // Move invaders
        invader_pos.y += constants::INVADER_SPEED * dt;

        // Reset out of window invaders
        if invader_pos.y > screen_height {
            *invader_pos = utility::get_init_invader_pos(screend_width);
            *life -= 1;
        }
    }
}

pub fn update_fires(
    context: &mut Context,
    fire_positions: &mut std::vec::Vec<na::Point2<f32>>,
    hit_fire: std::vec::Vec<na::Point2<f32>>,
) {
    let dt = ggez::timer::delta(context).as_secs_f32();

    // Delete out of window fires
    fire_positions.retain(|fire| fire.y > 0.0);

    // Delete hit fires
    fire_positions.retain(|fire| !hit_fire.contains(fire));
    for fire_pos in fire_positions {
        fire_pos.y -= constants::FIRE_SPEED * dt;
    }
}

pub fn create_fires(
    last_fire_time: &mut u128,
    fire_positions: &mut std::vec::Vec<na::Point2<f32>>,
    player_pos: na::Point2<f32>,
) {
    let now = utility::get_current_time_as_millis();

    let time_past_since_last_fire = now - *last_fire_time;
    if time_past_since_last_fire > constants::FIRE_PADDING as u128 {
        fire_positions.push(na::Point2::<f32>::new(player_pos.x, player_pos.y));
        fire_positions.push(na::Point2::<f32>::new(
            player_pos.x + constants::PLAYER_WIDTH,
            player_pos.y,
        ));
        *last_fire_time = now;
    }
}
