// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::arch::current::vecs::*;
use crate::intrin::upcast::*;

impl Upcast<u16x8> for u8x16 {
    #[inline(always)]
    fn upcast(self) -> (u16x8, u16x8) {
        (u16x8::new(self.extract(0) as u16,
                    self.extract(1) as u16,
                    self.extract(2) as u16,
                    self.extract(3) as u16,
                    self.extract(4) as u16,
                    self.extract(5) as u16,
                    self.extract(6) as u16,
                    self.extract(7) as u16),
         u16x8::new(self.extract(8) as u16,
                    self.extract(9) as u16,
                    self.extract(10) as u16,
                    self.extract(11) as u16,
                    self.extract(12) as u16,
                    self.extract(13) as u16,
                    self.extract(14) as u16,
                    self.extract(15) as u16))
    }
}

impl Upcast<i16x8> for i8x16 {
    #[inline(always)]
    fn upcast(self) -> (i16x8, i16x8) {
        (i16x8::new(self.extract(0) as i16,
                    self.extract(1) as i16,
                    self.extract(2) as i16,
                    self.extract(3) as i16,
                    self.extract(4) as i16,
                    self.extract(5) as i16,
                    self.extract(6) as i16,
                    self.extract(7) as i16),
         i16x8::new(self.extract(8) as i16,
                    self.extract(9) as i16,
                    self.extract(10) as i16,
                    self.extract(11) as i16,
                    self.extract(12) as i16,
                    self.extract(13) as i16,
                    self.extract(14) as i16,
                    self.extract(15) as i16))
    }
}

impl Upcast<u32x4> for u16x8 {
    #[inline(always)]
    fn upcast(self) -> (u32x4, u32x4) {
        (u32x4::new(self.extract(0) as u32,
                    self.extract(1) as u32,
                    self.extract(2) as u32,
                    self.extract(3) as u32),
         u32x4::new(self.extract(4) as u32,
                    self.extract(5) as u32,
                    self.extract(6) as u32,
                    self.extract(7) as u32))
    }
}

impl Upcast<i32x4> for i16x8 {
    #[inline(always)]
    fn upcast(self) -> (i32x4, i32x4) {
        (i32x4::new(self.extract(0) as i32,
                    self.extract(1) as i32,
                    self.extract(2) as i32,
                    self.extract(3) as i32),
         i32x4::new(self.extract(4) as i32,
                    self.extract(5) as i32,
                    self.extract(6) as i32,
                    self.extract(7) as i32))
    }
}

impl Upcast<u16x16> for u8x32 {
    #[inline(always)]
    fn upcast(self) -> (u16x16, u16x16) {
        (u16x16::new(self.extract(0) as u16,
                     self.extract(1) as u16,
                     self.extract(2) as u16,
                     self.extract(3) as u16,
                     self.extract(4) as u16,
                     self.extract(5) as u16,
                     self.extract(6) as u16,
                     self.extract(7) as u16,
                     self.extract(8) as u16,
                     self.extract(9) as u16,
                     self.extract(10) as u16,
                     self.extract(11) as u16,
                     self.extract(12) as u16,
                     self.extract(13) as u16,
                     self.extract(14) as u16,
                     self.extract(15) as u16),
         u16x16::new(self.extract(16) as u16,
                     self.extract(17) as u16,
                     self.extract(18) as u16,
                     self.extract(19) as u16,
                     self.extract(20) as u16,
                     self.extract(21) as u16,
                     self.extract(22) as u16,
                     self.extract(23) as u16,
                     self.extract(24) as u16,
                     self.extract(25) as u16,
                     self.extract(26) as u16,
                     self.extract(27) as u16,
                     self.extract(28) as u16,
                     self.extract(29) as u16,
                     self.extract(30) as u16,
                     self.extract(31) as u16))
    }
}

impl Upcast<i16x16> for i8x32 {
    #[inline(always)]
    fn upcast(self) -> (i16x16, i16x16) {
        (i16x16::new(self.extract(0) as i16,
                     self.extract(1) as i16,
                     self.extract(2) as i16,
                     self.extract(3) as i16,
                     self.extract(4) as i16,
                     self.extract(5) as i16,
                     self.extract(6) as i16,
                     self.extract(7) as i16,
                     self.extract(8) as i16,
                     self.extract(9) as i16,
                     self.extract(10) as i16,
                     self.extract(11) as i16,
                     self.extract(12) as i16,
                     self.extract(13) as i16,
                     self.extract(14) as i16,
                     self.extract(15) as i16),
         i16x16::new(self.extract(16) as i16,
                     self.extract(17) as i16,
                     self.extract(18) as i16,
                     self.extract(19) as i16,
                     self.extract(20) as i16,
                     self.extract(21) as i16,
                     self.extract(22) as i16,
                     self.extract(23) as i16,
                     self.extract(24) as i16,
                     self.extract(25) as i16,
                     self.extract(26) as i16,
                     self.extract(27) as i16,
                     self.extract(28) as i16,
                     self.extract(29) as i16,
                     self.extract(30) as i16,
                     self.extract(31) as i16))
    }
}

impl Upcast<u32x8> for u16x16 {
    #[inline(always)]
    fn upcast(self) -> (u32x8, u32x8) {
        (u32x8::new(self.extract(0) as u32,
                    self.extract(1) as u32,
                    self.extract(2) as u32,
                    self.extract(3) as u32,
                    self.extract(4) as u32,
                    self.extract(5) as u32,
                    self.extract(6) as u32,
                    self.extract(7) as u32),
         u32x8::new(self.extract(8) as u32,
                    self.extract(9) as u32,
                    self.extract(10) as u32,
                    self.extract(11) as u32,
                    self.extract(12) as u32,
                    self.extract(13) as u32,
                    self.extract(14) as u32,
                    self.extract(15) as u32))

    }
}

impl Upcast<i32x8> for i16x16 {
    #[inline(always)]
    fn upcast(self) -> (i32x8, i32x8) {
        (i32x8::new(self.extract(0) as i32,
                    self.extract(1) as i32,
                    self.extract(2) as i32,
                    self.extract(3) as i32,
                    self.extract(4) as i32,
                    self.extract(5) as i32,
                    self.extract(6) as i32,
                    self.extract(7) as i32),
         i32x8::new(self.extract(8) as i32,
                    self.extract(9) as i32,
                    self.extract(10) as i32,
                    self.extract(11) as i32,
                    self.extract(12) as i32,
                    self.extract(13) as i32,
                    self.extract(14) as i32,
                    self.extract(15) as i32))
    }
}

impl Upcast<f64x2> for f32x4 {
    #[inline(always)]
    fn upcast(self) -> (f64x2, f64x2) {
        (f64x2::new(self.extract(0) as f64,
                    self.extract(1) as f64),
         f64x2::new(self.extract(2) as f64,
                    self.extract(3) as f64))
    }
}

impl Upcast<f64x2> for i32x4 {
    #[inline(always)]
    fn upcast(self) -> (f64x2, f64x2) {
        (f64x2::new(self.extract(0) as f64,
                    self.extract(1) as f64),
         f64x2::new(self.extract(2) as f64,
                    self.extract(3) as f64))
    }
}

impl Upcast<i64x2> for i32x4 {
    #[inline(always)]
    fn upcast(self) -> (i64x2, i64x2) {
        (i64x2::new(self.extract(0) as i64,
                    self.extract(1) as i64),
         i64x2::new(self.extract(2) as i64,
                    self.extract(3) as i64))
    }
}

impl Upcast<u64x2> for u32x4 {
    #[inline(always)]
    fn upcast(self) -> (u64x2, u64x2) {
        (u64x2::new(self.extract(0) as u64,
                    self.extract(1) as u64),
         u64x2::new(self.extract(2) as u64,
                    self.extract(3) as u64))
    }
}

impl Upcast<f64x4> for f32x8 {
    #[inline(always)]
    fn upcast(self) -> (f64x4, f64x4) {
        (f64x4::new(self.extract(0) as f64,
                    self.extract(1) as f64,
                    self.extract(2) as f64,
                    self.extract(3) as f64),
         f64x4::new(self.extract(4) as f64,
                    self.extract(5) as f64,
                    self.extract(6) as f64,
                    self.extract(7) as f64))
    }
}

impl Upcast<f64x4> for i32x8 {
    #[inline(always)]
    fn upcast(self) -> (f64x4, f64x4) {
        (f64x4::new(self.extract(0) as f64,
                    self.extract(1) as f64,
                    self.extract(2) as f64,
                    self.extract(3) as f64),
         f64x4::new(self.extract(4) as f64,
                    self.extract(5) as f64,
                    self.extract(6) as f64,
                    self.extract(7) as f64))
    }
}

impl Upcast<i64x4> for i32x8 {
    #[inline(always)]
    fn upcast(self) -> (i64x4, i64x4) {
        (i64x4::new(self.extract(0) as i64,
                    self.extract(1) as i64,
                    self.extract(2) as i64,
                    self.extract(3) as i64),
         i64x4::new(self.extract(4) as i64,
                    self.extract(5) as i64,
                    self.extract(6) as i64,
                    self.extract(7) as i64))
    }
}

impl Upcast<u64x4> for u32x8 {
    #[inline(always)]
    fn upcast(self) -> (u64x4, u64x4) {
        (u64x4::new(self.extract(0) as u64,
                    self.extract(1) as u64,
                    self.extract(2) as u64,
                    self.extract(3) as u64),
         u64x4::new(self.extract(4) as u64,
                    self.extract(5) as u64,
                    self.extract(6) as u64,
                    self.extract(7) as u64))
    }
}

impl Upcast<f64x8> for f32x16 {
    #[inline(always)]
    fn upcast(self) -> (f64x8, f64x8) {
        (f64x8::new(self.extract(0) as f64,
                    self.extract(1) as f64,
                    self.extract(2) as f64,
                    self.extract(3) as f64,
                    self.extract(4) as f64,
                    self.extract(5) as f64,
                    self.extract(6) as f64,
                    self.extract(7) as f64),
         f64x8::new(self.extract(8) as f64,
                    self.extract(9) as f64,
                    self.extract(10) as f64,
                    self.extract(11) as f64,
                    self.extract(12) as f64,
                    self.extract(13) as f64,
                    self.extract(14) as f64,
                    self.extract(15) as f64))
    }
}

impl Upcast<f64x8> for i32x16 {
    #[inline(always)]
    fn upcast(self) -> (f64x8, f64x8) {
        (f64x8::new(self.extract(0) as f64,
                    self.extract(1) as f64,
                    self.extract(2) as f64,
                    self.extract(3) as f64,
                    self.extract(4) as f64,
                    self.extract(5) as f64,
                    self.extract(6) as f64,
                    self.extract(7) as f64),
         f64x8::new(self.extract(8) as f64,
                    self.extract(9) as f64,
                    self.extract(10) as f64,
                    self.extract(11) as f64,
                    self.extract(12) as f64,
                    self.extract(13) as f64,
                    self.extract(14) as f64,
                    self.extract(15) as f64))
    }
}

impl Upcast<i64x8> for i32x16 {
    #[inline(always)]
    fn upcast(self) -> (i64x8, i64x8) {
        (i64x8::new(self.extract(0) as i64,
                    self.extract(1) as i64,
                    self.extract(2) as i64,
                    self.extract(3) as i64,
                    self.extract(4) as i64,
                    self.extract(5) as i64,
                    self.extract(6) as i64,
                    self.extract(7) as i64),
         i64x8::new(self.extract(8) as i64,
                    self.extract(9) as i64,
                    self.extract(10) as i64,
                    self.extract(11) as i64,
                    self.extract(12) as i64,
                    self.extract(13) as i64,
                    self.extract(14) as i64,
                    self.extract(15) as i64))
    }
}

impl Upcast<u64x8> for u32x16 {
    #[inline(always)]
    fn upcast(self) -> (u64x8, u64x8) {
        (u64x8::new(self.extract(0) as u64,
                    self.extract(1) as u64,
                    self.extract(2) as u64,
                    self.extract(3) as u64,
                    self.extract(4) as u64,
                    self.extract(5) as u64,
                    self.extract(6) as u64,
                    self.extract(7) as u64),
         u64x8::new(self.extract(8) as u64,
                    self.extract(9) as u64,
                    self.extract(10) as u64,
                    self.extract(11) as u64,
                    self.extract(12) as u64,
                    self.extract(13) as u64,
                    self.extract(14) as u64,
                    self.extract(15) as u64))
    }
}
