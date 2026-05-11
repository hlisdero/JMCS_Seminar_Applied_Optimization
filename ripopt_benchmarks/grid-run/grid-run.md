# grid-run

## Command

```bash
make grid-run
```

## Output

```bash
make -C benchmarks grid-run
make[1]: Entering directory '/home/orazio/Projects/ripopt/benchmarks'
RESULTS_FILE=/home/orazio/Projects/ripopt/benchmarks/grid/grid_results.json \
cargo run --manifest-path /home/orazio/Projects/ripopt/Cargo.toml --release --features ipopt-native --example grid_benchmark \
        2> >(tee /home/orazio/Projects/ripopt/benchmarks/grid/grid_stderr.txt >&2)
   Compiling ripopt v0.8.0 (/home/orazio/Projects/ripopt)
    Finished `release` profile [optimized + debuginfo] target(s) in 36.23s
     Running `/home/orazio/Projects/ripopt/target/release/examples/grid_benchmark`

AC Optimal Power Flow Benchmark: ripopt vs ipopt
================================================

Problem                 n    m  nnz |   ripopt obj  iter  time(s) |    ipopt obj  iter  time(s)
-----------------------------------------------------------------------------------------------
Streaming results to /home/orazio/Projects/ripopt/benchmarks/grid/grid_results.jsonl
case3_lmbd             12   12   66 |      5812.64    10   0.0007 |      5812.64    10   0.0035
case5_pjm              20   22  126 |     17551.89    20   0.0029 |     17551.89    15   0.0053
case14_ieee            38   68  386 |      2178.08    14   0.0201 |      2178.08    11   0.0173
case30_ieee            72  142  788 |      8208.52    19   0.0961 |      8208.52    14   0.1289
  gap from known optimal: 1.57%
-----------------------------------------------------------------------------------------------
Results written to /home/orazio/Projects/ripopt/benchmarks/grid/grid_results.json
Grid benchmark complete.
make[1]: Leaving directory '/home/orazio/Projects/ripopt/benchmarks'
```
