use std::rc::Rc;

use ggez::graphics::{Canvas, DrawParam, Image};
use ggez::mint::{Point2, Vector2};
use rand::rngs::OsRng;
use rand::Rng;

use crate::assets::{Assets, Objects};

const LIFT_X_POS: f32 = 100.;

pub struct Map {
    immovable_objects: Vec<ImmovableObject>,
    rng: OsRng,
}

struct ImmovableObject {
    draw_param: DrawParam,
    image: Rc<Image>,
}

impl ImmovableObject {
    fn new(position: Point2<f32>, image: &Rc<Image>) -> Self {
        Self {
            draw_param: DrawParam::default().dest(position),
            image: image.clone(),
        }
    }
}

impl Map {
    pub fn new(assets: &Assets) -> Self {
        let mut rng = OsRng::default();
        let mut immovable_objects = vec![
            ImmovableObject::new([LIFT_X_POS, 100.].into(), &assets.objects.lift),
            ImmovableObject::new([LIFT_X_POS, 140.].into(), &assets.objects.chairlift),
        ];
        immovable_objects.extend(start_signs(assets).into_iter());
        immovable_objects.extend(freestyle_course(assets, &mut rng));
        Self {
            immovable_objects,
            rng,
        }
    }

    pub fn update(&mut self, assets: &Assets) {
        // call generators
        // move everything in a certain `direction`

        // check which objects need to be cleared (off top of screen)

        // generate for each "section" of the hill:
        // random left (wilderness),
        // slalom (flags + moguls),
        // freestyle (ramps),
        // tree slalom (lots of trees),
        // random right (wilderness)
        todo!()
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for o in &self.immovable_objects {
            canvas.draw(o.image.as_ref(), o.draw_param);
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
        ImmovableObject::new(slalom_xy.into(), slalom),
        ImmovableObject::new(freestyle_xy.into(), freestyle),
        ImmovableObject::new(tree_slalom_xy.into(), tree_slalom),
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
                let image = match rng.gen_range(0..3) {
                    0 => &assets.objects.bump_l,
                    1 => &assets.objects.bump_s,
                    2 => &assets.objects.ramp,
                    _ => continue,
                };
                objects.push(ImmovableObject::new([x, y].into(), image))
            }
        }
    }
    objects
}
