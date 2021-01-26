# Gamma Benchmark Iterator versus Slice

Compares the performance difference between `Graph` implementations using slice-based node id iteration versus a boxed `Iterator`.

It runs with nightly:

```bash
cargo +nightly bench
```