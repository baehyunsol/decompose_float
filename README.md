# Decompose-Float

A simple library that decomposes floating point numbers and gives you `{ is_neg: bool, exp: i32, mantissa: u128 }`. It supports [f16](https://doc.rust-lang.org/stable/std/primitive.f16.html) and [f128](https://doc.rust-lang.org/stable/std/primitive.f128.html) types via `f16` and `f128` features. The features are not enabled by default: you have to enable them if you need.

It handles subnormal numbers, infinities and NaNs correctly (I hope so).

If you want to use `f16` or `f128` feature, you have to use a nightly version of rust.
