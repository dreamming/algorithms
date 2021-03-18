pub fn shell_sort<T: Ord>(arr: &mut [T]) {
    let mut step = arr.len() / 2;
    while step > 0 {
        for i in 0..step {
            for j in ((i + step)..arr.len()).step_by(step) {
                let mut k = j;
                while k >= step && arr[k] < arr[k - step] {
                    arr.swap(k, k - step);
                    k -= step;
                }
            }
        }
        step = step / 2;
    };
}