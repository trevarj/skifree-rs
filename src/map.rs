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
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

const LIFT_X_POS: f32 = 100.;

mod objects;

pub struct Map {
    objects: Vec<Object>,
    rng: OsRng,
    y_distance: f32,
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
        Self {
            objects,
            rng,
            y_distance: 0.,
        }
    }

    pub fn check_collision(&self) -> Option<CollisionAction> {
        self.objects.iter().find_map(|o| {
            let pdistance = Vec2::from(o.position) - Vec2::from(Player::POSITION);
            if pdistance.length() < 10. {
                // TODO: idk what this val should be
                Some(o.collision_action)
            } else {
                None
            }
        })
    }

    pub fn update(&mut self, assets: &Assets, player: &Player) {
        // move everything in relation to the given movement_direction
        // while checking which objects need to be cleared off top of screen
        self.objects.retain_mut(|o| {
            if o.position.y < -50. {
                return false;
            } else if let Some((direction, magnitude)) = player
                .opposite_direction()
                .map(|d| (d, Player::PLAYER_SPEED))
            {
                o.shift(direction, magnitude);
            }
            o.apply_movement();
            true
        });

        if let Some(direction) = player.direction() {
            self.y_distance += vec_from_angle(direction).y;

            // dbg!(self.y_distance);
            // if (self.y_distance % (WINDOW_HEIGHT / (2. *
            // PLAYER_SPEED))).trunc() == 0. {     dbg!("half
            // screen"); }

            // player moved so call generators
            self.generate_objects(assets, direction);
        }

        // generate for each "section" of the hill:
        // random left (wilderness),
        // slalom (flags + moguls),
        // freestyle (ramps),
        // tree slalom (lots of trees),
        // random right (wilderness)
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for o in self.objects.iter().filter(|o| {
            o.position.x >= 0. - 100.
                && o.position.x <= WINDOW_WIDTH + 100.
                && o.position.y >= 0. - 100.
                && o.position.y <= WINDOW_HEIGHT + 100.
        }) {
            canvas.draw(o.image.as_ref(), o.position);
        }
    }

    /// Generates new random immovable objects
    fn generate_objects(&mut self, assets: &Assets, direction: f32) {
        // off-screen where Player is heading
        let pos = dbg!(vec_from_angle(direction) * 850.);
        let start_x = pos.x as i32;
        let start_y = pos.y as i32;
        for y in (start_y - 100..start_y + 100).step_by(30).map(|y| y as f32) {
            for x in (start_x - 100..start_x + 100).step_by(30).map(|x| x as f32) {
                if self.rng.gen_bool(0.001) {
                    let (image, action) = match self.rng.gen_range(0..3) {
                        0 => (&assets.objects.rock, CollisionAction::Fall),
                        1 => (&assets.objects.stump, CollisionAction::Fall),
                        2 => (&assets.objects.tree1, CollisionAction::Fall),
                        _ => continue,
                    };
                    self.objects
                        .push(Object::immovable([x, y].into(), image, action))
                }
            }
        }
        // for trees, rocks, bumps, lifts, etc
        // generate random coord
        // choose random object
    }
}

fn start_signs(assets: &Assets) -> [Object; 14] {
    let slalom = &assets.objects.slalom;
    let freestyle = &assets.objects.freestyle;
    let tree_slalom = &assets.objects.tree_slalom;

    let btree = &assets.objects.bigtree;
    let xtree = &assets.objects.xtree1;

    let x = 250.;
    let y = 250.;
    let spacing = 10.;
    let slalom_xy = [x, y];
    let freestyle_xy = [slalom_xy[0] + spacing + slalom.width() as f32, y];
    let tree_slalom_xy = [freestyle_xy[0] + spacing + freestyle.width() as f32, y];
    [
        Object::immovable([250., 210.].into(), btree, CollisionAction::Fall),
        Object::immovable([410., 210.].into(), btree, CollisionAction::Fall),
        Object::immovable([290., 210.].into(), btree, CollisionAction::Fall),
        Object::immovable([330., 210.].into(), btree, CollisionAction::Fall),
        Object::immovable([370., 210.].into(), btree, CollisionAction::Fall),
        Object::immovable([270., 220.].into(), btree, CollisionAction::Fall),
        Object::immovable([310., 220.].into(), btree, CollisionAction::Fall),
        Object::immovable([350., 220.].into(), btree, CollisionAction::Fall),
        Object::immovable([390., 220.].into(), btree, CollisionAction::Fall),
        Object::immovable([230., 250.].into(), xtree, CollisionAction::Fall),
        Object::immovable([410., 250.].into(), xtree, CollisionAction::Fall),
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
        for x in x_range.clone().step_by(x_spacing).map(|x| x as f32) {
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
