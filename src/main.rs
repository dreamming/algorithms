use rand::Rng;
use std::time::{Instant};
pub mod sorting;

fn main() {

    let mut v = [0;50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u32 = rng.gen();
        v[i] = gen_value;
    }

    let start_sort = Instant::now();
    sorting::heap_sort(&mut v);
    // sorting::quick_sort_recursive(&mut v);
    let sort_duration = start_sort.elapsed();
    println!("Time elapsed in sort_duration is: {:?}", sort_duration);
    assert_eq!(sorting::is_sorted(&v), true);
}