#![cfg(feature = "fuzz")]

// To assist in fuzz testing we leverage the `decimal` crate which
// consequently calls the C impl.
// Tests in this suite are random. Regressions that are found should be promoted
// to actual tests.

extern crate decimal;
extern crate rand;
extern crate rust_decimal;

use decimal::d128;
use rust_decimal::Decimal;
use std::str::FromStr;

const SIZE: usize = 100;

fn generate_valid() -> Vec<String> {
    let mut v = Vec::new();
    for _ in 0..SIZE {
        let i = rand::random::<u64>();
        let j = rand::random::<u64>();
        v.push(format!("{}.{}", i, j));
    }
    v
}

#[test]
fn fuzz_string() {
    let values = generate_valid();
    for v in values.iter() {
        let this = Decimal::from_str(v).unwrap();
        let that = d128::from_str(v).unwrap();
        assert_eq!(this.to_string(), that.to_string(), "Failed for {}", v);
    }
}

macro_rules! fuzz_op {
    ($name:ident, $op:tt) => {
        #[test]
        fn $name() {
            let values_a = generate_valid();
            let values_b = generate_valid();
            for i in 0..SIZE {
                let a = &values_a[i];
                let b = &values_b[i];
                let this_a = Decimal::from_str(a).unwrap();
                let this_b = Decimal::from_str(b).unwrap();
                let that_a = d128::from_str(a).unwrap();
                let that_b = d128::from_str(b).unwrap();
                let this = this_a $op this_b;
                let that = that_a $op that_b;
                assert_eq!(this.to_string(), that.to_string(), "Failed for {} {} {}", a, stringify!($op), b);
            }
        }
    }
}

fuzz_op!(fuzz_add, +);
fuzz_op!(fuzz_sub, -);
fuzz_op!(fuzz_mul, *);
fuzz_op!(fuzz_div, /);
