#![allow(clippy::too_many_arguments)]

use std::collections::HashMap;

pub struct Interpolation {
    origin: (f64, f64),
    step: (f64, f64),
    index_capacity: usize,
    data: HashMap<usize, f64>,
}

impl Interpolation {
    pub fn new(origin: (f64, f64), step: (f64, f64)) -> Self {
        Interpolation {
            origin,
            step,
            index_capacity: 100_000,
            data: HashMap::new(),
        }
    }

    pub fn with_capacity(origin: (f64, f64), step: (f64, f64), index_capacity: usize) -> Self {
        Interpolation {
            origin,
            step,
            index_capacity,
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, ix: usize, iy: usize, value: f64) {
        assert!(ix <= self.index_capacity);
        assert!(iy <= self.index_capacity);

        self.data.insert(self.compound_index(ix, iy), value);
    }

    pub fn evaluate(&self, x: f64, y: f64) -> Option<f64> {
        let (ix1, ix2) = bounding_indices(x, self.origin.0, self.step.0)?;
        let (iy1, iy2) = bounding_indices(y, self.origin.1, self.step.1)?;

        let f11 = self.data.get(&self.compound_index(ix1, iy1))?;
        let f12 = self.data.get(&self.compound_index(ix1, iy2))?;
        let f21 = self.data.get(&self.compound_index(ix2, iy1))?;
        let f22 = self.data.get(&self.compound_index(ix2, iy2))?;

        let x1 = self.origin.0 + self.step.0 * (ix1 as f64);
        let x2 = self.origin.0 + self.step.0 * (ix2 as f64);
        let y1 = self.origin.1 + self.step.1 * (iy1 as f64);
        let y2 = self.origin.1 + self.step.1 * (iy2 as f64);

        Some(get_approximate_z(
            x, y, x1, x2, y1, y2, *f11, *f12, *f21, *f22,
        ))
    }

    fn compound_index(&self, ix: usize, iy: usize) -> usize {
        ix * (self.index_capacity + 1) + iy
    }
}

fn bounding_indices(value: f64, origin_value: f64, step: f64) -> Option<(usize, usize)> {
    let d = value - origin_value;
    let r = d / step;

    let fl = r.floor();
    let cl = r.ceil();

    let fl = fl as isize;
    let cl = cl as isize;

    if fl < 0 || cl < 0 {
        return None;
    }

    let fl = fl as usize;
    let cl = cl as usize;

    if fl == cl {
        Some((fl, cl + 1))
    } else {
        Some((fl, cl))
    }
}

fn get_approximate_z(
    x: f64,
    y: f64,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    f11: f64,
    f12: f64,
    f21: f64,
    f22: f64,
) -> f64 {
    let a1 = x2 - x;
    let a2 = x - x1;

    let b1 = y2 - y;
    let b2 = y - y1;

    let t11 = a1 * f11 * b1;
    let t12 = a1 * f12 * b2;
    let t21 = a2 * f21 * b1;
    let t22 = a2 * f22 * b2;

    (t11 + t12 + t21 + t22) / ((x2 - x1) * (y2 - y1))
}
