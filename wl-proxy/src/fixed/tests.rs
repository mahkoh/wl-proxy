use crate::fixed::Fixed;

#[test]
fn test_binops() {
    let cases = [0.5, 1.0, 1.5, 2.0, 2.5];
    for lf in cases {
        for rf in cases {
            for (ml, mr) in [(-1.0, -1.0), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)] {
                let lf = ml * lf;
                let rf = mr * rf;

                let l = Fixed::from_f64_lossy(lf);
                let r = Fixed::from_f64_lossy(rf);

                macro_rules! cmp {
                    ($op:tt, $assign:tt) => {
                        assert_eq!(l $op r, Fixed::from_f64_lossy(lf $op rf));
                        let mut assign = l;
                        assign $assign r;
                        assert_eq!(assign, l $op r);
                    };
                }
                cmp!(+, +=);
                cmp!(-, -=);
                cmp!(*, *=);
                cmp!(/, /=);
                cmp!(%, %=);

                let li = (lf * 256.0) as i32;
                let ri = (rf * 256.0) as i32;

                macro_rules! cmp {
                    ($op:tt, $assign:tt) => {
                        assert_eq!(l $op r, Fixed::from_f64_lossy((li $op ri) as f64 / 256.0));
                        let mut assign = l;
                        assign $assign r;
                        assert_eq!(assign, l $op r);
                    };
                }
                cmp!(|, |=);
                cmp!(&, &=);
                cmp!(^, ^=);
            }
        }
    }
}

#[test]
fn test_shiftops() {
    let cases = [1.0, 2.0, 1.5, 0.5, 0.0];
    for ff in cases {
        for m in [1.0, -1.0] {
            let ff = ff * m;
            let i = (ff * 256.0) as i32;
            let f = Fixed::from_f64_lossy(ff);

            for s in 0..5 {
                macro_rules! cmp {
                ($op:tt, $assign:tt) => {
                    assert_eq!(f $op s, Fixed::from_f64_lossy((i $op s) as f64 / 256.0));
                    assert_eq!((&f) $op s, Fixed::from_f64_lossy((i $op s) as f64 / 256.0));
                    let mut assign = f;
                    assign $assign s;
                    assert_eq!(assign, f $op s);
                };
            }
                cmp!(<<, <<=);
                cmp!(>>, >>=);
            }
        }
    }
}

#[test]
fn test_unops() {
    let cases = [1.0, 2.0, 1.5, 0.5, 0.0];
    for ff in cases {
        for m in [1.0, -1.0] {
            let ff = ff * m;
            let i = (ff * 256.0) as i32;
            let f = Fixed::from_f64_lossy(ff);

            macro_rules! cmp {
                ($op:tt) => {
                    assert_eq!($op f, Fixed::from_f64_lossy(($op i) as f64 / 256.0));
                };
            }
            cmp!(!);
            cmp!(-);
        }
    }
}

#[test]
fn minima() {
    assert!(Fixed::EPSILON > Fixed::ZERO);
    assert!(Fixed::NEGATIVE_EPSILON < Fixed::ZERO);
    assert_eq!(-Fixed::EPSILON, Fixed::NEGATIVE_EPSILON);
    assert_eq!(Fixed::EPSILON / Fixed::TWO, Fixed::ZERO);
    assert_eq!(Fixed::NEGATIVE_EPSILON / Fixed::TWO, Fixed::ZERO);
}

#[test]
#[should_panic]
fn maximum() {
    let res = Fixed::MAX + Fixed::EPSILON;
    if res == Fixed::MIN {
        panic!();
    }
}

#[test]
#[should_panic]
fn minimum() {
    let res = Fixed::MIN - Fixed::EPSILON;
    if res == Fixed::MAX {
        panic!();
    }
}

#[test]
fn one_two() {
    assert_eq!(Fixed::ONE.to_f64(), 1.0);
    assert_eq!(Fixed::TWO.to_f64(), 2.0);
    assert_eq!(f64::from(Fixed::ONE), 1.0);
    assert_eq!(f64::from(Fixed::TWO), 2.0);
}

#[test]
fn display() {
    assert_eq!(Fixed::ONE.to_string(), "1");
    assert_eq!(Fixed::TWO.to_string(), "2");
    assert_eq!(Fixed::from_f32_lossy(1.5).to_string(), "1.5");
}

#[test]
fn debug() {
    assert_eq!(format!("{:?}", Fixed::ONE), "1.0");
    assert_eq!(format!("{:?}", Fixed::TWO), "2.0");
    assert_eq!(format!("{:?}", Fixed::from_f32_lossy(1.5)), "1.5");
}

#[test]
fn wire() {
    assert_eq!(Fixed::ONE.to_wire(), 256);
    assert_eq!(Fixed::ONE, Fixed::from_wire(256));
}

#[test]
fn to_f32() {
    assert_eq!(Fixed::ONE.to_f32_lossy(), 1.0);
    assert_eq!(Fixed::TWO.to_f32_lossy(), 2.0);
}

#[test]
fn from_f32() {
    assert_eq!(Fixed::ONE, Fixed::from_f32_lossy(1.0));
    assert_eq!(Fixed::TWO, Fixed::from_f32_lossy(2.0));
}

#[test]
fn from_i32() {
    assert_eq!(Fixed::ONE, Fixed::from_i32_saturating(1));
    assert_eq!(Fixed::TWO, Fixed::from_i32_saturating(2));
    assert_eq!(
        Fixed::from_f64_lossy(8388608.0),
        Fixed::from_i32_saturating(1 << 23)
    );
    assert_eq!(
        Fixed::from_f64_lossy(-8388608.0),
        Fixed::from_i32_saturating(-(1 << 23))
    );
    assert_eq!(Fixed::MAX, Fixed::from_i32_saturating(i32::MAX));
    assert_eq!(Fixed::MIN, Fixed::from_i32_saturating(i32::MIN));
}

#[test]
fn from_i64() {
    assert_eq!(Fixed::ONE, Fixed::from_i64_saturating(1));
    assert_eq!(Fixed::TWO, Fixed::from_i64_saturating(2));
    assert_eq!(
        Fixed::from_f64_lossy(8388608.0),
        Fixed::from_i64_saturating(1 << 23)
    );
    assert_eq!(
        Fixed::from_f64_lossy(-8388608.0),
        Fixed::from_i64_saturating(-(1 << 23))
    );
    assert_eq!(Fixed::MAX, Fixed::from_i64_saturating(i64::MAX));
    assert_eq!(Fixed::MIN, Fixed::from_i64_saturating(i64::MIN));
}

#[test]
fn to_i32_nearest() {
    let half = Fixed::ONE / Fixed::TWO;
    assert_eq!(half.to_i32_round_towards_nearest(), 1);
    assert_eq!((half + Fixed::EPSILON).to_i32_round_towards_nearest(), 1);
    assert_eq!((half - Fixed::EPSILON).to_i32_round_towards_nearest(), 0);
    assert_eq!(-half.to_i32_round_towards_nearest(), -1);
    assert_eq!((-half + Fixed::EPSILON).to_i32_round_towards_nearest(), 0);
    assert_eq!((-half - Fixed::EPSILON).to_i32_round_towards_nearest(), -1);
}

#[test]
fn to_i32_zero() {
    let one = Fixed::ONE;
    assert_eq!(one.to_i32_round_towards_zero(), 1);
    assert_eq!((one + Fixed::EPSILON).to_i32_round_towards_zero(), 1);
    assert_eq!((one - Fixed::EPSILON).to_i32_round_towards_zero(), 0);
    assert_eq!(-one.to_i32_round_towards_zero(), -1);
    assert_eq!((-one + Fixed::EPSILON).to_i32_round_towards_zero(), 0);
    assert_eq!((-one - Fixed::EPSILON).to_i32_round_towards_zero(), -1);
}

#[test]
fn to_i32_floor() {
    let one = Fixed::ONE;
    assert_eq!(one.to_i32_floor(), 1);
    assert_eq!((one + Fixed::EPSILON).to_i32_floor(), 1);
    assert_eq!((one - Fixed::EPSILON).to_i32_floor(), 0);
    assert_eq!(-one.to_i32_floor(), -1);
    assert_eq!((-one + Fixed::EPSILON).to_i32_floor(), -1);
    assert_eq!((-one - Fixed::EPSILON).to_i32_floor(), -2);
}

#[test]
fn to_i32_ceil() {
    let one = Fixed::ONE;
    assert_eq!(one.to_i32_ceil(), 1);
    assert_eq!((one + Fixed::EPSILON).to_i32_ceil(), 2);
    assert_eq!((one - Fixed::EPSILON).to_i32_ceil(), 1);
    assert_eq!(-one.to_i32_ceil(), -1);
    assert_eq!((-one + Fixed::EPSILON).to_i32_ceil(), 0);
    assert_eq!((-one - Fixed::EPSILON).to_i32_ceil(), -1);
}

#[test]
fn from_ints() {
    assert_eq!(Fixed::from(2i8), Fixed::TWO);
    assert_eq!(Fixed::from(2u8), Fixed::TWO);
    assert_eq!(Fixed::from(2i16), Fixed::TWO);
    assert_eq!(Fixed::from(2u16), Fixed::TWO);
}
