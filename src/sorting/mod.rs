pub use self::bubble_sort::bubble_sort;
pub use self::cocktail_sort::cocktail_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::insertion_sort::insertion_sort_by_index;
pub use self::merge_sort::merge_by_index;
pub use self::merge_sort::merge_sort;
pub use self::merge_sort::merge_sort_iterator;
pub use self::quick_sort::quick_sort_dual_pivots;
pub use self::quick_sort::quick_sort_iterator;
pub use self::quick_sort::quick_sort_recursive;
pub use self::selection_sort::selection_sort;
pub use self::shell_sort::shell_sort;
pub use self::tim_sort::tim_sort;
pub use self::heap_sort::heap_sort;


mod bubble_sort;
mod selection_sort;
mod insertion_sort;
mod cocktail_sort;
mod shell_sort;
mod quick_sort;
mod merge_sort;
mod tim_sort;
mod heap_sort;

pub fn is_sorted<T>(arr: &[T]) -> bool
    where T: Ord
{
    if arr.is_empty() {
        return true;
    }

    let mut prev = &arr[0];
    for item in arr.iter().skip(1) {
        if prev > item {
            return false;
        }
        prev = &item;
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_sorted() {
        use super::*;

        assert!(is_sorted(&[] as &[isize]));
        assert!(is_sorted(&["a"]));
        assert!(is_sorted(&[1, 2, 3]));
        assert!(is_sorted(&[0, 1, 1]));

        assert_eq!(is_sorted(&[1, 0]), false);
        assert_eq!(is_sorted(&[2, 3, 1, -1, 5]), false);
    }

    #[test]
    fn random_sort() {
        use std::thread;
        use super::is_sorted;
        use super::selection_sort;
        use super::bubble_sort;
        use super::insertion_sort;
        use super::cocktail_sort;
        use super::shell_sort;
        use super::quick_sort_iterator;
        use super::quick_sort_recursive;
        use super::quick_sort_dual_pivots;
        use super::tim_sort;
        use super::heap_sort;
        use rand::Rng;
        use std::time::{Instant};
        let mut selection_v = [0; 50000];
        let mut bubble_v = [0; 50000];
        let mut insertion_v = [0; 50000];
        let mut cocktail_v = [0; 50000];
        let mut shell_v = [0; 50000];
        let mut quick_v = [0; 50000];
        let mut tim_v = [0; 50000];
        let mut heap_v = [0; 50000];

        for i in 0..50000 {
            let mut rng = rand::thread_rng();
            let gen_value: u16 = rng.gen();
            selection_v[i] = gen_value;
            bubble_v[i] = gen_value;
            insertion_v[i] = gen_value;
            cocktail_v[i] = gen_value;
            shell_v[i] = gen_value;
            quick_v[i] = gen_value;
            tim_v[i] = gen_value;
            heap_v[i] = gen_value;
        }


        let selection = thread::spawn(move || {
            // 选择排序
            let start_selection_sort = Instant::now();
            selection_sort(&mut selection_v);
            let selection_sort_duration = start_selection_sort.elapsed();
            println!("Time elapsed in selection_sort_duration is: {:?}", selection_sort_duration);
            assert_eq!(is_sorted(&selection_v), true);
        });


        let bubble = thread::spawn(move || {
            // 冒泡排序
            let start_bubble_sort = Instant::now();
            bubble_sort(&mut bubble_v);
            let bubble_sort_duration = start_bubble_sort.elapsed();
            println!("Time elapsed in bubble_sort_duration is: {:?}", bubble_sort_duration);
            assert_eq!(is_sorted(&bubble_v), true);
        });

        let insertion = thread::spawn(move || {
            // 插入排序
            let start_insertion_sort = Instant::now();
            insertion_sort(&mut insertion_v);
            let insertion_sort_duration = start_insertion_sort.elapsed();
            println!("Time elapsed in insertion_sort_duration is: {:?}", insertion_sort_duration);
            assert_eq!(is_sorted(&insertion_v), true);
        });
        //
        let cocktail = thread::spawn(move || {
            // 鸡尾酒排序
            let start_cocktail_sort = Instant::now();
            cocktail_sort(&mut cocktail_v);
            let cocktail_sort_duration = start_cocktail_sort.elapsed();
            println!("Time elapsed in cocktail_sort_duration is: {:?}", cocktail_sort_duration);
            assert_eq!(is_sorted(&cocktail_v), true);
        });
        //
        let shell = thread::spawn(move || {
            // 希尔排序
            let start_shell_sort = Instant::now();
            shell_sort(&mut shell_v);
            let shell_sort_duration = start_shell_sort.elapsed();
            println!("Time elapsed in shell_sort_duration is: {:?}", shell_sort_duration);
            assert_eq!(is_sorted(&shell_v), true);
        });

        let quick = thread::spawn(move || {
            // 快速排序
            let start_quick_sort = Instant::now();
            quick_sort_dual_pivots(&mut quick_v);
            let quick_sort_duration = start_quick_sort.elapsed();
            println!("Time elapsed in quick_sort_duration is: {:?}", quick_sort_duration);
            assert_eq!(is_sorted(&quick_v), true);
        });

        let tim = thread::spawn(move || {
            // TIM排序
            let start_tim_sort = Instant::now();
            tim_sort(&mut tim_v);
            let tim_sort_duration = start_tim_sort.elapsed();
            println!("Time elapsed in tim_sort_duration is: {:?}", tim_sort_duration);
            assert_eq!(is_sorted(&tim_v), true);
        });

        let heap = thread::spawn(move || {
            // 堆排序
            let start_heap_sort = Instant::now();
            heap_sort(&mut heap_v);
            let heap_sort_duration = start_heap_sort.elapsed();
            println!("Time elapsed in heap_sort_duration is: {:?}", heap_sort_duration);
            assert_eq!(is_sorted(&heap_v), true);
        });

        heap.join().unwrap();
        tim.join().unwrap();
        shell.join().unwrap();
        quick.join().unwrap();
        selection.join().unwrap();
        bubble.join().unwrap();
        insertion.join().unwrap();
        cocktail.join().unwrap();
    }

    #[test]
    fn few_uniques_sort() {
        use std::thread;
        use super::is_sorted;
        use super::selection_sort;
        use super::bubble_sort;
        use super::insertion_sort;
        use super::cocktail_sort;
        use super::shell_sort;
        use super::quick_sort_iterator;
        use super::quick_sort_recursive;
        use super::quick_sort_dual_pivots;
        use super::tim_sort;
        use super::heap_sort;

        use rand::Rng;
        use std::time::{Instant};
        let mut selection_v = [0; 50000];
        let mut bubble_v = [0; 50000];
        let mut insertion_v = [0; 50000];
        let mut cocktail_v = [0; 50000];
        let mut shell_v = [0; 50000];
        let mut quick_v = [0; 50000];
        let mut tim_v = [0; 50000];
        let mut heap_v = [0; 50000];

        for i in 0..50000 {
            let mut rng = rand::thread_rng();
            let gen_value: u8 = rng.gen_range(1, 100);
            selection_v[i] = gen_value;
            bubble_v[i] = gen_value;
            insertion_v[i] = gen_value;
            cocktail_v[i] = gen_value;
            shell_v[i] = gen_value;
            quick_v[i] = gen_value;
            tim_v[i] = gen_value;
            heap_v[i] = gen_value;
        }


        let selection = thread::spawn(move || {
            //     选择排序
            let start_selection_sort = Instant::now();
            selection_sort(&mut selection_v);
            let selection_sort_duration = start_selection_sort.elapsed();
            println!("Time elapsed in selection_sort_duration is: {:?}", selection_sort_duration);
            assert_eq!(is_sorted(&selection_v), true);
        });

        let bubble = thread::spawn(move || {
            // 冒泡排序
            let start_bubble_sort = Instant::now();
            bubble_sort(&mut bubble_v);
            let bubble_sort_duration = start_bubble_sort.elapsed();
            println!("Time elapsed in bubble_sort_duration is: {:?}", bubble_sort_duration);
            assert_eq!(is_sorted(&bubble_v), true);
        });

        let insertion = thread::spawn(move || {
            // 插入排序
            let start_insertion_sort = Instant::now();
            insertion_sort(&mut insertion_v);
            let insertion_sort_duration = start_insertion_sort.elapsed();
            println!("Time elapsed in insertion_sort_duration is: {:?}", insertion_sort_duration);
            assert_eq!(is_sorted(&insertion_v), true);
        });

        let cocktail = thread::spawn(move || {
            // 鸡尾酒排序
            let start_cocktail_sort = Instant::now();
            cocktail_sort(&mut cocktail_v);
            let cocktail_sort_duration = start_cocktail_sort.elapsed();
            println!("Time elapsed in cocktail_sort_duration is: {:?}", cocktail_sort_duration);
            assert_eq!(is_sorted(&cocktail_v), true);
        });

        let shell = thread::spawn(move || {
            // 希尔排序
            let start_shell_sort = Instant::now();
            shell_sort(&mut shell_v);
            let shell_sort_duration = start_shell_sort.elapsed();
            println!("Time elapsed in shell_sort_duration is: {:?}", shell_sort_duration);
            assert_eq!(is_sorted(&shell_v), true);
        });

        let quick = thread::spawn(move || {
            // 快速排序
            let start_quick_sort = Instant::now();
            quick_sort_dual_pivots(&mut quick_v);
            let quick_sort_duration = start_quick_sort.elapsed();
            println!("Time elapsed in quick_sort_duration is: {:?}", quick_sort_duration);
            assert_eq!(is_sorted(&quick_v), true);
        });

        let tim = thread::spawn(move || {
            // TIM排序
            let start_tim_sort = Instant::now();
            tim_sort(&mut tim_v);
            let tim_sort_duration = start_tim_sort.elapsed();
            println!("Time elapsed in tim_sort_duration is: {:?}", tim_sort_duration);
            assert_eq!(is_sorted(&tim_v), true);
        });

        let heap = thread::spawn(move || {
            // 堆排序
            let start_heap_sort = Instant::now();
            heap_sort(&mut heap_v);
            let heap_sort_duration = start_heap_sort.elapsed();
            println!("Time elapsed in heap_sort_duration is: {:?}", heap_sort_duration);
            assert_eq!(is_sorted(&heap_v), true);
        });

        heap.join().unwrap();
        tim.join().unwrap();
        selection.join().unwrap();
        bubble.join().unwrap();
        insertion.join().unwrap();
        cocktail.join().unwrap();
        shell.join().unwrap();
        quick.join().unwrap();
    }

    #[test]
    fn nearly_sorted_sort() {
        use std::thread;
        use super::is_sorted;
        use super::selection_sort;
        use super::bubble_sort;
        use super::insertion_sort;
        use super::cocktail_sort;
        use super::shell_sort;
        use super::quick_sort_iterator;
        use super::quick_sort_recursive;
        use super::quick_sort_dual_pivots;
        use super::tim_sort;
        use super::heap_sort;
        use rand::Rng;
        use std::time::{Instant};
        let mut selection_v = [0; 50000];
        let mut bubble_v = [0; 50000];
        let mut insertion_v = [0; 50000];
        let mut cocktail_v = [0; 50000];
        let mut shell_v = [0; 50000];
        let mut quick_v = [0; 50000];
        let mut tim_v = [0; 50000];
        let mut heap_v = [0; 50000];


        for i in 0..10000 {
            selection_v[i] = i as u16;
            bubble_v[i] = i as u16;
            insertion_v[i] = i as u16;
            cocktail_v[i] = i as u16;
            shell_v[i] = i as u16;
            quick_v[i] = i as u16;
            tim_v[i] = i as u16;
            heap_v[i] = i as u16;
        }

        for i in 10000..20000 {
            let mut rng = rand::thread_rng();
            let gen_value: u16 = rng.gen();
            selection_v[i] = gen_value;
            bubble_v[i] = gen_value;
            insertion_v[i] = gen_value;
            cocktail_v[i] = gen_value;
            shell_v[i] = gen_value;
            quick_v[i] = gen_value;
            tim_v[i] = gen_value;
            heap_v[i] = gen_value;
        }

        for i in 20000..50000 {
            selection_v[i] = i as u16;
            bubble_v[i] = i as u16;
            insertion_v[i] = i as u16;
            cocktail_v[i] = i as u16;
            shell_v[i] = i as u16;
            quick_v[i] = i as u16;
            tim_v[i] = i as u16;
            heap_v[i] = i as u16;
        }
        //
        let selection = thread::spawn(move || {
            // 选择排序
            let start_selection_sort = Instant::now();
            selection_sort(&mut selection_v);
            let selection_sort_duration = start_selection_sort.elapsed();
            println!("Time elapsed in selection_sort_duration is: {:?}", selection_sort_duration);
            assert_eq!(is_sorted(&selection_v), true);
        });

        let bubble = thread::spawn(move || {
            //     冒泡排序
            let start_bubble_sort = Instant::now();
            bubble_sort(&mut bubble_v);
            let bubble_sort_duration = start_bubble_sort.elapsed();
            println!("Time elapsed in bubble_sort_duration is: {:?}", bubble_sort_duration);
            assert_eq!(is_sorted(&bubble_v), true);
        });

        let insertion = thread::spawn(move || {
            // 插入排序
            let start_insertion_sort = Instant::now();
            insertion_sort(&mut insertion_v);
            let insertion_sort_duration = start_insertion_sort.elapsed();
            println!("Time elapsed in insertion_sort_duration is: {:?}", insertion_sort_duration);
            assert_eq!(is_sorted(&insertion_v), true);
        });

        let cocktail = thread::spawn(move || {
            // 鸡尾酒排序
            let start_cocktail_sort = Instant::now();
            cocktail_sort(&mut cocktail_v);
            let cocktail_sort_duration = start_cocktail_sort.elapsed();
            println!("Time elapsed in cocktail_sort_duration is: {:?}", cocktail_sort_duration);
            assert_eq!(is_sorted(&cocktail_v), true);
        });

        let shell = thread::spawn(move || {
            // 希尔排序
            let start_shell_sort = Instant::now();
            shell_sort(&mut shell_v);
            let shell_sort_duration = start_shell_sort.elapsed();
            println!("Time elapsed in shell_sort_duration is: {:?}", shell_sort_duration);
            assert_eq!(is_sorted(&shell_v), true);
        });

        let quick = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
            // 快速排序
            let start_quick_sort = Instant::now();
            quick_sort_dual_pivots(&mut quick_v);
            let quick_sort_duration = start_quick_sort.elapsed();
            println!("Time elapsed in quick_sort_duration is: {:?}", quick_sort_duration);
            assert_eq!(is_sorted(&quick_v), true);
        });

        let tim = thread::spawn(move || {
            // TIM排序
            let start_tim_sort = Instant::now();
            tim_sort(&mut tim_v);
            let tim_sort_duration = start_tim_sort.elapsed();
            println!("Time elapsed in tim_sort_duration is: {:?}", tim_sort_duration);
            assert_eq!(is_sorted(&tim_v), true);
        });

        let heap = thread::spawn(move || {
            // 堆排序
            let start_heap_sort = Instant::now();
            heap_sort(&mut heap_v);
            let heap_sort_duration = start_heap_sort.elapsed();
            println!("Time elapsed in heap_sort_duration is: {:?}", heap_sort_duration);
            assert_eq!(is_sorted(&heap_v), true);
        });

        heap.join().unwrap();

        tim.join().unwrap();

        selection.join().unwrap();
        bubble.join().unwrap();
        insertion.join().unwrap();
        cocktail.join().unwrap();
        shell.join().unwrap();
        quick.unwrap().join().unwrap();
    }


    #[test]
    fn sorted_sort() {
        use std::thread;
        use super::is_sorted;
        use super::selection_sort;
        use super::bubble_sort;
        use super::insertion_sort;
        use super::cocktail_sort;
        use super::shell_sort;
        use super::quick_sort_iterator;
        use super::quick_sort_recursive;
        use super::quick_sort_dual_pivots;
        use super::tim_sort;
        use super::heap_sort;
        use rand::Rng;
        use std::time::{Instant};
        let mut selection_v = [0; 50000];
        let mut bubble_v = [0; 50000];
        let mut insertion_v = [0; 50000];
        let mut cocktail_v = [0; 50000];
        let mut shell_v = [0; 50000];
        let mut quick_v = [0; 50000];
        let mut tim_v = [0; 50000];
        let mut heap_v = [0; 50000];


        for i in 0..50000 {
            selection_v[i] = i as u16;
            bubble_v[i] = i as u16;
            insertion_v[i] = i as u16;
            cocktail_v[i] = i as u16;
            shell_v[i] = i as u16;
            quick_v[i] = i as u16;
            tim_v[i] = i as u16;
            heap_v[i] = i as u16;
        }

        //
        let selection = thread::spawn(move || {
            // 选择排序
            let start_selection_sort = Instant::now();
            selection_sort(&mut selection_v);
            let selection_sort_duration = start_selection_sort.elapsed();
            println!("Time elapsed in selection_sort_duration is: {:?}", selection_sort_duration);
            assert_eq!(is_sorted(&selection_v), true);
        });

        let bubble = thread::spawn(move || {
            //     冒泡排序
            let start_bubble_sort = Instant::now();
            bubble_sort(&mut bubble_v);
            let bubble_sort_duration = start_bubble_sort.elapsed();
            println!("Time elapsed in bubble_sort_duration is: {:?}", bubble_sort_duration);
            assert_eq!(is_sorted(&bubble_v), true);
        });

        let insertion = thread::spawn(move || {
            // 插入排序
            let start_insertion_sort = Instant::now();
            insertion_sort(&mut insertion_v);
            let insertion_sort_duration = start_insertion_sort.elapsed();
            println!("Time elapsed in insertion_sort_duration is: {:?}", insertion_sort_duration);
            assert_eq!(is_sorted(&insertion_v), true);
        });

        let cocktail = thread::spawn(move || {
            // 鸡尾酒排序
            let start_cocktail_sort = Instant::now();
            cocktail_sort(&mut cocktail_v);
            let cocktail_sort_duration = start_cocktail_sort.elapsed();
            println!("Time elapsed in cocktail_sort_duration is: {:?}", cocktail_sort_duration);
            assert_eq!(is_sorted(&cocktail_v), true);
        });

        let shell = thread::spawn(move || {
            // 希尔排序
            let start_shell_sort = Instant::now();
            shell_sort(&mut shell_v);
            let shell_sort_duration = start_shell_sort.elapsed();
            println!("Time elapsed in shell_sort_duration is: {:?}", shell_sort_duration);
            assert_eq!(is_sorted(&shell_v), true);
        });

        let quick = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
            // 快速排序
            let start_quick_sort = Instant::now();
            quick_sort_dual_pivots(&mut quick_v);
            let quick_sort_duration = start_quick_sort.elapsed();
            println!("Time elapsed in quick_sort_duration is: {:?}", quick_sort_duration);
            assert_eq!(is_sorted(&quick_v), true);
        });

        let tim = thread::spawn(move || {
            // TIM排序
            let start_tim_sort = Instant::now();
            tim_sort(&mut tim_v);
            let tim_sort_duration = start_tim_sort.elapsed();
            println!("Time elapsed in tim_sort_duration is: {:?}", tim_sort_duration);
            assert_eq!(is_sorted(&tim_v), true);
        });

        let heap = thread::spawn(move || {
            // 堆排序
            let start_heap_sort = Instant::now();
            heap_sort(&mut heap_v);
            let heap_sort_duration = start_heap_sort.elapsed();
            println!("Time elapsed in heap_sort_duration is: {:?}", heap_sort_duration);
            assert_eq!(is_sorted(&heap_v), true);
        });

        heap.join().unwrap();

        tim.join().unwrap();

        selection.join().unwrap();
        bubble.join().unwrap();
        insertion.join().unwrap();
        cocktail.join().unwrap();
        shell.join().unwrap();
        quick.unwrap().join().unwrap();
    }

    #[test]
    fn reversed_sort() {
        use std::thread;
        use super::is_sorted;
        use super::selection_sort;
        use super::bubble_sort;
        use super::insertion_sort;
        use super::cocktail_sort;
        use super::shell_sort;
        use super::quick_sort_iterator;
        use super::quick_sort_recursive;
        use super::quick_sort_dual_pivots;
        use super::tim_sort;
        use super::heap_sort;
        use std::time::{Instant};
        let mut selection_v = [0; 50000];
        let mut bubble_v = [0; 50000];
        let mut insertion_v = [0; 50000];
        let mut cocktail_v = [0; 50000];
        let mut shell_v = [0; 50000];
        let mut quick_v = [0; 50000];
        let mut tim_v = [0; 50000];
        let mut heap_v = [0; 50000];

        for i in (0..50000).rev() {
            selection_v[i] = 50000 - i;
            bubble_v[i] = 50000 - i;
            insertion_v[i] = 50000 - i;
            cocktail_v[i] = 50000 - i;
            shell_v[i] = 50000 - i;
            quick_v[i] = 50000 - i;
            tim_v[i] = 50000 - i;
            heap_v[i] = 50000 - i;
        }

        let selection = thread::spawn(move || {
        // 选择排序
        let start_selection_sort = Instant::now();
        selection_sort(&mut selection_v);
        let selection_sort_duration = start_selection_sort.elapsed();
        println!("Time elapsed in selection_sort_duration is: {:?}", selection_sort_duration);
        assert_eq!(is_sorted(&selection_v), true);
        });


        let bubble = thread::spawn(move || {
        // 冒泡排序
        let start_bubble_sort = Instant::now();
        bubble_sort(&mut bubble_v);
        let bubble_sort_duration = start_bubble_sort.elapsed();
        println!("Time elapsed in bubble_sort_duration is: {:?}", bubble_sort_duration);
        assert_eq!(is_sorted(&bubble_v), true);
        });

        let insertion = thread::spawn(move || {
        // 插入排序
        let start_insertion_sort = Instant::now();
        insertion_sort(&mut insertion_v);
        let insertion_sort_duration = start_insertion_sort.elapsed();
        println!("Time elapsed in insertion_sort_duration is: {:?}", insertion_sort_duration);
        assert_eq!(is_sorted(&insertion_v), true);
        });

        let cocktail = thread::spawn(move || {
        // 鸡尾酒排序
        let start_cocktail_sort = Instant::now();
        cocktail_sort(&mut cocktail_v);
        let cocktail_sort_duration = start_cocktail_sort.elapsed();
        println!("Time elapsed in cocktail_sort_duration is: {:?}", cocktail_sort_duration);
        assert_eq!(is_sorted(&cocktail_v), true);
        });
        // 希尔排序
        let start_shell_sort = Instant::now();
        shell_sort(&mut shell_v);
        let shell_sort_duration = start_shell_sort.elapsed();
        println!("Time elapsed in shell_sort_duration is: {:?}", shell_sort_duration);
        assert_eq!(is_sorted(&shell_v), true);

        // 快速排序
        let quick = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
            // 快速排序
            let start_quick_sort = Instant::now();
            quick_sort_dual_pivots(&mut quick_v);
            let quick_sort_duration = start_quick_sort.elapsed();
            println!("Time elapsed in quick_sort_duration is: {:?}", quick_sort_duration);
            assert_eq!(is_sorted(&quick_v), true);
        });
        quick.unwrap().join().unwrap();


        let tim = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
            // 快速排序
            let start_tim_sort = Instant::now();
            tim_sort(&mut tim_v);
            let tim_sort_duration = start_tim_sort.elapsed();
            println!("Time elapsed in tim_sort_duration is: {:?}", tim_sort_duration);
            assert_eq!(is_sorted(&tim_v), true);
        });

        let heap = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
            // 堆排序
            let start_heap_sort = Instant::now();
            heap_sort(&mut heap_v);
            let heap_sort_duration = start_heap_sort.elapsed();
            println!("Time elapsed in heap_sort_duration is: {:?}", heap_sort_duration);
            assert_eq!(is_sorted(&heap_v), true);
        });

        heap.unwrap().join().unwrap();
        tim.unwrap().join().unwrap();
        selection.join().unwrap();
        bubble.join().unwrap();
        insertion.join().unwrap();
        cocktail.join().unwrap();
    }
}