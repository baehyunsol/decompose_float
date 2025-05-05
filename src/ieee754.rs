use crate::{Decompose, DecomposeResult};

macro_rules! ieee754 {
    ($fty: ty, $ity: ty, $exponent_width: literal, $mantissa_width: literal) => {
        impl Decompose for $fty {
            fn decompose(&self) -> DecomposeResult {
                let uint = self.to_bits();
                let is_neg = uint >= (1 << ($exponent_width + $mantissa_width));
                let mut exp = ((uint >> $mantissa_width) & ((1 << $exponent_width) - 1)) as i32 - ((1 << ($exponent_width - 1)) - 1);
                let mut mantissa = uint & ((1 << $mantissa_width) - 1);

                if exp == -((1 << ($exponent_width - 1)) - 1) {
                    exp += 1;

                    if mantissa == 0 {
                        if is_neg {
                            return DecomposeResult::NegZero;
                        }

                        else {
                            return DecomposeResult::Zero;
                        }
                    }

                    let to_shift = $mantissa_width - mantissa.ilog2();
                    mantissa <<= to_shift;
                    exp -= to_shift as i32;
                    mantissa -= 1 << $mantissa_width;
                }

                else if exp == 1 << ($exponent_width - 1) {
                    if mantissa == 0 {
                        if is_neg {
                            return DecomposeResult::NegInfinity;
                        }

                        else {
                            return DecomposeResult::Infinity;
                        }
                    }

                    else {
                        return DecomposeResult::NotANumber;
                    }
                }

                let mut mantissa = mantissa as u128;
                mantissa <<= (127 - $mantissa_width);
                mantissa |= 1 << 127;
                DecomposeResult::Normal { is_neg, exp, mantissa }
            }
        }

        /// It implements `From<DecomposeResult>`, not `TryFrom<DecomposeResult>`.
        /// If `DecomposeResult` is too big, it returns INFINITY or NEG_INFINITY instead
        /// of throwing an overflow error. If it's too small, it returns 0.0 or -0.0 instead
        /// of throwing an underflow error.
        impl From<DecomposeResult> for $fty {
            fn from(r: DecomposeResult) -> $fty {
                match r {
                    DecomposeResult::Normal { is_neg, exp, mantissa } => if 1 << ($exponent_width - 1) <= exp && is_neg {
                        <$fty>::NEG_INFINITY
                    } else if 1 << ($exponent_width - 1) <= exp {
                        <$fty>::INFINITY
                    } else if exp <= -((1 << ($exponent_width - 1)) + $mantissa_width - 1) && is_neg {
                        -0.0
                    } else if exp <= -((1 << ($exponent_width - 1)) + $mantissa_width - 1) {
                        0.0
                    } else if -((1 << ($exponent_width - 1)) + $mantissa_width - 2) <= exp && exp <= -((1 << ($exponent_width - 1)) - 1) {
                        let is_neg = (is_neg as $ity) << ($exponent_width + $mantissa_width);
                        let mut mantissa = (mantissa >> (127 - $mantissa_width)) as $ity;
                        let to_shift = -(((1 << ($exponent_width - 1)) - 1) - 1) - exp;
                        mantissa >>= to_shift;
                        <$fty>::from_bits(is_neg | mantissa)
                    } else {
                        let is_neg = (is_neg as $ity) << ($exponent_width + $mantissa_width);
                        let exp = ((exp + ((1 << ($exponent_width - 1)) - 1)) as $ity) << $mantissa_width;
                        let mantissa = ((mantissa >> (127 - $mantissa_width)) & ((1 << $mantissa_width) - 1)) as $ity;
                        <$fty>::from_bits(is_neg | exp | mantissa)
                    },
                    DecomposeResult::Zero => 0.0,
                    DecomposeResult::NegZero => -0.0,
                    DecomposeResult::Infinity => <$fty>::INFINITY,
                    DecomposeResult::NegInfinity => <$fty>::NEG_INFINITY,
                    DecomposeResult::NotANumber => <$fty>::NAN,
                }
            }
        }
    };
}

#[cfg(feature = "f16")]
ieee754!(f16, u16, 5, 10);

ieee754!(f32, u32, 8, 23);
ieee754!(f64, u64, 11, 52);

#[cfg(feature = "f128")]
ieee754!(f128, u128, 15, 112);
