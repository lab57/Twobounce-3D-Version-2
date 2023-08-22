use crate::vector::{Vector, Vector2};
use std::cmp::min;
use std::ops::{Add, Div, Mul, Sub};
#[derive(Clone)]
struct Triangle {
    coords: [Vector; 3],
    texture: [Vector2; 3],
    normal: Vector,
    object: &'TriObject,
}

impl Triangle {
    fn set_texture(&mut self, at: Vector2, bt: Vector2, ct: Vector2) {
        self.at = Some(at);
        self.bt = Some(bt);
        self.ct = Some(ct);
    }

    fn get_area(&self) -> f32 {
        // ... (implement the area calculation)
    }

    fn intersect(&self, ray_start: &Vector, ray_vec: &Vector) -> Hit {
        // ... (implement the intersection logic)
    }

    fn plot(&self) {
        // ... (implement the plotting logic)
    }
}
#[derive(Copy, Clone)]
struct Pixel {
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

struct Hit {
    tri: Triangle,
    obj: TriObject,
    u: f32,
    v: f32,
    t: f32,
    origin: Vector,
    dir: Vector,
}

impl Hit {
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
#[derive(Clone)]
struct TriObject {
    name: String,
    triangles: Vec<Triangle>,
    points: Vec<Vector>,
    resolution: i32,
    texture: Vec<Vec<Pixel>>,
    bounding_box: [Vector; 2],
    skip: bool,
}

impl TriObject {
    fn init_texture(&mut self) {
        self.texture =
            vec![vec![Pixel { status: 0 }; self.resolution as usize]; self.resolution as usize];
    }

    fn surface_area(&self) -> f32 {
        // ... (implement the surface_area calculation)
        let SA: f32 = 0.0;
        for tri in self.triangles {
            SA += tri.get_area()
        }
        return SA;
    }

    fn calc_bounding_box(&mut self) {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut min_z = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;
        let mut max_z = f32::MIN;

        for vec in &self.points {
            if vec.x < min_x {
                min_x = vec.x;
            }
            if vec.y < min_y {
                min_y = vec.y;
            }
            if vec.z < min_z {
                min_z = vec.z;
            }
            if vec.x > max_x {
                max_x = vec.x;
            }
            if vec.y > max_y {
                max_y = vec.y;
            }
            if vec.z > max_z {
                max_z = vec.z;
            }
        }

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

        self.bounding_box = [min_pt, max_pt];
    }
}
