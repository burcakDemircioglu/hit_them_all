use ggez::{
    self, event,
    graphics::{self, Color, DrawMode, DrawParam, Mesh},
    input::keyboard::{self, KeyCode},
    nalgebra as na, Context, GameResult,
};

mod constants;
mod utilities;

pub struct GameState {
    player_pos: na::Point2<f32>,
    invader_pos: std::collections::HashSet<na::Point2<f32>>,
    fire_pos: std::collections::HashSet<na::Point2<f32>>,
    score: i32,
    life: i32,
}

impl GameState {
    pub fn new(context: &mut Context) -> Self {
        let (screend_width, screen_hight) = graphics::drawable_size(context);
        let (screend_width_half, screen_hight_half) = (screend_width * 0.5, screen_hight * 0.5);

        GameState {
            player_pos: na::Point2::new(
                screend_width_half - constants::PLAYER_WIDTH_HALF,
                screen_hight - constants::PLAYER_PADDING,
            ),
            invader_pos: std::collections::HashSet::new(),
            fire_pos: std::collections::HashSet::new(),
            score: 0,
            life: 3,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(context).as_secs_f32();
        let (screend_width, screen_hight) = graphics::drawable_size(context);

        if keyboard::is_key_pressed(context, KeyCode::Right) {
            self.player_pos.x += constants::PLAYER_SPEED * dt;
            utilities::clamp(
                &mut self.player_pos.x,
                0.0,
                screend_width - constants::PLAYER_WIDTH,
            )
        }

        if keyboard::is_key_pressed(context, KeyCode::Left) {
            self.player_pos.x -= constants::PLAYER_SPEED * dt;
            utilities::clamp(
                &mut self.player_pos.x,
                0.0,
                screend_width - constants::PLAYER_WIDTH,
            )
        }

        Ok(())
    }
    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, Color::from_rgb(100, 0, 0));
        let (screend_width, screen_hight) = graphics::drawable_size(context);
        let (screend_width_half, screen_hight_half) = (screend_width * 0.5, screen_hight * 0.5);

        // Draw player
        let player = graphics::Rect::new(
            self.player_pos.x,
            self.player_pos.y,
            constants::PLAYER_WIDTH,
            constants::PLAYER_HEIGHT,
        );
        let player_mesh = Mesh::new_rectangle(context, DrawMode::fill(), player, graphics::WHITE)?;
        graphics::draw(context, &player_mesh, DrawParam::default())?;

        graphics::present(context)?;
        Ok(())
    }
}
