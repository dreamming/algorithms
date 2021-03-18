pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        };
    }
}


pub fn insertion_sort_by_index<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    for i in lo + 1..=hi {
        let mut j = i;
        while j > lo && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    };
}


#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;
    use crate::sorting::is_sorted;

    #[test]
    fn insertion_sort_by_index_test() {
        let mut v = [0; 10000];
        for i in 0..v.len() {
            let mut rng = rand::thread_rng();
            let gen_value: u8 = rng.gen();
            v[i] = gen_value;
        }
        let len = v.len() - 1;
        insertion_sort_by_index(&mut v, 0, len);
        assert_eq!(is_sorted(&v), true);
    }
}
