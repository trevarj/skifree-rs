use std::rc::Rc;

use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Image};
use ggez::mint::Point2;
use rand::rngs::OsRng;
use rand::Rng;

use crate::assets::Assets;
use crate::map::objects::Object;
use crate::player::{CollisionAction, Player};
use crate::util::vec_from_angle;

const LIFT_X_POS: f32 = 100.;

mod objects;

pub struct Map {
    objects: Vec<Object>,
    rng: OsRng,
}

impl Map {
    pub fn new(assets: &Assets) -> Self {
        let mut rng = OsRng::default();
        let mut objects = vec![
            Object::immovable(
                [LIFT_X_POS, 100.].into(),
                &assets.objects.lift,
                CollisionAction::Fall,
            ),
            Object::movable(
                [LIFT_X_POS, 140.].into(),
                &assets.objects.chairlift,
                CollisionAction::Nothing,
                |o| o.position.y -= 0.2,
            ),
        ];
        objects.extend(start_signs(assets).into_iter());
        objects.extend(freestyle_course(assets, &mut rng));
        Self { objects, rng }
    }

    pub fn check_collision(&self) -> Option<CollisionAction> {
        self.objects.iter().find_map(|o| {
            let pdistance = Vec2::from(o.position) - Vec2::from(Player::POSITION);
            if pdistance.length() < 20. {
                // TODO: idk what this val should be
                Some(o.collision_action)
            } else {
                None
            }
        })
    }

    pub fn update(&mut self, _assets: &Assets, movement_direction: Option<f32>) {
        // move everything in relation to the given movement_direction
        self.objects.retain_mut(|o| {
            if o.position.y < -50. {
                return false;
            } else if let Some(direction) = movement_direction {
                o.shift(direction);
            }
            o.apply_movement();
            true
        });

        // call generators

        // check which objects need to be cleared (off top of screen)

        // generate for each "section" of the hill:
        // random left (wilderness),
        // slalom (flags + moguls),
        // freestyle (ramps),
        // tree slalom (lots of trees),
        // random right (wilderness)
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for o in &self.objects {
            canvas.draw(o.image.as_ref(), o.position);
        }
    }

    /// Generates new random immovable objects
    fn generate_objects(&mut self) {
        // for trees, rocks, bumps, lifts, etc
        // generate random coord
        // choose random object
    }
}

fn start_signs(assets: &Assets) -> [Object; 3] {
    let slalom = &assets.objects.slalom;
    let freestyle = &assets.objects.freestyle;
    let tree_slalom = &assets.objects.tree_slalom;

    let x = 250.;
    let y = 250.;
    let spacing = 10.;
    let slalom_xy = [x, y];
    let freestyle_xy = [slalom_xy[0] + spacing + slalom.width() as f32, y];
    let tree_slalom_xy = [freestyle_xy[0] + spacing + freestyle.width() as f32, y];
    [
        Object::immovable(slalom_xy.into(), slalom, CollisionAction::Fall),
        Object::immovable(freestyle_xy.into(), freestyle, CollisionAction::Fall),
        Object::immovable(tree_slalom_xy.into(), tree_slalom, CollisionAction::Fall),
    ]
}

fn freestyle_course(assets: &Assets, rng: &mut OsRng) -> Vec<Object> {
    let x_range = 200..400;
    let x_spacing = 40;

    let y_range = 500..4000;
    let y_spacing = 100;
    let y_iter = y_range.step_by(y_spacing).map(|y| y as f32);

    let mut objects = vec![];
    for y in y_iter {
        for x in x_range.clone().step_by(x_spacing).map(|y| y as f32) {
            if rng.gen_bool(0.33) {
                let (image, action) = match rng.gen_range(0..3) {
                    0 => (&assets.objects.bump_l, CollisionAction::JumpSmall),
                    1 => (&assets.objects.bump_s, CollisionAction::JumpSmall),
                    2 => (&assets.objects.ramp, CollisionAction::JumpLarge),
                    _ => continue,
                };
                objects.push(Object::immovable([x, y].into(), image, action))
            }
        }
    }
    objects
}
