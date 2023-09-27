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
use crate::triangle::TriObject;
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
    let (tris, objs) = load_obj("./", "2bounce_test_geo.obj");
    let r: Vec<Rc<Triangle>>;

    let mut rtree = RTree::new(tris, objs, 8);

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
    let src = PencilSource {
        start: st,
        end: end,
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
    let st2 = Vector::new(5.0, 0.0, 0.0);
    let r2 = rtree.twobounce_debug(0, 1, det, st2, test_pt - st2);
    println!("res {:?}", r2);
    //rtree.twobounce(400, 1, det, src);
    //export("2bounce_test_geo", &rtree.objs);
}
