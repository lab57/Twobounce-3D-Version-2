use crate::triangle::*;
//use crate::{Vector, Vector2};
use bvh::aabb::{Bounded, AABB};
use bvh::{Point3, Vector3};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;
#[derive(Clone)]
struct Pixel {
    pub color: Vec<f32>,
    position: Point3,
    uv: Vec<f32>,
}

impl Pixel {
    fn new() -> Pixel {
        return Pixel {
            color: vec![0.0, 0.0, 0.0],
            position: Point3::ZERO,
            uv: vec![0.0, 0.0],
        };
    }
}

fn in_triangle(tri: &Triangle, p: vec2) -> bool {
    // Extract texture vertices
    let uv1 = &tri.texture[0];
    let uv2 = &tri.texture[1];
    let uv3 = &tri.texture[2];

    // Calculate vectors from point p to each vertex of the triangle
    let v0 = vec2 {
        u: uv3.u - uv1.u,
        v: uv3.v - uv1.v,
    };
    let v1 = vec2 {
        u: uv2.u - uv1.u,
        v: uv2.v - uv1.v,
    };
    let v2 = vec2 {
        u: p.u - uv1.u,
        v: p.v - uv1.v,
    };

    // Calculate dot products
    let dot00 = v0.dot(&v0);
    let dot01 = v0.dot(&v1);
    let dot02 = v0.dot(&v2);
    let dot11 = v1.dot(&v1);
    let dot12 = v1.dot(&v2);

    // Compute barycentric coordinates
    let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

    // Check if point is inside the triangle
    (u >= 0.0) && (v >= 0.0) && (u + v < 1.0)
}

pub fn load_obj(path: &str, filename: &str, res: usize) -> (Vec<Triangle>) {
    let mut vertices = Vec::new();
    //let mut objects: Vec<TriObject> = Vec::new();
    let mut totalSA: f32 = 0.0;
    let mut triangles: Vec<Triangle> = Vec::new();
    let mut normals = Vec::new();
    let mut texture_vertices = Vec::new();
    let mut ATLAS = vec![vec![Pixel::new(); res]; res];

    match std::env::current_dir() {
        Ok(path) => {
            println!("The current directory is {}", path.display());
        }
        Err(e) => {
            panic!("Error loading PWD");
        }
    }

    let file_path = Path::new(path).join(filename);
    let file_result = File::open(&file_path);
    let file;
    match file_result {
        Ok(file_result) => file = file_result,
        Err(file_result) => {
            println!("Current file is {}", file_path.display());
            panic!("Error opening file")
        }
    }

    let reader = io::BufReader::new(file);

    let mut current_object: usize = 0;

    for line_res in reader.lines() {
        let line;
        match line_res {
            Ok(line_res) => line = line_res,
            Err(line_res) => panic!("Error reading file"),
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();

        match tokens.get(0) {
            Some(&"o") | Some(&"g") => {
                //object or group
                let name = tokens.get(1).unwrap_or(&"");
                let args: Vec<&str> = name.split("-").collect();

                // let mut obj = TriObject {
                //     name: name.to_string(),
                //     resolution: 0, // Assuming a default value, adjust as needed
                //     skip: args.contains(&"skip"),
                //     texture: Vec::new(), // Assuming a default value, adjust as needed
                // };
                //let mut obj = TriObject::new(name.to_string(), 120, args.contains(&"skip"));
                //let obj_rc = Rc::new(obj);
                //if !obj.skip {}
                //objects.push(obj);
                //current_object = objects.len() - 1 //Some(obj_rc); //Some(Rc::new(obj));
            }
            Some(&"v") if tokens.len() > 3 => {
                //vertex
                let vertex = Vector3::new(
                    tokens[1].parse().unwrap_or(0.0),
                    tokens[2].parse().unwrap_or(0.0),
                    tokens[3].parse().unwrap_or(0.0),
                );
                vertices.push(vertex);
                //println!("Vertex {}", vertex);
                //println!("{:?}", vertices);
            }
            Some(&"vn") if tokens.len() > 3 => {
                //normal vectors
                let normal = Vector3::new(
                    tokens[1].parse().unwrap_or(0.0),
                    tokens[2].parse().unwrap_or(0.0),
                    tokens[3].parse().unwrap_or(0.0),
                );
                normals.push(normal);
            }
            Some(&"vt") if tokens.len() > 2 => {
                //texture vertex
                texture_vertices.push(vec2::new(
                    tokens[1].parse().unwrap_or(0.0),
                    tokens[2].parse().unwrap_or(0.0),
                ));
            }
            Some(&"f") => {
                //face
                if tokens.len() > 3 {
                    let parse_face = |s: &str| -> (usize, usize, usize) {
                        let parts: Vec<usize> = s
                            .split('/')
                            .map(|part| part.parse().unwrap_or(1) - 1)
                            .collect();
                        (parts[0], parts[1], parts[2])
                    };

                    let (v1, vt1, vn1) = parse_face(tokens[1]);
                    let (v2, vt2, vn2) = parse_face(tokens[2]);
                    let (v3, vt3, vn3) = parse_face(tokens[3]);

                    // let mut triangle = Triangle {
                    //     coords: [vertices[v1], vertices[v2], vertices[v3]],
                    //     texture: [
                    //         texture_vertices[vt1],
                    //         texture_vertices[vt2],
                    //         texture_vertices[vt3],
                    //     ],
                    //     normal: normals[vn1],
                    //     object: current_object,
                    // };
                    let mut verts = vec![vertices[v1], vertices[v2], vertices[v3]];
                    let mut texts = vec![
                        texture_vertices[vt1],
                        texture_vertices[vt2],
                        texture_vertices[vt3],
                    ];
                    let mut tri = Triangle::new(verts, texts);
                    totalSA = totalSA + tri.surface_area();
                    triangles.push(tri);
                }
            }
            _ => {}
        }
    }

    //filter out triangles with a bounding box with 0 surface area
    // this happens when the difference in verticies is lost due to 32-bit floats
    // about ~.2% of triangles get lost here
    let filtered: Vec<Triangle> = triangles
        .into_iter()
        .filter(|tri| tri.aabb().surface_area() > 0.0)
        .collect();

    return filtered;
}
