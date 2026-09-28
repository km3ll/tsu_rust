#![allow(unused)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use p09_benchmark::sort_arr;

/**
 * pod: black_box function
 * - Prevents the compiler from optimizing away computations
 */
fn sort_arr_benchmark(c: &mut Criterion) {
    // Setup some data
    let mut arr: [i32; 7] = black_box([6, 2, 4, 1, 9, -2, 5]);

    // Create a benchmark
    c.bench_function(
        // ID
        "Sorting algorithm",
        // Closure
        |b| b.iter(|| sort_arr(&mut arr)),
    );
}

/**
 * pod: criterion_group!
 * - Macro to define a collection of functions to call
 *   with a common criterion configuration
 */
criterion_group!(benches, sort_arr_benchmark);

/**
 * pod: criterion_main!
 * - Macro that expands to a main function which runs all
 *   the benchmarks in a given group
 */
criterion_main!(benches);
