use ggez::graphics::{Canvas, Color, DrawParam, Mesh};
use ggez::Context;
use rand::rngs::OsRng;
use rand::Rng;

use self::objects::{LineObject, Shift};
use crate::assets::Assets;
use crate::map::objects::Object;
use crate::player::{CollisionAction, Player};
use crate::util::{draw_hitbox, vec2_from_angle};
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

const MAP_WIDTH: i32 = 3000;
const MAP_HEIGHT: i32 = 20_000;
const MAP_X_START: i32 = SLALOM_X_START;

const COURSE_WIDTH: i32 = MAP_WIDTH / 3;
const COURSE_Y_START: i32 = 300;
const SLALOM_X_START: i32 = MAP_WIDTH / -2;
const FREESTYLE_X_START: i32 = SLALOM_X_START + COURSE_WIDTH;
const TREE_SLALOM_X_START: i32 = FREESTYLE_X_START + COURSE_WIDTH;

const LIFT_X_POS: f32 = 100.;
const LIFT_Y_START: i32 = 100;

mod objects;
mod slalom;

pub struct Map {
    objects: Vec<Object>,
    lines: Vec<LineObject>,
    rng: OsRng,
    y_distance: f32,
}

impl Map {
    pub fn new(assets: &Assets) -> Self {
        let mut rng = OsRng;
        let mut objects = vec![];
        objects.extend(starting_objects(assets));
        objects.extend(slalom_course(assets, &mut rng));
        objects.extend(freestyle_course(assets, &mut rng));
        objects.extend(tree_slalom_course(assets, &mut rng));
        objects.extend(ski_lift(assets));

        let lines = vec![
            // Ski lift lines
            LineObject::new(
                [
                    [LIFT_X_POS, 0 as f32].into(),
                    [LIFT_X_POS, MAP_HEIGHT as f32].into(),
                ],
                1.,
                (0.9, 0.9, 0.9).into(),
            ),
            LineObject::new(
                [
                    [LIFT_X_POS + 25., 0 as f32].into(),
                    [LIFT_X_POS + 25., MAP_HEIGHT as f32].into(),
                ],
                1.,
                (0.9, 0.9, 0.9).into(),
            ),
        ];
        Self {
            objects,
            lines,
            rng,
            y_distance: 0.,
        }
    }

    pub fn check_collision(&mut self, player: &Player) -> Option<CollisionAction> {
        self.objects.iter_mut().find_map(|o| {
            if o.hitbox().overlaps(&player.hitbox()) {
                let action = o.collision_action;
                o.collision_action = CollisionAction::Nothing;
                Some(action)
            } else {
                None
            }
        })
    }

    pub fn update(&mut self, assets: &Assets, player: &Player) {
        let player_movement = player.opposite_direction().map(|d| (d, player.speed()));
        // move everything in relation to the given movement_direction
        // while checking which objects need to be cleared off top of screen
        self.objects.retain_mut(|o| {
            if o.position.y < -50. {
                return false;
            } else if let Some((direction, magnitude)) = player_movement {
                o.shift(direction, magnitude);
            }
            o.apply_movement();
            true
        });

        for line in &mut self.lines {
            if let Some((direction, magnitude)) = player_movement {
                line.shift(direction, magnitude);
            }
        }

        if let Some(direction) = player.direction() {
            self.y_distance += vec2_from_angle(direction).y;
        }
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas) {
        for o in self.objects.iter().filter(|o| {
            o.position.x >= 0. - 100.
                && o.position.x <= WINDOW_WIDTH + 100.
                && o.position.y >= 0. - 100.
                && o.position.y <= WINDOW_HEIGHT + 100.
        }) {
            #[cfg(debug_assertions)]
            draw_hitbox(ctx, canvas, o.hitbox());
            canvas.draw(o.image.as_ref(), o.position);
        }

        for line in &self.lines {
            line.draw(ctx, canvas);
        }
    }

    pub fn y_distance(&self) -> f32 {
        self.y_distance
    }
}

fn starting_objects(assets: &Assets) -> Vec<Object> {
    let slalom = &assets.objects.slalom;
    let freestyle = &assets.objects.freestyle;
    let tree_slalom = &assets.objects.tree_slalom;

    let btree = &assets.objects.bigtree;
    let xtree = &assets.objects.xtree1;

    let x = 250.;
    let y = 150.;
    let spacing = 10.;
    let slalom_xy = [x, y];
    let freestyle_xy = [slalom_xy[0] + spacing + slalom.width() as f32, y];
    let tree_slalom_xy = [freestyle_xy[0] + spacing + freestyle.width() as f32, y];
    [
        ([250., 110.], btree),
        ([410., 110.], btree),
        ([290., 110.], btree),
        ([330., 110.], btree),
        ([370., 110.], btree),
        ([270., 120.], btree),
        ([310., 120.], btree),
        ([350., 120.], btree),
        ([390., 120.], btree),
        ([230., 150.], xtree),
        ([410., 150.], xtree),
        (slalom_xy, slalom),
        (freestyle_xy, freestyle),
        (tree_slalom_xy, tree_slalom),
    ]
    .into_iter()
    .map(|o| Object::immovable(o.0.into(), o.1, CollisionAction::Fall))
    .collect()
}

fn ski_lift(assets: &Assets) -> Vec<Object> {
    let mut objects = vec![];
    for y in (LIFT_Y_START..MAP_HEIGHT).step_by(400).map(|i| i as f32) {
        objects.push(Object::movable(
            [LIFT_X_POS - 18., y + 100.].into(),
            &assets.objects.chairlift,
            CollisionAction::Nothing,
            |o| o.position.y += 0.2,
        ));
        objects.push(Object::movable(
            [LIFT_X_POS + 18., y + 400.].into(),
            &assets.objects.lifters,
            CollisionAction::Nothing,
            |o| o.position.y -= 0.2,
        ));
        objects.push(Object::immovable(
            [LIFT_X_POS, y].into(),
            &assets.objects.lift,
            CollisionAction::Fall,
        ));
    }
    objects
}

fn slalom_course(assets: &Assets, rng: &mut OsRng) -> Vec<Object> {
    let x_range = SLALOM_X_START..FREESTYLE_X_START;
    let x_spacing = 40;

    let y_range = COURSE_Y_START..MAP_HEIGHT;
    let y_spacing = 100;
    let y_iter = y_range.step_by(y_spacing).map(|y| y as f32);

    let mut objects = vec![];
    for y in y_iter {
        for x in x_range.clone().step_by(x_spacing).map(|x| x as f32) {
            if rng.gen_bool(0.33) {
                let (image, action) = match rng.gen_range(0..5) {
                    0 => (&assets.objects.bump_l, CollisionAction::JumpSmall),
                    1 => (&assets.objects.bump_s, CollisionAction::JumpSmall),
                    2 => (&assets.objects.mogul, CollisionAction::JumpSmall),
                    3 => (&assets.objects.rock, CollisionAction::Fall),
                    4 => (&assets.objects.tree1, CollisionAction::Fall),
                    _ => continue,
                };
                objects.push(Object::immovable([x, y].into(), image, action))
            }
        }
    }
    objects
}

fn freestyle_course(assets: &Assets, rng: &mut OsRng) -> Vec<Object> {
    let x_range = FREESTYLE_X_START..TREE_SLALOM_X_START;
    let x_spacing = 40;

    let y_range = COURSE_Y_START..MAP_HEIGHT;
    let y_spacing = 100;
    let y_iter = y_range.step_by(y_spacing).map(|y| y as f32);

    let mut objects = vec![];
    for y in y_iter {
        for x in x_range.clone().step_by(x_spacing).map(|x| x as f32) {
            if rng.gen_bool(0.30) {
                let (image, action) = match rng.gen_range(0..14) {
                    0..=2 => (&assets.objects.bump_l, CollisionAction::JumpSmall),
                    3..=5 => (&assets.objects.bump_s, CollisionAction::JumpSmall),
                    6..=8 => (&assets.objects.ramp, CollisionAction::JumpLarge),
                    9 => (&assets.objects.rock, CollisionAction::Fall),
                    10 => (&assets.objects.tree1, CollisionAction::Fall),
                    11 => (&assets.objects.stump, CollisionAction::Fall),
                    12 => (&assets.objects.xtree1, CollisionAction::Fall),
                    13 => (&assets.objects.xtree2, CollisionAction::Fall),
                    _ => continue,
                };
                objects.push(Object::immovable([x, y].into(), image, action))
            }
        }
    }
    objects
}

fn tree_slalom_course(assets: &Assets, rng: &mut OsRng) -> Vec<Object> {
    let x_range = TREE_SLALOM_X_START..MAP_WIDTH;
    let x_spacing = 40;

    let y_range = COURSE_Y_START..MAP_HEIGHT;
    let y_spacing = 100;
    let y_iter = y_range.step_by(y_spacing).map(|y| y as f32);

    let mut objects = vec![];
    for y in y_iter {
        for x in x_range.clone().step_by(x_spacing).map(|x| x as f32) {
            if rng.gen_bool(0.30) {
                let (image, action) = match rng.gen_range(0..15) {
                    0..=4 => (&assets.objects.bigtree, CollisionAction::Fall),
                    5 => (&assets.objects.tree1, CollisionAction::Fall),
                    6 => (&assets.objects.tree2, CollisionAction::Fall),
                    7 => (&assets.objects.tree3, CollisionAction::Fall),
                    8 => (&assets.objects.tree4, CollisionAction::Fall),
                    9 => (&assets.objects.xtree1, CollisionAction::Fall),
                    10 => (&assets.objects.xtree2, CollisionAction::Fall),
                    11 => (&assets.objects.xtree3, CollisionAction::Fall),
                    12 => (&assets.objects.stump, CollisionAction::Fall),
                    13 => (&assets.objects.mushroom, CollisionAction::Nothing),
                    14 => (&assets.objects.rock, CollisionAction::Fall),
                    _ => continue,
                };
                objects.push(Object::immovable([x, y].into(), image, action))
            }
        }
    }
    objects
}
