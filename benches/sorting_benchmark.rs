use criterion::{black_box, criterion_group, criterion_main, Criterion};
use algorithms::{quick_sort_dual_pivot_random, quick_sort_recursive, shell_sort_random, insertion_sort_random, quick_sort_Bentley_Mcilroy_few_uniques, quick_sort_dual_pivot_few_uniques, insertion_sort_few_uniques, bubble_sort_few_uniques, merge_sort_random, merge_sort_iterator_random, quick_sort_iterator, tim_sort_random, quick_sort_sorted, tim_sort_sorted};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("quick_sort", |b| b.iter(|| tim_sort_sorted()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);