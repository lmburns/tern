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
}

#[test]
fn complex_type() -> Result<()> {
    let v = vec![1, 3, 5, 7];

    let res = t!((*v.get(0).context("no first")?) == 1 ? "equals 1" : "not 1");

    assert_eq!(res, "equals 1");

    Ok(())
}
