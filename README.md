# Bilinear interpolation

https://en.wikipedia.org/wiki/Bilinear_interpolation


## How to use it

Needs to be documented but currently the best might be to have a look at
[tests/integration_tests.rs](tests/integration_tests.rs) on how I insert the
data and then obtain approximate values.

I first want to test it a bit in another project to stabilize/sharpen the API
and then I will document in detail how to set it up and evaluate.


## Input data can have "holes"

When interpolating, the code first finds the right "tile", then checks that we
have data available for all 4 corners. If yes, then the value is interpolated
and used. If one or more of the 4 corner values are missing, then the function
evaluates to `None`.


## Code development

Testing the code:
```
$ cargo test --release -- --nocapture
```

## Implementation notes

- `HashMap<usize, f64>` turned out significantly faster compared to
  `HashMap<(usize, usize), f64>`, both when creating and evaluating.
