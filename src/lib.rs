/*
 * Copyright (c) 2022 Janosch Reppnow <janoschre+rust@gmail.com>.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use proc_macro::TokenStream;
use std::collections::VecDeque;

use syn::{Arm, Block, Expr, ExprBlock, Ident, ItemFn, Pat, PathSegment, Stmt};
use syn::__private::{TokenStream2, ToTokens};

struct Tree<T> {
    item: T,
    children: Vec<Tree<T>>,
}

impl<T> Tree<T> {
    fn root(item: T) -> Self {
        Self {
            item,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, item: T) {
        self.children.push(Self::root(item))
    }

    fn leaf(&self) -> bool {
        self.children.is_empty()
    }


    fn paths(&self) -> VecDeque<VecDeque<T>> where T: Clone {
        if self.leaf() {
            VecDeque::from(vec!(VecDeque::from(vec!(self.item.clone()))))
        } else {
            let mut paths = vec![];

            for child in &self.children {
                for mut path in child.paths() {
                    path.push_front(self.item.clone());
                    paths.push(path);
                }
            }

            VecDeque::from(paths)
        }
    }
}


struct Leafs<'tree, Item> {
    tree: &'tree Tree<Item>,
    current_child_iter: Option<Box<Leafs<'tree, Item>>>,
}


#[proc_macro_attribute]
pub fn crossroads(args: TokenStream, input: TokenStream) -> TokenStream {
    let function = syn::parse_macro_input!(input as ItemFn);

    let name = function.sig.ident.to_string().clone();

    let mut forks = Tree::root(name);

    for stmt in &function.block.stmts {
        match stmt {
            Stmt::Local(_) => todo!(),
            Stmt::Expr(expr) | Stmt::Semi(expr, _) => {
                match expr {
                    // Expr::Array(_) => {}
                    // Expr::Assign(_) => {}
                    // Expr::AssignOp(_) => {}
                    // Expr::Async(_) => {}
                    // Expr::Await(_) => {}
                    // Expr::Binary(_) => {}
                    // Expr::Block(_) => {}
                    // Expr::Box(_) => {}
                    // Expr::Break(_) => {}
                    // Expr::Call(_) => {}
                    // Expr::Cast(_) => {}
                    // Expr::Closure(_) => {}
                    // Expr::Continue(_) => {}
                    // Expr::Field(_) => {}
                    // Expr::ForLoop(_) => {}
                    // Expr::Group(_) => {}
                    // Expr::If(_) => {}
                    // Expr::Index(_) => {}
                    // Expr::Let(_) => {}
                    // Expr::Lit(_) => {}
                    // Expr::Loop(_) => {}
                    // Expr::Macro(_) => {}
                    Expr::Match(mtch) => {
                        match mtch.expr.as_ref() {
                            Expr::Macro(mac) => {
                                match mac.mac.path.segments.first() {
                                    Some(segment) if segment.ident.to_string() == "fork" => {
                                        for arm in &mtch.arms {
                                            if let Pat::Ident(ident) = &arm.pat {
                                                // TODO: Check if valid?
                                                forks.add_child(ident.ident.to_string());
                                                // TODO: Recursion!
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                // TODO: Proper handling of namespace..
                                // match mac.mac.path.segments.iter().map(|segment| segment.ident.to_string()).collect::<Vec<String>>().as_ref::<[&str]>() {
                                //     ["fork"] | ["crossroads", "fork"] => {}
                                //     _ => {}
                                // }
                            }
                            _ => {}
                        }
                    }
                    // Expr::MethodCall(_) => {}
                    // Expr::Paren(_) => {}
                    // Expr::Path(_) => {}
                    // Expr::Range(_) => {}
                    // Expr::Reference(_) => {}
                    // Expr::Repeat(_) => {}
                    // Expr::Return(_) => {}
                    // Expr::Struct(_) => {}
                    // Expr::Try(_) => {}
                    // Expr::TryBlock(_) => {}
                    // Expr::Tuple(_) => {}
                    // Expr::Type(_) => {}
                    // Expr::Unary(_) => {}
                    // Expr::Unsafe(_) => {}
                    // Expr::Verbatim(_) => {}
                    // Expr::While(_) => {}
                    // Expr::Yield(_) => {}
                    // Expr::__NonExhaustive => {}
                    _ => {}
                }
            }
            _ => {}
        }
    }

    let mut new_functions: Vec<ItemFn> = vec!();

    for mut path in forks.paths() {
        path.pop_front();
        let mut function = function.clone();

        let mut new_name = function.sig.ident.to_string();
        for fork in &path {
            new_name.push_str("_"); // TODO: Replacement character and corresponding rules!
            new_name.push_str(&fork)
        }

        function.sig.ident = Ident::new(&new_name, function.sig.ident.span());

        for stmt in &mut function.block.stmts {
            match stmt {
                Stmt::Local(_) => todo!(),
                Stmt::Expr(expr) | Stmt::Semi(expr, _) => {
                    if let Some(mut replacement) = match expr {
                        // Expr::Array(_) => {}
                        // Expr::Assign(_) => {}
                        // Expr::AssignOp(_) => {}
                        // Expr::Async(_) => {}
                        // Expr::Await(_) => {}
                        // Expr::Binary(_) => {}
                        // Expr::Block(_) => {}
                        // Expr::Box(_) => {}
                        // Expr::Break(_) => {}
                        // Expr::Call(_) => {}
                        // Expr::Cast(_) => {}
                        // Expr::Closure(_) => {}
                        // Expr::Continue(_) => {}
                        // Expr::Field(_) => {}
                        // Expr::ForLoop(_) => {}
                        // Expr::Group(_) => {}
                        // Expr::If(_) => {}
                        // Expr::Index(_) => {}
                        // Expr::Let(_) => {}
                        // Expr::Lit(_) => {}
                        // Expr::Loop(_) => {}
                        // Expr::Macro(_) => {}
                        Expr::Match(mtch) => {
                            match mtch.expr.as_ref() {
                                Expr::Macro(mac) => {
                                    match mac.mac.path.segments.first() {
                                        Some(segment) if segment.ident.to_string() == "fork" => {
                                            let mut val = None;
                                            for arm in &mtch.arms {
                                                if let Pat::Ident(ident) = &arm.pat {
                                                    if ident.ident.to_string() == *path.front().unwrap() {
                                                        val = Some(Expr::Block(ExprBlock {
                                                            attrs: vec![], // TODO
                                                            label: None, // TODO
                                                            block: Block { brace_token: Default::default(), stmts: vec![Stmt::Expr((*arm.body).clone())] },
                                                        }));
                                                        break;
                                                    }
                                                }
                                            }
                                            val
                                        }
                                        _ => None
                                    }
                                    // TODO: Proper handling of namespace..
                                    // match mac.mac.path.segments.iter().map(|segment| segment.ident.to_string()).collect::<Vec<String>>().as_ref::<[&str]>() {
                                    //     ["fork"] | ["crossroads", "fork"] => {}
                                    //     _ => {}
                                    // }
                                }
                                _ => None
                            }
                        }
                        // Expr::MethodCall(_) => {}
                        // Expr::Paren(_) => {}
                        // Expr::Path(_) => {}
                        // Expr::Range(_) => {}
                        // Expr::Reference(_) => {}
                        // Expr::Repeat(_) => {}
                        // Expr::Return(_) => {}
                        // Expr::Struct(_) => {}
                        // Expr::Try(_) => {}
                        // Expr::TryBlock(_) => {}
                        // Expr::Tuple(_) => {}
                        // Expr::Type(_) => {}
                        // Expr::Unary(_) => {}
                        // Expr::Unsafe(_) => {}
                        // Expr::Verbatim(_) => {}
                        // Expr::While(_) => {}
                        // Expr::Yield(_) => {}
                        // Expr::__NonExhaustive => {}
                        _ => None
                    } {
                        std::mem::swap(&mut replacement, expr);
                    }
                }
                _ => {}
            }
            {}
        }
        new_functions.push(function);
    }

    let mut tokens = TokenStream2::new();
    for function in new_functions {
        function.to_tokens(&mut tokens);
    }
    tokens.into()
    // new_functions.drain().map(|function| function.into_token_stream()).
}