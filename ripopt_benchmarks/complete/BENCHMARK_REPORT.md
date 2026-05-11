# ripopt Benchmark Report

Generated: 2026-05-11 18:35:33

## Executive Summary

| Metric | ripopt | Ipopt |
|--------|--------|-------|
| Optimal | **562/745** (75.4%) | **574/745** (77.0%) |
| Acceptable | 16 | 3 |
| Total solved (Optimal + Acceptable) | 578 (77.6%) | 577 (77.4%) |
| Solved exclusively | 21 | 20 |
| Both solved | 557 | |
| Matching objectives (< 0.01%) | 537/557 | |
| Acceptable at worse local min | 4 | |

> **Note:** ripopt uses fallback strategies (L-BFGS Hessian, AL, SQP, slack
> reformulation) that Ipopt does not have, which accounts for much of the
> Acceptable count difference. The "Different Local Minima" section below
> lists Acceptable solutions where ripopt converged to a worse local minimum.

## Per-Suite Summary

| Suite | Problems | ripopt solved | Ipopt solved | ripopt only | Ipopt only | Both solved | Match |
|-------|----------|--------------|-------------|-------------|------------|------------|-------|
| CUTEst | 727 | 561 (77.2%) | 561 (77.2%) | 20 | 20 | 541 | 522/541 |
| Electrolyte | 13 | 13 (100.0%) | 12 (92.3%) | 1 | 0 | 12 | 11/12 |
| Grid | 4 | 4 (100.0%) | 4 (100.0%) | 0 | 0 | 4 | 4/4 |
| CHO | 1 | 0 (0.0%) | 0 (0.0%) | 0 | 0 | 0 | 0/1 |

## CUTEst Suite — Performance

On 541 commonly-solved problems:

| Metric | ripopt | Ipopt |
|--------|--------|-------|
| Median time | 289us | 2.9ms |
| Total time | 29.76s | 27.98s |
| Mean iterations | 38.6 | 37.1 |
| Median iterations | 13 | 12 |

- **Geometric mean speedup**: 7.5x
- **Median speedup**: 10.5x
- ripopt faster: 478/541 (88%)
- ripopt 10x+ faster: 288/541
- Ipopt faster: 63/541

## Electrolyte Suite — Performance

On 12 commonly-solved problems:

| Metric | ripopt | Ipopt |
|--------|--------|-------|
| Median time | 121us | 1.5ms |
| Total time | 30.1ms | 82.0ms |
| Mean iterations | 203.1 | 27.9 |
| Median iterations | 7 | 7 |

- **Geometric mean speedup**: 11.1x
- **Median speedup**: 14.8x
- ripopt faster: 11/12 (92%)
- ripopt 10x+ faster: 11/12
- Ipopt faster: 1/12

## Grid Suite — Performance

On 4 commonly-solved problems:

| Metric | ripopt | Ipopt |
|--------|--------|-------|
| Median time | 19.0ms | 16.2ms |
| Total time | 114.7ms | 157.4ms |
| Mean iterations | 15.8 | 12.5 |
| Median iterations | 19 | 14 |

- **Geometric mean speedup**: 1.8x
- **Median speedup**: 1.8x
- ripopt faster: 3/4 (75%)
- ripopt 10x+ faster: 0/4
- Ipopt faster: 1/4

## Failure Analysis

### CUTEst Suite

| Failure Mode | ripopt | Ipopt |
|-------------|--------|-------|
| ErrorInStepComputation | 0 | 2 |
| EvaluationError | 3 | 0 |
| Infeasible | 0 | 10 |
| InvalidNumberDetected | 0 | 1 |
| IpoptStatus(-10) | 0 | 123 |
| IpoptStatus(4) | 0 | 3 |
| LocalInfeasibility | 27 | 0 |
| MaxIterations | 22 | 12 |
| MaxTimeExceeded | 15 | 0 |
| NumericalError | 11 | 0 |
| RestorationFailed | 70 | 4 |
| StopAtTinyStep | 1 | 0 |
| Timeout | 17 | 11 |

### Electrolyte Suite

| Failure Mode | ripopt | Ipopt |
|-------------|--------|-------|
| Infeasible | 0 | 1 |

### CHO Suite

| Failure Mode | ripopt | Ipopt |
|-------------|--------|-------|
| MaxIter | 0 | 1 |
| MaxIterations | 1 | 0 |

## Regressions (Ipopt solves, ripopt fails)

| Problem | Suite | n | m | ripopt status | Ipopt obj |
|---------|-------|---|---|--------------|-----------|
| DECONVBNE | CUTEst | 63 | 40 | Timeout | 0.000000e+00 |
| DISCS | CUTEst | 36 | 66 | Timeout | 1.528822e+01 |
| HAIFAS | CUTEst | 13 | 9 | NumericalError | -4.500000e-01 |
| HATFLDFLNE | CUTEst | 3 | 3 | RestorationFailed | 0.000000e+00 |
| HIMMELP4 | CUTEst | 2 | 3 | NumericalError | -5.901318e+01 |
| HS59 | CUTEst | 2 | 3 | NumericalError | -6.749505e+00 |
| HS91 | CUTEst | 5 | 1 | NumericalError | 1.362646e+00 |
| LOGHAIRY | CUTEst | 2 | 0 | MaxIterations | 1.823216e-01 |
| OET6 | CUTEst | 5 | 1002 | NumericalError | 2.069727e-03 |
| OET7 | CUTEst | 7 | 1002 | NumericalError | 4.446419e-05 |
| PALMER1 | CUTEst | 4 | 0 | MaxIterations | 1.175460e+04 |
| PALMER2 | CUTEst | 4 | 0 | MaxIterations | 3.651098e+03 |
| PALMER3 | CUTEst | 4 | 0 | MaxIterations | 2.416980e+03 |
| PALMER4 | CUTEst | 4 | 0 | MaxIterations | 2.285383e+03 |
| PFIT3 | CUTEst | 3 | 3 | RestorationFailed | 0.000000e+00 |
| SIPOW2 | CUTEst | 2 | 2000 | MaxIterations | -1.000000e+00 |
| SSINE | CUTEst | 3 | 2 | MaxIterations | 0.000000e+00 |
| TAXR13322 | CUTEst | 72 | 1261 | MaxTimeExceeded | -6.449419e+04 |
| TRO3X3 | CUTEst | 30 | 13 | MaxIterations | 8.967215e+00 |
| WEEDS | CUTEst | 3 | 0 | MaxIterations | 2.587277e+00 |

## Wins (ripopt solves, Ipopt fails) — 21 problems

| Problem | Suite | n | m | Ipopt status | ripopt obj |
|---------|-------|---|---|-------------|------------|
| AVION2 | CUTEst | 49 | 15 | MaxIterations | 9.468013e+07 |
| BEALENE | CUTEst | 2 | 3 | IpoptStatus(-10) | 0.000000e+00 |
| BIGGS6NE | CUTEst | 6 | 13 | IpoptStatus(-10) | 0.000000e+00 |
| BOX3NE | CUTEst | 3 | 10 | IpoptStatus(-10) | 0.000000e+00 |
| BROWNBSNE | CUTEst | 2 | 3 | IpoptStatus(-10) | 0.000000e+00 |
| DECONVB | CUTEst | 63 | 0 | MaxIterations | 3.318319e-03 |
| DENSCHNBNE | CUTEst | 2 | 3 | IpoptStatus(-10) | 0.000000e+00 |
| DEVGLA1NE | CUTEst | 4 | 24 | IpoptStatus(-10) | 0.000000e+00 |
| DEVGLA2NE | CUTEst | 5 | 16 | IpoptStatus(-10) | 0.000000e+00 |
| ENGVAL2NE | CUTEst | 3 | 5 | IpoptStatus(-10) | 0.000000e+00 |
| EQC | CUTEst | 9 | 3 | ErrorInStepComputation | -8.293542e+02 |
| EXP2NE | CUTEst | 2 | 10 | IpoptStatus(-10) | 0.000000e+00 |
| GROUPING | CUTEst | 100 | 125 | IpoptStatus(-10) | 1.385040e+01 |
| HS25NE | CUTEst | 3 | 99 | IpoptStatus(-10) | 0.000000e+00 |
| LANCZOS1 | CUTEst | 6 | 24 | IpoptStatus(-10) | 0.000000e+00 |
| LEVYMONE5 | CUTEst | 2 | 4 | IpoptStatus(-10) | 0.000000e+00 |
| LEWISPOL | CUTEst | 6 | 9 | IpoptStatus(-10) | 2.999995e+00 |
| PFIT2 | CUTEst | 3 | 3 | RestorationFailed | 0.000000e+00 |
| PFIT4 | CUTEst | 3 | 3 | RestorationFailed | 0.000000e+00 |
| POLAK3 | CUTEst | 12 | 10 | MaxIterations | 5.933003e+00 |
| Seawater speciation | Electrolyte | 15 | 8 | Infeasible | -1.348272e+00 |

## Different Local Minima — 4 problems

ripopt converged (Acceptable) but to a different — usually worse — local
minimum than Ipopt found. Both solvers satisfied first-order KKT conditions
at their respective solutions. For nonconvex problems this is expected;
for convex problems it indicates the solver trajectory went astray.

| Problem | Suite | n | m | ripopt obj | Ipopt obj | Rel. error |
|---------|-------|---|---|------------|-----------|------------|
| MGH17LS | CUTEst | 5 | 0 | 1.022414e+00 | 7.895088e-05 | 100.0% |
| STREG | CUTEst | 4 | 0 | 4.526251e-01 | 8.901950e-02 | 36.4% |
| HS108 | CUTEst | 9 | 13 | -6.749814e-01 | -5.000000e-01 | 17.5% |
| HS54 | CUTEst | 6 | 1 | -8.652402e-01 | -9.080748e-01 | 4.3% |

## Acceptable (not Optimal) — 16 problems

These problems converged within relaxed tolerances but not strict tolerances.

| Problem | Suite | n | m | Ipopt status | ripopt obj | Ipopt obj |
|---------|-------|---|---|-------------|------------|-----------|
| ALLINITA | CUTEst | 4 | 4 | Optimal | 3.329878e+01 | 3.329611e+01 |
| ALLINITC | CUTEst | 4 | 1 | Optimal | 3.049223e+01 | 3.049261e+01 |
| BT8 | CUTEst | 5 | 2 | Optimal | 1.000000e+00 | 1.000000e+00 |
| DECONVU | CUTEst | 63 | 0 | Optimal | 1.823152e-12 | 1.120518e-15 |
| DENSCHNBNE | CUTEst | 2 | 3 | IpoptStatus(-10) | 0.000000e+00 | 0.000000e+00 |
| EQC | CUTEst | 9 | 3 | ErrorInStepComputation | -8.293542e+02 | -8.617556e+02 |
| HAIFAM | CUTEst | 99 | 150 | Optimal | -4.500036e+01 | -4.500036e+01 |
| HS108 | CUTEst | 9 | 13 | Optimal | -6.749814e-01 | -5.000000e-01 |
| HS54 | CUTEst | 6 | 1 | Optimal | -8.652402e-01 | -9.080748e-01 |
| HYDC20LS | CUTEst | 99 | 0 | Optimal | 2.250620e-16 | 3.621870e-15 |
| LSC2LS | CUTEst | 3 | 0 | Optimal | 1.334125e+01 | 1.333395e+01 |
| MEYER3 | CUTEst | 3 | 0 | Optimal | 8.794586e+01 | 8.794586e+01 |
| MGH17LS | CUTEst | 5 | 0 | Optimal | 1.022414e+00 | 7.895088e-05 |
| MISTAKE | CUTEst | 9 | 13 | Optimal | -1.000000e+00 | -1.000000e+00 |
| POLAK5 | CUTEst | 3 | 2 | Optimal | 5.000000e+01 | 5.000000e+01 |
| STREG | CUTEst | 4 | 0 | Optimal | 4.526251e-01 | 8.901950e-02 |

## Large-Scale Synthetic Problems — ripopt vs Ipopt

Synthetic problems with known structure, up to 100K variables.
Both solvers receive the exact same NlpProblem struct via the Rust trait interface.

| Problem | n | m | ripopt | iters | time | Ipopt | iters | time | speedup |
|---------|---|---|--------|-------|------|-------|-------|------|---------|
| Rosenbrock 500 | 500 | 0 | Optimal | 751 | 0.127s | Optimal | 749 | 0.280s | 2.2x |
| SparseQP 1K | 500 | 500 | Optimal | 6 | 0.007s | Optimal | 6 | 0.006s | 0.8x |
| Bratu 1K | 1,000 | 998 | Optimal | 2 | 0.004s | Optimal | 2 | 0.003s | 0.7x |
| OptControl 2.5K | 2,499 | 1,250 | Optimal | 1 | 0.017s | Optimal | 1 | 0.004s | 0.3x |
| Rosenbrock 5K | 5,000 | 0 | MaxIterations | 2999 | 5.340s | Failed | 3000 | 7.093s | 1.3x |
| Poisson 2.5K | 5,000 | 2,500 | Optimal | 1 | 0.104s | Optimal | 1 | 0.018s | 0.2x |
| Bratu 10K | 10,000 | 9,998 | Optimal | 1 | 0.039s | Optimal | 2 | 0.023s | 0.6x |
| OptControl 20K | 19,999 | 10,000 | Optimal | 1 | 0.167s | Optimal | 1 | 0.034s | 0.2x |
| Poisson 50K | 49,928 | 24,964 | Optimal | 1 | 2.416s | Optimal | 1 | 0.248s | 0.1x |
| SparseQP 100K | 50,000 | 50,000 | Optimal | 6 | 1.192s | Optimal | 6 | 0.638s | 0.5x |

ripopt: **9/10 solved** in 9.4s total
Ipopt: **9/10 solved** in 8.3s total

---
*Generated by benchmark_report.py*