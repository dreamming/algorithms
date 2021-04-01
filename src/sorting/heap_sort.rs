pub fn heap_sort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    _heapify(arr);
    for i in (1..arr.len()).rev() {
        arr.swap(0, i);
        _move_down(&mut arr[..i], 0);
    }
}

fn _heapify<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    let last_parent_p = (arr.len() - 1) / 2;
    for i in (0..=last_parent_p).rev() {
        _move_down(arr, i);
    }
}

fn _move_down<T: Ord + std::fmt::Debug>(arr: &mut [T], mut root: usize) {
    let last_index = arr.len() - 1;
    loop {
        let left_index = 2 * root + 1;
        if left_index > last_index {
            break;
        }
        let right_index = left_index + 1;
        let max_index = if right_index <= last_index && arr[right_index] > arr[left_index] { // right_index > last_index 则会返回 left_index
            right_index
        } else {
            left_index
        };

        // let max_index = if right_index <= last_index && arr[left_index] > arr[right_index] { // 是错误的，程序会中止
        //     left_index
        // } else {
        //     right_index
        // };


        if arr[max_index] > arr[root] {
            arr.swap(max_index, root);
            root = max_index;
        } else {
            break;
        }
    }
}

