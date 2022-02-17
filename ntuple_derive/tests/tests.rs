use ntuple::*;
use ntuple_derive::*;

#[derive(NTupleNewtype)]
struct Test(NTuple<f64, 3>);

#[test]
fn basic_test() {
    let n = ntuple!(0.0, 0.0, 0.0);
    let a = Test(n);
    let b = a.ntuple();
    assert_eq!(n, b);
}

#[test]
fn to_from() {
    let n = ntuple!(0.0, 0.0, 0.0);
    let a = Test::from(n);
    let b = NTuple::from(a);
    assert_eq!(n, b);
}

