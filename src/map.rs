use std::rc::Rc;

use ggez::glam::Vec2;
use ggez::graphics::{Canvas, DrawParam, Image};
use ggez::mint::Point2;
use rand::rngs::OsRng;
use rand::Rng;

use crate::assets::Assets;
use crate::player::Player;
use crate::util::vec_from_angle;

const LIFT_X_POS: f32 = 100.;

pub struct Map {
    immovable_objects: Vec<ImmovableObject>,
    rng: OsRng,
}

#[derive(Debug, Clone, Copy)]
pub enum CollisionAction {
    Nothing,
    Fall,
    JumpSmall,
    JumpLarge,
}

struct ImmovableObject {
    position: Point2<f32>,
    image: Rc<Image>,
    /// What happens when a player hits this object
    collision_action: CollisionAction,
}

impl ImmovableObject {
    fn new(position: Point2<f32>, image: &Rc<Image>, collision_action: CollisionAction) -> Self {
        Self {
            position,
            image: image.clone(),
            collision_action,
        }
    }

    fn shift(&mut self, direction: f32) {
        let mut pos: Vec2 = self.position.into();
        let v2 = vec_from_angle(direction);
        pos += v2 * 3.; // TODO: change speed
        self.position = pos.into();
    }
}

impl Map {
    pub fn new(assets: &Assets) -> Self {
        let mut rng = OsRng::default();
        let mut immovable_objects = vec![
            ImmovableObject::new(
                [LIFT_X_POS, 100.].into(),
                &assets.objects.lift,
                CollisionAction::Fall,
            ),
            ImmovableObject::new(
                [LIFT_X_POS, 140.].into(),
                &assets.objects.chairlift,
                CollisionAction::Nothing,
            ),
        ];
        immovable_objects.extend(start_signs(assets).into_iter());
        immovable_objects.extend(freestyle_course(assets, &mut rng));
        Self {
            immovable_objects,
            rng,
        }
    }

    pub fn check_collision(&self) -> Option<CollisionAction> {
        self.immovable_objects.iter().find_map(|o| {
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
        self.immovable_objects.iter_mut().for_each(|o| {
            if let Some(direction) = movement_direction {
                o.shift(direction)
            }
        });

        // call generators
        // move everything in a certain `direction`

        // check which objects need to be cleared (off top of screen)

        // generate for each "section" of the hill:
        // random left (wilderness),
        // slalom (flags + moguls),
        // freestyle (ramps),
        // tree slalom (lots of trees),
        // random right (wilderness)
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for o in &self.immovable_objects {
            canvas.draw(o.image.as_ref(), o.position);
        }
    }

    /// Generates new random immovable objects
    fn generate_immovables(&mut self) {
        // for trees, rocks, bumps, lifts, etc
        // generate random coord
        // choose random object
    }
}

fn start_signs(assets: &Assets) -> [ImmovableObject; 3] {
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
        ImmovableObject::new(slalom_xy.into(), slalom, CollisionAction::Fall),
        ImmovableObject::new(freestyle_xy.into(), freestyle, CollisionAction::Fall),
        ImmovableObject::new(tree_slalom_xy.into(), tree_slalom, CollisionAction::Fall),
    ]
}

fn freestyle_course(assets: &Assets, rng: &mut OsRng) -> Vec<ImmovableObject> {
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
                objects.push(ImmovableObject::new([x, y].into(), image, action))
            }
        }
    }
    objects
}
