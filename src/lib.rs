#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]

mod fmt;
mod ieee754;

#[cfg(test)]
mod tests;

/// It's literally `DecomposeResult`, not a number. If it were a number,
/// `Zero` and `NegZero` has to be equal. But they're not equal because they're
/// results of decompositions of different bit patterns.
/// Also, that's why it implements `Eq` but not `Ord`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecomposeResult {
    /// `n = (-1)^is_neg * 2^exp * mantissa / 2^127`
    ///
    /// always `2^127 <= mantissa < 2^128`
    Normal {
        is_neg: bool,
        exp: i32,
        mantissa: u128,
    },
    Zero,
    NegZero,
    Infinity,
    NegInfinity,

    /// It treats NaNs equal, and that's intentional. IEEE754 treats NaNs unequal because
    /// NaN is an indicator of an invalid operation. Here, `DecomposeResult::NotANumber` is
    /// a result of a successful `n.decompose()`, so it has to implement `Eq`.
    NotANumber,
}

pub trait Decompose {
    fn decompose(&self) -> DecomposeResult;
}
