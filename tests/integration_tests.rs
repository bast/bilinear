use rand::{Rng, SeedableRng, rngs::StdRng};

use std::time::Instant;

fn gaussian(r: (f64, f64), center: (f64, f64), prefactor: f64) -> f64 {
    -prefactor * ((r.0 - center.0).powi(2) + (r.1 - center.1).powi(2))
}

fn example_function(x: f64, y: f64) -> f64 {
    let f1 = gaussian((x, y), (0.0, 2.0), 1.0);
    let f2 = gaussian((x, y), (-1.0, -2.0), 0.25);
    let f3 = gaussian((x, y), (2.0, -2.0), 0.5);
    let f4 = gaussian((x, y), (4.0, 4.0), 0.5);
    let f5 = 0.02 * x + 0.05 * y;

    f1.exp() + f2.exp() - f3.exp() + 0.5 * f4.exp() + f5
}

fn percent_error(v: f64, v_approx: f64) -> f64 {
    let e = (v - v_approx).abs();
    100.0 * e / v.abs()
}

#[test]
fn basic() {
    let (x_min, x_max) = (-4.0, 4.0);
    let (y_min, y_max) = (-4.0, 4.0);

    let num_steps_x = 2_000;
    let num_steps_y = 2_000;

    let origin = (x_min, y_min);
    let step = (
        (x_max - x_min) / (num_steps_x as f64),
        (y_max - y_min) / (num_steps_y as f64),
    );
    let num_grid_points = (num_steps_x + 1, num_steps_y + 1);

    let start = Instant::now();
    let mut interpolation = bilinear::Interpolation::new(origin, step, num_grid_points);
    for ix in 0..=num_steps_x {
        let x = origin.0 + step.0 * (ix as f64);
        for iy in 0..=num_steps_y {
            let y = origin.1 + step.1 * (iy as f64);
            interpolation.insert(ix, iy, example_function(x, y));
        }
    }
    println!("time elapsed setting up: {:?}", start.elapsed());

    let num_points = 5_000_000;
    let mut rng = StdRng::from_seed([0; 32]);
    let random_points: Vec<(f64, f64)> = (0..num_points)
        .map(|_| {
            (
                rng.random_range(x_min..x_max),
                rng.random_range(y_min..y_max),
            )
        })
        .collect();

    let start = Instant::now();
    for (x, y) in random_points {
        let z = example_function(x, y);
        let z_approx = interpolation.evaluate(x, y).unwrap();
        if z.abs() > 0.001 {
            assert!(percent_error(z, z_approx) < 1.0);
        }
    }
    println!("time elapsed evaluating: {:?}", start.elapsed());
}
