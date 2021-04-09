use rand::Rng;
use std::time::{Instant};
use std::fs::File;
pub mod sorting;

fn main() {
    let result = File::open("a.txt");
    // match result {
    //     Ok(f) => println!(" exist "),
    //     Err(e) => println!(" exist "),
    // }

    // if let Ok(file) = result {
    //     println!(" exist ");
    // } else{
    //     println!(" exist ");
    // };

    // let x = File::open("a.txt").unwrap();
    // println!("hello ");

    let x = File::open("a.txt").expect("fail to open file ");
    // println!("hello {:?}", x);
    // let mut v = [0;50000];
    // for i in 0..v.len() {
    //     let mut rng = rand::thread_rng();
    //     let gen_value: u32 = rng.gen();
    //     v[i] = gen_value;
    // }
    //
    // let start_sort = Instant::now();
    // sorting::heap_sort(&mut v);
    // // sorting::quick_sort_recursive(&mut v);
    // let sort_duration = start_sort.elapsed();
    // println!("Time elapsed in sort_duration is: {:?}", sort_duration);
    // assert_eq!(sorting::is_sorted(&v), true);
}