mod ObjectLoader;
mod triangle;
mod vector;
use crate::triangle::TriObject;
use std::rc::Rc;
use triangle::Triangle;
use vector::{Vector, Vector2};
use ObjectLoader::loadObjects;
fn main() {
    let v1 = Vector {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    let v2 = Vector {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };
    let v3 = Vector {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    let vt1 = Vector2 { x: 1.0, y: 0.0 };
    let vt2 = Vector2 { x: 0.0, y: 1.0 };
    let vt3 = Vector2 { x: 0.5, y: 0.5 };

    let normal = Vector {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    let obj = TriObject::new("test".to_string(), 100, false);
    let mut obj_rc = Rc::new(obj);
    let tri = Triangle {
        coords: [v1, v2, v3],
        texture: [vt1, vt2, vt3],
        normal: normal,
        object: obj_rc,
    };
    println!("functional!");
    println!("{:?}", tri.object);

    loadObjects("../../CUBE_TST.obj")
}
