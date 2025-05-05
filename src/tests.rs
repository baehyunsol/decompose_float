use super::{Decompose, DecomposeResult};

#[test]
fn subnormal() {
    #[cfg(feature = "f16")]
    {
        let subnormal_f16s = vec![
            (f16::from_bits(1), DecomposeResult::Normal { is_neg: false, exp: -24, mantissa: 1 << 127 }),
            (f16::from_bits(2), DecomposeResult::Normal { is_neg: false, exp: -23, mantissa: 1 << 127 }),
            (f16::from_bits(3), DecomposeResult::Normal { is_neg: false, exp: -23, mantissa: 3 << 126 }),
            (f16::from_bits(4), DecomposeResult::Normal { is_neg: false, exp: -22, mantissa: 1 << 127 }),
            (f16::from_bits(5), DecomposeResult::Normal { is_neg: false, exp: -22, mantissa: 5 << 125 }),
            (f16::from_bits(6), DecomposeResult::Normal { is_neg: false, exp: -22, mantissa: 3 << 126 }),
            (f16::from_bits(0x03ff), DecomposeResult::Normal { is_neg: false, exp: -15, mantissa: u128::MAX - (1 << 118) + 1 }),
        ];

        for (n, d) in subnormal_f16s {
            let dd = n.decompose();
            assert_eq!(d, dd);

            let nn = f16::from(dd);
            assert_eq!(n, nn);

            let ddd = nn.decompose();
            assert_eq!(dd, ddd);
        }
    }

    let subnormal_f32s = vec![
        (f32::from_bits(1), DecomposeResult::Normal { is_neg: false, exp: -149, mantissa: 1 << 127 }),
        (f32::from_bits(2), DecomposeResult::Normal { is_neg: false, exp: -148, mantissa: 1 << 127 }),
        (f32::from_bits(3), DecomposeResult::Normal { is_neg: false, exp: -148, mantissa: 3 << 126 }),
        (f32::from_bits(4), DecomposeResult::Normal { is_neg: false, exp: -147, mantissa: 1 << 127 }),
        (f32::from_bits(5), DecomposeResult::Normal { is_neg: false, exp: -147, mantissa: 5 << 125 }),
        (f32::from_bits(6), DecomposeResult::Normal { is_neg: false, exp: -147, mantissa: 3 << 126 }),
        (f32::from_bits(0x007f_ffff), DecomposeResult::Normal { is_neg: false, exp: -127, mantissa: u128::MAX - (1 << 105) + 1 }),
    ];

    for (n, d) in subnormal_f32s {
        let dd = n.decompose();
        assert_eq!(d, dd);

        let nn = f32::from(dd);
        assert_eq!(n, nn);

        let ddd = nn.decompose();
        assert_eq!(dd, ddd);

        // f16 underflow -> becomes zero, preserving sign
        #[cfg(feature = "f16")]
        {
            let md = (-n).decompose();
            let n_f16 = f16::from(d);
            let mn_f16 = f16::from(md);
            let dd_f16 = n_f16.decompose();
            let mdd_f16 = mn_f16.decompose();
            assert_eq!(dd_f16, DecomposeResult::Zero);
            assert_eq!(mdd_f16, DecomposeResult::NegZero);
        }
    }

    let subnormal_f64s = vec![
        (f64::from_bits(1), DecomposeResult::Normal { is_neg: false, exp: -1074, mantissa: 1 << 127 }),
        (f64::from_bits(2), DecomposeResult::Normal { is_neg: false, exp: -1073, mantissa: 1 << 127 }),
        (f64::from_bits(3), DecomposeResult::Normal { is_neg: false, exp: -1073, mantissa: 3 << 126 }),
        (f64::from_bits(4), DecomposeResult::Normal { is_neg: false, exp: -1072, mantissa: 1 << 127 }),
        (f64::from_bits(5), DecomposeResult::Normal { is_neg: false, exp: -1072, mantissa: 5 << 125 }),
        (f64::from_bits(6), DecomposeResult::Normal { is_neg: false, exp: -1072, mantissa: 3 << 126 }),
        (f64::from_bits(0x000f_ffff_ffff_ffff), DecomposeResult::Normal { is_neg: false, exp: -1023, mantissa: u128::MAX - (1 << 76) + 1 }),
    ];

    for (n, d) in subnormal_f64s {
        let dd = n.decompose();
        assert_eq!(d, dd);

        let nn = f64::from(dd);
        assert_eq!(n, nn);

        let ddd = nn.decompose();
        assert_eq!(dd, ddd);

        // f32 underflow -> becomes zero, perserving sign
        let md = (-n).decompose();
        let n_f32 = f32::from(d);
        let mn_f32 = f32::from(md);
        let dd_f32 = n_f32.decompose();
        let mdd_f32 = mn_f32.decompose();
        assert_eq!(dd_f32, DecomposeResult::Zero);
        assert_eq!(mdd_f32, DecomposeResult::NegZero);
    }

    #[cfg(feature = "f128")]
    {
        let subnormal_f128s = vec![
            (f128::from_bits(1), DecomposeResult::Normal { is_neg: false, exp: -16494, mantissa: 1 << 127 }),
            (f128::from_bits(2), DecomposeResult::Normal { is_neg: false, exp: -16493, mantissa: 1 << 127 }),
            (f128::from_bits(3), DecomposeResult::Normal { is_neg: false, exp: -16493, mantissa: 3 << 126 }),
            (f128::from_bits(4), DecomposeResult::Normal { is_neg: false, exp: -16492, mantissa: 1 << 127 }),
            (f128::from_bits(5), DecomposeResult::Normal { is_neg: false, exp: -16492, mantissa: 5 << 125 }),
            (f128::from_bits(6), DecomposeResult::Normal { is_neg: false, exp: -16492, mantissa: 3 << 126 }),
            (f128::from_bits(0xffff_ffff_ffff_ffff_ffff_ffff_ffff), DecomposeResult::Normal { is_neg: false, exp: -16383, mantissa: u128::MAX - (1 << 16) + 1 }),
        ];

        for (n, d) in subnormal_f128s {
            let dd = n.decompose();
            assert_eq!(d, dd);

            let nn = f128::from(dd);
            assert_eq!(n, nn);

            let ddd = nn.decompose();
            assert_eq!(dd, ddd);

            // f64 underflow -> becomes zero, perserving sign
            let md = (-n).decompose();
            let n_f64 = f64::from(d);
            let mn_f64 = f64::from(md);
            let dd_f64 = n_f64.decompose();
            let mdd_f64 = mn_f64.decompose();
            assert_eq!(dd_f64, DecomposeResult::Zero);
            assert_eq!(mdd_f64, DecomposeResult::NegZero);
        }
    }
}

#[test]
fn decompose_and_compose() {
    #[cfg(feature = "f16")]
    {
        let f16s = vec![
            1.0, 0.0, -1.0, -0.0,
            15.0, 17.0, 31.0, 33.0,
            0.171875, -0.171875,
            1e2, 1e4, 1.5e4,
            -1e2, -1e4, -1.5e4,
            f16::from_bits(1),
            f16::from_bits(3),
            f16::from_bits(9),
            f16::from_bits(27),
            3e-6, 4e-7, 5e-8,
        ];

        for n in f16s {
            let d = n.decompose();
            let nn = f16::from(d);
            let dd = nn.decompose();
            let nnn = f16::from(dd);

            assert_eq!(d, dd);
            assert_eq!(nn, nnn);

            // an f32 value can represent ALL f16 values losslessly.
            let n_f32 = n as f32;
            let d_f32 = n_f32.decompose();
            let nn_f32 = f32::from(d_f32);
            let dd_f32 = nn_f32.decompose();
            let nnn_f32 = f32::from(dd_f32);
            assert_eq!(d_f32, dd_f32);
            assert_eq!(dd, dd_f32);
            assert_eq!(nn_f32, nnn_f32);
        }
    }

    let f32s = vec![
        1.0, 0.0, -1.0, -0.0,
        15.0, 17.0, 31.0, 33.0,
        0.171875, -0.171875,
        1e20, 1e30, 1.5e30,
        -1e20, -1e30, -1.5e30,
        f32::from_bits(1),
        f32::from_bits(3),
        f32::from_bits(9),
        f32::from_bits(27),
        2e-40, 3e-41, 4e-42,
        5e-43, 6e-44, 7e-45,
    ];

    for n in f32s {
        let d = n.decompose();
        let nn = f32::from(d);
        let dd = nn.decompose();
        let nnn = f32::from(dd);

        assert_eq!(d, dd);
        assert_eq!(nn, nnn);

        // an f64 value can represent ALL f32 values losslessly.
        let n_f64 = n as f64;
        let d_f64 = n_f64.decompose();
        let nn_f64 = f64::from(d_f64);
        let dd_f64 = nn_f64.decompose();
        let nnn_f64 = f64::from(dd_f64);
        assert_eq!(d_f64, dd_f64);
        assert_eq!(dd, dd_f64);
        assert_eq!(nn_f64, nnn_f64);
    }

    let large_f64s = vec![
        1e39, 2e40, 3e41, 4e42,
        -5e43, -6e44, -7e45, -8e46,
        f64::MAX, -f64::MAX,
    ];

    for n in large_f64s {
        let d = n.decompose();
        let nn = f64::from(d);
        let dd = nn.decompose();
        let nnn = f64::from(dd);
        assert_eq!(d, dd);
        assert_eq!(nn.to_string(), nnn.to_string());

        let nn_f32 = f32::from(d);
        let dd_f32 = nn_f32.decompose();

        if n < 0.0 {
            assert_eq!(dd_f32, DecomposeResult::NegInfinity);
        }

        else {
            assert_eq!(dd_f32, DecomposeResult::Infinity);
        }
    }

    #[cfg(feature = "f128")]
    {
        let f128s = vec![
            1.0, 0.0, -1.0, -0.0,
            15.0, 17.0, 31.0, 33.0,
            0.171875, -0.171875,
            1e4910, 1e4920, 1.5e4930,
            -1e4910, -1e4920, -1.5e4930,
            f128::from_bits(1),
            f128::from_bits(3),
            f128::from_bits(9),
            f128::from_bits(27),
            2e-4961, 3e-4962, 4e-4963,
            5e-4964, 6e-4965, 7e-4966,
        ];

        for n in f128s {
            let d = n.decompose();
            let nn = f128::from(d);
            let dd = nn.decompose();
            let nnn = f128::from(dd);

            assert_eq!(d, dd);
            assert_eq!(nn, nnn);
        }
    }
}

#[test]
fn conversions() {
    #[cfg(feature = "f16")]
    {
        let ns = vec![
            (0.0f16, 0.0f32), (-0.0f16, -0.0f32),
            (1.0f16, 1.0f32), (-1.0f16, -1.0f32),
            (2.0f16, 2.0f32), (-2.0f16, -2.0f32),
            (3.0f16, 3.0f32), (-3.0f16, -3.0f32),
            (17.0f16, 17.0f32), (-17.0f16, -17.0f32),
            (1.75f16, 1.75f32), (-1.75f16, -1.75f32),
        ];

        for (n_f16, n_f32) in ns {
            let d_f16 = n_f16.decompose();
            let d_f32 = n_f32.decompose();
            let d_f16_c = (n_f32 as f16).decompose();
            let d_f32_c = (n_f16 as f32).decompose();

            assert_eq!(d_f16, d_f32);
            assert_eq!(d_f16, d_f16_c);
            assert_eq!(d_f16, d_f32_c);
        }
    }

    let ns = vec![
        (0.0f32, 0.0f64), (-0.0f32, -0.0f64),
        (1.0f32, 1.0f64), (-1.0f32, -1.0f64),
        (2.0f32, 2.0f64), (-2.0f32, -2.0f64),
        (3.0f32, 3.0f64), (-3.0f32, -3.0f64),
        (17.0f32, 17.0f64), (-17.0f32, -17.0f64),
        (1.75f32, 1.75f64), (-1.75f32, -1.75f64),
    ];

    for (n_f32, n_f64) in ns {
        let d_f32 = n_f32.decompose();
        let d_f64 = n_f64.decompose();
        let d_f32_c = (n_f64 as f32).decompose();
        let d_f64_c = (n_f32 as f64).decompose();

        assert_eq!(d_f32, d_f64);
        assert_eq!(d_f32, d_f32_c);
        assert_eq!(d_f32, d_f64_c);
    }

    #[cfg(feature = "f128")]
    {
        let ns = vec![
            (0.0f64, 0.0f128), (-0.0f64, -0.0f128),
            (1.0f64, 1.0f128), (-1.0f64, -1.0f128),
            (2.0f64, 2.0f128), (-2.0f64, -2.0f128),
            (3.0f64, 3.0f128), (-3.0f64, -3.0f128),
            (17.0f64, 17.0f128), (-17.0f64, -17.0f128),
            (1.75f64, 1.75f128), (-1.75f64, -1.75f128),
        ];

        for (n_f64, n_f128) in ns {
            let d_f64 = n_f64.decompose();
            let d_f128 = n_f128.decompose();
            let d_f64_c = (n_f128 as f64).decompose();
            let d_f128_c = (n_f64 as f128).decompose();

            assert_eq!(d_f64, d_f128);
            assert_eq!(d_f64, d_f64_c);
            assert_eq!(d_f64, d_f128_c);
        }
    }
}

#[test]
fn special_values() {
    #[cfg(feature = "f16")]
    {
        let f16s = vec![
            (0.0, DecomposeResult::Zero),
            (-0.0, DecomposeResult::NegZero),
            (f16::INFINITY, DecomposeResult::Infinity),
            (f16::NEG_INFINITY, DecomposeResult::NegInfinity),
            (f16::NAN, DecomposeResult::NotANumber),
        ];

        for (n, d) in f16s {
            let dd = n.decompose();
            let nn = f16::from(dd);
            let ddd = nn.decompose();
            assert_eq!(d, dd);
            assert_eq!(dd, ddd);
        }
    }

    let f32s = vec![
        (0.0, DecomposeResult::Zero),
        (-0.0, DecomposeResult::NegZero),
        (f32::INFINITY, DecomposeResult::Infinity),
        (f32::NEG_INFINITY, DecomposeResult::NegInfinity),
        (f32::NAN, DecomposeResult::NotANumber),
    ];

    for (n, d) in f32s {
        let dd = n.decompose();
        let nn = f32::from(dd);
        let ddd = nn.decompose();
        assert_eq!(d, dd);
        assert_eq!(dd, ddd);
    }

    let f64s = vec![
        (0.0, DecomposeResult::Zero),
        (-0.0, DecomposeResult::NegZero),
        (f64::INFINITY, DecomposeResult::Infinity),
        (f64::NEG_INFINITY, DecomposeResult::NegInfinity),
        (f64::NAN, DecomposeResult::NotANumber),
    ];

    for (n, d) in f64s {
        let dd = n.decompose();
        let nn = f64::from(dd);
        let ddd = nn.decompose();
        assert_eq!(d, dd);
        assert_eq!(dd, ddd);
    }

    #[cfg(feature = "f128")]
    {
        let f128s = vec![
            (0.0, DecomposeResult::Zero),
            (-0.0, DecomposeResult::NegZero),
            (f128::INFINITY, DecomposeResult::Infinity),
            (f128::NEG_INFINITY, DecomposeResult::NegInfinity),
            (f128::NAN, DecomposeResult::NotANumber),
        ];

        for (n, d) in f128s {
            let dd = n.decompose();
            let nn = f128::from(dd);
            let ddd = nn.decompose();
            assert_eq!(d, dd);
            assert_eq!(dd, ddd);
        }
    }
}

#[test]
fn three() {
    #[cfg(feature = "f16")]
    assert_eq!(
        3.0f16,
        DecomposeResult::Normal { is_neg: false, exp: 1, mantissa: (1 << 127) + (1 << 126) }.into(),
    );

    assert_eq!(
        3.0f32,
        DecomposeResult::Normal { is_neg: false, exp: 1, mantissa: (1 << 127) + (1 << 126) }.into(),
    );
    assert_eq!(
        3.0f64,
        DecomposeResult::Normal { is_neg: false, exp: 1, mantissa: (1 << 127) + (1 << 126) }.into(),
    );

    #[cfg(feature = "f128")]
    assert_eq!(
        3.0f128,
        DecomposeResult::Normal { is_neg: false, exp: 1, mantissa: (1 << 127) + (1 << 126) }.into(),
    );
}

#[test]
fn format() {
    assert_eq!(
        3.0f32.decompose().to_string(),
        "1.5*2^1",
    );
    assert_eq!(
        5.0f32.decompose().to_string(),
        "1.25*2^2",
    );
    assert_eq!(
        (-5.0f32).decompose().to_string(),
        "-1.25*2^2",
    );
    assert_eq!(
        1.0f32.decompose().to_string(),
        "2^0",
    );
    assert_eq!(
        16384.0f32.decompose().to_string(),
        "2^14",
    );
    assert_eq!(
        2.15625f32.decompose().to_string(),
        "1.078125*2^1",
    );
    assert_eq!(
        1.0009765625f32.decompose().to_string(),
        "1.00097656*2^0",
    );
    assert_eq!(
        1.1f64.decompose().to_string(),
        "1.1*2^0",
    );
    assert_eq!(
        0.3f64.decompose().to_string(),
        "1.2*2^(-2)",
    );
    assert_eq!(
        0.2f64.decompose().to_string(),
        "1.6*2^(-3)",
    );
    assert_eq!(
        0.1f64.decompose().to_string(),
        "1.6*2^(-4)",
    );
}
