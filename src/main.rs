#![feature(never_type)]

use std::path::PathBuf;

use anyhow::Result;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::KeyInput;
use ggez::winit::event::VirtualKeyCode;
use ggez::{Context, ContextBuilder, GameResult};

use crate::assets::Assets;

mod assets;
mod map;
mod player;

const WINDOW_WIDTH: f32 = 480.;
const WINDOW_HEIGHT: f32 = 640.;

struct SkiFree {
    assets: Assets,
}

impl SkiFree {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        // Load/create resources such as images here.
        Ok(Self {
            assets: Assets::new(ctx)?,
        })
    }
}

impl EventHandler for SkiFree {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);

        // Draw code here...
        canvas.finish(ctx)?;

        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        if input.keycode == Some(VirtualKeyCode::Escape) || input.keycode == Some(VirtualKeyCode::Q)
        {
            ctx.request_quit();
        }
        Ok(())
    }
}

fn main() -> Result<!> {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir).join("assets")
    } else {
        PathBuf::from("./assets")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("skifree-rs", "trevarj")
        .add_resource_path(resource_dir)
        .window_setup(WindowSetup::default().title("SkiFree"))
        .window_mode(
            WindowMode::default()
                .resizable(false)
                .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT),
        )
        .build()?;
    let game = SkiFree::new(&mut ctx)?;
    event::run(ctx, event_loop, game);
}
