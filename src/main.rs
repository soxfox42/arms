use ggez::*;
use glam::Vec2;

struct State {
    pos: Vec2,
}

impl State {
    fn new() -> Self {
        Self { pos: Vec2::ZERO }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse_pos: glam::Vec2 = input::mouse::position(ctx).into();
        self.pos = mouse_pos.normalize_or_zero() * 200.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        if self.pos != Vec2::ZERO {
            let line = graphics::Mesh::new_line(
                ctx,
                &[Vec2::ZERO, self.pos],
                10.0,
                graphics::Color::from_rgb(255, 0, 127),
            )?;
            graphics::draw(ctx, &line, graphics::DrawParam::default())?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let state = State::new();

    let (ctx, event_loop) = ContextBuilder::new("arms", "soxfox42")
        .window_setup(conf::WindowSetup::default().title("The Arms"))
        .window_mode(conf::WindowMode::default().dimensions(1200.0, 800.0))
        .build()
        .unwrap();
    event::run(ctx, event_loop, state);
}
