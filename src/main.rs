#![feature(never_type)]

use std::path::PathBuf;

use anyhow::Result;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{Canvas, Color, Sampler};
use ggez::input::keyboard::KeyInput;
use ggez::winit::event::VirtualKeyCode;
use ggez::{Context, ContextBuilder, GameResult};

use crate::assets::Assets;
use crate::map::Map;
use crate::player::Player;
use crate::util::vec2_from_angle;

mod assets;
mod map;
mod player;
mod util;

const WINDOW_WIDTH: f32 = 480.;
const WINDOW_HEIGHT: f32 = 640.;

struct SkiFree {
    assets: Assets,
    map: Map,
    player: Player,
}

impl SkiFree {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        // Load/create resources such as images here.
        let assets = Assets::new(ctx)?;
        let map = Map::new(&assets);
        let player = Player::new(&assets);
        Ok(Self {
            assets,
            map,
            player,
        })
    }

    fn handle_collisions(&mut self) {
        if let Some(action) = self.map.check_collision(&self.player) {
            self.player.collision(action);
        }
    }
}

impl EventHandler for SkiFree {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while ctx.time.check_update_time(DESIRED_FPS) {
            let _seconds = 1.0 / (DESIRED_FPS as f32);

            self.handle_collisions();

            self.map.update(&self.assets, &self.player);

            self.player.maybe_next_state(&self.assets);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        canvas.set_sampler(Sampler::nearest_clamp());

        // Draw code here...
        self.map.draw(ctx, &mut canvas);
        self.player.draw(ctx, &self.assets, &mut canvas);

        canvas.finish(ctx)?;

        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, repeated: bool) -> GameResult {
        if let Some(keycode) = input.keycode {
            if repeated {
                return Ok(());
            }
            match keycode {
                VirtualKeyCode::Escape | VirtualKeyCode::Q => ctx.request_quit(),
                VirtualKeyCode::Left => self.player.left(),
                VirtualKeyCode::Right => self.player.right(),
                _ => {}
            }
        }
        Ok(())
    }

    /// A keyboard button was released.
    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                VirtualKeyCode::Left => self.player.slide_left(),
                VirtualKeyCode::Right => self.player.slide_right(),
                _ => {}
            }
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
