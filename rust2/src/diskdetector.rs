//use crate::vector::Vector;
use crate::triangle::*;
use bvh::bvh::BVH;
use glam::Vec3;
use std::f32::consts::PI;

fn rotate(vec: &Vec3, axis: Vec3, angle: f32) -> Vec3 {
    return *vec * angle.cos()
        + axis.cross(*vec) * angle.sin()
        + axis * (axis.dot(*vec) * (1.0 - angle.cos()));
}

pub struct DiskDetector {
    pub r: f32,
    pub center: Vec3,
    pub normal: Vec3,
    pub surface_points: Vec<Vec3>,
}

impl DiskDetector {
    pub fn new(r: f32, center: Vec3, normal: Vec3, n: usize) -> Self {
        let surface_points = DiskDetector::get_surface_points(r, center, normal, n);
        println!(
            "Disk Detector initalized with {} surface points",
            surface_points.len()
        );
        DiskDetector {
            r,
            center,
            normal,
            surface_points,
        }
    }

    fn get_rotation_axis_and_angle(initial_normal: Vec3, target_normal: Vec3) -> (Vec3, f32) {
        let initial_normal = initial_normal.normalize();
        let target_normal = target_normal.normalize();
        if (initial_normal.dot(target_normal) - 1.0).abs() < 1e-8 {
            let dot_product = initial_normal.dot(target_normal);
            if dot_product > 0.0 {
                return (
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    0.0,
                );
            } else {
                return (
                    Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    PI,
                );
            }
        }
        let rotation_axis = initial_normal.cross(target_normal).normalize();
        let cos_angle = initial_normal.dot(target_normal);
        let rotation_angle = cos_angle.acos();
        (rotation_axis, rotation_angle)
    }

    pub fn get_surface_points(r: f32, center: Vec3, normal: Vec3, n: usize) -> Vec<Vec3> {
        let mut points = Vec::new();

        let side_length = 2.0 * r / (n as f32).sqrt();
        let num_points = (2.0 * r / side_length).round() as usize;

        for i in 0..num_points {
            for j in 0..num_points {
                let x = -r + i as f32 * side_length;
                let y = -r + j as f32 * side_length;

                if x.powi(2) + y.powi(2) <= r.powi(2) {
                    let mut point = Vec3 { x, y, z: 0.0 };
                    let (rotation_axis, rotation_angle) = Self::get_rotation_axis_and_angle(
                        Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        normal.clone(),
                    );
                    point = rotate(&point, rotation_axis, rotation_angle);
                    point = point + center;
                    points.push(point);
                }
            }
        }

        points
    }

    pub fn is_visible(&self, bvh: BVH, shapes: Vec<Triangle>, point: Vec3) {}

    // pub fn is_visible(&self, rtree: &RTree, source: Vector) -> bool {
    //     for point in &self.surface_points {
    //         let dir = *point - source;
    //         if let Some(hit) = rtree.check_intersections(source.clone(), dir.clone()) {
    //             return false;
    //         }
    //     }
    //     true
    // }
}
