use ggez::{self, event, graphics, GameResult};
mod game;
mod utilities;

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("Hit Them All", "BurcakKam");
    let (context, event_loop) = &mut context_builder.build()?;
    graphics::set_window_title(context, "Hit Them All");

    let mut state = game::GameState::new(context);
    event::run(context, event_loop, &mut state)?;
    Ok(())
}
