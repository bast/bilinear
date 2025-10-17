pub struct Interpolation {
    origin: (f64, f64),
    step: (f64, f64),
    nx: usize,
    ny: usize,
    data: Vec<Vec<f64>>,
}

impl Interpolation {
    pub fn new(
        origin: (f64, f64),
        step: (f64, f64),
        num_steps_x: usize,
        num_steps_y: usize,
    ) -> Self {
        let nx = num_steps_x + 1;
        let ny = num_steps_y + 1;

        Interpolation {
            origin,
            step,
            nx,
            ny,
            data: vec![vec![0.0; ny]; nx],
        }
    }

    pub fn insert(&mut self, ix: usize, iy: usize, value: f64) {
        assert!(ix < self.nx, "ix out of bounds");
        assert!(iy < self.ny, "iy out of bounds");
        self.data[ix][iy] = value;
    }

    pub fn evaluate(&self, x: f64, y: f64) -> Option<f64> {
        let (ix1, ix2, tx) = self.bounding_indices(x, self.origin.0, self.step.0, self.nx)?;
        let (iy1, iy2, ty) = self.bounding_indices(y, self.origin.1, self.step.1, self.ny)?;

        let f11 = self.data[ix1][iy1];
        let f12 = self.data[ix1][iy2];
        let f21 = self.data[ix2][iy1];
        let f22 = self.data[ix2][iy2];

        Some((f11 * (1.0 - tx) + f21 * tx) * (1.0 - ty) + (f12 * (1.0 - tx) + f22 * tx) * ty)
    }

    fn bounding_indices(
        &self,
        value: f64,
        origin: f64,
        step: f64,
        max_index: usize,
    ) -> Option<(usize, usize, f64)> {
        if step == 0.0 {
            return None;
        }

        let r = (value - origin) / step;
        if r < 0.0 || r > (max_index - 1) as f64 {
            return None; // out of bounds
        }

        let ix1 = r.floor() as usize;
        let ix2 = (ix1 + 1).min(max_index - 1);
        let t = r - ix1 as f64; // normalized factor between ix1 and ix2

        Some((ix1, ix2, t))
    }
}
