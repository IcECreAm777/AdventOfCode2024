#[derive(Debug)]
pub struct Cell {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool
}

impl Cell {
    pub fn get_num_fences(&self) -> u8 {
        let mut fences = 0;

        if !self.up { fences += 1; }
        if !self.down { fences += 1; }
        if !self.left { fences += 1; }
        if !self.right { fences += 1; }

        fences
    }
}