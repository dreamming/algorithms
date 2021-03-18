pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() - 1 {
        let mut smallest_index = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[smallest_index] {
                smallest_index = j;
            }
        }
        if smallest_index == i {
            continue;
        }
        arr.swap(i, smallest_index);
    }
}