mod triangle;
use std::f32::INFINITY;

use crate::triangle::*;

mod ObjectLoader;
use crate::ObjectLoader::*;

mod pencilsource;
use crate::pencilsource::*;
mod diskdetector;
use crate::diskdetector::*;

mod intersection;
use crate::intersection::*;

use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use bvh::ray::Ray;
use bvh::{Point3, Vector3};
use rand::Rng;

fn main() {
    //let mut tris: Vec<Triangle> = Vec::new();
    println!("Hello, world!");

    let mut tris = load_obj("./", "MOLLER.obj", 2000);
    println!("{} triangles in scene", tris.len());
    let mut min = INFINITY;
    let mut counts = 0;
    for tri in &tris {
        if tri.aabb().surface_area() < min {
            min = tri.aabb().surface_area()
        }
        let SA = tri.aabb().surface_area();
        if (SA == 0.0) {
            //println!("{:?}", tri);
            counts = counts + 1;
        }
    }
    println!("{}", min);
    println!("{}", counts);

    let bvh = BVH::build(&mut tris);

    // println!("built tree");
    // let origin = Point3::new(0.0, 0.0, 0.0);
    // let direction = Vector3::new(1.0, 0.0, 0.0);
    // let ray = Ray::new(origin, direction);

    //let hit = bvh.traverse(&ray, &tris);
    //println!("{:?}", hit.len());
    //bvh.pretty_print();
    //println!("{:?}", tris[0]);
}
