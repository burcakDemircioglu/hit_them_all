use crate::utilities::constants;
use ggez::{
    self,
    graphics::{self, Color, DrawMode, DrawParam, Mesh, Text},
    nalgebra as na, Context, GameResult,
};

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

pub fn draw_score_board(
    context: &mut Context,
    life: i32,
    highest_score: i32,
    score: i32,
) -> Result<(), ggez::GameError> {
    let screen_width = graphics::drawable_size(context).0;
    let screen_width_half = screen_width * 0.5;

    let mut life_text = Text::new(format!("Life: {}", life));
    life_text.set_font(graphics::Font::default(), graphics::Scale::uniform(24.0));
    let (life_text_w, life_text_h) = life_text.dimensions(context);
    let mut life_pos = na::Point2::new(screen_width_half, constants::SCORE_BOARD_PADDING);
    life_pos -= na::Vector2::new(life_text_w as f32 * 0.5, life_text_h as f32 * 0.5);
    let mut draw_param = graphics::DrawParam::default();
    draw_param.dest = life_pos.into();
    graphics::draw(context, &life_text, draw_param)?;
    let mut score_text = Text::new(format!("Score: {}", score));
    score_text.set_font(graphics::Font::default(), graphics::Scale::uniform(24.0));
    let (score_text_w, score_text_h) = score_text.dimensions(context);
    let mut score_pos = na::Point2::new(
        screen_width_half,
        constants::SCORE_BOARD_PADDING + life_text_h as f32,
    );
    score_pos -= na::Vector2::new(score_text_w as f32 * 0.5, score_text_h as f32 * 0.5);
    draw_param.dest = score_pos.into();
    graphics::draw(context, &score_text, draw_param)?;
    let mut highest_score_text = Text::new(format!("Highest Score: {}", highest_score));
    highest_score_text.set_font(graphics::Font::default(), graphics::Scale::uniform(24.0));
    let (highest_score_text_w, highest_score_text_h) = highest_score_text.dimensions(context);
    let mut highest_score_pos = na::Point2::new(
        screen_width_half,
        constants::SCORE_BOARD_PADDING + life_text_h as f32 + score_text_h as f32,
    );
    highest_score_pos -= na::Vector2::new(
        highest_score_text_w as f32 * 0.5,
        highest_score_text_h as f32 * 0.5,
    );
    draw_param.dest = highest_score_pos.into();
    graphics::draw(context, &highest_score_text, draw_param)?;
    Ok(())
}

pub fn draw_fires(
    fire_positions: &mut std::vec::Vec<na::Point2<f32>>,
    context: &mut Context,
) -> Result<(), ggez::GameError> {
    Ok(for fire_pos in fire_positions.iter() {
        let origin = *fire_pos;
        let dest = na::Point2::new(fire_pos.x, fire_pos.y + constants::FIRE_LENGTH);

        let fire_mesh = Mesh::new_line(
            context,
            &[origin, dest],
            constants::FIRE_WIDTH,
            Color::from_rgb(100, 0, 0),
        )?;
        graphics::draw(context, &fire_mesh, DrawParam::default())?;
    })
}

pub fn draw_invaders(
    invader_positions: &mut std::vec::Vec<na::Point2<f32>>,
    context: &mut Context,
) -> Result<(), ggez::GameError> {
    Ok(for invader_pos in invader_positions.iter() {
        let invader = graphics::Rect::new(
            invader_pos.x,
            invader_pos.y,
            constants::INVADER_SIZE,
            constants::INVADER_SIZE,
        );
        let invader_mesh =
            Mesh::new_rectangle(context, DrawMode::fill(), invader, graphics::WHITE)?;
        graphics::draw(context, &invader_mesh, DrawParam::default())?;
    })
}

pub fn draw_player(
    player_pos: na::Point2<f32>,
    context: &mut Context,
) -> Result<(), ggez::GameError> {
    let player = graphics::Rect::new(
        player_pos.x,
        player_pos.y,
        constants::PLAYER_WIDTH,
        constants::PLAYER_HEIGHT,
    );
    let player_mesh = Mesh::new_rectangle(context, DrawMode::fill(), player, graphics::WHITE)?;
    graphics::draw(context, &player_mesh, DrawParam::default())?;
    Ok(())
}
