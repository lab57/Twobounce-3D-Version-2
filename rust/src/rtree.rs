//use crate::triangle::{intersect, Hit, Triangle};

use crate::triangle::{intersect, Hit, Triangle};
use crate::vector::{Vector, Vector2};
use crate::ObjectLoader::load_obj;
use std::f32;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct BoundingBox {
    min_point: Vector,
    max_point: Vector,
}

impl BoundingBox {
    fn intersects(&self, origin: Vector, direction: Vector) -> bool {
        fn compute_t(plane_coord: f32, origin_coord: f32, direction_coord: f32) -> f32 {
            if direction_coord == 0.0 {
                if origin_coord < plane_coord {
                    return f32::INFINITY;
                } else {
                    return f32::NEG_INFINITY;
                }
            }
            (plane_coord - origin_coord) / direction_coord
        }

        let t1 = compute_t(self.min_point.x, origin.x, direction.x);
        let t2 = compute_t(self.max_point.x, origin.x, direction.x);
        let t3 = compute_t(self.min_point.y, origin.y, direction.y);
        let t4 = compute_t(self.max_point.y, origin.y, direction.y);
        let t5 = compute_t(self.min_point.z, origin.z, direction.z);
        let t6 = compute_t(self.max_point.z, origin.z, direction.z);

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));
        tmax > tmin.max(0.0)
    }
}

struct RTreeNode {
    bounding_box: BoundingBox,
    triangles: Vec<Rc<Triangle>>,
    children: Vec<RTreeNode>,
}

pub struct RTree {
    root: Option<RTreeNode>,
    triangles: Vec<Rc<Triangle>>,
}

impl RTree {
    pub fn new(triangles: Vec<Rc<Triangle>>, max_triangles_per_leaf: usize) -> Self {
        let root = Self::build_tree(&triangles, max_triangles_per_leaf);
        RTree { root, triangles }
    }

    fn build_tree(triangles: &[Rc<Triangle>], max_triangles_per_leaf: usize) -> Option<RTreeNode> {
        if triangles.is_empty() {
            return None;
        }

        if triangles.len() <= max_triangles_per_leaf {
            let (min_point, max_point) = Self::compute_bounds(triangles);
            let bounding_box = BoundingBox {
                min_point,
                max_point,
            };
            println!("New node!");
            return Some(RTreeNode {
                bounding_box,
                triangles: triangles.to_vec(),
                children: Vec::new(),
            });
        }

        let axis = Self::choose_split_axis(triangles);
        let mut sorted_triangles = triangles.to_vec();
        sorted_triangles.sort_by(|a, b| {
            a.coords[0].getArr()[axis as usize]
                .partial_cmp(&b.coords[0].getArr()[axis as usize])
                .unwrap()
        });

        let mid = sorted_triangles.len() / 2;
        let left_child = Self::build_tree(&sorted_triangles[..mid], max_triangles_per_leaf);
        let right_child = Self::build_tree(&sorted_triangles[mid..], max_triangles_per_leaf);

        let min_point = Vector {
            x: left_child
                .as_ref()
                .unwrap()
                .bounding_box
                .min_point
                .x
                .min(right_child.as_ref().unwrap().bounding_box.min_point.x),
            y: left_child
                .as_ref()
                .unwrap()
                .bounding_box
                .min_point
                .y
                .min(right_child.as_ref().unwrap().bounding_box.min_point.y),
            z: left_child
                .as_ref()
                .unwrap()
                .bounding_box
                .min_point
                .z
                .min(right_child.as_ref().unwrap().bounding_box.min_point.z),
        };
        let max_point = Vector {
            x: left_child
                .as_ref()
                .unwrap()
                .bounding_box
                .max_point
                .x
                .max(right_child.as_ref().unwrap().bounding_box.max_point.x),
            y: left_child
                .as_ref()
                .unwrap()
                .bounding_box
                .max_point
                .y
                .max(right_child.as_ref().unwrap().bounding_box.max_point.y),
            z: left_child
                .as_ref()
                .unwrap()
                .bounding_box
                .max_point
                .z
                .max(right_child.as_ref().unwrap().bounding_box.max_point.z),
        };
        let bounding_box = BoundingBox {
            min_point,
            max_point,
        };

        Some(RTreeNode {
            bounding_box,
            triangles: Vec::new(),
            children: vec![left_child.unwrap(), right_child.unwrap()],
        })
    }

    fn compute_bounds(triangles: &[Rc<Triangle>]) -> (Vector, Vector) {
        let mut min_point = Vector {
            x: f32::INFINITY,
            y: f32::INFINITY,
            z: f32::INFINITY,
        };
        let mut max_point = Vector {
            x: f32::NEG_INFINITY,
            y: f32::NEG_INFINITY,
            z: f32::NEG_INFINITY,
        };

        for triangle in triangles {
            for vertex in &triangle.coords {
                min_point.x = min_point.x.min(vertex.x);
                min_point.y = min_point.y.min(vertex.y);
                min_point.z = min_point.z.min(vertex.z);
                max_point.x = max_point.x.max(vertex.x);
                max_point.y = max_point.y.max(vertex.y);
                max_point.z = max_point.z.max(vertex.z);
            }
        }

        (min_point, max_point)
    }

    fn choose_split_axis(triangles: &[Rc<Triangle>]) -> usize {
        let mut extents = [0.0; 3];
        for axis in 0..3 {
            let (min_coord, max_coord): (Vec<f32>, Vec<f32>) = triangles
                .iter()
                .map(|t| (t.coords[0].getArr()[axis], t.coords[2].getArr()[axis]))
                .unzip();
            extents[axis] = max_coord.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
                - min_coord.iter().cloned().fold(f32::INFINITY, f32::min);
        }
        extents
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0
    }

    pub fn query_ray(&self, origin: Vector, direction: Vector) -> Vec<Rc<Triangle>> {
        self._query_ray_recursive(&self.root.as_ref().unwrap(), origin, direction)
    }

    fn _query_ray_recursive(
        &self,
        node: &RTreeNode,
        origin: Vector,
        direction: Vector,
    ) -> Vec<Rc<Triangle>> {
        if !node.bounding_box.intersects(origin, direction) {
            return vec![];
        }

        if !node.children.is_empty() {
            let mut result = Vec::new();
            for child in &node.children {
                result.extend(self._query_ray_recursive(child, origin, direction));
            }
            result
        } else {
            node.triangles.clone()
        }
    }

    pub fn check_intersections(&self, st: Vector, dir: Vector) -> Option<Hit> {
        let mut min_t = f32::INFINITY;
        let mut best_hit: Option<Hit> = None;
        let triangles = self.query_ray(st, dir);
        for tri in triangles {
            let new_hit = intersect(&Rc::clone(&tri), st, dir);
            if let Some(hit) = new_hit {
                if hit.t < min_t {
                    min_t = hit.t;
                    best_hit = Some(hit);
                }
            }
        }
        best_hit
    }
}
