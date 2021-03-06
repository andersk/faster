// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(dead_code)]

//! Vector types which aren't interpreted as SIMD vectors, for systems which
//! don't have SIMD support.
use crate::std::ops::*;
use crate::std::mem::*;
use crate::std::ptr::*;
use crate::std::fmt::*;
use crate::vecs::*;

macro_rules! impl_packed_type {
    ($el:ident, $pvec:ident, $vec:ident, $sz:expr, [$($elname:ident),+]) => {
        // Blocked by const generics (or impl {Debug, PartialEq} for [T; 64])
        #[derive(Clone, Copy, /*Debug, PartialEq*/)]
        #[allow(non_camel_case_types)]
        pub struct $vec { data: [$el; $sz] }

        // PartialEq shim until const generics arrive
        impl PartialEq<Self> for $vec {
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool {
                self.data.iter().zip(other.data.iter()).fold(true, |acc, (a, b)| acc && a == b)
            }
        }

        // Debug shim until const generics arrive
        impl Debug for $vec {
            #[inline(always)]
            fn fmt(&self, f: &mut Formatter) -> Result {
                write!(f, "$vec(")?;
                for n in self.data.iter() {
                    write!(f, "{:?}, ", n)?;
                }
                write!(f, ")")?;
                Ok(())
            }
        }

        impl $vec {
            #[inline(always)]
            pub fn new($($elname: $el),*) -> $vec {
                $vec { data: [$($elname),*] }
            }

            #[inline(always)]
            pub fn len() -> i32 {
                $sz
            }

            #[inline(always)]
            pub fn splat(value: $el) -> $vec {
                $vec { data: [value; $sz] }
            }

            #[inline(always)]
            pub fn extract(self, idx: usize) -> $el {
                self.data[idx]
            }

            #[inline(always)]
            pub unsafe fn extract_unchecked(self, idx: usize) -> $el {
                // Maintain unsafe API with stdsimd
                self.data[idx]
            }

            #[inline(always)]
            pub fn replace(mut self, idx: usize, val: $el) -> $vec {
                self.data[idx] = val;
                self
            }

            #[inline(always)]
            pub unsafe fn replace_unchecked(mut self, idx: usize, val: $el) -> $vec {
                // Maintain unsafe API with stdsimd
                self.data[idx] = val;
                self
            }

            #[inline(always)]
            pub fn store(self, slice: &mut [$el], offset: usize) {
                assert!(slice.len() >= $sz);
                unsafe { self.store_unchecked(slice, offset) }
            }

            #[inline(always)]
            pub fn store_unaligned(self, slice: &mut [$el]) {
                assert!(slice.len() >= $sz);
                unsafe { self.store_unchecked(slice, 0) }
            }

            #[inline(always)]
            pub unsafe fn store_unchecked(self, slice: &mut [$el], offset: usize) {
                copy_nonoverlapping(
                    &self as *const $vec as *const u8,
                    slice[offset..].as_mut_ptr() as *mut u8,
                    size_of::<$vec>());
            }

            // TODO: Actually check alignment
            #[inline(always)]
            pub unsafe fn store_aligned_unchecked(self, slice: &mut [$el]) {
                copy_nonoverlapping(
                    &self as *const $vec as *const u8,
                    slice.as_mut_ptr() as *mut u8,
                    size_of::<$vec>());
            }

            #[inline(always)]
            pub unsafe fn store_unaligned_unchecked(self, slice: &mut [$el]) {
                copy_nonoverlapping(
                    &self as *const $vec as *const u8,
                    slice.as_mut_ptr() as *mut u8,
                    size_of::<$vec>());
            }

            #[inline(always)]
            pub fn load(slice: &[$el], offset: usize) -> $vec {
                assert!(slice.len() >= $sz);
                unsafe { $vec::load_unchecked(slice, offset) }
            }

            #[inline(always)]
            pub fn load_unaligned(slice: &[$el]) -> $vec {
                assert!(slice.len() >= $sz);
                unsafe { $vec::load_unchecked(slice, 0) }
            }

            #[inline(always)]
            pub unsafe fn load_unchecked(slice: &[$el], offset: usize) -> $vec {
                let mut x = $vec::splat(0 as $el);
                copy_nonoverlapping(
                    slice[offset..].as_ptr() as *const u8,
                    &mut x as *mut $vec as *mut u8,
                    size_of::<$vec>());
                x
            }

            // TODO: Actually check alignment
            #[inline(always)]
            pub unsafe fn load_aligned_unchecked(slice: &[$el]) -> $vec {
                let mut x = $vec::splat(0 as $el);
                copy_nonoverlapping(
                    slice.as_ptr() as *const u8,
                    &mut x as *mut $vec as *mut u8,
                    size_of::<$vec>());
                x
            }

            #[inline(always)]
            pub unsafe fn load_unaligned_unchecked(slice: &[$el]) -> $vec {
                let mut x = $vec::splat(0 as $el);
                copy_nonoverlapping(
                    slice.as_ptr() as *const u8,
                    &mut x as *mut $vec as *mut u8,
                    size_of::<$vec>());
                x
            }
        }
    }
}

macro_rules! impl_from {
    ($to:ident, $($from:ident),+) => {
        $(
            impl From<$from> for $to {
                #[inline(always)]
                fn from(f: $from) -> $to {
                    unsafe { transmute(f) }
                }
            }
        )+
    }
}

macro_rules! impl_ops {
    ($el:ty, $vec:ty, $([$trait:tt, $fn:tt, $op:tt]),*) => {
        $(
            impl $trait <Self> for $vec {
                type Output = Self;
                #[inline(always)]
                fn $fn(self, rhs: Self) -> Self::Output {
                    let mut ret = Self::splat(0 as $el);
                    for (i, (x, y)) in self.data.iter().zip(rhs.data.iter()).enumerate() {
                        ret.data[i] = x $op y;
                    }
                    ret
                }
            }
        )*
    }
}

macro_rules! impl_assignops {
    ($el:ty, $vec:ty, $([$trait:tt, $fn:tt, $op:tt]),*) => {
        $(
            impl $trait <Self> for $vec {
                #[inline(always)]
                fn $fn(&mut self, rhs: Self) {
                    for (i, y) in rhs.data.iter().enumerate() {
                        self.data[i] $op y;
                    }
                }
            }
        )*
    }
}

macro_rules! impl_cast {
    ($vec:ty, $tovec:tt, $el:ty, $name:ident) => {
        impl $vec {
            #[inline(always)]
            pub fn $name(self) -> $tovec {
                let mut ret = $tovec::splat(0 as $el);
                for (i, x) in self.data.iter().enumerate() {
                    ret.data[i] = *x as $el;
                }
                ret
            }
        }
    }
}

// "undefined" is just a string that should not match any target-feature.
impl_packed!(u8, u8s, u8x16, 1, 16, [], ["undefined"]);
impl_packed!(i8, i8s, i8x16, 1, 16, [], ["undefined"]);
impl_packed!(u16, u16s, u16x8, 2, 8, [], ["undefined"]);
impl_packed!(i16, i16s, i16x8, 2, 8, [], ["undefined"]);
impl_packed!(u32, u32s, u32x4, 4, 4, [], ["undefined"]);
impl_packed!(i32, i32s, i32x4, 4, 4, [], ["undefined"]);
impl_packed!(f32, f32s, f32x4, 4, 4, [], ["undefined"]);
impl_packed!(u64, u64s, u64x2, 8, 2, [], ["undefined"]);
impl_packed!(i64, i64s, i64x2, 8, 2, [], ["undefined"]);
impl_packed!(f64, f64s, f64x2, 8, 2, [], ["undefined"]);

impl_packed_type!(f64, f64s, f64x2, 2, [x0, x1]);
impl_packed_type!(f64, f64s, f64x4, 4, [x0, x1, x2, x3]);
impl_packed_type!(f64, f64s, f64x8, 8, [x0, x1, x2, x3, x4, x5, x6, x7]);
impl_packed_type!(u64, u64s, u64x2, 2, [x0, x1]);
impl_packed_type!(u64, u64s, u64x4, 4, [x0, x1, x2, x3]);
impl_packed_type!(u64, u64s, u64x8, 8, [x0, x1, x2, x3, x4, x5, x6, x7]);
impl_packed_type!(i64, i64s, i64x2, 2, [x0, x1]);
impl_packed_type!(i64, i64s, i64x4, 4, [x0, x1, x2, x3]);
impl_packed_type!(i64, i64s, i64x8, 8, [x0, x1, x2, x3, x4, x5, x6, x7]);
impl_packed_type!(f32, f32s, f32x4, 4, [x0, x1, x2, x3]);
impl_packed_type!(f32, f32s, f32x8, 8, [x0, x1, x2, x3, x4, x5, x6, x7]);
impl_packed_type!(f32, f32s, f32x16, 16, [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15]);
impl_packed_type!(u32, u32s, u32x4, 4, [x0, x1, x2, x3]);
impl_packed_type!(u32, u32s, u32x8, 8, [x0, x1, x2, x3, x4, x5, x6, x7]);
impl_packed_type!(u32, u32s, u32x16, 16, [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15]);
impl_packed_type!(i32, i32s, i32x4, 4, [x0, x1, x2, x3]);
impl_packed_type!(i32, i32s, i32x8, 8, [x0, x1, x2, x3, x4, x5, x6, x7]);
impl_packed_type!(i32, i32s, i32x16, 16, [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15]);
impl_packed_type!(u16, u16s, u16x8, 8, [x0, x1, x2, x3, x4, x5, x6, x7]);
impl_packed_type!(u16, u16s, u16x16, 16, [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15]);
impl_packed_type!(u16, u16s, u16x32, 32, [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31]);
impl_packed_type!(i16, i16s, i16x8, 8, [x0, x1, x2, x3, x4, x5, x6, x7]);
impl_packed_type!(i16, i16s, i16x16, 16, [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15]);
impl_packed_type!(i16, i16s, i16x32, 32, [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31]);
impl_packed_type!(u8, u8s, u8x16, 16, [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15]);
impl_packed_type!(u8, u8s, u8x32, 32, [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31]);
impl_packed_type!(u8, u8s, u8x64, 64, [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31, x32, x33, x34, x35, x36, x37, x38, x39, x40, x41, x42, x43, x44, x45, x46, x47, x48, x49, x50, x51, x52, x53, x54, x55, x56, x57, x58, x59, x60, x61, x62, x63]);
impl_packed_type!(i8, i8s, i8x16, 16, [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15]);
impl_packed_type!(i8, i8s, i8x32, 32, [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31]);
impl_packed_type!(i8, i8s, i8x64, 64, [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31, x32, x33, x34, x35, x36, x37, x38, x39, x40, x41, x42, x43, x44, x45, x46, x47, x48, x49, x50, x51, x52, x53, x54, x55, x56, x57, x58, x59, x60, x61, x62, x63]);

impl_from!(u64x2, i64x2, u32x4, i32x4, u16x8, i16x8, u8x16, i8x16);
impl_from!(i64x2, u64x2, u32x4, i32x4, u16x8, i16x8, u8x16, i8x16);
impl_from!(u32x4, u64x2, i64x2, i32x4, u16x8, i16x8, u8x16, i8x16);
impl_from!(i32x4, u64x2, i64x2, u32x4, u16x8, i16x8, u8x16, i8x16);
impl_from!(u16x8, u64x2, i64x2, u32x4, i32x4, i16x8, u8x16, i8x16);
impl_from!(i16x8, u64x2, i64x2, u32x4, i32x4, u16x8, u8x16, i8x16);
impl_from!(u8x16, u64x2, i64x2, u32x4, i32x4, u16x8, i16x8, i8x16);
impl_from!(i8x16, u64x2, i64x2, u32x4, i32x4, u16x8, i16x8, u8x16);

impl_from!(u64x4, i64x4, u32x8, i32x8, u16x16, i16x16, u8x32, i8x32);
impl_from!(i64x4, u64x4, u32x8, i32x8, u16x16, i16x16, u8x32, i8x32);
impl_from!(u32x8, u64x4, i64x4, i32x8, u16x16, i16x16, u8x32, i8x32);
impl_from!(i32x8, u64x4, i64x4, u32x8, u16x16, i16x16, u8x32, i8x32);
impl_from!(u16x16, u64x4, i64x4, u32x8, i32x8, i16x16, u8x32, i8x32);
impl_from!(i16x16, u64x4, i64x4, u32x8, i32x8, u16x16, u8x32, i8x32);
impl_from!(u8x32, u64x4, i64x4, u32x8, i32x8, u16x16, i16x16, i8x32);
impl_from!(i8x32, u64x4, i64x4, u32x8, i32x8, u16x16, i16x16, u8x32);

impl_from!(u64x8, i64x8, u32x16, i32x16, u16x32, i16x32, u8x64, i8x64);
impl_from!(i64x8, u64x8, u32x16, i32x16, u16x32, i16x32, u8x64, i8x64);
impl_from!(u32x16, u64x8, i64x8, i32x16, u16x32, i16x32, u8x64, i8x64);
impl_from!(i32x16, u64x8, i64x8, u32x16, u16x32, i16x32, u8x64, i8x64);
impl_from!(u16x32, u64x8, i64x8, u32x16, i32x16, i16x32, u8x64, i8x64);
impl_from!(i16x32, u64x8, i64x8, u32x16, i32x16, u16x32, u8x64, i8x64);
impl_from!(u8x64, u64x8, i64x8, u32x16, i32x16, u16x32, i16x32, i8x64);
impl_from!(i8x64, u64x8, i64x8, u32x16, i32x16, u16x32, i16x32, u8x64);

impl_ops!(i8, i8x16, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(u8, u8x16, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(i16, i16x8, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(u16, u16x8, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(i32, i32x4, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(u32, u32x4, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(f32, f32x4, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -]);
impl_ops!(i64, i64x2, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(u64, u64x2, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(f64, f64x2, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -]);

impl_ops!(i8, i8x32, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(u8, u8x32, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(i16, i16x16, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(u16, u16x16, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(i32, i32x8, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(u32, u32x8, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(f32, f32x8, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -]);
impl_ops!(i64, i64x4, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(u64, u64x4, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(f64, f64x4, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -]);

impl_ops!(i8, i8x64, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(u8, u8x64, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(i16, i16x32, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(u16, u16x32, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(i32, i32x16, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(u32, u32x16, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(f32, f32x16, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -]);
impl_ops!(i64, i64x8, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(u64, u64x8, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -],
          [Shl, shl, <<], [Shr, shr, >>], [Rem, rem, %], [BitAnd, bitand, &],
          [BitOr, bitor, |], [BitXor, bitxor, ^]);
impl_ops!(f64, f64x8, [Mul, mul, *], [Div, div, /], [Add, add, +], [Sub, sub, -]);

impl_assignops!(i8, i8x16, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(u8, u8x16, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(i16, i16x8, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(u16, u16x8, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(i32, i32x4, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(u32, u32x4, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(f32, f32x4, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=]);
impl_assignops!(i64, i64x2, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(u64, u64x2, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(f64, f64x2, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=]);

impl_assignops!(i8, i8x32, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(u8, u8x32, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(i16, i16x16, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(u16, u16x16, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(i32, i32x8, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(u32, u32x8, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(f32, f32x8, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=]);
impl_assignops!(i64, i64x4, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(u64, u64x4, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(f64, f64x4, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=]);

impl_assignops!(i8, i8x64, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(u8, u8x64, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(i16, i16x32, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(u16, u16x32, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(i32, i32x16, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(u32, u32x16, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(f32, f32x16, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=]);
impl_assignops!(i64, i64x8, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(u64, u64x8, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=],
                [ShlAssign, shl_assign, <<=], [ShrAssign, shr_assign, >>=], [RemAssign, rem_assign, %=], [BitAndAssign, bitand_assign, &=],
                [BitOrAssign, bitor_assign, |=], [BitXorAssign, bitxor_assign, ^=]);
impl_assignops!(f64, f64x8, [MulAssign, mul_assign, *=], [DivAssign, div_assign, /=], [AddAssign, add_assign, +=], [SubAssign, sub_assign, -=]);

impl_cast!(i8x16, u8x16, u8, as_u8x16);
impl_cast!(u8x16, i8x16, i8, as_i8x16);

impl_cast!(i8x32, u8x32, u8, as_u8x32);
impl_cast!(u8x32, i8x32, i8, as_i8x32);

impl_cast!(i8x64, u8x64, u8, as_u8x64);
impl_cast!(u8x64, i8x64, i8, as_i8x64);

impl_cast!(i16x8, u16x8, u16, as_u16x8);
impl_cast!(u16x8, i16x8, i16, as_i16x8);

impl_cast!(i16x16, u16x16, u16, as_u16x16);
impl_cast!(u16x16, i16x16, i16, as_i16x16);

impl_cast!(i16x32, u16x32, u16, as_u16x32);
impl_cast!(u16x32, i16x32, i16, as_i16x32);

impl_cast!(i32x4, u32x4, u32, as_u32x4);
impl_cast!(f32x4, u32x4, u32, as_u32x4);
impl_cast!(f32x4, i32x4, i32, as_i32x4);
impl_cast!(u32x4, i32x4, i32, as_i32x4);
impl_cast!(u32x4, f32x4, f32, as_f32x4);
impl_cast!(i32x4, f32x4, f32, as_f32x4);

impl_cast!(i32x8, u32x8, u32, as_u32x8);
impl_cast!(f32x8, u32x8, u32, as_u32x8);
impl_cast!(f32x8, i32x8, i32, as_i32x8);
impl_cast!(u32x8, i32x8, i32, as_i32x8);
impl_cast!(u32x8, f32x8, f32, as_f32x8);
impl_cast!(i32x8, f32x8, f32, as_f32x8);

impl_cast!(i32x16, u32x16, u32, as_u32x16);
impl_cast!(f32x16, u32x16, u32, as_u32x16);
impl_cast!(f32x16, i32x16, i32, as_i32x16);
impl_cast!(u32x16, i32x16, i32, as_i32x16);
impl_cast!(u32x16, f32x16, f32, as_f32x16);
impl_cast!(i32x16, f32x16, f32, as_f32x16);

impl_cast!(i64x2, u64x2, u64, as_u64x2);
impl_cast!(f64x2, u64x2, u64, as_u64x2);
impl_cast!(f64x2, i64x2, i64, as_i64x2);
impl_cast!(u64x2, i64x2, i64, as_i64x2);
impl_cast!(u64x2, f64x2, f64, as_f64x2);
impl_cast!(i64x2, f64x2, f64, as_f64x2);

impl_cast!(i64x4, u64x4, u64, as_u64x4);
impl_cast!(f64x4, u64x4, u64, as_u64x4);
impl_cast!(f64x4, i64x4, i64, as_i64x4);
impl_cast!(u64x4, i64x4, i64, as_i64x4);
impl_cast!(u64x4, f64x4, f64, as_f64x4);
impl_cast!(i64x4, f64x4, f64, as_f64x4);

impl_cast!(i64x8, u64x8, u64, as_u64x8);
impl_cast!(f64x8, u64x8, u64, as_u64x8);
impl_cast!(f64x8, i64x8, i64, as_i64x8);
impl_cast!(u64x8, i64x8, i64, as_i64x8);
impl_cast!(u64x8, f64x8, f64, as_f64x8);
impl_cast!(i64x8, f64x8, f64, as_f64x8);
