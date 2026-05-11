# electrolyte-run

## Command

```bash
make electrolyte-run
```

## Output

```bash
make -C benchmarks electrolyte-run
make[1]: Entering directory '/home/orazio/Projects/ripopt/benchmarks'
RESULTS_FILE=/home/orazio/Projects/ripopt/benchmarks/electrolyte/electrolyte_results.json \
cargo run --manifest-path /home/orazio/Projects/ripopt/Cargo.toml --release --features ipopt-native --example electrolyte_benchmark \
        2> >(tee /home/orazio/Projects/ripopt/benchmarks/electrolyte/electrolyte_stderr.txt >&2)
   Compiling ripopt v0.8.0 (/home/orazio/Projects/ripopt)
    Finished `release` profile [optimized + debuginfo] target(s) in 3m 28s
     Running `/home/orazio/Projects/ripopt/target/release/examples/electrolyte_benchmark`

Electrolyte Thermodynamics Benchmark: ripopt vs ipopt
====================================================

Problem                     n   m |  ripopt obj  iter  time(s) |   ipopt obj  iter  time(s)
-------------------------------------------------------------------------------------------
--- Speciation / Chemical Equilibrium ---
Streaming results to /home/orazio/Projects/ripopt/benchmarks/electrolyte/electrolyte_results.jsonl
Water autoionization        1   0 |   3.7993e-7     7   0.0001 |   3.1004e-7     7   0.0018
CO2-water speciation        5   2 |   7.7559e-4    12   0.0002 |  -6.9337e-3    28   0.0070
NaCl speciation             4   3 |  -4.8327e-1     5   0.0002 |  -4.8327e-1     7   0.0018
CaCl2+NaCl mixed            6   4 |  -7.7237e-1  2253   0.0446 |  -7.7237e-1     9   0.0036
Phosphoric acid             6   2 |  -5.5312e-2     7   0.0002 |  -5.5312e-2     6   0.0024
--- Phase Equilibrium ---
HCl mean activity           1   0 |  8.5897e-16     7   0.0001 |  8.8924e-17     5   0.0017
NaCl solubility             1   0 |  1.6006e-17     7   0.0001 |  7.9733e-22     5   0.0017
BuOH-water LLE              2   2 |  7.8258e-10     6   0.0001 |  7.8258e-10     4   0.0015
Saturated brine             3   3 |    0.0000e0     6   0.0002 |    0.0000e0     4   0.0018
--- Parameter Fitting ---
Pitzer NaCl fit             3   0 |  6.0695e-15     6   0.0001 |  3.6776e-16     5   0.0019
Multi-salt DH fit           8   0 |  8.8853e-12   111   0.0019 |  4.4770e-12   247   0.0680
eNRTL T-dep fit             4   0 |  5.2853e-13    10   0.0002 |  3.7403e-12     8   0.0021
--- Scale-Up ---
Seawater speciation        15   8 |   -1.3483e0    24   0.0011 |   -1.3707e0    20   0.0077
  status: ipopt=Infeasible
-------------------------------------------------------------------------------------------
Results written to /home/orazio/Projects/ripopt/benchmarks/electrolyte/electrolyte_results.json
Electrolyte benchmark complete.
make[1]: Leaving directory '/home/orazio/Projects/ripopt/benchmarks'
```
