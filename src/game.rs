use crate::constants;
use crate::utilities;
use ggez::{
    self, event,
    graphics::{self, Color, DrawMode, DrawParam, Mesh, Text},
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
            invader_positions: utilities::create_invaders(screend_width),
            fire_positions: std::vec::Vec::new(),
            score: 0,
            life: 3,
            last_fire_time: utilities::get_current_time_as_millis(),
            highest_score: utilities::get_highest_score(),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(context).as_secs_f32();
        let (screend_width, screen_height) = graphics::drawable_size(context);

        utilities::set_controls(
            context,
            dt,
            screend_width,
            &mut self.player_pos,
            &mut self.life,
            &mut self.score,
        );

        if self.life <= 0 {
            utilities::reset_the_game(
                context,
                &mut self.player_pos,
                &mut self.invader_positions,
                &mut self.fire_positions,
            );
            return Ok(());
        }

        // Delete out of window fires
        self.fire_positions.retain(|fire| fire.y > 0.0);

        // Find the hits
        let (hit_fire, hit_invader) = utilities::get_hits(
            &mut self.fire_positions,
            &mut self.invader_positions,
            &mut self.score,
        );

        // Delete hit fires
        self.fire_positions.retain(|fire| !hit_fire.contains(fire));

        // Move fires
        for fire_pos in &mut self.fire_positions {
            fire_pos.y -= constants::FIRE_SPEED * dt;
        }

        // Move invaders
        for invader_pos in &mut self.invader_positions {
            // Reset hit invaders
            if hit_invader.contains(invader_pos) {
                *invader_pos = utilities::get_init_invader_pos(screend_width);
            }

            // Move invaders
            invader_pos.y += constants::INVADER_SPEED * dt;

            // Reset out of window invaders
            if invader_pos.y > screen_height {
                *invader_pos = utilities::get_init_invader_pos(screend_width);
                self.life -= 1;
            }
        }

        utilities::create_fires(
            &mut self.last_fire_time,
            &mut self.fire_positions,
            self.player_pos,
        );

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, Color::from_rgb_u32(constants::SCREEN_COLOR));
        let screen_width = graphics::drawable_size(context).0;
        let screen_width_half = screen_width * 0.5;

        // Draw score board
        let mut life_text = Text::new(format!("Life: {}", self.life));
        life_text.set_font(graphics::Font::default(), graphics::Scale::uniform(24.0));

        let (life_text_w, life_text_h) = life_text.dimensions(context);
        let mut life_pos = na::Point2::new(screen_width_half, constants::SCORE_BOARD_PADDING);
        life_pos -= na::Vector2::new(life_text_w as f32 * 0.5, life_text_h as f32 * 0.5);

        let mut draw_param = graphics::DrawParam::default();
        draw_param.dest = life_pos.into();
        graphics::draw(context, &life_text, draw_param)?;

        let mut score_text = Text::new(format!("Score: {}", self.score));
        score_text.set_font(graphics::Font::default(), graphics::Scale::uniform(24.0));

        let (score_text_w, score_text_h) = score_text.dimensions(context);
        let mut score_pos = na::Point2::new(
            screen_width_half,
            constants::SCORE_BOARD_PADDING + life_text_h as f32,
        );
        score_pos -= na::Vector2::new(score_text_w as f32 * 0.5, score_text_h as f32 * 0.5);

        draw_param.dest = score_pos.into();
        graphics::draw(context, &score_text, draw_param)?;

        let mut highest_score_text = Text::new(format!("Highest Score: {}", self.highest_score));
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

        if self.life <= 0 {
            if self.score > self.highest_score {
                utilities::set_highest_score(self.score)?;
                self.highest_score = self.score;
            }
            utilities::draw_game_over_screen(context)?;
            graphics::present(context)?;
            return Ok(());
        }

        // Draw player
        let player = graphics::Rect::new(
            self.player_pos.x,
            self.player_pos.y,
            constants::PLAYER_WIDTH,
            constants::PLAYER_HEIGHT,
        );
        let player_mesh = Mesh::new_rectangle(context, DrawMode::fill(), player, graphics::WHITE)?;
        graphics::draw(context, &player_mesh, DrawParam::default())?;

        // Draw invaders
        for invader_pos in self.invader_positions.iter() {
            let invader = graphics::Rect::new(
                invader_pos.x,
                invader_pos.y,
                constants::INVADER_SIZE,
                constants::INVADER_SIZE,
            );
            let invader_mesh =
                Mesh::new_rectangle(context, DrawMode::fill(), invader, graphics::WHITE)?;
            graphics::draw(context, &invader_mesh, DrawParam::default())?;
        }

        // Draw fires
        for fire_pos in self.fire_positions.iter() {
            let origin = *fire_pos;
            let dest = na::Point2::new(fire_pos.x, fire_pos.y + constants::FIRE_LENGTH);

            let fire_mesh = Mesh::new_line(
                context,
                &[origin, dest],
                constants::FIRE_WIDTH,
                Color::from_rgb(100, 0, 0),
            )?;
            graphics::draw(context, &fire_mesh, DrawParam::default())?;
        }

        graphics::present(context)?;
        Ok(())
    }
}
