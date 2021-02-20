use event::run;
use ggez::{self, event, graphics, Context, ContextBuilder, GameResult};
use graphics::{DrawParam, Drawable};
// use graphics::set_window_title;

struct MainState {}

impl MainState {
    fn new() -> Self {
        MainState {}
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let rect = graphics::Rect::new(10.0, 10.0, 300.0, 150.0);

        // let color = [0.0, 0.0, 1.0, 1.0].into();

        let rect_mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;

        // let rect_mesh =
        //     graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
        graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())?;
        // graphics::draw(ctx, &rect_mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));

        graphics::present(ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("ping_pong", "tolumide");
    let (ctx, event_loop) = &mut cb.build()?;
    dbg!("did that there");
    graphics::set_window_title(ctx, "Pong");

    let mut state = MainState::new();
    event::run(ctx, event_loop, &mut state);

    Ok(())
}
