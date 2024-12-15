#[derive(Debug)]
pub struct Robot {
    start_point: (usize, usize),
    velocity: (i32, i32)
}

impl Robot {
    pub fn new(start: (usize, usize), vel: (i32, i32)) -> Robot {
        Robot {
            start_point: start,
            velocity: vel
        }
    }

    pub fn move_robot(&self, width: i32, height: i32, times: usize) -> (i32, i32) {
        let new_point = (self.start_point.0 as i32 + self.velocity.0 * times as i32, self.start_point.1 as i32 + self.velocity.1 * times as i32);

        let raw_x = new_point.0 % width;
        let raw_y = new_point.1 % height;

        let x = if raw_x < 0 { width - raw_x.abs() } else { raw_x };
        let y = if raw_y < 0 { height - raw_y.abs() } else { raw_y };

        (x, y)
    }
}