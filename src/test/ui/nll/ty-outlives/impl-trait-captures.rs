// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags:-Znll -Zborrowck=mir -Zverbose

#![allow(warnings)]
#![feature(conservative_impl_trait)]

trait Foo<'a> {
}

impl<'a, T> Foo<'a> for T { }

fn foo<'a, T>(x: &T) -> impl Foo<'a> {
    x
        //~^ WARNING not reporting region error due to -Znll
        //~| ERROR explicit lifetime required in the type of `x` [E0621]
}

fn main() {}
