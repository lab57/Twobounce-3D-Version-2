mod triangle;
mod vector;
use triangle::*;
use vector::Vector;
fn main() {
    let v1 = Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    let t = Triangle{}
    println!("{}", v1)
}
