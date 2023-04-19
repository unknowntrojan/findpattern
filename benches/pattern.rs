#![feature(test)]

use findpattern::{find_pattern, find_pattern_par, pattern};
use rand::Rng;
use test::Bencher;

extern crate test;

#[bench]
fn bench_pattern_1gig(b: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let size: usize = 1024 * 1024 * 1024;
    let mut test_pattern: Vec<u8> = (0..size).map(|_| rng.gen_range(0..=255)).collect();

    // let pattern = size / 2;
    let pattern = size - 5;

    test_pattern[pattern] = 0xDE;
    test_pattern[pattern + 1] = 0xAD;
    test_pattern[pattern + 2] = 0xFF;
    test_pattern[pattern + 3] = 0xBE;
    test_pattern[pattern + 4] = 0xEF;

    b.iter(|| {
        assert_eq!(
            find_pattern(&test_pattern, pattern!(0xDE, 0xAD, _, 0xBE, 0xEF)),
            Some(pattern)
        );
    });
}

#[bench]
fn bench_pattern_parallel_1gig(b: &mut Bencher) {
    let mut rng = rand::thread_rng();

    let size: usize = 1024 * 1024 * 1024;
    let mut test_pattern: Vec<u8> = (0..size).map(|_| rng.gen_range(0..=255)).collect();

    // let pattern = size / 2;
    let pattern = size - 5;

    test_pattern[pattern] = 0xDE;
    test_pattern[pattern + 1] = 0xAD;
    test_pattern[pattern + 2] = 0xFF;
    test_pattern[pattern + 3] = 0xBE;
    test_pattern[pattern + 4] = 0xEF;

    b.iter(|| {
        assert_eq!(
            find_pattern_par(&test_pattern, pattern!(0xDE, 0xAD, _, 0xBE, 0xEF)),
            Some(pattern)
        );
    });
}
