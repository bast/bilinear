# Bilinear interpolation

https://en.wikipedia.org/wiki/Bilinear_interpolation


## How to use it

Needs to be documented but currently the best might be to have a look at
[tests/integration_tests.rs](tests/integration_tests.rs) on how I insert the
data and then obtain approximate values.

I first want to test it a bit in another project to stabilize/sharpen the API
and then I will document in detail how to set it up and evaluate.


## Testing the code

```bash
$ cargo test --release -- --nocapture
```
