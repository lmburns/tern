# `tern`

A procedural macro for Rust that emulates a ternary expression with C/Perl like syntax.

Note: This is rewrite of [`alexschrod/conditional`](https://github.com/alexschrod/conditional) to make the library compile.

There isn't much more than that. The same thing can be achieved with the following macro:

```rust
macro_rules! ternary {
    ($cond:expr, $if_true:expr, $if_false:expr) => {
        if $cond {
            $if_true
        } else {
            $if_false
        }
    };
}
```

However, this crate was created to allow for the same syntax that is found in many other programming languages.

### Examples

A regular if-expression.
```rust
let foo = 111;
let bar = 113;

let res = if bar > foo {
    "bar is greater"
} else {
    "bar is lesser"
};

assert_eq!(res, "bar is greater");
```

Doing the same thing, except using the ternary operator with the `t!` macro.
 ```rust
let foo = 111;
let bar = 113;

let res = tern::t!(bar > foo ? "bar is greater" : "bar is lesser");
assert_eq!(res, "bar is greater");
 ```

A more complex example needs to be wrapped in parentheses.
```rust
let v = vec![1, 3, 5, 7];
let res = t!((*v.get(0).context("no first")?) == 1 ? "equals 1" : "not 1");

assert_eq!(res, "equals 1");
```

A nested example.
```rust
let a = 40;
let b = 30;
let c = 20;

let res = t!(b > a ? b : t!(c > b ? c : a));
assert_eq!(res, a);
```

## Goals
There are two goals of the project:
  1. Report better errors than `unexpected end of input, expected a token tree`.
  2. Make the macro recursive itself. It already allows nested macros,
     but I would like for this functionality to be in place without having
     to use more than one macro
