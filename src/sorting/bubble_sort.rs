pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() - 1 {
        let mut swap = false;
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swap = true;
            }
        }
        if swap == false {
            break;
        }
    }
}