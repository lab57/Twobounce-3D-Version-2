fn main() {
    println!("Hello, world!");
}

struct parent {
    children: Vec<i32>,
    data: Vec<f32>,
}

struct child {
    parent: i32,
    data: i32,
}

struct Scene {
    children: Vec<child>,
    parents: Vec<parent>,
}

impl Scene {
    fn addEntry(self, children: Vec<child>, parents: Vec<parent>) {
        self.chil

        return
    }
}
