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

    pub fn is_corner(&self) -> bool {
        self.up && self.left && !self.right && !self.down || self.up && self.right && !self.left && !self.down 
            || self.down && self.left && !self.up && !self.right || self.down && self.right && !self.left && !self.up
    }

    pub fn is_dead_end(&self) -> bool {
        self.get_num_fences() == 3
    }

    pub fn is_single_piece(&self) -> bool {
        !self.up && !self.down && !self.left && !self.right
    }
}