pub struct Dependency {
    previous: Vec<i32>,
    next: Vec<i32>
}

impl Dependency {
    pub fn new() -> Dependency {
        Dependency{
            previous: vec![], 
            next: vec![]
        }
    }

    pub fn is_previous(&self, num: i32) -> bool {
        self.previous.contains(&num)
    }

    pub fn is_next(&self, num: i32) -> bool {
        self.next.contains(&num)
    }

    pub fn add_previous(&mut self, prev: i32) {
        self.previous.push(prev);
    }

    pub fn add_next(&mut self, next: i32) {
        self.next.push(next);
    }
}