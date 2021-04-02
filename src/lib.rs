use std::time::Instant;

use rand::Rng;

use crate::sorting::is_sorted;

pub mod sorting;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use rand::Rng;

    use crate::sorting::is_sorted;

    use super::sorting;

    #[test]
    fn bubble_sort_random() {
        let mut v = [0; 50000];
        for i in 0..v.len() {
            let mut rng = rand::thread_rng();
            let gen_value: u32 = rng.gen();
            v[i] = gen_value;
        }
        let start_sort = Instant::now();
        sorting::bubble_sort(&mut v);
        let sort_duration = start_sort.elapsed();
        println!("Time elapsed in sort_duration is: {:?}", sort_duration);
        assert_eq!(is_sorted(&v), true);
    }

    #[test]
    fn selection_sort_random() {
        let mut v = [0; 50000];
        for i in 0..v.len() {
            let mut rng = rand::thread_rng();
            let gen_value: u32 = rng.gen();
            v[i] = gen_value;
        }
        let start_sort = Instant::now();
        sorting::selection_sort(&mut v);
        let sort_duration = start_sort.elapsed();
        println!("Time elapsed in sort_duration is: {:?}", sort_duration);
        assert_eq!(is_sorted(&v), true);
    }

    #[test]
    fn insertion_sort_random() {
        let mut v = [0; 5000];
        for i in 0..v.len() {
            let mut rng = rand::thread_rng();
            let gen_value: u32 = rng.gen();
            v[i] = gen_value;
        }
        let start_sort = Instant::now();
        sorting::insertion_sort(&mut v);
        let sort_duration = start_sort.elapsed();
        println!("Time elapsed in sort_duration is: {:?}", sort_duration);
        assert_eq!(is_sorted(&v), true);
    }

    #[test]
    fn cocktail_sort_random() {
        let mut v = [0; 5000];
        for i in 0..v.len() {
            let mut rng = rand::thread_rng();
            let gen_value: u32 = rng.gen();
            v[i] = gen_value;
        }
        let start_sort = Instant::now();
        sorting::cocktail_sort(&mut v);
        let sort_duration = start_sort.elapsed();
        println!("Time elapsed in sort_duration is: {:?}", sort_duration);
        assert_eq!(is_sorted(&v), true);
    }

    #[test]
    fn shell_sort_random() {
        let mut v = [0; 200];
        for i in 0..v.len() {
            let mut rng = rand::thread_rng();
            let gen_value: u8 = rng.gen();
            v[i] = gen_value;
        }
        let start_sort = Instant::now();
        sorting::shell_sort(&mut v);
        let sort_duration = start_sort.elapsed();
        println!("Time elapsed in sort_duration is: {:?}", sort_duration);
        assert_eq!(is_sorted(&v), true);
    }

    #[test]
    fn quick_sort_dual_pivot_random() {
        let mut v = [0; 50000];
        for i in 0..v.len() {
            let mut rng = rand::thread_rng();
            let gen_value: u8 = rng.gen();
            v[i] = gen_value;
        }
        let start_sort = Instant::now();
        sorting::quick_sort_dual_pivots(&mut v);
        let sort_duration = start_sort.elapsed();
        println!("Time elapsed in sort_duration is: {:?}", sort_duration);
        assert_eq!(is_sorted(&v), true);
    }

    #[test]
    fn quick_sort_Bentley_Mcilroy_random() {
        let mut v = [0; 50000];
        for i in 0..v.len() {
            let mut rng = rand::thread_rng();
            let gen_value: u8 = rng.gen();
            v[i] = gen_value;
        }
        let start_sort = Instant::now();
        sorting::quick_sort_recursive(&mut v);
        let sort_duration = start_sort.elapsed();
        println!("Time elapsed in sort_duration is: {:?}", sort_duration);
        assert_eq!(is_sorted(&v), true);
    }


    #[test]
    fn merge_sort_random() {
        let mut v = [0; 10];
        for i in 0..v.len() {
            let mut rng = rand::thread_rng();
            let gen_value: u8 = rng.gen();
            v[i] = gen_value;
        }
        let start_sort = Instant::now();
        sorting::merge_sort_iterator(&mut v);
        let sort_duration = start_sort.elapsed();
        println!("Time elapsed in sort_duration is: {:?}", sort_duration);
        assert_eq!(is_sorted(&v), true);
    }

    #[test]
    fn tim_sort_random() {
        let mut v = [0; 50000];
        for i in 0..v.len() {
            let mut rng = rand::thread_rng();
            let gen_value: u8 = rng.gen();
            v[i] = gen_value;
        }
        let start_sort = Instant::now();
        sorting::tim_sort(&mut v);
        let sort_duration = start_sort.elapsed();
        println!("Time elapsed in sort_duration is: {:?}", sort_duration);
        assert_eq!(is_sorted(&v), true);
    }

    #[test]
    fn heap_sort_random() {
        let mut v = [0; 50000];
        for i in 0..v.len() {
            let mut rng = rand::thread_rng();
            let gen_value: u8 = rng.gen();
            v[i] = gen_value;
        }
        let start_sort = Instant::now();
        sorting::heap_sort(&mut v);
        let sort_duration = start_sort.elapsed();
        println!("Time elapsed in sort_duration is: {:?}", sort_duration);
        assert_eq!(is_sorted(&v), true);
    }
}

pub fn merge_sort_random() {
    let mut v = [0; 50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u8 = rng.gen();
        v[i] = gen_value;
    }
    sorting::merge_sort(&mut v);
}

pub fn merge_sort_iterator_random() {
    let mut v = [0; 50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u8 = rng.gen();
        v[i] = gen_value;
    }
    sorting::merge_sort_iterator(&mut v);
}

pub fn quick_sort_recursive() {
    let mut v = [0; 50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u8 = rng.gen();
        v[i] = gen_value;
    }
    sorting::quick_sort_recursive(&mut v);
}

pub fn quick_sort_iterator() {
    let mut v = [0; 50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u8 = rng.gen();
        v[i] = gen_value;
    }
    sorting::quick_sort_iterator(&mut v);
}

pub fn quick_sort_dual_pivot_random() {
    let mut v = [0; 50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u8 = rng.gen();
        v[i] = gen_value;
    }
    sorting::quick_sort_dual_pivots(&mut v);
}

pub fn shell_sort_random() {
    let mut v = [0; 50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u8 = rng.gen();
        v[i] = gen_value;
    }
    sorting::shell_sort(&mut v);
}

pub fn bubble_sort_random() {
    let mut v = [0; 50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u32 = rng.gen();
        v[i] = gen_value;
    }
    sorting::bubble_sort(&mut v);
}


pub fn selection_sort_random() {
    let mut v = [0; 50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u32 = rng.gen();
        v[i] = gen_value;
    }
    sorting::selection_sort(&mut v);
}

pub fn insertion_sort_random() {
    let mut v = [0; 50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u32 = rng.gen();
        v[i] = gen_value;
    }
    sorting::insertion_sort(&mut v);
}


pub fn cocktail_sort_random() {
    let mut v = [0; 50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u32 = rng.gen();
        v[i] = gen_value;
    }
    sorting::cocktail_sort(&mut v);
}

pub fn tim_sort_random() {
    let mut v = [0; 50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u32 = rng.gen();
        v[i] = gen_value;
    }
    sorting::tim_sort(&mut v);
}

pub fn heap_sort_random() {
    let mut v = [0; 50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u32 = rng.gen();
        v[i] = gen_value;
    }
    sorting::heap_sort(&mut v);
}

// FEW UNIQUES
pub fn quick_sort_Bentley_Mcilroy_few_uniques() {
    let mut v = [0; 50000];
    for i in 0..50000 {
        let mut rng = rand::thread_rng();
        let gen_value: u8 = rng.gen_range(1, 100);
        v[i] = gen_value;
    }
    sorting::quick_sort_recursive(&mut v);
}

pub fn quick_sort_dual_pivot_few_uniques() {
    let mut v = [0; 50000];
    for i in 0..50000 {
        let mut rng = rand::thread_rng();
        let gen_value: u8 = rng.gen_range(1, 100);
        v[i] = gen_value;
    }
    sorting::quick_sort_dual_pivots(&mut v);
}

pub fn quick_sort_random() {
    let mut v = [0; 50000];
    for i in 0..v.len() {
        let mut rng = rand::thread_rng();
        let gen_value: u32 = rng.gen();
        v[i] = gen_value;
    }
    sorting::quick_sort_recursive(&mut v);
}

pub fn shell_sort_few_uniques() {
    let mut v = [0; 50000];
    for i in 0..50000 {
        let mut rng = rand::thread_rng();
        let gen_value: u8 = rng.gen_range(1, 100);
        v[i] = gen_value;
    }
    sorting::shell_sort(&mut v);
}

pub fn bubble_sort_few_uniques() {
    let mut v = [0; 50000];
    for i in 0..50000 {
        let mut rng = rand::thread_rng();
        let gen_value: u8 = rng.gen_range(1, 100);
        v[i] = gen_value;
    }
    sorting::bubble_sort(&mut v);
}


pub fn selection_sort_few_uniques() {
    let mut v = [0; 50000];
    for i in 0..50000 {
        let mut rng = rand::thread_rng();
        let gen_value: u8 = rng.gen_range(1, 100);
        v[i] = gen_value;
    }
    sorting::selection_sort(&mut v);
}

pub fn insertion_sort_few_uniques() {
    let mut v = [0; 50000];
    for i in 0..50000 {
        let mut rng = rand::thread_rng();
        let gen_value: u8 = rng.gen_range(1, 100);
        v[i] = gen_value;
    }
    sorting::insertion_sort(&mut v);
}


pub fn cocktail_sort_few_uniques() {
    let mut v = [0; 50000];
    for i in 0..50000 {
        let mut rng = rand::thread_rng();
        let gen_value: u8 = rng.gen_range(1, 100);
        v[i] = gen_value;
    }
    sorting::cocktail_sort(&mut v);
}

pub fn quick_sort_sorted() {
    let mut v = [0; 50000];
    for i in 0..50000 {
        v[i] = i;
    }
    sorting::quick_sort_recursive(&mut v);
}

pub fn tim_sort_sorted() {
    let mut v = [0; 50000];
    for i in 0..50000 {
        v[i] = i;
    }
    sorting::tim_sort(&mut v);
}