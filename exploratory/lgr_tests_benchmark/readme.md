# lgr_tests_benchmark

Let's Get Rusty: Benchmarking

```
.
├── Cargo.lock
├── Cargo.toml
├── benches
│   └── sorting_benchmark.rs
├── readme.md
└── src
    └── lib.rs
```

Benchmark tests

```
# command
cargo bench

# bubble_sort
sorting algorithm       time:   [2.7459 ns 2.7609 ns 2.7794 ns]                               
                        change: [-6.4160% -5.0166% -3.5795%] (p = 0.00 < 0.05)
                        Performance has improved.

# selection_sort
sorting algorithm       time:   [8.7278 ns 8.8371 ns 8.9571 ns]                               
                        change: [+216.14% +220.23% +224.32%] (p = 0.00 < 0.05)
                        Performance has regressed.
```