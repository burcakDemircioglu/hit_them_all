use crate::utilities::constants;
use ggez::{
    self, graphics,
    input::keyboard::{self, KeyCode},
    nalgebra as na, Context,
};
use rand::{thread_rng, Rng};
use std::fs;
use std::time::SystemTime;

pub fn get_current_time_as_millis() -> u128 {
    return SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
}

pub fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

pub fn get_highest_score() -> i32 {
    let contents = fs::read_to_string(constants::HIGHEST_SCORE_FILE_NAME);
    if contents.is_err() {
        return 0;
    }
    let first_line = contents.unwrap().lines().next().unwrap().to_string();
    let highest_score = first_line.parse::<i32>();
    if highest_score.is_err() {
        return 0;
    }
    return highest_score.unwrap();
}

pub fn set_highest_score(score: i32) -> std::io::Result<()> {
    fs::write(constants::HIGHEST_SCORE_FILE_NAME, score.to_string())
}

pub fn set_controls(
    context: &mut Context,
    player_pos: &mut na::Point2<f32>,
    life: &mut i32,
    score: &mut i32,
) {
    let screend_width = graphics::drawable_size(context).0;
    let dt = ggez::timer::delta(context).as_secs_f32();

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

    if *life <= 0 && keyboard::is_key_pressed(context, KeyCode::Space) {
        *score = 0;
        *life = 3;
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

pub fn get_init_invader_pos(screend_width: f32) -> na::Point<f32, na::U2> {
    return na::Point2::new(
        thread_rng().gen_range(0.0, screend_width - constants::INVADER_SIZE),
        0.0 - constants::INVADER_SIZE,
    );
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

pub fn create_invaders(screend_width: f32) -> Vec<na::Point<f32, na::U2>> {
    let mut rng = thread_rng();

    let mut invaders = std::vec::Vec::<na::Point2<f32>>::new();
    for _i in 0..constants::INVADER_AMOUNT {
        invaders.push(na::Point2::<f32>::new(
            rng.gen_range(0.0, screend_width - constants::INVADER_SIZE),
            rng.gen_range(-1000.0, 0.0),
        ));
    }
    invaders
}

pub fn reset_the_game(
    context: &mut Context,
    player_pos: &mut na::Point2<f32>,
    invader_positions: &mut std::vec::Vec<na::Point2<f32>>,
    fire_positions: &mut std::vec::Vec<na::Point2<f32>>,
) {
    let (screend_width, screen_height) = graphics::drawable_size(context);

    *player_pos = na::Point2::new(
        (screend_width * 0.5) - constants::PLAYER_WIDTH_HALF,
        screen_height - constants::PLAYER_PADDING,
    );
    *invader_positions = create_invaders(screend_width);
    *fire_positions = std::vec::Vec::new();
}
