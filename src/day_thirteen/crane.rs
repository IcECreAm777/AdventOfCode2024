#[derive(Debug)]
pub struct Crane {
    a_x: u64,
    a_y: u64,
    b_x: u64,
    b_y: u64,
    r_x: u64,
    r_y: u64
}

impl Crane {
    pub fn new() -> Crane {
        Crane { a_x: 0, a_y: 0, b_x: 0, b_y: 0, r_x: 0, r_y: 0 }
    }

    pub fn set_a_button(&mut self, x: u64, y: u64) {
        self.a_x = x;
        self.a_y = y;
    }

    pub fn set_b_button(&mut self, x: u64, y: u64) {
        self.b_x = x;
        self.b_y = y;
    }

    pub fn set_result(&mut self, x:u64, y: u64) {
        self.r_x = x;
        self.r_y = y;
    }

    pub fn correct_result(&mut self) {
        self.r_x += 10000000000000;
        self.r_y += 10000000000000;
    }

    pub fn solve(&self) -> u64 {
        let det_a = (self.a_x * self.b_y) as i128 - (self.b_x * self.a_y) as i128;
        let det_one = (self.r_x * self.b_y) as i128 - (self.b_x * self.r_y) as i128;
        let det_two = (self.a_x * self.r_y) as i128 - (self.r_x * self.a_y) as i128;

        if det_one % det_a != 0 || det_two % det_a != 0 {
            return 0
        }

        let solution_one = det_one / det_a;
        let solution_two = det_two / det_a;

        if solution_one < 0 || solution_two < 0 {
            return 0;
        }

        solution_one as u64 * 3 + solution_two as u64
    }
}