# cho-run

## Command

```bash
make cho-run
```

## Output

```bash
make -C benchmarks cho-run
make[1]: Entering directory '/home/orazio/Projects/ripopt/benchmarks'
RESULTS_FILE=/home/orazio/Projects/ripopt/benchmarks/cho/cho_results.json \
CHO_NL_PATH=/home/orazio/Projects/ripopt/benchmarks/cho/nl_export_results/cho_parmest.nl \
cargo run --manifest-path /home/orazio/Projects/ripopt/Cargo.toml --release --features ipopt-native --example cho_benchmark \
        2> >(tee /home/orazio/Projects/ripopt/benchmarks/cho/cho_stderr.txt >&2)
   Compiling ripopt v0.8.0 (/home/orazio/Projects/ripopt)
    Finished `release` profile [optimized + debuginfo] target(s) in 31.36s
     Running `/home/orazio/Projects/ripopt/target/release/examples/cho_benchmark`

CHO Parameter Estimation Benchmark: ripopt vs ipopt
===================================================
Problem: 21672 variables, 21660 constraints

Problem              |   ripopt obj  iter  time(s) |    ipopt obj  iter  time(s)
--------------------------------------------------------------------------------
Streaming results to /home/orazio/Projects/ripopt/benchmarks/cho/cho_results.jsonl
CHO parmest          |     6.8179e4     1   643.31 |     6.7673e4    15    12.01
  status: ripopt=MaxTimeExceeded
--------------------------------------------------------------------------------
Results written to /home/orazio/Projects/ripopt/benchmarks/cho/cho_results.json
CHO benchmark complete.
make[1]: Leaving directory '/home/orazio/Projects/ripopt/benchmarks'
```
