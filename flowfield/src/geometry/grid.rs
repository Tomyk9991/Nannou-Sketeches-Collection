pub struct Grid {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    rows: usize,
    cols: usize,
}

impl Grid {
    pub fn new(x_min: f32, x_max: f32, y_min: f32, y_max: f32, rows: usize, cols: usize) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
            rows,
            cols,
        }
    }

    pub fn iter(&self) -> GridIterator {
        GridIterator {
            x: self.x_min,
            y: self.y_min,
            x_step: (self.x_max - self.x_min) / (self.cols as f32),
            y_step: (self.y_max - self.y_min) / (self.rows as f32),
            row: 0,
            col: 0,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

pub struct GridIterator {
    x: f32,
    y: f32,
    x_step: f32,
    y_step: f32,
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
}

impl Iterator for GridIterator {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.rows {
            return None;
        }
        let point = (self.x, self.y);
        self.x += self.x_step;
        self.col += 1;
        if self.col >= self.cols {
            self.col = 0;
            self.x = self.x - self.x_step * (self.cols as f32);
            self.y += self.y_step;
            self.row += 1;
        }
        Some(point)
    }
}