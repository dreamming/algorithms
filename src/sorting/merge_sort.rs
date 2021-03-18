use std::cmp::min;

pub fn merge_sort<T: Ord + std::marker::Copy>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut temp = arr.to_vec();
    _merge(&arr[..mid], &arr[mid..], &mut temp);
    arr.copy_from_slice(&temp);
}

pub fn merge_sort_iterator<T: Ord + std::marker::Copy + std::fmt::Debug>(arr: &mut [T]) {
    let mut temp = arr.to_vec();
    let mut width = 1;
    let len = arr.len();
    while width < len {
        for i in (0..len).step_by(2 * width) {
            let mid = min(i + width, len);
            let upper = min(i + 2 * width, len);
            _merge(&arr[i..mid], &arr[mid..upper], &mut temp[i..upper]);
            arr[i..upper].copy_from_slice(&temp[i..upper]);
        }
        width *= 2
    }
}

fn _merge<T: Ord + std::marker::Copy>(left: &[T], right: &[T], temp: &mut [T]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            temp[index] = left[left_index];
            index += 1;
            left_index += 1;
        } else {
            temp[index] = right[right_index];
            index += 1;
            right_index += 1;
        }
    }

    if left_index < left.len() {
        temp[index..].copy_from_slice(&left[left_index..])
    }

    if right_index < right.len() {
        temp[index..].copy_from_slice(&right[right_index..])
    }
}

pub fn merge_by_index<T: Ord + std::marker::Copy + std::fmt::Debug>(arr: &mut [T], lo: usize, mid: usize, hi: usize) {
    let mut tmp = arr[lo..=hi].to_vec();
    let mut index = 0;
    let mut left_index = lo;
    let mut right_index = mid + 1;
    while left_index <= mid && right_index <= hi {
        if arr[left_index] < arr[right_index] {
            tmp[index] = arr[left_index];
            left_index += 1;
            index += 1;
        } else {
            tmp[index] = arr[right_index];
            right_index += 1;
            index += 1;
        }
    }
    if left_index <= mid {
        tmp[index..].copy_from_slice(&arr[left_index..=mid]);
    }
    if right_index <= hi {
        tmp[index..].copy_from_slice(&arr[right_index..=hi]);
    }
    arr[lo..=hi].copy_from_slice(&tmp[..]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn merge_by_index_test() {
        let mut v = vec![1160, 202, 77, 99];
        merge_by_index(&mut v, 0, 1, 3);
        println!(" sorted arr is {:?}", v);
    }
}