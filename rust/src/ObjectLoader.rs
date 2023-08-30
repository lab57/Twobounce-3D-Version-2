use crate::triangle::*;
use crate::vector::{Vector, Vector2};
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
pub fn loadObjects(path: &str) {
    let contents = fs::read_to_string(path).expect("File read");
    println!("{}", contents);
    let points: Vec<Vector> = Vec::new();
    let objects: Vec<TriObject> = Vec::new();
    let triangles: Vec<Triangle> = Vec::new();
    let normals: Vec<Vector> = Vec::new();
    let texture_verticies: Vec<Vector2> = Vec::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let current_object;
        match tokens.get(0) {
            Some(&"o") | Some(&"g") => {
                let name = tokens.get(1).unwrap_or(&"").to_string();
                let args: Vec<&str> = name.split("-").collect();
                let mut obj = TriObject::new(name.clone(), 100, args.contains(&"skip"));
                if !obj.skip {
                    objects.push(obj.clone());
                }
                current_object = Some(obj);
            }

            _ => {}
        }
    }
}
