mod ObjectLoader;
mod diskdetector;
mod export;
mod pencilsource;
mod rtree;
mod triangle;
mod vector;
use crate::export::*;
use crate::rtree::RTree;
use crate::triangle::TriObject;
use crate::triangle::*;
use crate::{diskdetector::DiskDetector, pencilsource::*};
use std::env;
use std::rc::Rc;
use triangle::Triangle;
use vector::{Vector, Vector2};
use ObjectLoader::load_obj;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let (tris, objs) = load_obj("../", "CUBE_TST.obj");
    let r: Vec<Rc<Triangle>>;

    let rtree = RTree::new(tris, 8);

    println!("Performing one bounce");
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
    let pencil = PencilSource {
        start: st,
        end: end,
    };

    let norm = Vector::new(-1.0, 0.0, 0.0);
    let det = DiskDetector::new(
        5.0,
        Vector {
            x: 10.0,
            y: 0.0,
            z: 0.0,
        },
        norm,
        15,
    );

    let vector_sets = pencil.get_emission_rays(1000, 6);

    let mut vis_to_source: Vec<Hit> = Vec::new();

    for core in vector_sets {
        for vector in core {
            let res = rtree.check_intersections(vector.0, vector.1);
            match res {
                Some(res) => {
                    let (x, y) = res.get_pixel();
                    res.obj.getPixel(x, y);
                    res.obj.setPixel(x, y);
                    println!("ouch");
                    println!("x:{}, y:{}", x, y);
                    println!("{}", res.obj.name);
                    vis_to_source.push(res);
                }
                None => {}
            }
        }
    }
    println!("Completed one bounce");
    println!("{:?}", objs[0].texture);

    export("../CUBE_TST", objs);
}
