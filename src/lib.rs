/*
 * Copyright (c) 2022 Janosch Reppnow <janoschre+rust@gmail.com>.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use proc_macro::TokenStream;
use std::collections::VecDeque;

use syn::__private::{ToTokens, TokenStream2};
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{visit, visit_mut, Expr, Ident, ItemFn, Pat};

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
                                dbg!(mac);
                                let mut new_paths = Paths::default();
                                assert!(!mtch.arms.is_empty(), "Must have at least one branch in match branches with fork!()! {:?}", mtch.span());
                                for arm in &mtch.arms {
                                    if let Pat::Ident(ident) = &arm.pat {
                                        let mut this_paths = self.paths.clone();
                                        for path in &mut this_paths {
                                            path.push(ident.ident.to_string());
                                        }

                                        dbg!(&this_paths);

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
                                        ret = Some(Expr::clone(arm.body.as_ref()));
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
        }
        visit_mut::visit_expr_mut(self, expr);
    }
}

#[proc_macro_attribute]
pub fn crossroads(_: TokenStream, input: TokenStream) -> TokenStream {
    let function = syn::parse_macro_input!(input as ItemFn);

    let name = function.sig.ident.to_string();

    let mut paths = PathFinder::new(vec![vec![name]]);
    paths.visit_block(&function.block);

    let paths = dbg!(paths.into_inner());

    let mut new_functions: Vec<ItemFn> = Vec::with_capacity(paths.len());

    for path in paths {
        let mut path = VecDeque::from(path);
        path.pop_front();
        let mut function = function.clone();

        let mut new_name = function.sig.ident.to_string();
        for fork in &path {
            new_name.push('_'); // TODO: Replacement character and corresponding rules!
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
    // new_functions.drain().map(|function| function.into_token_stream()).
}
