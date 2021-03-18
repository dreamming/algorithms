use std::time::Instant;

fn _quick_sort_recursive<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = _Hoare_2way_partition(arr, lo, hi);
        let h = p.0;
        let l = p.1;
        _quick_sort_recursive(arr, lo, h);
        _quick_sort_recursive(arr, l, hi);
    }
}

// tail call optimization
fn _quick_sort_recursive_optimization<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    let mut left = lo;
    let mut right = hi;
    while left < right {
        let p = _Bentley_Mcilroy_3way_partition(arr, left, right);
        let h = p.0;
        let l = p.1;
        if h - left < right - l {
            _quick_sort_recursive_optimization(arr, left, h);
            left = l;
        } else {
            _quick_sort_recursive_optimization(arr, l, right);
            right = h;
        }
    }
}

fn _quick_sort_iterator<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let len = (hi - lo + 1) as usize;
        let mut stack = Vec::with_capacity(len);
        stack.push(lo);
        stack.push(hi);
        while !stack.is_empty() {
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();
            let p = _Bentley_Mcilroy_3way_partition(arr, left, right);
            let h = p.0;
            let l = p.1;
            if left < h {
                stack.push(left);
                stack.push(h);
            }
            if l < right {
                stack.push(l);
                stack.push(right);
            }
        }
    }
}

// *   left part    center part              right part
// * +-------------------------------------------------+
// * |  < pivot  |   == pivot   |     ?    |  > pivot  |
// * +-------------------------------------------------+
// *              ^              ^        ^
// *              |              |        |
// *             less            k      great
// *
// Dijkstra's 3-way (Dutch National Flag)
fn _Dijkstra_3way_partition<T: Ord>(arr: &mut [T], lo: isize, hi: isize) -> (isize, isize) {
    let pivot = hi;
    let median_index = _median_index(&arr, lo, hi);
    arr.swap(median_index as usize, hi as usize);

    let mut lt = lo;
    let mut mid = lo;
    let mut gt = hi;
    while mid < gt {
        if arr[mid as usize] < arr[pivot as usize] {
            arr.swap(mid as usize, lt as usize);
            mid += 1;
            lt += 1;
        } else if arr[mid as usize] > arr[pivot as usize] {
            gt -= 1;
            arr.swap(mid as usize, gt as usize);
        } else {
            mid += 1;
        }
    }
    arr.swap(mid as usize, pivot as usize);
    (lt - 1, mid)
}

// https://en.wikipedia.org/wiki/Jon_Bentley_(computer_scientist)
// Bentley-Mcilroy's 3-way
fn _Bentley_Mcilroy_3way_partition<T: Ord>(arr: &mut [T], lo: isize, hi: isize) -> (isize, isize) {
    let pivot = hi as usize;
    // median middle
    let median_index = _median_index(&arr, lo, hi);
    arr.swap(median_index as usize, hi as usize);

    let mut lt = lo - 1;
    let mut le_mid = lo - 1;
    let mut gt = hi;
    let mut re_mid = hi;
    loop {
        lt += 1;
        while lt <= hi && arr[lt as usize] < arr[pivot] {
            lt += 1;
        }

        gt -= 1;
        while gt >= 0 && arr[gt as usize] > arr[pivot] {
            gt -= 1;
        }

        if lt >= gt {
            break;
        }

        arr.swap(lt as usize, gt as usize);

        if arr[lt as usize] == arr[pivot as usize] {
            le_mid += 1;
            arr.swap(le_mid as usize, lt as usize);
        }

        if arr[gt as usize] == arr[pivot as usize] {
            re_mid -= 1;
            arr.swap(re_mid as usize, gt as usize);
        }
    }
    arr.swap(pivot as usize, lt as usize); // pivot 放置适当的位置

    lt = gt + 1;
    for i in lo..le_mid {
        arr.swap(i as usize, gt as usize);
        gt -= 1;
    }
    for j in (re_mid..hi).rev() {
        arr.swap(j as usize, lt as usize);
        lt += 1;
    }

    (gt, lt)
}

// https://cs.stackexchange.com/questions/11458/quicksort-partitioning-hoare-vs-lomuto
// 2-way partition
fn _Hoare_2way_partition<T: Ord>(arr: &mut [T], lo: isize, hi: isize) -> (isize, isize) {
    let pivot = hi as usize;
    // median middle
    let median_index = _median_index(&arr, lo, hi);
    arr.swap(median_index as usize, hi as usize);

    let mut lt = lo - 1;
    let mut gt = hi;
    loop {
        lt += 1;
        while lt <= hi && arr[lt as usize] < arr[pivot] {
            lt += 1;
        }

        gt -= 1;
        while gt >= 0 && arr[gt as usize] > arr[pivot] {
            gt -= 1;
        }

        if lt >= gt {
            break;
        }
        arr.swap(lt as usize, gt as usize);
    }
    arr.swap(pivot as usize, lt as usize); // pivot 放置适当的位置
    (lt - 1, lt + 1)
}

// 2-way partition
fn _Lomuto_2way_partition<T: Ord>(arr: &mut [T], lo: isize, hi: isize) -> (isize, isize) {
    let pivot = hi as usize;
    let median_index = _median_index(&arr, lo, hi);
    arr.swap(median_index as usize, hi as usize);
    let mut index = lo;
    for j in lo..hi {
        if arr[j as usize] < arr[pivot] {
            arr.swap(index as usize, j as usize);
            index += 1;
        }
    }
    arr.swap(index as usize, pivot as usize); // pivot 放置适当的位置
    (index - 1, index + 1)
}

fn _median_index<T: Ord>(arr: &[T], lo: isize, hi: isize) -> isize {
    // 序列的中间可使用 lo + (hi-lo)/2 OR (lo + hi )>>>1 防止溢出 ； (lo+hi)/2 会导致溢出
    // tukey ninther , 9N , median of medians
    let n = hi - lo + 1; // 防止溢位
    let esp = n / 8;
    let mid = lo + n / 2;
    let m1 = _median(arr, lo, lo + esp, lo + esp + esp);
    let m2 = _median(arr, mid - esp, mid, mid + esp);
    let m3 = _median(arr, hi - esp - esp, hi - esp, hi);
    let ninther = _median(arr, m1, m2, m3);
    ninther
}

fn _median<T: Ord>(arr: &[T], lo: isize, mid: isize, hi: isize) -> isize {
    // let middle_index = (lo + (hi - lo + 1) / 2) as usize;
    let first = &arr[lo as usize];
    let middle = &arr[mid as usize];
    let last = &arr[hi as usize];
    let median_index = if first > middle { if middle > last { mid } else if last > first { lo } else { mid } } else { if last > middle { mid } else if first > last { lo } else { hi } };
    median_index
}

pub fn quick_sort_recursive<T: Ord>(arr: &mut [T]) {
    let len = arr.len() - 1;
    _quick_sort_recursive_optimization(arr, 0, len as isize);
}

pub fn quick_sort_iterator<T: Ord>(arr: &mut [T]) {
    let len = arr.len() - 1;
    _quick_sort_iterator(arr, 0, len as isize);
}

pub fn quick_sort_dual_pivots<T: Ord>(arr: &mut [T]) {
    let len = arr.len() - 1;
    _quick_sort_dual_pivots(arr, 0, len as isize);
}

pub fn _quick_sort_dual_pivots<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = _quick_sort_partition_dual_pivots(arr, lo, hi);
        let h = p.0;
        let l = p.1;
        _quick_sort_dual_pivots(arr, lo, h - 1);
        _quick_sort_dual_pivots(arr, h + 1, l - 1);
        _quick_sort_dual_pivots(arr, l + 1, hi);
    }
}

pub fn _quick_sort_partition_dual_pivots<T: Ord>(arr: &mut [T], lo: isize, hi: isize) -> (isize, isize) {
    let mut middle = lo + 1;
    let mut lt = lo + 1;
    let mut gt = hi - 1;
    let less_pivot = lo;
    let great_pivot = hi;
    if arr[lo as usize] > arr[hi as usize] {
        arr.swap(lo as usize, hi as usize);
    }

    while middle <= gt {
        if arr[middle as usize] < arr[less_pivot as usize] {
            arr.swap(middle as usize, lt as usize);
            lt += 1;
        } else if arr[middle as usize] >= arr[great_pivot as usize] {
            while arr[gt as usize] > arr[great_pivot as usize] && middle < gt {
                gt -= 1;
            }
            arr.swap(middle as usize, gt as usize);
            gt -= 1;
            if arr[middle as usize] < arr[less_pivot as usize] {
                arr.swap(lt as usize, middle as usize);
                lt += 1;
            }
        }
        middle += 1;
    }

    lt -= 1;
    gt += 1;
    arr.swap(lt as usize, less_pivot as usize);
    arr.swap(gt as usize, great_pivot as usize);
    (middle, gt)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn median() {
        let v = vec![1160, 202, 77];
        let median = _median_index(&v[0..v.len()], 0, (v.len() - 1) as isize);
        println!(" median is {}", median);
    }
}



