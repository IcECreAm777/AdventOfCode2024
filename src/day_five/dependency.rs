pub struct Dependency {
    next: Vec<i32>
}

impl Dependency {
    pub fn new() -> Dependency {
        Dependency{
            next: vec![]
        }
    }

    pub fn is_next(&self, num: i32) -> bool {
        self.next.contains(&num)
    }

    pub fn add_next(&mut self, next: i32) {
        self.next.push(next);
    }
}