use crate::utilities::constants;
use crate::utilities::draw;
use crate::utilities::update;
use crate::utilities::utility;
use ggez::{
    self, event,
    graphics::{self, Color},
    nalgebra as na, Context, GameResult,
};

pub struct GameState {
    player_pos: na::Point2<f32>,
    invader_positions: std::vec::Vec<na::Point2<f32>>,
    fire_positions: std::vec::Vec<na::Point2<f32>>,
    score: i32,
    life: i32,
    last_fire_time: u128,
    highest_score: i32,
}

impl GameState {
    pub fn new(context: &mut Context) -> Self {
        let (screend_width, screen_height) = graphics::drawable_size(context);

        GameState {
            player_pos: na::Point2::new(
                (screend_width * 0.5) - constants::PLAYER_WIDTH_HALF,
                screen_height - constants::PLAYER_PADDING,
            ),
            invader_positions: utility::create_invaders(screend_width),
            fire_positions: std::vec::Vec::new(),
            score: 0,
            life: 3,
            last_fire_time: utility::get_current_time_as_millis(),
            highest_score: utility::get_highest_score(),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        utility::set_controls(
            context,
            &mut self.player_pos,
            &mut self.life,
            &mut self.score,
        );

        if self.life <= 0 {
            utility::reset_the_game(
                context,
                &mut self.player_pos,
                &mut self.invader_positions,
                &mut self.fire_positions,
            );
            return Ok(());
        }

        let (hit_fire, hit_invader) = utility::get_hits(
            &mut self.fire_positions,
            &mut self.invader_positions,
            &mut self.score,
        );

        update::update_fires(context, &mut self.fire_positions, hit_fire);
        update::create_fires(
            &mut self.last_fire_time,
            &mut self.fire_positions,
            self.player_pos,
        );

        update::update_invaders(
            context,
            &mut self.invader_positions,
            hit_invader,
            &mut self.life,
        );

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, Color::from_rgb_u32(constants::SCREEN_COLOR));

        draw::draw_score_board(context, self.life, self.highest_score, self.score)?;

        if self.life <= 0 {
            if self.score > self.highest_score {
                utility::set_highest_score(self.score)?;
                self.highest_score = self.score;
            }
            draw::draw_game_over_screen(context)?;
            graphics::present(context)?;
            return Ok(());
        }

        draw::draw_player(self.player_pos, context)?;

        draw::draw_invaders(&mut self.invader_positions, context)?;

        draw::draw_fires(&mut self.fire_positions, context)?;

        graphics::present(context)?;
        Ok(())
    }
}
