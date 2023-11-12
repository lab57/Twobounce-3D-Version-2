use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use bvh::ray::Ray;
use bvh::{Point3, Vector3};
use rand::Rng;
#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    uv: [usize; 2], //UV coordinates in texture space
    center: Point3, //center coordinates in R3
    i: usize,       //array indicies in atlas
    j: usize,
}
#[derive(Debug, Clone, Copy)]
pub struct vec2 {
    pub u: f32,
    pub v: f32,
}

impl vec2 {
    pub fn new(u: f32, v: f32) -> vec2 {
        return vec2 { u, v };
    }
    pub fn ZERO() -> vec2 {
        return vec2 { u: 0.0, v: 0.0 };
    }
    pub fn dot(&self, other: &vec2) -> f32 {
        return other.u * self.u + other.v * self.v;
    }
}

#[derive(Debug)]
pub struct Triangle {
    pub position: Vec<Point3>,
    pub texture: Vec<vec2>,
    pub node_index: usize,
    pub pixels: Vec<Pixel>,
}

impl Triangle {
    pub fn new(position: Vec<Point3>, texture: Vec<vec2>) -> Triangle {
        return Triangle {
            position,
            texture,
            node_index: 0,
            pixels: Vec::new(),
        };
    }

    pub fn surface_area(&self) -> f32 {
        if self.position.len() >= 3 {
            // Get vectors representing two sides of the triangle
            let u = self.position[1] - self.position[0];
            let v = self.position[2] - self.position[0];

            // The cross product of u and v gives a vector perpendicular to the triangle's surface
            // whose length is equal to the area of the parallelogram formed by u and v
            let cross_product = u.cross(v);

            // The area of the triangle is half the area of the parallelogram
            0.5 * cross_product.length()
        } else {
            // If there are not enough vertices to define a triangle, the area is zero
            0.0
        }
    }
    fn intersect(&self, ray_start: Vector3, ray_vec: Vector3) -> Option<Hit> {
        let eps = 0.000001;
        // ... (implement the intersection logic)
        let edge1 = self.position[1] - self.position[0];
        let edge2 = self.position[2] - self.position[0];

        let pvec = ray_vec.cross(edge2);
        let det = edge1.dot(pvec);

        if (det.abs() < eps) {
            return None;
        }
        let inv_det = 1.0 / det;
        let tvec = ray_start - self.position[0];
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
            tri: Rc::clone(&tri),
            obj: tri.object,
            u,
            v,
            t,
            origin: ray_start,
            dir: ray_vec,
        });
    }
}

impl Bounded for Triangle {
    fn aabb(&self) -> AABB {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut min_z = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        let mut max_z = f32::NEG_INFINITY;

        for point in &self.position {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            min_z = min_z.min(point.z);

            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
            max_z = max_z.max(point.z);
        }
        let min = Point3 {
            x: min_x,
            y: min_y,
            z: min_z,
        };
        let max = Point3 {
            x: max_x,
            y: max_y,
            z: max_z,
        };
        AABB::with_bounds(min, max)
    }
    // fn aabb(&self) -> AABB {
    //     let (min, max) = self.position.iter().fold(
    //         (self.position[0], self.position[0]),
    //         |(mut min, mut max), &point| {
    //             min.x = min.x.min(point.x);
    //             min.y = min.y.min(point.y);
    //             min.z = min.z.min(point.z);

    //             max.x = max.x.max(point.x);
    //             max.y = max.y.max(point.y);
    //             max.z = max.z.max(point.z);

    //             (min, max)
    //         },
    //     );

    //     AABB::with_bounds(min, max)
    // }
}

impl BHShape for Triangle {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}
