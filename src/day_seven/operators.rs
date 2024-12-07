pub struct Operators {
    operators: Vec<i8>
}

impl Operators {
    pub fn new(len: usize) -> Operators {
        let mut ops = vec![0; len-1];
        ops[0] = -1;

        Operators { 
            operators: ops
        }
    }
}

impl Iterator for Operators {
    type Item = Vec<i8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut over = 0;

        for i in 0..self.operators.len() {
            over = 0;
            self.operators[i] += 1;

            if self.operators[i] < 3 {
                break;
            }

            self.operators[i] = 0;
            over = 1;
        }

        if over == 0 {Some(self.operators.clone())} else {None}
    }
}