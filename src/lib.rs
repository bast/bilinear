pub struct Interpolation {
    /// Origin (x0, y0) of the grid
    origin: (f64, f64),
    /// Step size (dx, dy) between grid points
    step: (f64, f64),
    /// Number of grid points (nx, ny)
    num_grid_points: (usize, usize),
    /// Flattened data in row-major order
    data: Vec<f64>,
}

impl Interpolation {
    pub fn new(origin: (f64, f64), step: (f64, f64), num_grid_points: (usize, usize)) -> Self {
        let num_total_points = num_grid_points.0 * num_grid_points.1;

        Interpolation {
            origin,
            step,
            num_grid_points,
            data: vec![0.0; num_total_points],
        }
    }

    fn index(&self, ix: usize, iy: usize) -> usize {
        iy * self.num_grid_points.0 + ix
    }

    fn get(&self, ix: usize, iy: usize) -> f64 {
        debug_assert!(ix < self.num_grid_points.0, "ix out of bounds");
        debug_assert!(iy < self.num_grid_points.1, "iy out of bounds");

        self.data[self.index(ix, iy)]
    }

    pub fn insert(&mut self, ix: usize, iy: usize, value: f64) {
        debug_assert!(ix < self.num_grid_points.0, "ix out of bounds");
        debug_assert!(iy < self.num_grid_points.1, "iy out of bounds");

        let idx = self.index(ix, iy);
        self.data[idx] = value;
    }

    pub fn evaluate(&self, x: f64, y: f64) -> Option<f64> {
        let (ix1, ix2, tx) =
            self.bounding_indices(x, self.origin.0, self.step.0, self.num_grid_points.0)?;
        let (iy1, iy2, ty) =
            self.bounding_indices(y, self.origin.1, self.step.1, self.num_grid_points.1)?;

        let f11 = self.get(ix1, iy1);
        let f12 = self.get(ix1, iy2);
        let f21 = self.get(ix2, iy1);
        let f22 = self.get(ix2, iy2);

        let fx1 = f11 + tx * (f21 - f11);
        let fx2 = f12 + tx * (f22 - f12);
        Some(fx1 + ty * (fx2 - fx1))
    }

    fn bounding_indices(
        &self,
        value: f64,
        origin: f64,
        step: f64,
        max_index: usize,
    ) -> Option<(usize, usize, f64)> {
        if step <= f64::EPSILON {
            return None;
        }

        let r = (value - origin) / step;
        if r < 0.0 || r >= (max_index - 1) as f64 {
            return None; // out of bounds
        }

        let ix1 = r.floor() as usize;
        let ix2 = (ix1 + 1).min(max_index - 1);
        let t = r - ix1 as f64; // normalized factor between ix1 and ix2

        Some((ix1, ix2, t))
    }
}
