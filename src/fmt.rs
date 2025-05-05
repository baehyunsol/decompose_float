use crate::DecomposeResult;
use std::fmt;

impl fmt::Display for DecomposeResult {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let s = match self {
            DecomposeResult::Normal { is_neg, exp, mantissa } => format!(
                "{}{}2^{}",
                if *is_neg { "-" } else { "" },
                if *mantissa == 1 << 127 {
                    String::new()
                } else {
                    let mut m = mantissa >> 96;
                    m &= 0x7fff_ffff;  // 0 ~ 2_147_483_647
                    m *= 1_000_000_000;
                    m /= 0x8000_0000;  // 0 ~ 999_999_999
                    let rem = m % 10;
                    m /= 10;

                    // TODO: There's a small edge case
                    // if rem > 4 && m == 99_999_999, it has to add `exp` by 1, but it doesn't
                    if rem > 4 && m != 99_999_999 {
                        m += 1;
                    }

                    let mut m = format!("{m:08}");

                    while m.len() > 0 && m.chars().last().unwrap() == '0' {
                        m = m.get(..(m.len() - 1)).unwrap().to_string();
                    }

                    if m.is_empty() {
                        String::new()
                    }

                    else {
                        format!("1.{m}*")
                    }
                },
                if *exp >= 0 { exp.to_string() } else { format!("({exp})") },
            ),
            DecomposeResult::Zero => String::from("0"),
            DecomposeResult::NegZero => String::from("-0"),
            DecomposeResult::Infinity => String::from("inf"),
            DecomposeResult::NegInfinity => String::from("-inf"),
            DecomposeResult::NotANumber => String::from("NaN"),
        };

        write!(formatter, "{s}")
    }
}
