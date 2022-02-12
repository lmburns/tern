use anyhow::{Context, Result};
use tern::t;

#[test]
fn normal() {
    let a = 20;
    let b = 30;
    let c = 40;

    let res = t!(a > b ? b : c);
    assert_eq!(res, c);

    let res = t!(a>b?b:c);
    assert_eq!(res, c);
}

#[test]
fn negated() {
    let a = 20;
    let b = 30;
    let c = 40;

    let res = t!(!(a > b) ? b : c);
    assert_eq!(res, b);
}

#[test]
fn nested() {
    let a = 40;
    let b = 30;
    let c = 20;

    let res = t!(b > a ? b : t!(c > b ? c : a));
    assert_eq!(res, a);

    let res = t!(b>a?b:t!(c>b?c:a));
    assert_eq!(res, a);

    let res = t!(
        b == a ? "b == a" :
     t!(b >  a ? "b > a"  :
     t!(c >  b ? "c > b"  :
                 "other"
    )));
    assert_eq!(res, "other");
}

#[test]
fn complex_type() -> Result<()> {
    let v = vec![1, 3, 5, 7];

    let res = t!((*v.get(0).context("no first")?) == 1 ? "equals 1" : "not 1");

    assert_eq!(res, "equals 1");

    Ok(())
}

#[test]
fn paths() -> Result<()> {
    let v = vec![10, 3, 5, 7];

    let res = t!((*v.get(0).context("no first")?) == inner::NUM ? "equals 10" : "not 10");
    assert_eq!(res, "equals 10");

    let res = t!(
        (*v.get(0).context("no first")? as inner::Test) == inner::NUM
        ? "equals 10"
        : "not 10"
    );
    assert_eq!(res, "equals 10");

    let res = t!(
        (*v.get(0).context("no first")?) == <inner::Test as inner::Trait1>::new()
        ? "equals 10"
        : "not 10"
    );
    assert_eq!(res, "equals 10");

    Ok(())
}

mod inner {
    pub(super) type Test = usize;
    pub(super) const NUM: usize = 10;

    pub(super) trait Trait1 {
        fn new() -> usize;
    }

    impl Trait1 for Test {
        fn new() -> usize {
            10
        }
    }
}
