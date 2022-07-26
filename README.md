# Bilinear interpolation

https://en.wikipedia.org/wiki/Bilinear_interpolation


## How to use it

Needs to be documented but currently the best might be to have a look at
[tests/integration_tests.rs](tests/integration_tests.rs) on how I insert the
data and then obtain approximate values.


## Code development

Testing the code:
```
$ cargo test --release -- --nocapture
```

## Implementation notes

- `HashMap<usize, f64>` turned out significantly faster compared to
  `HashMap<(usize, usize), f64>`, both when creating and evaluating.
