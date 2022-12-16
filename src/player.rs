use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, PI};
use std::rc::Rc;

use ggez::graphics::{Canvas, DrawParam, Image, Rect};
use ggez::Context;

use crate::assets::Assets;
use crate::util::draw_hitbox;

#[derive(Debug)]
pub struct Player {
    state: PlayerState,
    image: Rc<Image>,
    speed: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum CollisionAction {
    Nothing,
    Fall,
    JumpSmall,
    JumpLarge,
}

#[derive(Debug, Clone, Copy)]
pub enum TrickType {
    Trick1,
    Trick2,
    Flip,
}

impl TrickType {
    fn required_frames(&self) -> i8 {
        match self {
            TrickType::Trick1 => TRICK1_FRAMES,
            TrickType::Trick2 => TRICK2_FRAMES,
            TrickType::Flip => FLIP_FRAMES,
        }
    }
}

impl Player {
    pub const POSITION: [f32; 2] = [240., 200.];
    const PLAYER_SPEED_NORMAL: f32 = 3.;

    pub fn new(assets: &Assets) -> Self {
        Self {
            state: PlayerState::RightStop,
            image: assets.player.skier_r.clone(),
            speed: Self::PLAYER_SPEED_NORMAL,
        }
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn collision(&mut self, action: CollisionAction) {
        self.state = match action {
            CollisionAction::Fall if self.is_upright() => PlayerState::Fallen(FALLEN_FRAMES),
            CollisionAction::JumpSmall if self.is_upright() => PlayerState::Jump(JUMP_FRAMES_SHORT),
            CollisionAction::JumpLarge if self.is_upright() => PlayerState::Jump(JUMP_FRAMES_LONG),
            _ => self.state,
        };
    }

    pub fn do_trick(&mut self, trick: TrickType) {
        let trick_frames = trick.required_frames();
        if let Some(jump_frame) = self.jump_frame() {
            let success = trick_frames <= jump_frame;
            self.state = match trick {
                TrickType::Trick1 => PlayerState::Trick1(jump_frame, success),
                TrickType::Trick2 => PlayerState::Trick2(jump_frame, success),
                TrickType::Flip => PlayerState::Flip(FlipSequence::Flip1(jump_frame), success),
            };
        }
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
            | PlayerState::Flip(..)
            | PlayerState::Jump(..)
            | PlayerState::Trick1(..)
            | PlayerState::Trick2(..) => Some(0.),
            PlayerState::LeftMove => Some(3. * FRAC_PI_2),
            PlayerState::RightMove => Some(FRAC_PI_2),
            PlayerState::Left30 => Some(11. * FRAC_PI_6),
            PlayerState::Left45 => Some(7. * FRAC_PI_4),
            PlayerState::Right30 => Some(FRAC_PI_6),
            PlayerState::Right45 => Some(FRAC_PI_4),
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
            | PlayerState::Flip(..)
            | PlayerState::Jump(_)
            | PlayerState::Trick1(..)
            | PlayerState::Trick2(..) => Some(PI),
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
            PlayerState::Flip(fs, _) => match fs {
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
            PlayerState::Trick1(..) => &assets.player.skier_trick,
            PlayerState::Trick2(..) => &assets.player.skier_trick2,
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

    fn is_tricking(&self) -> bool {
        matches!(
            self.state,
            PlayerState::Flip(..) | PlayerState::Trick1(..) | PlayerState::Trick2(..)
        )
    }

    pub fn jump_frame(&self) -> Option<i8> {
        if let PlayerState::Jump(frame) = self.state {
            Some(frame)
        } else {
            // not jumping
            None
        }
    }
}

type Frames = i8;

const FALLEN_FRAMES: i8 = 60;
const SITTING_FRAMES: i8 = 60;
const JUMP_FRAMES_SHORT: i8 = 20;
const JUMP_FRAMES_LONG: i8 = 60;
const TRICK1_FRAMES: i8 = 40;
const TRICK2_FRAMES: i8 = 40;
const FLIP_FRAMES: i8 = 50;

#[derive(Debug, Clone, Copy)]
enum PlayerState {
    Downward,
    Fallen(Frames),
    Sitting(Frames),
    Flip(FlipSequence, bool),
    Jump(Frames),
    LeftStop,
    LeftMove,
    RightStop,
    RightMove,
    Left30,
    Left45,
    Right30,
    Right45,
    Trick1(Frames, bool),
    Trick2(Frames, bool),
}

impl PlayerState {
    fn next_state(self) -> PlayerState {
        match self {
            PlayerState::Fallen(f) if f > 0 => PlayerState::Fallen(f - 1),
            PlayerState::Sitting(f) if f > 0 => PlayerState::Sitting(f - 1),
            PlayerState::Jump(f) if f > 0 => PlayerState::Jump(f - 1),
            PlayerState::Trick1(f, s) if f > 0 => PlayerState::Trick1(f - 1, s),
            PlayerState::Trick1(_, s) => {
                if s {
                    PlayerState::Downward
                } else {
                    PlayerState::Fallen(FALLEN_FRAMES)
                }
            }
            PlayerState::Trick2(f, s) if f > 0 => PlayerState::Trick2(f - 1, s),
            PlayerState::Trick2(_, s) => {
                if s {
                    PlayerState::Downward
                } else {
                    PlayerState::Fallen(FALLEN_FRAMES)
                }
            }
            PlayerState::Flip(flip, s) => flip.next_state(s),
            PlayerState::Fallen(_) => PlayerState::Sitting(SITTING_FRAMES),
            PlayerState::Sitting(_) | PlayerState::Jump(_) => PlayerState::Downward,
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

#[derive(Debug, Clone, Copy)]
enum FlipSequence {
    Flip1(Frames),
    Flip2(Frames),
    Flip3(Frames),
    Flip4(Frames),
}

impl FlipSequence {
    fn next_state(self, success: bool) -> PlayerState {
        PlayerState::Flip(
            match self {
                FlipSequence::Flip1(f) => {
                    if f == (FLIP_FRAMES / 4) * 3 {
                        FlipSequence::Flip2(f)
                    } else if f <= 0 {
                        if !success {
                            return PlayerState::Fallen(FALLEN_FRAMES);
                        } else {
                            return PlayerState::Downward;
                        }
                    } else {
                        FlipSequence::Flip1(f - 1)
                    }
                }
                FlipSequence::Flip2(f) => {
                    if f == (FLIP_FRAMES / 4) * 2 {
                        FlipSequence::Flip3(f)
                    } else if f <= 0 {
                        if !success {
                            return PlayerState::Fallen(FALLEN_FRAMES);
                        } else {
                            return PlayerState::Downward;
                        }
                    } else {
                        FlipSequence::Flip2(f - 1)
                    }
                }
                FlipSequence::Flip3(f) => {
                    if f == (FLIP_FRAMES / 4) * 2 {
                        FlipSequence::Flip4(f)
                    } else if f <= 0 {
                        if !success {
                            return PlayerState::Fallen(FALLEN_FRAMES);
                        } else {
                            return PlayerState::Downward;
                        }
                    } else {
                        FlipSequence::Flip3(f - 1)
                    }
                }
                FlipSequence::Flip4(f) => {
                    if f == 0 {
                        if !success {
                            return PlayerState::Fallen(FALLEN_FRAMES);
                        } else {
                            return PlayerState::Downward;
                        }
                    } else {
                        FlipSequence::Flip4(f - 1)
                    }
                }
            },
            success,
        )
    }
}
