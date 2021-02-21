use event::run;
use ggez::{self, event, graphics, nalgebra as na, Context, ContextBuilder, GameResult};
use graphics::{DrawParam, Drawable};
use na::Point2;

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RAKCET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;

struct MainState {
    player_1_pos: na::Point2<f32>,
    player_2_pos: na::Point2<f32>,
}

impl MainState {
    fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_h, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        MainState {
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF, screen_h_half),
            player_2_pos: na::Point2::new(screen_w - RACKET_WIDTH_HALF, screen_h_half),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let racket_rect = graphics::Rect::new(
            -RACKET_WIDTH_HALF,
            -RAKCET_HEIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
        );

        // let color = [0.0, 0.0, 1.0, 1.0].into();

        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket_rect,
            graphics::WHITE,
        )?;

        let mut draw_param = graphics::DrawParam::default();

        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.player_2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        graphics::present(ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("ping_pong", "tolumide");
    let (ctx, event_loop) = &mut cb.build()?;
    dbg!("did that there");
    graphics::set_window_title(ctx, "Pong");

    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop, &mut state);

    Ok(())
}
