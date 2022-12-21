/*
 * Copyright (c) 2022 Janosch Reppnow <janoschre+rust@gmail.com>.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! A proc macro that turns one function into many - along user-defined forks points.
//!
//! # Motivation and Usage
//! This crate allows you to define multiple functions sharing significant part of their logic.
//! To understand why this is useful, consider the following example for a set of unit tests
//! (the ```#[test]``` attribute is only commented out to have these be picked up by doctest):
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! // #[test]
//! fn empty_be_default() {
//!     let map: HashMap<String, usize> = Default::default();
//!     
//!     assert!(map.is_empty());
//! }
//!
//! // #[test]
//! fn empty_after_clear() {
//!     let mut map: HashMap<String, usize> = Default::default();
//!
//!     map.insert("test".to_string(), 1);
//!     map.clear();
//!
//!     assert!(map.is_empty());
//! }
//!
//! // #[test]
//! fn empty_after_remove() {
//!     let mut map: HashMap<String, usize> = Default::default();
//!
//!     map.insert("test".to_string(), 1);
//!     map.remove("test");
//!
//!     assert!(map.is_empty());
//! }
//! ```
//!
//! With this crate, you can write the following instead:
//!
//! ```rust
//! use std::collections::HashMap;
//! use crossroads::crossroads;
//!
//! #[crossroads]
//! // #[test]
//! fn empty() {
//!     let mut map: HashMap<String, usize> = Default::default();
//!
//!     match fork!() {
//!         by_default => {}
//!         after_add => {
//!             map.insert("Key".to_owned(), 1337);
//!             match fork!() {
//!                 and_remove => map.remove("Key"),
//!                 and_clear => map.clear(),
//!             };
//!         }
//!     }
//!
//!     assert!(map.is_empty());
//! }
//! ```
//!
//! The ```#[crossroads]``` macro will replace the function with as many functions as there are distinct paths through your fork points.
//! In this case, it will generate:
//! ```rust
//! // #[test]
//! fn empty_by_default() { /* ... */ }
//! // #[test]
//! fn empty_after_add_and_remove() { /* ... */ }
//! // #[test]
//! fn empty_after_add_and_clear() { /* ... */ }
//! ```
//!
//! The contents of the methods are the result of replacing the ```match``` expressions with a
//! block containing the expression specified in the corresponding ```match``` arms.
//!
//! You can find the above example in the ```examples``` folder and confirm that it will indeed produce the following output when run as a test:
//! ```text
//! running 3 tests
//! test empty_by_default ... ok
//! test empty_after_add_and_clear ... ok
//! test empty_after_add_and_remove ... ok
//! ```
//!
//! # Questions and Answers
//!
//! 1. Why did you decide to use the ```match```-based syntax and not implement a new one?
//! The main reason for using the ```match``` syntax in the way this crate does is to make it as
//! compatible as possible with code formattting tools such as ```rustfmt```.
//! See the ```select!``` macros used in the async context for an example of issues a new syntax can cause.

use proc_macro::TokenStream;
use std::collections::VecDeque;

use syn::__private::{ToTokens, TokenStream2};
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{visit, visit_mut, Block, Expr, ExprBlock, Ident, ItemFn, Pat, Stmt};

type Paths<T> = Vec<Vec<T>>;

struct PathFinder {
    paths: Paths<String>,
}

impl PathFinder {
    fn new(paths: Paths<String>) -> Self {
        Self { paths }
    }

    fn into_inner(self) -> Paths<String> {
        self.paths
    }
}

impl<'ast> Visit<'ast> for PathFinder {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        if match expr {
            Expr::Match(mtch) => {
                match mtch.expr.as_ref() {
                    Expr::Macro(mac) => {
                        match mac.mac.path.segments.first() {
                            Some(segment) if segment.ident == "fork" => {
                                let mut new_paths = Paths::default();
                                assert!(!mtch.arms.is_empty(), "Must have at least one branch in match branches with fork!()! {:?}", mtch.span());
                                for arm in &mtch.arms {
                                    if let Pat::Ident(ident) = &arm.pat {
                                        let mut this_paths = self.paths.clone();
                                        for path in &mut this_paths {
                                            path.push(ident.ident.to_string());
                                        }

                                        let mut this_pathfinder = PathFinder::new(this_paths);
                                        this_pathfinder.visit_expr(arm.body.as_ref());

                                        new_paths.append(&mut this_pathfinder.into_inner());
                                    } else {
                                        panic!(
                                            "Must use only idents with a fork!() match! {:?}",
                                            arm.span()
                                        );
                                    }
                                }

                                self.paths = new_paths;
                                false
                            }
                            _ => true,
                        }
                        // TODO: Proper handling of namespace..
                        // match mac.mac.path.segments.iter().map(|segment| segment.ident.to_string()).collect::<Vec<String>>().as_ref::<[&str]>() {
                        //     ["fork"] | ["crossroads", "fork"] => {}
                        //     _ => {}
                        // }
                    }
                    _ => true,
                }
            }
            _ => true,
        } {
            visit::visit_expr(self, expr);
        }
    }
}

struct Rewriter {
    along_path: VecDeque<String>,
}

impl Rewriter {
    fn new(path: impl Into<VecDeque<String>>) -> Self {
        Self {
            along_path: path.into(),
        }
    }
}

impl VisitMut for Rewriter {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Some(mut replacement) = if let Expr::Match(mtch) = &expr {
            match mtch.expr.as_ref() {
                Expr::Macro(mac) => {
                    match mac.mac.path.segments.first() {
                        Some(segment) if segment.ident == "fork" => {
                            let current = self
                                .along_path
                                .pop_front()
                                .expect("There should always be enough identifiers in this list.");
                            assert!(!mtch.arms.is_empty(), "Must have at least one branch in match branches with fork!()! {:?}", mtch.span());

                            let mut ret = None;

                            for arm in &mtch.arms {
                                if let Pat::Ident(ident) = &arm.pat {
                                    if ident.ident == current {
                                        ret = Some(Expr::Block(ExprBlock {
                                            attrs: mtch.attrs.clone(),
                                            label: None,
                                            block: Block {
                                                brace_token: Default::default(),
                                                stmts: vec![Stmt::Expr(Expr::clone(
                                                    arm.body.as_ref(),
                                                ))],
                                            },
                                        }));
                                        break;
                                    }
                                } else {
                                    panic!(
                                        "Must use only idents with a fork!() match! {:?}",
                                        arm.span()
                                    );
                                }
                            }

                            if let Some(ret) = ret {
                                Some(ret)
                            } else {
                                panic!("Did not find identifier {} in corresponding match statement. This is almost certainly a bug, please feel free to report it. {:?}", current, mtch.span());
                            }
                        }
                        _ => None,
                    }
                    // TODO: Proper handling of namespace..
                    // match mac.mac.path.segments.iter().map(|segment| segment.ident.to_string()).collect::<Vec<String>>().as_ref::<[&str]>() {
                    //     ["fork"] | ["crossroads", "fork"] => {}
                    //     _ => {}
                    // }
                }
                _ => None,
            }
        } else {
            None
        } {
            std::mem::swap(expr, &mut replacement);
            // This is kind of mean: If the expression that we are putting in place of the match is itself another match,
            // it gets skipped here (as the recursive method assumes you have already visited the node that you give).
            // As such, we need to manually recurse in this specific case.
            self.visit_expr_mut(expr);
        } else {
            visit_mut::visit_expr_mut(self, expr);
        }
    }
}

/// An attribute macro that can be placed above ```FnItem```s, i.e. freestanding functions everywhere.
/// It will replace the function with a set of functions induced by the different paths through the
/// function along the ```match fork!() { a => { ... }, ... }``` points, where the name of the function is induced by the
/// sequence of the ```identifier``` specified in the patterns of the ```match``` branches used with the for that specific function instance.
///
/// See the crate-level documentation for a concrete example.
#[proc_macro_attribute]
pub fn crossroads(_: TokenStream, input: TokenStream) -> TokenStream {
    let function = syn::parse_macro_input!(input as ItemFn);

    let name = function.sig.ident.to_string();

    let mut paths = PathFinder::new(vec![vec![name]]);
    paths.visit_block(&function.block);

    let paths = paths.into_inner();

    let mut new_functions: Vec<ItemFn> = Vec::with_capacity(paths.len());

    for path in paths {
        let mut path = VecDeque::from(path);
        path.pop_front();
        let mut function = function.clone();

        let mut new_name = function.sig.ident.to_string();
        for fork in &path {
            new_name.push('_');
            new_name.push_str(fork)
        }

        function.sig.ident = Ident::new(&new_name, function.sig.ident.span());

        let mut rewriter = Rewriter::new(path);
        rewriter.visit_block_mut(&mut function.block);
        new_functions.push(function);
    }

    let mut tokens = TokenStream2::new();
    for function in new_functions {
        function.to_tokens(&mut tokens);
    }
    tokens.into()
}
