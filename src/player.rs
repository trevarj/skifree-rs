use std::f32::consts::PI;

use ggez::graphics::{Canvas, DrawParam};
use ggez::mint::Point2;

use crate::assets::Assets;

#[derive(Debug)]
pub struct Player {
    state: PlayerState,
}

impl Player {
    pub fn new() -> Self {
        Self {
            state: PlayerState::RightStop,
        }
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

    pub fn draw(&self, assets: &Assets, canvas: &mut Canvas) {
        let image = match self.state {
            PlayerState::Downward => &assets.player.skier_down,
            PlayerState::Fallen => &assets.player.skier_fall,
            PlayerState::Sitting => &assets.player.skier_sit,
            PlayerState::Flip1 => &assets.player.skier_flip,
            PlayerState::Flip2 => &assets.player.skier_flip2,
            PlayerState::Flip3 => &assets.player.skier_flip3,
            PlayerState::Flip4 => &assets.player.skier_flip4,
            PlayerState::Jump => &assets.player.skier_jump,
            PlayerState::LeftStop => &assets.player.skier_l,
            PlayerState::LeftMove => &assets.player.skier_l2,
            PlayerState::RightStop => &assets.player.skier_r,
            PlayerState::RightMove => &assets.player.skier_r2,
            PlayerState::Left30 => &assets.player.skier_l30,
            PlayerState::Left45 => &assets.player.skier_l45,
            PlayerState::Right30 => &assets.player.skier_r30,
            PlayerState::Right45 => &assets.player.skier_r45,
            PlayerState::Trick1 => &assets.player.skier_trick,
            PlayerState::Trick2 => &assets.player.skier_trick2,
        };
        let point: Point2<f32> = [240., 320.].into();
        canvas.draw(image.as_ref(), DrawParam::default().dest(point));
    }

    /// Return the angle in radians which is opposite of the angle that the
    /// player is moving. None if player is not moving
    pub fn opposite_direction(&self) -> Option<f32> {
        match &self.state {
            PlayerState::Downward
            | PlayerState::Flip1
            | PlayerState::Flip2
            | PlayerState::Flip3
            | PlayerState::Flip4
            | PlayerState::Jump
            | PlayerState::Trick1
            | PlayerState::Trick2 => Some(0.),
            PlayerState::LeftMove => Some(PI / 2.),
            PlayerState::RightMove => Some(3. * PI / 2.),
            PlayerState::Left30 => Some(11. * PI / 6.),
            PlayerState::Left45 => Some(7. * PI / 4.),
            PlayerState::Right30 => Some(7. * PI / 6.),
            PlayerState::Right45 => Some(5. * PI / 4.),
            PlayerState::LeftStop
            | PlayerState::RightStop
            | PlayerState::Fallen
            | PlayerState::Sitting => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum PlayerState {
    Downward,
    Fallen,
    Sitting,
    Flip1,
    Flip2,
    Flip3,
    Flip4,
    Jump,
    LeftStop,
    LeftMove,
    RightStop,
    RightMove,
    Left30,
    Left45,
    Right30,
    Right45,
    Trick1,
    Trick2,
}
