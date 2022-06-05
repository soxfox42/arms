mod ik;

use ggez::*;
use glam::Vec2;

struct State {
    arms: ik::Arms,
    colors: Vec<graphics::Color>,
}

impl State {
    fn new(radius: f32) -> Self {
        Self {
            arms: ik::Arms::new(3, radius / 3.0),
            colors: vec![
                graphics::Color::from_rgb(255, 127, 0),
                graphics::Color::from_rgb(255, 0, 127),
                graphics::Color::from_rgb(0, 127, 255),
            ],
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse_pos = input::mouse::position(ctx).into();
        self.arms.update_angles(mouse_pos);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        let size: Vec2 = graphics::size(&ctx).into();
        let min_size = size.min_element();
        graphics::set_screen_coordinates(
            ctx,
            graphics::Rect::new(-size.x / 2.0, -size.y / 2.0, size.x, size.y),
        )?;

        let stroke_options = graphics::StrokeOptions::default()
            .with_line_width(min_size / 50.0)
            .with_line_cap(graphics::LineCap::Round);

        for (&positions, &color) in self.arms.arms().iter().zip(&self.colors) {
            let line = graphics::Mesh::new_polyline(
                ctx,
                graphics::DrawMode::Stroke(stroke_options),
                &[positions.0, positions.1],
                color,
            )?;
            graphics::draw(ctx, &line, graphics::DrawParam::default())?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("arms", "soxfox42")
        .window_setup(
            conf::WindowSetup::default()
                .title("The Arms")
                .samples(conf::NumSamples::Sixteen),
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(1200.0, 800.0)
                .resizable(true),
        )
        .build()
        .unwrap();

    let size: Vec2 = graphics::size(&ctx).into();
    let radius = (size.min_element() - 50.0) / 2.0;

    let state = State::new(radius);

    event::run(ctx, event_loop, state);
}
