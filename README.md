# Bilinear interpolation

https://en.wikipedia.org/wiki/Bilinear_interpolation

Testing the code:
```
$ cargo test --release -- --nocapture
```

## Implementation notes

- `HashMap<usize, f64>` turned out significantly faster compared to
  `HashMap<(usize, usize), f64>`, both when creating and evaluating.
