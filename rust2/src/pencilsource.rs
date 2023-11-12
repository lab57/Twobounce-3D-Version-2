//use crate::vector::{Vector, Vector2};
use glam::Vec3;
use rand::Rng;
use std::f32::consts::PI;
use std::ops::{Add, Div, Sub};
pub struct PencilSource {
    pub start: Vec3,
    pub end: Vec3,
}

impl PencilSource {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        PencilSource { start, end }
    }

    pub fn random_spherical_vector(&self) -> Vec3 {
        let theta = 2.0 * PI * rand::thread_rng().gen::<f32>();
        let phi = PI * rand::thread_rng().gen::<f32>();
        let sin_phi = phi.sin();
        Vec3 {
            x: sin_phi * theta.cos(),
            y: sin_phi * theta.sin(),
            z: phi.cos(),
        }
    }

    pub fn get_emission_rays(&self, n: usize, num_cores: usize) -> Vec<Vec<(Vec3, Vec3)>> {
        let add_vec = (self.end - self.start) / n as f32;
        let mut out = Vec::new();
        let mut st_i = 0;
        let mut end_i = n / num_cores;
        for _ in 0..num_cores {
            let mut core_vecs = Vec::new();
            for i in st_i..end_i {
                core_vecs.push((
                    self.start + add_vec * (i as f32),
                    self.random_spherical_vector(),
                ));
            }
            st_i = end_i;
            end_i += n / num_cores;
            out.push(core_vecs);
        }
        out.last_mut()
            .unwrap()
            .push((self.end, self.random_spherical_vector()));
        out
    }
}

pub struct PencilSourceDebug {
    start: Vec3,
    end: Vec3,
}

impl PencilSourceDebug {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        PencilSourceDebug { start, end }
    }

    pub fn random_spherical_vector(&self) -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn get_emission_rays(&self, n: usize, num_cores: usize) -> Vec<Vec<(Vec3, Vec3)>> {
        let add_vec = (self.end - self.start) / n as f32;
        let mut out = Vec::new();
        let mut st_i = 0;
        let mut end_i = n / num_cores;
        for _ in 0..num_cores {
            let mut core_vecs = Vec::new();
            for i in st_i..end_i {
                core_vecs.push((
                    self.start + add_vec * (i as f32),
                    self.random_spherical_vector(),
                ));
            }
            st_i = end_i;
            end_i += n / num_cores;
            out.push(core_vecs);
        }
        out.last_mut()
            .unwrap()
            .push((self.end, self.random_spherical_vector()));
        out
    }
}

fn main() {
    // Example usage
    let start = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let end = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let pencil_source = PencilSource::new(start, end);
    let emission_rays = pencil_source.get_emission_rays(10, 2);
    println!("{:?}", emission_rays);

    let pencil_source_debug = PencilSourceDebug::new(start, end);
    let emission_rays_debug = pencil_source_debug.get_emission_rays(10, 2);
    println!("{:?}", emission_rays_debug);
}
