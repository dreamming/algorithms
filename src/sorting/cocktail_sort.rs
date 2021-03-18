pub fn cocktail_sort<T: Ord>(arr: &mut [T]) {
    let mut swaped = true;
    let mut top = arr.len()-1;
    let mut bottom = 0;
    while swaped {
        swaped = false;
        for i in bottom..top {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swaped = true;
            }
        }
        top = top - 1;
        for j in  ((bottom + 1)..=top).rev() {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                swaped = true;
            }
        }
        bottom = bottom + 1;
    }
}