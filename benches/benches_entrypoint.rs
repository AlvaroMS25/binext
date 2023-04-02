mod benchmarks;

use criterion::criterion_main;

criterion_main! {
    benchmarks::multiple_fields::multiple_fields,
    benchmarks::single_buffer::single_buffer,
}
