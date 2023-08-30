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
    fn addEntry(mut self, children: Vec<child>, parent: parent) {
        let p_idx = self.parents.len();
        let c_idx = self.children.len();
        self.parents.push(parent);
        for c in children {
            
        }
        return;
    }
}
