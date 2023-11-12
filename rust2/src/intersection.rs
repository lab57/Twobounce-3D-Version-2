use crate::triangle::*;

use crate::ObjectLoader::*;

use crate::diskdetector::*;
use crate::pencilsource::*;

use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use bvh::ray;
use bvh::ray::Ray;
use bvh::{Point3, Vector3};
use rand::Rng;

fn intersect(tris: Vec<Triangle>, ray: Ray) {
    


}
