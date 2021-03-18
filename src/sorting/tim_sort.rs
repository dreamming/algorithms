use std::cmp::min;

use crate::sorting::merge_by_index;

use super::insertion_sort_by_index;

pub fn tim_sort<T: Ord + std::marker::Copy + std::fmt::Debug>(arr: &mut [T]) {
    let len = arr.len();
    let mini_run_len = min_run_len(32);
    for i in (0..arr.len()).step_by(mini_run_len) {
        let hi = min(i + 31, len - 1);
        insertion_sort_by_index(arr, i, hi);
    }
    let mut run = mini_run_len;
    while run < len {
        for i in (0..len).step_by(2 * run) {
            let mid = min(i + run - 1, len - 1);
            let hi = min((i + 2 * run) - 1, len - 1);
            merge_by_index(arr, i, mid, hi);
        }
        run *= 2;
    }
}

fn min_run_len(len: usize) -> usize {
    let mut n = len;
    let mut r = 0;
    while n >= 32 {
        r |= (n & 1);
        n >>= 1;
    }
    n + r
}