mod ik;

use ggez::*;
use glam::Vec2;

struct State {
    arms: ik::Arms,
    colors: Vec<graphics::Color>,
    size: Vec2,
}

impl State {
    fn new() -> Self {
        let size = Vec2::new(1200.0, 800.0);
        let radius = (size.min_element() - 50.0) / 2.0;

        Self {
            arms: ik::Arms::new(3, radius / 3.0),
            colors: vec![
                graphics::Color::from_rgb(255, 127, 0),
                graphics::Color::from_rgb(255, 0, 127),
                graphics::Color::from_rgb(0, 127, 255),
            ],
            size,
        }
    }
}

impl event::EventHandler for State {
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let radius = (f32::min(width, height) - 50.0) / 2.0;
        self.arms.set_arm_length(radius / 3.0);

        self.size = Vec2::new(width, height);

        graphics::set_screen_coordinates(
            ctx,
            graphics::Rect::new(-width / 2.0, -height / 2.0, width, height),
        )
        .expect("Failed to update screen coordinates");
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse_pos = Vec2::from(input::mouse::position(ctx)) - self.size / 2.0;
        self.arms.update_angles(mouse_pos);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        let min_size = self.size.min_element();

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
    let (mut ctx, event_loop) = ContextBuilder::new("arms", "soxfox42")
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

    graphics::set_screen_coordinates(
        &mut ctx,
        graphics::Rect::new(-600.0, -400.0, 1200.0, 800.0),
    )
    .expect("Failed to update screen coordinates");

    let state = State::new();

    event::run(ctx, event_loop, state);
}
