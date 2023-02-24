//! A shorthand way of writing an if-else statement using C-like ternary syntax.
//!
//! This is rewrite of [`alexschrod/conditional`](https://github.com/alexschrod/conditional) to make the library compile.
//!
//! # Goals
//! There are two goals of the project:
//!  1. Report better errors than 'unexpected end of input, expected a token
//!     tree'
//!  2. Make the macro recursive itself. It already allows nested macros,
//!     but I would like for this functionality to be in place without having
//!     to use more than one macro
//!
//! # Examples
//! ### Normal way
//! ```rust
//! let foo = 111;
//! let bar = 113;
//!
//! let res = if bar > foo {
//!     "bar is greater"
//! } else {
//!     "bar is lesser"
//! };
//! ```
//!
//! ### Ternary way
//! ```rust
//! let foo = 111;
//! let bar = 113;
//!
//! let res = tern::t!(bar > foo ? "bar is greater" : "bar is lesser");
//! ```

// `TokenTree` from `proc_macro` doesn't allow parsing into
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    Expr,
    Token,
};

// Recursive could possibly be implemented with something like
// Vec<(Punctuated<Expr, Token![?]>, Punctuated<Expr, Token[:]>)>

/// The conditional statement
struct Ternary {
    condition: Expr,
    if_true:   Expr,
    if_false:  Expr,
}

// This implementation needs to be refactored in a way in which doesn't use the
// `while` loop if possible. When using `nom`, I've often used `take_while` or
// `take_until`, but I'm unsure if `syn` has been implemented differently to
// extract an expression by simply using `Expr::parse(input)?`.
//
// I've been unsuccessful in getting that to work

impl Parse for Ternary {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the conditional expression
        let condition = (|input: ParseStream| -> Result<Expr> {
            let mut tokens = vec![];

            // TODO: Report error `expected a '?'` here

            while !input.peek(Token![?]) || (input.peek(Token![?]) && input.peek2(Token![?])) {
                tokens.push(TokenTree::parse(input)?);
            }

            syn::parse2(tokens.into_iter().collect())
        })(input)?;

        // Discard the question mark operator
        input.parse::<Token![?]>()?;

        // Parse the `if_true` branch
        let if_true = (|input: ParseStream| -> Result<Expr> {
            let mut tokens = vec![];

            // TODO: Report error `expected a ':'` here

            // Statement after `||` is for paths
            while !input.peek(Token![:]) || input.peek(Token![::]) {
                tokens.push(TokenTree::parse(input)?);
            }

            syn::parse2(tokens.into_iter().collect())
        })(input)?;

        // Discard the colon operator
        input.parse::<Token![:]>()?;

        // Parse the `if_false` branch
        let if_false = Expr::parse(input)?;

        Ok(Self { condition, if_true, if_false })
    }
}

/// The ternary operator procedural macro. The name is short to allow for better
/// reading of nested expressions.
///
/// # Examples
/// **An example which needs to be surrounded by parentheses**
/// ```rust
/// let v = vec![1, 3, 5, 7];
/// let res = t!((*v.get(0).context("no first")?) == 1 ? "equals 1" : "not 1");
///
/// assert_eq!(res, "equals 1");
/// ```
///
/// **A nested example**
/// ```rust
/// let a = 40;
/// let b = 30;
/// let c = 20;
///
/// let res = t!(b > a ? b : t!(c > b ? c : a));
/// assert_eq!(res, a);
/// ```
#[proc_macro]
pub fn t(input: TokenStream) -> TokenStream {
    let Ternary { condition, if_true, if_false } = parse_macro_input!(input as Ternary);

    let tokens = quote! {
        if #condition {
            #if_true
        } else {
            #if_false
        }
    };

    TokenStream::from(tokens)
}
