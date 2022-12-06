use std::f32::consts::PI;
use std::rc::Rc;

use ggez::graphics::{Canvas, Color, DrawParam, Image, Mesh, Rect};
use ggez::Context;

use crate::assets::Assets;
use crate::util::draw_hitbox;

#[derive(Debug)]
pub struct Player {
    state: PlayerState,
    image: Rc<Image>,
}

#[derive(Debug, Clone, Copy)]
pub enum CollisionAction {
    Nothing,
    Fall,
    JumpSmall,
    JumpLarge,
}

impl Player {
    pub const POSITION: [f32; 2] = [240., 320.];
    pub const PLAYER_SPEED: f32 = 3.;

    pub fn new(assets: &Assets) -> Self {
        Self {
            state: PlayerState::RightStop,
            image: assets.player.skier_r.clone(),
        }
    }

    pub fn collision(&mut self, action: CollisionAction) {
        self.state = match action {
            CollisionAction::Fall if self.is_upright() => PlayerState::Fallen(FALLEN_FRAMES),
            CollisionAction::JumpSmall if self.is_upright() => PlayerState::Jump(JUMP_FRAMES_SHORT),
            CollisionAction::JumpLarge if self.is_upright() => PlayerState::Jump(JUMP_FRAMES_LONG),
            _ => self.state,
        };
    }

    pub fn maybe_next_state(&mut self, assets: &Assets) {
        self.state = self.state.next_state();
        self.image = self.image(assets);
    }

    pub fn slide_left(&mut self) {
        self.state = match self.state {
            PlayerState::LeftMove => PlayerState::LeftStop,
            _ => self.state,
        };
    }

    pub fn left(&mut self) {
        self.state = match self.state {
            PlayerState::Downward => PlayerState::Left30,
            PlayerState::LeftStop => PlayerState::LeftMove,
            PlayerState::RightStop | PlayerState::RightMove => PlayerState::Right45,
            PlayerState::Left30 => PlayerState::Left45,
            PlayerState::Left45 | PlayerState::LeftMove => PlayerState::LeftStop,
            PlayerState::Right30 => PlayerState::Downward,
            PlayerState::Right45 => PlayerState::Right30,
            _ => self.state,
        };
    }

    pub fn slide_right(&mut self) {
        self.state = match self.state {
            PlayerState::RightMove => PlayerState::RightStop,
            _ => self.state,
        };
    }

    pub fn right(&mut self) {
        self.state = match self.state {
            PlayerState::Downward => PlayerState::Right30,
            PlayerState::LeftStop | PlayerState::LeftMove => PlayerState::Left45,
            PlayerState::RightStop => PlayerState::RightMove,
            PlayerState::Left30 => PlayerState::Downward,
            PlayerState::Left45 => PlayerState::Left30,
            PlayerState::Right30 => PlayerState::Right45,
            PlayerState::Right45 | PlayerState::RightMove => PlayerState::RightStop,
            _ => self.state,
        };
    }

    pub fn hitbox(&self) -> Rect {
        Rect::new(
            Self::POSITION[0],
            Self::POSITION[1] + self.image.height() as f32,
            self.image.width() as f32,
            5.,
        )
    }

    pub fn draw(&self, ctx: &Context, assets: &Assets, canvas: &mut Canvas) {
        let image = self.image(assets);
        #[cfg(debug_assertions)]
        draw_hitbox(ctx, canvas, self.hitbox());
        canvas.draw(image.as_ref(), DrawParam::default().dest(Self::POSITION));
    }

    pub fn direction(&self) -> Option<f32> {
        match &self.state {
            PlayerState::Downward
            | PlayerState::Flip(_)
            | PlayerState::Jump(_)
            | PlayerState::Trick1(_)
            | PlayerState::Trick2(_) => Some(0.),
            PlayerState::LeftMove => Some(3. * PI / 2.),
            PlayerState::RightMove => Some(PI / 2.),
            PlayerState::Left30 => Some(11. * PI / 6.),
            PlayerState::Left45 => Some(7. * PI / 4.),
            PlayerState::Right30 => Some(PI / 6.),
            PlayerState::Right45 => Some(PI / 4.),
            PlayerState::LeftStop
            | PlayerState::RightStop
            | PlayerState::Fallen(_)
            | PlayerState::Sitting(_) => None,
        }
    }

    /// Return the angle in radians which is opposite of the angle that the
    /// player is moving. None if player is not moving
    pub fn opposite_direction(&self) -> Option<f32> {
        match &self.state {
            PlayerState::Downward
            | PlayerState::Flip(_)
            | PlayerState::Jump(_)
            | PlayerState::Trick1(_)
            | PlayerState::Trick2(_) => Some(PI),
            PlayerState::LeftMove => Some(PI / 2.),
            PlayerState::RightMove => Some(3. * PI / 2.),
            PlayerState::Left30 => Some(5. * PI / 6.),
            PlayerState::Left45 => Some(3. * PI / 4.),
            PlayerState::Right30 => Some(7. * PI / 6.),
            PlayerState::Right45 => Some(5. * PI / 4.),
            PlayerState::LeftStop
            | PlayerState::RightStop
            | PlayerState::Fallen(_)
            | PlayerState::Sitting(_) => None,
        }
    }

    fn image(&self, assets: &Assets) -> Rc<Image> {
        let image = match self.state {
            PlayerState::Downward => &assets.player.skier_down,
            PlayerState::Fallen(_) => &assets.player.skier_fall,
            PlayerState::Sitting(_) => &assets.player.skier_sit,
            PlayerState::Flip(fs) => match fs {
                FlipSequence::Flip1(_) => &assets.player.skier_flip,
                FlipSequence::Flip2(_) => &assets.player.skier_flip2,
                FlipSequence::Flip3(_) => &assets.player.skier_flip3,
                FlipSequence::Flip4(_) => &assets.player.skier_flip4,
            },
            PlayerState::Jump(_) => &assets.player.skier_jump,
            PlayerState::LeftStop => &assets.player.skier_l,
            PlayerState::LeftMove => &assets.player.skier_l2,
            PlayerState::RightStop => &assets.player.skier_r,
            PlayerState::RightMove => &assets.player.skier_r2,
            PlayerState::Left30 => &assets.player.skier_l30,
            PlayerState::Left45 => &assets.player.skier_l45,
            PlayerState::Right30 => &assets.player.skier_r30,
            PlayerState::Right45 => &assets.player.skier_r45,
            PlayerState::Trick1(_) => &assets.player.skier_trick,
            PlayerState::Trick2(_) => &assets.player.skier_trick2,
        };
        image.clone()
    }

    /// Moving, standing, but not jumping, tricking or fallen
    fn is_upright(&self) -> bool {
        matches!(
            self.state,
            PlayerState::Downward
                | PlayerState::LeftStop
                | PlayerState::LeftMove
                | PlayerState::RightStop
                | PlayerState::RightMove
                | PlayerState::Left30
                | PlayerState::Left45
                | PlayerState::Right30
                | PlayerState::Right45
        )
    }
}

type Frames = u8;

const FALLEN_FRAMES: u8 = 60;
const SITTING_FRAMES: u8 = 60;
const JUMP_FRAMES_SHORT: u8 = 20;
const JUMP_FRAMES_LONG: u8 = 40;

#[derive(Debug, Clone, Copy)]
enum PlayerState {
    Downward,
    Fallen(Frames),
    Sitting(Frames),
    Flip(FlipSequence),
    Jump(Frames),
    LeftStop,
    LeftMove,
    RightStop,
    RightMove,
    Left30,
    Left45,
    Right30,
    Right45,
    Trick1(Frames),
    Trick2(Frames),
}

impl PlayerState {
    fn next_state(self) -> PlayerState {
        match self {
            PlayerState::Fallen(f) if f > 0 => PlayerState::Fallen(f - 1),
            PlayerState::Sitting(f) if f > 0 => PlayerState::Sitting(f - 1),
            PlayerState::Jump(f) if f > 0 => PlayerState::Jump(f - 1),
            PlayerState::Trick1(f) if f > 0 => PlayerState::Trick1(f - 1),
            PlayerState::Trick2(f) if f > 0 => PlayerState::Trick2(f - 1),
            PlayerState::Flip(flip) => todo!(),
            PlayerState::Fallen(_) => PlayerState::Sitting(SITTING_FRAMES),
            PlayerState::Sitting(_)
            | PlayerState::Jump(_)
            | PlayerState::Trick1(_)
            | PlayerState::Trick2(_) => PlayerState::Downward,
            PlayerState::Downward
            | PlayerState::LeftStop
            | PlayerState::LeftMove
            | PlayerState::RightStop
            | PlayerState::RightMove
            | PlayerState::Left30
            | PlayerState::Left45
            | PlayerState::Right30
            | PlayerState::Right45 => self,
        }
    }
}

const FLIP_FRAMES: u8 = 3;
#[derive(Debug, Clone, Copy)]
enum FlipSequence {
    Flip1(Frames),
    Flip2(Frames),
    Flip3(Frames),
    Flip4(Frames),
}
