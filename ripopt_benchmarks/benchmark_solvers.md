# benchmark_solvers

## Command

```bash
cargo run --example benchmark_solvers
```

## Output

```txt
Solver Benchmark: ripopt(faer) vs ipopt
=========================================================

Problem                   n      m | ripopt obj  iter  time(s)
--------------------------------------------------------------
--- Medium scale ---
Rosenbrock 500          500      0 | 1.4233e-14   750    1.415
Bratu 1K               1000    998 |   0.0000e0     1    0.027
OptControl 2.5K        2499   1250 |  1.1770e-1     1    0.152
Poisson 2.5K           2450   1225 |  9.9445e-2     1    0.363
SparseQP 1K             500    500 |  -1.2482e2     6    0.074
--------------------------------------------------------------
--- Large scale ---
Rosenbrock 5K          5000      0 |   2.9275e3  2999   60.398
  status: ripopt=MaxIterations
Bratu 10K             10000   9998 |   0.0000e0     0    0.031
OptControl 20K        19999  10000 |  1.1738e-1     1    1.426
Poisson 50K           49928  24964 |  9.9470e-2     1   16.763
SparseQP 100K         50000  50000 |  -1.2500e4     6    9.740
--------------------------------------------------------------
```
