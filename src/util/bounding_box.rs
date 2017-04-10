use util::sizes;

pub struct BoundingBox {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl BoundingBox {
    pub fn is_in_bound(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    pub fn move_in_direction(&mut self, dir_x: i32, dir_y: i32) -> (i32, i32) {
        let pos_x_new = self.x + dir_x;
        let pos_y_new = self.y + dir_y;

        let mut moved_x = 0;
        let mut moved_y = 0;

        if pos_x_new < 0 {
            self.x = 0;
            moved_x = dir_x - (0 - pos_x_new);
        } else if pos_x_new + self.width > sizes::MAX_X {
            self.x = sizes::MAX_X - self.width;
            moved_x = dir_x - (pos_x_new - sizes::MAX_X);
        } else {
            self.x = pos_x_new;
            moved_x = dir_x;
        }

        if pos_y_new < 0 {
            self.y = 0;
            moved_y = dir_y - (0 - pos_y_new);
        } else if pos_y_new + self.height > sizes::MAX_Y {
            self.y = sizes::MAX_Y - self.height;
            moved_y = dir_y - (pos_y_new - sizes::MAX_Y);
        } else {
            self.y = pos_y_new;
            moved_y = dir_y;
        }

        (moved_x, moved_y)
    }
}
