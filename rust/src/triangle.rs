use crate::vector::{Vector, Vector2};
use std::cell::RefCell;
use std::cmp::min;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::option;
use std::rc::Rc;
#[derive(Clone)]
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
        return 1.0 / 2.0
            * (self.coords[0] - self.coords[1])
                .cross(self.coords[0] - self.coords[2])
                .abs();
    }
}

impl fmt::Debug for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Triangle")
            .field("Normal", &self.normal)
            .finish()
    }
}

pub fn intersect(tri: &Rc<Triangle>, ray_start: Vector, ray_vec: Vector) -> Option<Hit> {
    let eps = 0.000001;
    // ... (implement the intersection logic)
    let edge1 = tri.coords[1] - tri.coords[0];
    let edge2 = tri.coords[2] - tri.coords[0];

    let pvec = ray_vec.cross(edge2);
    let det = edge1.dot(pvec);

    if (det.abs() < eps) {
        return None;
    }
    let inv_det = 1.0 / det;
    let tvec = ray_start - tri.coords[0];
    let u = tvec.dot(pvec) * inv_det;
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let qvec = tvec.cross(edge1);
    let v = ray_vec.dot(qvec) * inv_det;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = edge2.dot(qvec) * inv_det;
    if t < eps {
        return None;
    }

    return Some(Hit {
        tri: Rc::clone(tri),
        obj: Rc::clone(&tri.object),
        u,
        v,
        t,
        origin: ray_start,
        dir: ray_vec,
    });
}

//#[derive(Copy, Clone, Debug)]
// pub struct Pixel {
//     pub status: i32,
// }

// impl Pixel {
//     pub fn new() -> Self {
//         return Pixel { status: 0 };
//     }
//     pub fn get_color(&self) -> (u8, u8, u8) {
//         match self.status {
//             0 => (150, 150, 150),
//             1 => (0, 255, 0),
//             2 => (255, 0, 0),
//             _ => (0, 0, 0),
//         }
//     }

//     pub fn setStatus(&mut self, n: i32) -> i32 {
//         self.status = n;
//         println!("setting status to {}", n);
//         println!("{}", self.status);
//         return self.status;
//     }

//     pub fn get_status(&self) -> i32 {
//         return self.status;
//     }
// }
#[derive(Debug)]
pub struct Hit {
    pub tri: Rc<Triangle>,
    pub obj: Rc<TriObject>,
    pub u: f32,
    pub v: f32,
    pub t: f32,
    pub origin: Vector,
    pub dir: Vector,
}

impl Hit {
    pub fn get_pixel(&self) -> (usize, usize) {
        let hit_pt: [f32; 3] = [1.0 - self.u - self.v, self.u, self.v];
        let texture_loc: Vector2 = self.tri.texture[0] * hit_pt[0]
            + self.tri.texture[1] * hit_pt[1]
            + self.tri.texture[2] * hit_pt[2];

        let loc = (
            ((self.obj.resolution as f32 * texture_loc.y) as i32),
            ((self.obj.resolution as f32 * texture_loc.x) as i32),
        );
        // println!("Getting pixel, {} {}", loc.1, loc.0);
        // println!("{:?}", texture_loc);
        // println!("{:?}", loc);
        // println!("{}", self.obj.resolution as f32 * texture_loc.y);
        // println!("{}", (self.obj.resolution as f32 * texture_loc.y) as i32);
        return (loc.1 as usize, loc.0 as usize);
        //return self.obj.texture[loc.1 as usize][loc.0 as usize];
    }

    pub fn cartesian(&self) -> Vector {
        return self.dir.calc_coord(self.origin, self.t);
    }
}
#[derive(Clone)]
pub struct TriObject {
    pub name: String,
    pub resolution: i32,
    pub skip: bool,
    pub texture: Vec<Vec<u8>>,
}

impl TriObject {
    pub fn new(name: String, resolution: i32, skip: bool) -> Self {
        let texture = vec![vec![0; resolution as usize]; resolution as usize];
        TriObject {
            name,
            resolution,
            skip,
            texture,
        }
    }

    pub fn setPixel(&self, x: usize, y: usize) {
        self.texture[y][x] = 1;
    }

    pub fn getPixel(&self, x: usize, y: usize) -> u8 {
        return self.texture[y][x];
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
impl fmt::Debug for TriObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TriObject")
            .field("Name", &self.name)
            .finish()
    }
}
