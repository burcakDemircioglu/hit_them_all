use ggez::{
    self,
    graphics::{self, Text},
    input::keyboard::{self, KeyCode},
    nalgebra as na, Context, GameResult,
};
use rand::{thread_rng, Rng};
use std::fs;
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

pub fn set_controls(
    context: &mut Context,
    dt: f32,
    screend_width: f32,
    player_pos: &mut na::Point2<f32>,
    life: &mut i32,
    score: &mut i32,
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

    if *life <= 0 && keyboard::is_key_pressed(context, KeyCode::Space) {
        *score = 0;
        *life = 3;
    }
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

pub fn get_current_time_as_millis() -> u128 {
    return SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
}

pub fn create_fires(
    last_fire_time: &mut u128,
    fire_positions: &mut std::vec::Vec<na::Point2<f32>>,
    player_pos: na::Point2<f32>,
) {
    let now = get_current_time_as_millis();

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

pub fn draw_game_over_screen(context: &mut Context) -> GameResult {
    let mut game_over_text = Text::new(format!("Game Over!"));
    game_over_text.set_font(graphics::Font::default(), graphics::Scale::uniform(40.0));

    let (screen_width, screen_height) = graphics::drawable_size(context);
    let screen_width_half = screen_width * 0.5;
    let screen_height_half = screen_height * 0.5;
    let (game_over_text_w, game_over_text_h) = game_over_text.dimensions(context);
    let game_over_pos = na::Point2::new(
        screen_width_half - (game_over_text_w / 2) as f32,
        screen_height_half,
    );

    let mut draw_param = graphics::DrawParam::default();
    draw_param.dest = game_over_pos.into();
    graphics::draw(context, &game_over_text, draw_param)?;

    let mut start_again_text = Text::new(format!("Press space to start again."));
    start_again_text.set_font(graphics::Font::default(), graphics::Scale::uniform(20.0));

    let (screen_width, screen_height) = graphics::drawable_size(context);
    let screend_width_half = screen_width * 0.5;
    let screen_height_half = screen_height * 0.5;
    let start_again_text_w = start_again_text.dimensions(context).0;
    let start_again_pos = na::Point2::new(
        screend_width_half - (start_again_text_w / 2) as f32,
        screen_height_half + game_over_text_h as f32 + 10.0,
    );

    let mut draw_param = graphics::DrawParam::default();
    draw_param.dest = start_again_pos.into();
    graphics::draw(context, &start_again_text, draw_param)
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
