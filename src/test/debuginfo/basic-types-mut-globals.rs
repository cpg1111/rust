// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Caveats - gdb prints any 8-bit value (meaning rust I8 and u8 values)
// as its numerical value along with its associated ASCII char, there
// doesn't seem to be any way around this. Also, gdb doesn't know
// about UTF-32 character encoding and will print a rust char as only
// its numerical value.

// min-lldb-version: 310

// compile-flags:-g

// gdb-command:run

// Check initializers
// gdb-command:print 'basic_types_mut_globals::B'
// gdb-check:$1 = false
// gdb-command:print 'basic_types_mut_globals::I'
// gdb-check:$2 = -1
// gdb-command:print 'basic_types_mut_globals::C'
// gdb-check:$3 = 97
// gdb-command:print/d 'basic_types_mut_globals::I8'
// gdb-check:$4 = 68
// gdb-command:print 'basic_types_mut_globals::I16'
// gdb-check:$5 = -16
// gdb-command:print 'basic_types_mut_globals::I32'
// gdb-check:$6 = -32
// gdb-command:print 'basic_types_mut_globals::I64'
// gdb-check:$7 = -64
// gdb-command:print 'basic_types_mut_globals::U'
// gdb-check:$8 = 1
// gdb-command:print/d 'basic_types_mut_globals::U8'
// gdb-check:$9 = 100
// gdb-command:print 'basic_types_mut_globals::U16'
// gdb-check:$10 = 16
// gdb-command:print 'basic_types_mut_globals::U32'
// gdb-check:$11 = 32
// gdb-command:print 'basic_types_mut_globals::U64'
// gdb-check:$12 = 64
// gdb-command:print 'basic_types_mut_globals::F32'
// gdb-check:$13 = 2.5
// gdb-command:print 'basic_types_mut_globals::F64'
// gdb-check:$14 = 3.5
// gdb-command:continue

// Check new values
// gdb-command:print 'basic_types_mut_globals'::B
// gdb-check:$15 = true
// gdb-command:print 'basic_types_mut_globals'::I
// gdb-check:$16 = 2
// gdb-command:print 'basic_types_mut_globals'::C
// gdb-check:$17 = 102
// gdb-command:print/d 'basic_types_mut_globals'::I8
// gdb-check:$18 = 78
// gdb-command:print 'basic_types_mut_globals'::I16
// gdb-check:$19 = -26
// gdb-command:print 'basic_types_mut_globals'::I32
// gdb-check:$20 = -12
// gdb-command:print 'basic_types_mut_globals'::I64
// gdb-check:$21 = -54
// gdb-command:print 'basic_types_mut_globals'::U
// gdb-check:$22 = 5
// gdb-command:print/d 'basic_types_mut_globals'::U8
// gdb-check:$23 = 20
// gdb-command:print 'basic_types_mut_globals'::U16
// gdb-check:$24 = 32
// gdb-command:print 'basic_types_mut_globals'::U32
// gdb-check:$25 = 16
// gdb-command:print 'basic_types_mut_globals'::U64
// gdb-check:$26 = 128
// gdb-command:print 'basic_types_mut_globals'::F32
// gdb-check:$27 = 5.75
// gdb-command:print 'basic_types_mut_globals'::F64
// gdb-check:$28 = 9.25

#![allow(unused_variables)]
#![feature(omit_gdb_pretty_printer_section)]
#![omit_gdb_pretty_printer_section]

static mut B: bool = false;
static mut I: isize = -1;
static mut C: char = 'a';
static mut I8: i8 = 68;
static mut I16: i16 = -16;
static mut I32: i32 = -32;
static mut I64: i64 = -64;
static mut U: usize = 1;
static mut U8: u8 = 100;
static mut U16: u16 = 16;
static mut U32: u32 = 32;
static mut U64: u64 = 64;
static mut F32: f32 = 2.5;
static mut F64: f64 = 3.5;

fn main() {
    _zzz(); // #break

    unsafe {
        B = true;
        I = 2;
        C = 'f';
        I8 = 78;
        I16 = -26;
        I32 = -12;
        I64 = -54;
        U = 5;
        U8 = 20;
        U16 = 32;
        U32 = 16;
        U64 = 128;
        F32 = 5.75;
        F64 = 9.25;
    }

    _zzz(); // #break
}

fn _zzz() {()}
