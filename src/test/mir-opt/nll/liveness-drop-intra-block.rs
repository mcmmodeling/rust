// Copyright 2012-2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags:-Znll

#![allow(warnings)]

fn use_x(_: usize) -> bool { true }

fn main() {
    let mut x = 22;
    loop {
        // Key point: `x` not live on entry to this basic block.
        x = 55;
        if use_x(x) { break; }
    }
}

// END RUST SOURCE
// START rustc.main.nll.0.mir
//    | Live variables on entry to bb1: []
//    bb1: {
//            | Live variables at bb1[0]: []
//        _1 = const 55usize;
//            | Live variables at bb1[1]: [_1]
//        StorageLive(_3);
//            | Live variables at bb1[2]: [_1]
//        StorageLive(_4);
//            | Live variables at bb1[3]: [_1]
//        _4 = _1;
//            | Live variables at bb1[4]: [_4]
//        _3 = const use_x(move _4) -> bb2;
//    }
// END rustc.main.nll.0.mir
