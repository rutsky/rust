// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-tidy-linelength

// Issue 23030: Detect overflowing discriminant
//
// Check that we detect the overflow even if enum is not used.

// See also run-pass/discrim-explicit-23030.rs where the suggested
// workaround is tested.

use std::{i8,u8,i16,u16,i32,u32,i64, u64};

fn f_i8() {
    #[repr(i8)]
    enum A {
        Ok = i8::MAX - 1,
        Ok2,
        OhNo, //~ ERROR enum discriminant overflowed on value after 127: i8; set explicitly via OhNo = -128 if that is desired outcome
    }
}

fn f_u8() {
    #[repr(u8)]
    enum A {
        Ok = u8::MAX - 1,
        Ok2,
        OhNo, //~ ERROR enum discriminant overflowed on value after 255: u8; set explicitly via OhNo = 0 if that is desired outcome
    }
}

fn f_i16() {
    #[repr(i16)]
    enum A {
        Ok = i16::MAX - 1,
        Ok2,
        OhNo, //~ ERROR enum discriminant overflowed
    }
}

fn f_u16() {
    #[repr(u16)]
    enum A {
        Ok = u16::MAX - 1,
        Ok2,
        OhNo, //~ ERROR enum discriminant overflowed
    }
}

fn f_i32() {
    #[repr(i32)]
    enum A {
        Ok = i32::MAX - 1,
        Ok2,
        OhNo, //~ ERROR enum discriminant overflowed
    }
}

fn f_u32() {
    #[repr(u32)]
    enum A {
        Ok = u32::MAX - 1,
        Ok2,
        OhNo, //~ ERROR enum discriminant overflowed
    }
}

fn f_i64() {
    #[repr(i64)]
    enum A {
        Ok = i64::MAX - 1,
        Ok2,
        OhNo, //~ ERROR enum discriminant overflowed
    }
}

fn f_u64() {
    #[repr(u64)]
    enum A {
        Ok = u64::MAX - 1,
        Ok2,
        OhNo, //~ ERROR enum discriminant overflowed
    }
}

fn main() { }
