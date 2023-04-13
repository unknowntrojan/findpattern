#![feature(core_intrinsics)]
//!
//! This crate provides a naive implementation of a pattern finding algorithm.
//!
//! It also provides a macro to create the data structure required, `pattern`.
//! This crate isn't recommended to be used as a stand-alone, but wrapped by my `signature` crate.
//!

#[cfg(feature = "parallel")]
use rayon::{prelude::IndexedParallelIterator, slice::ParallelSlice};

pub type Pattern = Vec<Option<u8>>;

/// Creates a pattern.
///
/// Internally, this is a `[Option<u8>]`.
///
/// `None`'s, represented here as `_`, match anything.
///
/// `Some`'s match their content directly.
///
/// # Examples
/// ```
/// # use findpattern::pattern;
/// let pat = pattern!(0xE9, _, _, _, _);
/// dbg!(pat); // {Some(0xE9), None, None, None, None}
/// ```
#[macro_export]
macro_rules! pattern {
    ($($elem:tt),+) => {
        vec![$(pattern!(@el $elem)),+]
    };
    (@el $v:expr) => {
        Some($v)
    };
    (@el $v:tt) => {
        None
    };
}

#[inline(always)]
fn match_pattern(window: &[u8], pattern: &Pattern) -> bool {
    window.iter().zip(pattern).all(|(v, p)| match p {
        Some(x) => *v == *x,
        None => true,
    })
}

///
/// Returns the first position within `region` that matches `pattern`.
///
/// Short-circuiting.
///
pub fn find_pattern(region: &[u8], pattern: &Pattern) -> Option<usize> {
    region
        .windows(pattern.len())
        .position(|wnd| core::intrinsics::unlikely(match_pattern(wnd, pattern)))
}

///
/// Returns all positions within `region` that match `pattern`.
///
pub fn find_patterns(region: &[u8], pattern: &Pattern) -> Vec<usize> {
    region
        .windows(pattern.len())
        .enumerate()
        .filter(|(_, wnd)| core::intrinsics::unlikely(match_pattern(wnd, pattern)))
        .map(|(idx, _)| idx)
        .collect()
}

#[cfg(feature = "parallel")]
pub fn find_pattern_par(region: &[u8], pattern: &Pattern) -> Option<usize> {
    region
        .par_windows(pattern.len())
        .position_any(|wnd| core::intrinsics::unlikely(match_pattern(wnd, pattern)))
}

#[cfg(feature = "parallel")]
pub fn find_patterns_par(region: &'a [u8], pattern: &Pattern) -> Vec<usize> {
    region
        .par_windows(pattern.len())
        .enumerate()
        .filter(|(_, wnd)| core::intrinsics::unlikely(match_pattern(wnd, pattern)))
        .map(|(idx, _)| idx)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_fwd() {
        let test_pattern: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xDE, 0xAD, 0xFF, 0xBE, 0xEF, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];

        let pattern = pattern!(0xDE, 0xAD, ?, 0xBE, 0xEF);

        assert_eq!(find_pattern(&test_pattern, &pattern), Some(20));
    }

    #[test]
    fn pattern_fwd_multiple() {
        let test_pattern: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0xDE, 0xAD, 0xFF, 0xBE, 0xEF, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xDE, 0xAD, 0xFF, 0xBE, 0xEF, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];

        let pattern = pattern!(0xDE, 0xAD, ?, 0xBE, 0xEF);

        assert_eq!(find_patterns(&test_pattern, &pattern), vec![4, 20]);
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn pattern_fwd_par() {
        let test_pattern: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xDE, 0xAD, 0xFF, 0xBE, 0xEF, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];

        let pattern = pattern!(0xDE, 0xAD, ?, 0xBE, 0xEF);

        assert_eq!(find_pattern_par(&test_pattern, &pattern), Some(20));
    }

    #[test]
    fn matching() {
        assert!(match_pattern(
            &[0xDE, 0xAD, 0x00, 0xBE, 0xEF],
            &pattern!(0xDE, 0xAD, ?, 0xBE, 0xEF)
        ))
    }
}
