use crate::vector::{Vector, Vector2};
use std::cmp::min;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;
#[derive(Clone, Debug)]
pub struct Triangle {
    pub coords: [Vector; 3],
    pub texture: [Vector2; 3],
    pub normal: Vector,
    pub object: Rc<TriObject>,
}

impl Triangle {
    fn set_texture(&mut self, at: Vector2, bt: Vector2, ct: Vector2) {
        self.texture = [at, bt, ct];
    }

    fn get_area(&self) -> f32 {
        // ... (implement the area calculation)
        return 0.0;
    }

    fn intersect(&self, ray_start: &Vector, ray_vec: &Vector) -> Hit {
        // ... (implement the intersection logic)
        return Hit::Nil;
    }

    fn plot(&self) {
        // ... (implement the plotting logic)
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    status: i32,
}

impl Pixel {
    fn get_color(&self) -> (i32, i32, i32) {
        match self.status {
            0 => (150, 150, 150),
            1 => (0, 255, 0),
            2 => (255, 0, 0),
            _ => (0, 0, 0),
        }
    }
}

pub enum Hit {
    HitInfo,
    Nil,
}
pub struct HitInfo {
    tri: Triangle,
    obj: TriObject,
    u: f32,
    v: f32,
    t: f32,
    origin: Vector,
    dir: Vector,
}

impl HitInfo {
    fn get_pixel(&self) -> Pixel {
        let hit_pt: [f32; 3] = [1.0 - self.u - self.v, self.u, self.v];
        let texture_loc: Vector2 = self.tri.texture[0] * hit_pt[0]
            + self.tri.texture[1] * hit_pt[1]
            + self.tri.texture[2] * hit_pt[2];

        let loc = (
            (self.obj.resolution * texture_loc.y.round() as i32),
            (self.obj.resolution * texture_loc.x.round() as i32),
        );
        return self.obj.texture[loc.1 as usize][loc.0 as usize];
    }

    fn cartesian(&self) -> Vector {
        return self.dir.calc_coord(self.origin, self.t);
    }
}
#[derive(Clone, Debug)]
pub struct TriObject {
    pub name: String,
    pub resolution: i32,
    pub skip: bool,
    pub texture: Vec<Vec<Pixel>>,
}

impl TriObject {
    pub fn new(name: String, resolution: i32, skip: bool) -> Self {
        let texture = vec![vec![Pixel { status: 0 }; resolution as usize]; resolution as usize];
        TriObject {
            name,
            resolution,
            skip,
            texture,
        }
    }

    fn calc_bounding_box(&mut self) {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut min_z = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;
        let mut max_z = f32::MIN;

        // for vec in &self.points {
        //     if vec.x < min_x {
        //         min_x = vec.x;
        //     }
        //     if vec.y < min_y {
        //         min_y = vec.y;
        //     }
        //     if vec.z < min_z {
        //         min_z = vec.z;
        //     }
        //     if vec.x > max_x {
        //         max_x = vec.x;
        //     }
        //     if vec.y > max_y {
        //         max_y = vec.y;
        //     }
        //     if vec.z > max_z {
        //         max_z = vec.z;
        //     }
        // }

        let min_pt = Vector {
            x: min_x,
            y: min_y,
            z: min_z,
        };
        let max_pt = Vector {
            x: max_x,
            y: max_y,
            z: max_z,
        };
    }
}
