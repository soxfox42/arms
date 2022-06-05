use ggez::*;

#[derive(Default)]
struct State {}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(255, 127, 0));
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let state = State::default();

    let (ctx, event_loop) = ContextBuilder::new("arms", "soxfox42")
        .window_setup(conf::WindowSetup::default().title("The Arms"))
        .window_mode(conf::WindowMode::default().dimensions(1200.0, 800.0))
        .build()
        .unwrap();
    event::run(ctx, event_loop, state);
}
