use std::io::Read;

use ggez::*;
use glam::Vec2;

struct State {
    angles: Vec<f32>,
    colors: Vec<graphics::Color>,
}

impl State {
    fn new() -> Self {
        Self {
            angles: vec![0.0, 0.0, 0.0],
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
        self.angles[0] += timer::delta(ctx).as_secs_f32();
        self.angles[1] -= timer::delta(ctx).as_secs_f32() * 0.9;
        self.angles[2] += timer::delta(ctx).as_secs_f32() * 1.1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        let size: Vec2 = graphics::size(ctx).into();
        let min_size = size.min_element() - 50.0;
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, size.x, size.y))?;

        let stroke_options = graphics::StrokeOptions::default()
            .with_line_width(min_size / 50.0)
            .with_line_cap(graphics::LineCap::Round);

        let arm_length = min_size / (self.angles.len() as f32) / 2.0;

        let mut pos = size * 0.5;
        let mut angle_sum = 0.0;
        for (&angle, &color) in self.angles.iter().zip(self.colors.iter()) {
            angle_sum += angle;
            let arm_offset = Vec2::new(angle_sum.cos(), angle_sum.sin()) * arm_length;
            let new_pos = pos + arm_offset;
            let line = graphics::Mesh::new_polyline(
                ctx,
                graphics::DrawMode::Stroke(stroke_options),
                &[pos, new_pos],
                color,
            )?;
            graphics::draw(ctx, &line, graphics::DrawParam::default())?;

            pos = new_pos;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let state = State::new();

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
    event::run(ctx, event_loop, state);
}
