// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::intrin::transmute::*;
use crate::intrin::upcast::*;
use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::std::mem::transmute;

impl Upcast<u16x8> for u8x16 {
    #[inline(always)]
    #[cfg(target_feature = "sse4.1")]
    fn upcast(self) -> (u16x8, u16x8) {
        // Shuffle the vector as i32s for better perf
        optimized!();
        unsafe {
            (_mm_cvtepu8_epi16(self).be_u16s(),
             _mm_cvtepu8_epi16(_mm_shuffle_epi32(self.be_i32s(), 0x0E).be_u8s()).be_u16s())
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn upcast(self) -> (u16x8, u16x8) {
        fallback!();
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
    #[cfg(target_feature = "sse4.1")]
    fn upcast(self) -> (i16x8, i16x8) {
        // Shuffle the vector as i32s for better perf
        optimized!();
        unsafe {
            (_mm_cvtepi8_epi16(self),
             _mm_cvtepi8_epi16(_mm_shuffle_epi32(self.be_i32s(), 0x0E).be_i8s()))
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn upcast(self) -> (i16x8, i16x8) {
        fallback!();
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
    #[cfg(target_feature = "sse4.1")]
    fn upcast(self) -> (u32x4, u32x4) {
        optimized!();
        unsafe {
            (_mm_cvtepu16_epi32(self).be_u32s(),
             _mm_cvtepu16_epi32(_mm_shuffle_epi32(self.be_i32s(), 0x0E).be_u16s()).be_u32s())
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn upcast(self) -> (u32x4, u32x4) {
        fallback!();
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
    #[cfg(target_feature = "sse4.1")]
    fn upcast(self) -> (i32x4, i32x4) {
        optimized!();
        unsafe {
            (_mm_cvtepi16_epi32(self),
             _mm_cvtepi16_epi32(_mm_shuffle_epi32(self.be_i32s(), 0x0E).be_i16s()))
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn upcast(self) -> (i32x4, i32x4) {
        fallback!();
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
    #[cfg(target_feature = "avx2")]
    fn upcast(self) -> (u16x16, u16x16) {
        optimized!();
        unsafe {
            (_mm256_cvtepu8_epi16(transmute(_mm256_castsi256_si128(transmute(self)))).be_u16s(),
             _mm256_cvtepu8_epi16(transmute(_mm256_castsi256_si128(transmute(_mm256_permute4x64_epi64(self.be_i64s(), 0x0E).be_u16s())))).be_u16s())
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (u16x16, u16x16) {
        fallback!();
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
    #[cfg(target_feature = "avx2")]
    fn upcast(self) -> (i16x16, i16x16) {
        optimized!();
        unsafe {
            (_mm256_cvtepi8_epi16(
                transmute(
                    _mm256_castsi256_si128(
                        transmute(self)))),
             _mm256_cvtepi8_epi16(
                 transmute(
                     _mm256_castsi256_si128(
                         transmute(
                             _mm256_permute4x64_epi64(self.be_i64s(), 0x0E))))))
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (i16x16, i16x16) {
        fallback!();
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
    #[cfg(target_feature = "avx2")]
    fn upcast(self) -> (u32x8, u32x8) {
        optimized!();
        unsafe {
            (_mm256_cvtepu16_epi32(transmute(_mm256_castsi256_si128(transmute(self)))).be_u32s(),
             _mm256_cvtepu16_epi32(transmute(_mm256_castsi256_si128(transmute(_mm256_permute4x64_epi64(self.be_i64s(), 0x0E).be_u32s())))).be_u32s())
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (u32x8, u32x8) {
        fallback!();
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
    #[cfg(target_feature = "avx2")]
    fn upcast(self) -> (i32x8, i32x8) {
        optimized!();
        unsafe {
            (_mm256_cvtepi16_epi32(
                transmute(
                    _mm256_castsi256_si128(
                        transmute(self)))),
             _mm256_cvtepi16_epi32(
                 transmute(
                     _mm256_castsi256_si128(
                         transmute(
                             _mm256_permute4x64_epi64(self.be_i64s(), 0x0E))))))
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (i32x8, i32x8) {
        fallback!();
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
    #[cfg(target_feature = "sse2")]
    fn upcast(self) -> (f64x2, f64x2) {
        // Shuffle the vector as i32s for better perf
        optimized!();
        unsafe { (_mm_cvtps_pd(self), _mm_cvtps_pd(_mm_shuffle_epi32(self.be_i32s(), 0x0E).be_f32s_unchecked())) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn upcast(self) -> (f64x2, f64x2) {
        fallback!();
        (f64x2::new(self.extract(0) as f64,
                    self.extract(1) as f64),
         f64x2::new(self.extract(2) as f64,
                    self.extract(3) as f64))
    }
}

impl Upcast<f64x2> for i32x4 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn upcast(self) -> (f64x2, f64x2) {
        optimized!();
        unsafe { (_mm_cvtepi32_pd(self), _mm_cvtepi32_pd(_mm_shuffle_epi32(self, 0x0E))) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn upcast(self) -> (f64x2, f64x2) {
        fallback!();
        (f64x2::new(self.extract(0) as f64,
                    self.extract(1) as f64),
         f64x2::new(self.extract(2) as f64,
                    self.extract(3) as f64))
    }
}

impl Upcast<i64x2> for i32x4 {
    #[inline(always)]
    #[cfg(target_feature = "sse4.1")]
    fn upcast(self) -> (i64x2, i64x2) {
        optimized!();
        unsafe {
            (_mm_cvtepi32_epi64(self),
             _mm_cvtepi32_epi64(_mm_shuffle_epi32(self, 0x0E)))
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn upcast(self) -> (i64x2, i64x2) {
        fallback!();
        (i64x2::new(self.extract(0) as i64,
                    self.extract(1) as i64),
         i64x2::new(self.extract(2) as i64,
                    self.extract(3) as i64))
    }
}

impl Upcast<u64x2> for u32x4 {
    #[inline(always)]
    #[cfg(target_feature = "sse4.1")]
    fn upcast(self) -> (u64x2, u64x2) {
        optimized!();
        unsafe {
            (_mm_cvtepu32_epi64(self).be_u64s(),
             _mm_cvtepu32_epi64(_mm_shuffle_epi32(self.be_i32s(), 0x0E).be_u32s()).be_u64s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse4.1"))]
    fn upcast(self) -> (u64x2, u64x2) {
        fallback!();
        (u64x2::new(self.extract(0) as u64,
                    self.extract(1) as u64),
         u64x2::new(self.extract(2) as u64,
                    self.extract(3) as u64))
    }
}

impl Upcast<f64x4> for f32x8 {
    #[inline(always)]
    #[cfg(target_feature = "avx2")]
    fn upcast(self) -> (f64x4, f64x4) {
        // Shuffle the vector as i32s for better perf
        optimized!();
        unsafe {
            (_mm256_cvtps_pd(
                transmute(
                    _mm256_castsi256_si128(
                        transmute(self)))),
             _mm256_cvtps_pd(
                 transmute(
                     _mm256_castsi256_si128(
                         transmute(
                             _mm256_permute4x64_epi64(self.be_i64s(), 0x0E))))))
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (f64x4, f64x4) {
        fallback!();
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
    #[cfg(target_feature = "avx2")]
    fn upcast(self) -> (f64x4, f64x4) {
        optimized!();
        unsafe {
            (_mm256_cvtepi32_pd(
                transmute(
                    _mm256_castsi256_si128(
                        transmute(self)))),
             _mm256_cvtepi32_pd(
                 transmute(
                     _mm256_castsi256_si128(
                         transmute(
                             _mm256_permute4x64_epi64(self.be_i64s(), 0x0E))))))
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (f64x4, f64x4) {
        fallback!();
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
    #[cfg(target_feature = "avx2")]
    fn upcast(self) -> (i64x4, i64x4) {
        optimized!();
        unsafe {
            (_mm256_cvtepi32_epi64(
                transmute(
                    _mm256_castsi256_si128(
                        transmute(self)))),
             _mm256_cvtepi32_epi64(
                 transmute(
                     _mm256_castsi256_si128(
                         transmute(
                             _mm256_permute4x64_epi64(self.be_i64s(), 0x0E))))))
        }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (i64x4, i64x4) {
        fallback!();
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
    #[cfg(target_feature = "avx2")]
    fn upcast(self) -> (u64x4, u64x4) {
        optimized!();
        unsafe {
            (_mm256_cvtepu32_epi64(transmute(_mm256_castsi256_si128(transmute(self)))).be_u64s(),
             _mm256_cvtepu32_epi64(transmute(_mm256_castsi256_si128(transmute(_mm256_permute4x64_epi64(transmute(self), 0x0E).be_u32s())))).be_u64s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "avx2"))]
    fn upcast(self) -> (u64x4, u64x4) {
        fallback!();
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
        fallback!();
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
        fallback!();
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
        fallback!();
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
        fallback!();
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
