use ggez::{self, event, nalgebra as na, Context, GameResult, graphics::{self, Color}};

pub struct GameState {
    player_pos: na::Point2<f32>,
    invader_pos: std::collections::HashSet<na::Point2<f32>>,
    fire_pos: std::collections::HashSet<na::Point2<f32>>,
    score: i32,
    life: i32,
}

impl GameState {
    pub fn new(context: &mut Context) -> Self {
        GameState {
            player_pos: na::Point2::new(0.0, 0.0),
            invader_pos:std::collections::HashSet::new(),
            fire_pos:std::collections::HashSet::new(),
            score:0,
            life:3,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::Color::from_rgb(100,0, 0));

        graphics::present(context)?;
        Ok(())
    }
}
