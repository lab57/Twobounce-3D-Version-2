mod ObjectLoader;
mod conesource;
mod diskdetector;
mod export;
mod pencilsource;
mod pointsource;
mod rtree;
mod scene;
mod triangle;
mod vector;
use crate::conesource::*;
use crate::export::*;
use crate::pointsource::*;
use crate::rtree::RTree;
use crate::triangle::*;
use crate::{diskdetector::DiskDetector, pencilsource::*};
use std::env;
use std::rc::Rc;
use triangle::Triangle;
use vector::{Vector, Vector2};
use ObjectLoader::load_obj;

// fn main() {
//     let v = Vector::new(1.1, 2.2, 3.0);
//     let mut S = Scene::empty();
//     println!("{:?}", S);
//     S.tris.push(v);
//     println!("{:?}", S);
//     println!("{:?}", S.tris[0]);
// }

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let filename = "test-fusion-fixed.obj";

    let (tris) = load_obj("./", filename);
    let r: Vec<Rc<Triangle>>;

    let mut rtree = RTree::new(tris, 1000, 8);

    let st = Vector {
        x: 5.0,
        y: 0.0,
        z: -1.0,
    };
    let end = Vector {
        x: 5.0,
        y: 0.0,
        z: 1.0,
    };
    //let src = PencilSource {
    //start: st,
    //end: end,
    //};
    let src = PointSource {
        position: Vector::new(0.0, 0.0, 0.0),
    };

    let test_pt = Vector::new(3.36, -2.80, -1.614804);
    let norm = Vector::new(-1.0, 0.0, 0.0);
    let det = DiskDetector::new(
        1.0,
        Vector {
            x: -5.0,
            y: 0.0,
            z: 0.0,
        },
        norm,
        10,
    );

    //let dir = Vector::new();
    let r = det.is_visible(&rtree, test_pt);
    println!("Res: {r}",);
    println!("Detector points: {:?}", det.surface_points);
    // let st2 = Vector::new(5.0, 0.0, 0.0);
    // let r2 = rtree.twobounce_debug(0, 1, det, st2, test_pt - st2);
    // println!("res {:?}", r2);
    rtree.twobounce(30000, 1, det, src);
    let mut counter = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if rtree.texture[i][j] != 0 {
                counter = counter + 1;
            }
        }
    }
    println!("counter, {}", counter);
    match filename.strip_suffix(".obj") {
        Some(f) => {
            export(f, &rtree);
        }
        None => println!("File name error"),
    }
}
