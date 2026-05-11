# CUTEst Benchmark Report

Comparison of ripopt vs Ipopt (C++) on the CUTEst test set.

## Executive Summary

- **Total problems**: 727
- **ripopt solved**: 561/727 (77.2%)
- **Ipopt solved**: 561/727 (77.2%)
- **Both solved**: 541/727
- **Matching solutions** (rel obj diff < 1e-4): 522/541

## Accuracy Statistics (where both solve)

Relative difference = |r_obj - i_obj| / max(|r_obj|, |i_obj|, 1.0).  
The 1.0 floor prevents near-zero objectives from inflating the metric.

**Matching solutions** (522 problems, rel diff < 1e-4):

| Metric | Rel Diff |
|--------|----------|
| Mean   | 1.97e-07 |
| Median | 1.80e-13 |
| Max    | 8.01e-05 |

**All both-solved** (541 problems, including 19 mismatches):

| Metric | Rel Diff |
|--------|----------|
| Mean   | 1.08e-02 |
| Median | 6.71e-13 |
| Max    | 1.00e+00 |

## Category Breakdown

| Category | Total | ripopt | Ipopt | Both | Match |
|----------|-------|--------|-------|------|-------|
| constrained | 493 | 348 | 343 | 329 | 317 |
| unconstrained | 234 | 213 | 218 | 212 | 205 |

## Detailed Results

| Problem | n | m | ripopt | Ipopt | Obj Diff | r_iter | i_iter | r_time | i_time | Speedup | Status |
|---------|---|---|--------|-------|----------|--------|--------|--------|--------|---------|--------|
| 3PK | 30 | 0 | Optimal | Optimal | 1.02e-12 | 9 | 9 | 469us | 2.3ms | 4.8x | PASS |
| ACOPP14 | 38 | 68 | Optimal | Optimal | 1.28e-10 | 14 | 9 | 41.4ms | 4.5ms | 0.1x | PASS |
| ACOPP30 | 72 | 142 | Optimal | Optimal | 1.26e-10 | 13 | 13 | 24.0ms | 9.5ms | 0.4x | PASS |
| ACOPR14 | 38 | 82 | Optimal | Optimal | 6.57e-11 | 28 | 13 | 23.9ms | 7.2ms | 0.3x | PASS |
| ACOPR30 | 72 | 172 | Optimal | Optimal | 1.01e-10 | 43 | 212 | 49.7ms | 164.4ms | 3.3x | PASS |
| AIRCRFTA | 8 | 5 | Optimal | Optimal | 0.00e+00 | 3 | 3 | 56us | 740us | 13.2x | PASS |
| AIRCRFTB | 8 | 0 | Optimal | Optimal | 2.88e-28 | 15 | 15 | 124us | 2.6ms | 20.9x | PASS |
| AIRPORT | 84 | 42 | Optimal | Optimal | 4.30e-12 | 13 | 13 | 25.0ms | 8.5ms | 0.3x | PASS |
| AKIVA | 2 | 0 | Optimal | Optimal | 0.00e+00 | 6 | 6 | 109us | 1.1ms | 9.7x | PASS |
| ALLINIT | 4 | 0 | Optimal | Optimal | 0.00e+00 | 23 | 20 | 219us | 4.5ms | 20.4x | PASS |
| ALLINITA | 4 | 4 | Acceptable | Optimal | 8.01e-05 | 51 | 12 | 1.0ms | 2.9ms | 2.8x | PASS |
| ALLINITC | 4 | 1 | Acceptable | Optimal | 1.25e-05 | 25 | 17 | 338us | 3.5ms | 10.3x | PASS |
| ALLINITU | 4 | 0 | Optimal | Optimal | 0.00e+00 | 14 | 14 | 100us | 2.8ms | 27.7x | PASS |
| ALSOTAME | 2 | 1 | Optimal | Optimal | 7.04e-09 | 8 | 8 | 142us | 1.9ms | 13.1x | PASS |
| ANTWERP | 27 | 10 | Optimal | Optimal | 6.88e-02 | 138 | 104 | 22.9ms | 34.8ms | 1.5x | MISMATCH |
| ARGAUSS | 3 | 15 | RestorationF | IpoptStatus( | N/A | 18 | 0 | 10.3ms | 114us | 0.0x | BOTH_FAIL |
| AVGASA | 8 | 10 | Optimal | Optimal | 5.39e-09 | 10 | 9 | 546us | 2.4ms | 4.4x | PASS |
| AVGASB | 8 | 10 | Optimal | Optimal | 5.89e-09 | 11 | 11 | 664us | 2.7ms | 4.1x | PASS |
| AVION2 | 49 | 15 | Optimal | MaxIteration | N/A | 90 | 3000 | 58.4ms | 1.10s | 18.9x | ipopt_FAIL |
| BA-L1 | 57 | 12 | Optimal | Optimal | 0.00e+00 | 6 | 6 | 1.5ms | 2.1ms | 1.4x | PASS |
| BA-L1LS | 57 | 0 | Optimal | Optimal | 1.59e-23 | 10 | 10 | 2.1ms | 3.1ms | 1.5x | PASS |
| BA-L1SP | 57 | 12 | Optimal | Optimal | 0.00e+00 | 5 | 5 | 3.3ms | 3.6ms | 1.1x | PASS |
| BA-L1SPLS | 57 | 0 | Optimal | Optimal | 3.38e-22 | 9 | 9 | 4.8ms | 6.2ms | 1.3x | PASS |
| BARD | 3 | 0 | Optimal | Optimal | 8.67e-18 | 8 | 8 | 60us | 1.5ms | 25.5x | PASS |
| BARDNE | 3 | 15 | RestorationF | IpoptStatus( | N/A | 13 | 0 | 10.0ms | 111us | 0.0x | BOTH_FAIL |
| BATCH | 48 | 73 | Optimal | Optimal | 2.15e-12 | 31 | 29 | 16.8ms | 11.0ms | 0.7x | PASS |
| BEALE | 2 | 0 | Optimal | Optimal | 0.00e+00 | 8 | 8 | 49us | 1.7ms | 35.4x | PASS |
| BEALENE | 2 | 3 | Optimal | IpoptStatus( | N/A | 25 | 0 | 401us | 109us | 0.3x | ipopt_FAIL |
| BENNETT5 | 3 | 154 | RestorationF | IpoptStatus( | N/A | 14 | 0 | 165.6ms | 139us | 0.0x | BOTH_FAIL |
| BENNETT5LS | 3 | 0 | Optimal | Optimal | 3.60e-15 | 21 | 21 | 1.1ms | 5.2ms | 4.7x | PASS |
| BIGGS3 | 6 | 0 | Optimal | Optimal | 0.00e+00 | 9 | 9 | 112us | 2.2ms | 19.1x | PASS |
| BIGGS5 | 6 | 0 | Optimal | Optimal | 3.69e-26 | 20 | 20 | 223us | 4.3ms | 19.3x | PASS |
| BIGGS6 | 6 | 0 | Optimal | Optimal | 2.91e-18 | 78 | 79 | 841us | 15.9ms | 18.9x | PASS |
| BIGGS6NE | 6 | 13 | Optimal | IpoptStatus( | N/A | 49 | 0 | 19.5ms | 113us | 0.0x | ipopt_FAIL |
| BIGGSC4 | 4 | 7 | Optimal | Optimal | 6.88e-10 | 17 | 17 | 635us | 3.9ms | 6.1x | PASS |
| BLEACHNG | 17 | 0 | Timeout | Timeout | N/A | 0 | 0 | 60.00s | 60.00s | 1.0x | BOTH_FAIL |
| BOOTH | 2 | 2 | Optimal | Optimal | 0.00e+00 | 1 | 1 | 14us | 322us | 22.6x | PASS |
| BOX2 | 3 | 0 | Optimal | Optimal | 0.00e+00 | 8 | 8 | 68us | 1.5ms | 21.8x | PASS |
| BOX3 | 3 | 0 | Optimal | Optimal | 3.76e-28 | 9 | 9 | 70us | 1.8ms | 25.0x | PASS |
| BOX3NE | 3 | 10 | Optimal | IpoptStatus( | N/A | 8 | 0 | 148us | 113us | 0.8x | ipopt_FAIL |
| BOXBOD | 2 | 6 | RestorationF | IpoptStatus( | N/A | 21 | 0 | 3.8ms | 113us | 0.0x | BOTH_FAIL |
| BOXBODLS | 2 | 0 | Optimal | Optimal | 0.00e+00 | 13 | 13 | 94us | 2.8ms | 29.6x | PASS |
| BQP1VAR | 1 | 0 | Optimal | Optimal | 5.15e-09 | 6 | 5 | 58us | 1.1ms | 18.7x | PASS |
| BQPGABIM | 50 | 0 | Optimal | Optimal | 5.20e-08 | 12 | 12 | 1.2ms | 3.5ms | 3.0x | PASS |
| BQPGASIM | 50 | 0 | Optimal | Optimal | 4.95e-08 | 12 | 12 | 1.4ms | 3.3ms | 2.3x | PASS |
| BRANIN | 2 | 0 | Optimal | Optimal | 0.00e+00 | 7 | 7 | 83us | 1.7ms | 20.9x | PASS |
| BRKMCC | 2 | 0 | Optimal | Optimal | 0.00e+00 | 3 | 3 | 22us | 583us | 27.1x | PASS |
| BROWNBS | 2 | 0 | Optimal | Optimal | 0.00e+00 | 7 | 7 | 37us | 1.2ms | 33.3x | PASS |
| BROWNBSNE | 2 | 3 | Optimal | IpoptStatus( | N/A | 4 | 0 | 613us | 123us | 0.2x | ipopt_FAIL |
| BROWNDEN | 4 | 0 | Optimal | Optimal | 0.00e+00 | 8 | 8 | 82us | 1.4ms | 17.2x | PASS |
| BROWNDENE | 4 | 20 | RestorationF | IpoptStatus( | N/A | 78 | 0 | 30.3ms | 113us | 0.0x | BOTH_FAIL |
| BT1 | 2 | 1 | Optimal | Optimal | 1.21e-09 | 7 | 7 | 96us | 1.5ms | 15.7x | PASS |
| BT10 | 2 | 2 | Optimal | Optimal | 2.79e-09 | 7 | 6 | 49us | 1.3ms | 27.1x | PASS |
| BT11 | 5 | 3 | Optimal | Optimal | 4.44e-16 | 8 | 8 | 97us | 1.4ms | 14.2x | PASS |
| BT12 | 5 | 3 | Optimal | Optimal | 0.00e+00 | 4 | 4 | 63us | 853us | 13.6x | PASS |
| BT13 | 5 | 1 | Optimal | Optimal | 4.99e-09 | 24 | 24 | 335us | 5.1ms | 15.3x | PASS |
| BT2 | 3 | 1 | Optimal | Optimal | 0.00e+00 | 12 | 12 | 96us | 1.8ms | 18.4x | PASS |
| BT3 | 5 | 3 | Optimal | Optimal | 9.33e-15 | 1 | 1 | 52us | 460us | 8.9x | PASS |
| BT4 | 3 | 2 | Optimal | Optimal | 1.20e-16 | 9 | 9 | 96us | 1.9ms | 20.3x | PASS |
| BT5 | 3 | 2 | Optimal | Optimal | 0.00e+00 | 7 | 7 | 77us | 1.3ms | 17.1x | PASS |
| BT6 | 5 | 2 | Optimal | Optimal | 1.09e-14 | 13 | 13 | 139us | 2.0ms | 14.4x | PASS |
| BT7 | 5 | 3 | Optimal | Optimal | 0.00e+00 | 16 | 16 | 176us | 3.1ms | 17.9x | PASS |
| BT8 | 5 | 2 | Acceptable | Optimal | 3.73e-09 | 66 | 14 | 583us | 3.2ms | 5.4x | PASS |
| BT9 | 4 | 2 | Optimal | Optimal | 0.00e+00 | 13 | 13 | 182us | 2.3ms | 12.5x | PASS |
| BURKEHAN | 1 | 1 | LocalInfeasi | Infeasible | N/A | 2999 | 11 | 245.9ms | 3.5ms | 0.0x | BOTH_FAIL |
| BYRDSPHR | 3 | 2 | Optimal | Optimal | 4.53e-13 | 18 | 12 | 160us | 2.5ms | 15.4x | PASS |
| CAMEL6 | 2 | 0 | Optimal | Optimal | 0.00e+00 | 8 | 8 | 94us | 2.0ms | 21.3x | PASS |
| CANTILVR | 5 | 1 | Optimal | Optimal | 3.74e-09 | 11 | 11 | 223us | 3.1ms | 14.1x | PASS |
| CB2 | 3 | 3 | Optimal | Optimal | 5.15e-09 | 8 | 8 | 174us | 1.8ms | 10.3x | PASS |
| CB3 | 3 | 3 | Optimal | Optimal | 7.18e-09 | 8 | 8 | 164us | 1.8ms | 11.2x | PASS |
| CERI651A | 7 | 61 | RestorationF | IpoptStatus( | N/A | 17 | 0 | 91.3ms | 129us | 0.0x | BOTH_FAIL |
| CERI651ALS | 7 | 0 | Optimal | Optimal | 4.67e-10 | 102 | 103 | 3.9ms | 21.3ms | 5.4x | PASS |
| CERI651B | 7 | 66 | RestorationF | IpoptStatus( | N/A | 13 | 0 | 115.2ms | 137us | 0.0x | BOTH_FAIL |
| CERI651BLS | 7 | 0 | Optimal | Optimal | 3.99e-09 | 53 | 55 | 2.3ms | 11.2ms | 4.8x | PASS |
| CERI651C | 7 | 56 | LocalInfeasi | IpoptStatus( | N/A | 347 | 0 | 1.84s | 131us | 0.0x | BOTH_FAIL |
| CERI651CLS | 7 | 0 | Optimal | Optimal | 2.09e-09 | 53 | 50 | 1.9ms | 9.4ms | 5.0x | PASS |
| CERI651D | 7 | 67 | RestorationF | IpoptStatus( | N/A | 30 | 0 | 125.8ms | 246us | 0.0x | BOTH_FAIL |
| CERI651DLS | 7 | 0 | Optimal | Optimal | 2.15e-09 | 48 | 61 | 2.0ms | 13.3ms | 6.5x | PASS |
| CERI651E | 7 | 64 | RestorationF | IpoptStatus( | N/A | 7 | 0 | 23.6ms | 135us | 0.0x | BOTH_FAIL |
| CERI651ELS | 7 | 0 | Optimal | Optimal | 2.68e-09 | 40 | 45 | 1.6ms | 8.6ms | 5.5x | PASS |
| CHACONN1 | 3 | 3 | Optimal | Optimal | 5.19e-09 | 7 | 6 | 151us | 1.5ms | 10.1x | PASS |
| CHACONN2 | 3 | 3 | Optimal | Optimal | 7.01e-09 | 8 | 6 | 162us | 1.4ms | 8.9x | PASS |
| CHWIRUT1 | 3 | 214 | RestorationF | IpoptStatus( | N/A | 8 | 0 | 254.2ms | 134us | 0.0x | BOTH_FAIL |
| CHWIRUT1LS | 3 | 0 | Optimal | Optimal | 5.72e-16 | 6 | 6 | 354us | 1.6ms | 4.6x | PASS |
| CHWIRUT2 | 3 | 54 | RestorationF | IpoptStatus( | N/A | 52 | 0 | 70.1ms | 120us | 0.0x | BOTH_FAIL |
| CHWIRUT2LS | 3 | 0 | Optimal | Optimal | 6.65e-16 | 6 | 6 | 105us | 1.6ms | 15.0x | PASS |
| CLIFF | 2 | 0 | Optimal | Optimal | 5.83e-16 | 23 | 23 | 112us | 3.3ms | 29.5x | PASS |
| CLUSTER | 2 | 2 | Optimal | Optimal | 0.00e+00 | 9 | 9 | 67us | 1.8ms | 27.4x | PASS |
| CLUSTERLS | 2 | 0 | Optimal | Optimal | 0.00e+00 | 17 | 17 | 93us | 2.9ms | 31.5x | PASS |
| CONCON | 15 | 11 | Optimal | Optimal | 3.25e-12 | 8 | 7 | 398us | 1.8ms | 4.6x | PASS |
| CONGIGMZ | 3 | 5 | Optimal | Optimal | 4.47e-10 | 23 | 20 | 499us | 4.5ms | 9.0x | PASS |
| COOLHANS | 9 | 9 | Optimal | Optimal | 0.00e+00 | 9 | 9 | 173us | 1.8ms | 10.2x | PASS |
| COOLHANSLS | 9 | 0 | Optimal | Optimal | 5.59e-22 | 25 | 25 | 330us | 5.0ms | 15.0x | PASS |
| CORE1 | 65 | 59 | Optimal | Optimal | 1.07e-09 | 31 | 31 | 13.9ms | 10.6ms | 0.8x | PASS |
| CRESC100 | 6 | 200 | MaxTimeExcee | Infeasible | N/A | 415 | 454 | 29.26s | 550.8ms | 0.0x | BOTH_FAIL |
| CRESC132 | 6 | 2654 | MaxTimeExcee | Timeout | N/A | 5 | 0 | 58.02s | 60.00s | 1.0x | BOTH_FAIL |
| CRESC4 | 6 | 8 | Optimal | Optimal | 3.19e-08 | 645 | 84 | 40.4ms | 25.4ms | 0.6x | PASS |
| CRESC50 | 6 | 100 | NumericalErr | Infeasible | N/A | 216 | 562 | 2.48s | 520.8ms | 0.2x | BOTH_FAIL |
| CSFI1 | 5 | 4 | Optimal | Optimal | 1.24e-10 | 24 | 11 | 521us | 2.7ms | 5.2x | PASS |
| CSFI2 | 5 | 4 | Optimal | Optimal | 3.84e-11 | 29 | 14 | 608us | 3.3ms | 5.5x | PASS |
| CUBE | 2 | 0 | Optimal | Optimal | 0.00e+00 | 27 | 27 | 120us | 4.4ms | 36.5x | PASS |
| CUBENE | 2 | 2 | Optimal | Optimal | 0.00e+00 | 1 | 1 | 17us | 361us | 21.4x | PASS |
| DALLASS | 46 | 31 | Optimal | Optimal | 1.18e-10 | 21 | 22 | 11.9ms | 6.6ms | 0.6x | PASS |
| DANIWOOD | 2 | 6 | RestorationF | IpoptStatus( | N/A | 24 | 0 | 2.8ms | 116us | 0.0x | BOTH_FAIL |
| DANIWOODLS | 2 | 0 | Optimal | Optimal | 8.67e-19 | 10 | 10 | 63us | 1.9ms | 30.7x | PASS |
| DANWOOD | 2 | 6 | RestorationF | IpoptStatus( | N/A | 20 | 0 | 3.0ms | 109us | 0.0x | BOTH_FAIL |
| DANWOODLS | 2 | 0 | Optimal | Optimal | 9.63e-17 | 11 | 11 | 74us | 2.1ms | 28.0x | PASS |
| DECONVB | 63 | 0 | Optimal | MaxIteration | N/A | 1392 | 3000 | 384.5ms | 989.8ms | 2.6x | ipopt_FAIL |
| DECONVBNE | 63 | 40 | Timeout | Optimal | N/A | 0 | 457 | 60.00s | 195.9ms | 0.0x | ripopt_FAIL |
| DECONVC | 63 | 1 | Optimal | Optimal | 2.57e-03 | 39 | 31 | 11.6ms | 12.1ms | 1.0x | MISMATCH |
| DECONVNE | 63 | 40 | Optimal | Acceptable | 0.00e+00 | 2 | 251 | 2.2ms | 369.9ms | 171.9x | PASS |
| DECONVU | 63 | 0 | Acceptable | Optimal | 1.82e-12 | 313 | 347 | 73.2ms | 127.6ms | 1.7x | PASS |
| DEGENLPA | 20 | 15 | Optimal | Optimal | 4.10e-07 | 30 | 18 | 1.8ms | 4.3ms | 2.4x | PASS |
| DEGENLPB | 20 | 15 | Optimal | Optimal | 1.07e-09 | 37 | 19 | 2.2ms | 4.6ms | 2.1x | PASS |
| DEMBO7 | 16 | 20 | Optimal | Optimal | 9.59e-11 | 203 | 45 | 28.3ms | 12.0ms | 0.4x | PASS |
| DEMYMALO | 3 | 3 | Optimal | Optimal | 5.96e-09 | 13 | 9 | 261us | 2.2ms | 8.4x | PASS |
| DENSCHNA | 2 | 0 | Optimal | Optimal | 5.88e-39 | 6 | 6 | 31us | 982us | 32.0x | PASS |
| DENSCHNB | 2 | 0 | Optimal | Optimal | 0.00e+00 | 7 | 7 | 42us | 1.3ms | 31.1x | PASS |
| DENSCHNBNE | 2 | 3 | Acceptable | IpoptStatus( | N/A | 46 | 0 | 375us | 107us | 0.3x | ipopt_FAIL |
| DENSCHNC | 2 | 0 | Optimal | Optimal | 0.00e+00 | 10 | 10 | 51us | 1.5ms | 29.3x | PASS |
| DENSCHNCNE | 2 | 2 | Optimal | Optimal | 0.00e+00 | 7 | 7 | 50us | 1.3ms | 25.3x | PASS |
| DENSCHND | 3 | 0 | Optimal | Optimal | 8.01e-17 | 26 | 26 | 145us | 4.5ms | 30.9x | PASS |
| DENSCHNDNE | 3 | 3 | Optimal | Acceptable | 0.00e+00 | 23 | 22 | 158us | 3.8ms | 23.9x | PASS |
| DENSCHNE | 3 | 0 | Optimal | Optimal | 1.92e-24 | 14 | 14 | 78us | 2.9ms | 37.7x | PASS |
| DENSCHNENE | 3 | 3 | LocalInfeasi | Infeasible | N/A | 12 | 10 | 1.1ms | 2.7ms | 2.5x | BOTH_FAIL |
| DENSCHNF | 2 | 0 | Optimal | Optimal | 0.00e+00 | 6 | 6 | 34us | 1.0ms | 29.6x | PASS |
| DENSCHNFNE | 2 | 2 | Optimal | Optimal | 0.00e+00 | 5 | 5 | 38us | 961us | 25.0x | PASS |
| DEVGLA1 | 4 | 0 | Optimal | Optimal | 2.42e-22 | 23 | 23 | 363us | 4.5ms | 12.4x | PASS |
| DEVGLA1B | 4 | 0 | Optimal | Optimal | 1.39e-16 | 20 | 20 | 446us | 5.3ms | 11.8x | PASS |
| DEVGLA1NE | 4 | 24 | Optimal | IpoptStatus( | N/A | 31 | 0 | 2.5ms | 114us | 0.0x | ipopt_FAIL |
| DEVGLA2 | 5 | 0 | Optimal | Optimal | 6.42e-24 | 13 | 13 | 244us | 2.5ms | 10.1x | PASS |
| DEVGLA2B | 5 | 0 | Optimal | Optimal | 3.11e-10 | 15 | 24 | 354us | 5.5ms | 15.7x | PASS |
| DEVGLA2NE | 5 | 16 | Optimal | IpoptStatus( | N/A | 16 | 0 | 1.2ms | 113us | 0.1x | ipopt_FAIL |
| DGOSPEC | 3 | 0 | Optimal | Optimal | 3.76e-13 | 28 | 27 | 283us | 6.1ms | 21.7x | PASS |
| DIAMON2D | 66 | 4643 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 21.5ms | 0.0x | BOTH_FAIL |
| DIAMON2DLS | 66 | 0 | MaxTimeExcee | Timeout | N/A | 420 | 0 | 30.25s | 60.00s | 2.0x | BOTH_FAIL |
| DIAMON3D | 99 | 4643 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 37.3ms | 0.0x | BOTH_FAIL |
| DIAMON3DLS | 99 | 0 | MaxTimeExcee | Timeout | N/A | 190 | 0 | 30.55s | 60.00s | 2.0x | BOTH_FAIL |
| DIPIGRI | 7 | 4 | Optimal | Optimal | 1.80e-11 | 9 | 9 | 275us | 2.1ms | 7.8x | PASS |
| DISC2 | 29 | 23 | Optimal | Optimal | 4.20e-09 | 34 | 24 | 6.9ms | 11.3ms | 1.6x | PASS |
| DISCS | 36 | 66 | Timeout | Optimal | N/A | 0 | 138 | 60.00s | 91.5ms | 0.0x | ripopt_FAIL |
| DIXCHLNG | 10 | 5 | Optimal | Optimal | 0.00e+00 | 10 | 10 | 225us | 1.8ms | 8.0x | PASS |
| DJTL | 2 | 0 | Optimal | Optimal | 0.00e+00 | 1927 | 1524 | 10.5ms | 195.9ms | 18.6x | PASS |
| DMN15102 | 66 | 4643 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 22.3ms | 0.0x | BOTH_FAIL |
| DMN15102LS | 66 | 0 | MaxTimeExcee | Timeout | N/A | 440 | 0 | 30.50s | 60.00s | 2.0x | BOTH_FAIL |
| DMN15103 | 99 | 4643 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 37.9ms | 0.0x | BOTH_FAIL |
| DMN15103LS | 99 | 0 | MaxTimeExcee | Timeout | N/A | 190 | 0 | 30.46s | 60.00s | 2.0x | BOTH_FAIL |
| DMN15332 | 66 | 4643 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 20.7ms | 0.0x | BOTH_FAIL |
| DMN15332LS | 66 | 0 | MaxTimeExcee | Timeout | N/A | 450 | 0 | 30.57s | 60.00s | 2.0x | BOTH_FAIL |
| DMN15333 | 99 | 4643 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 39.1ms | 0.0x | BOTH_FAIL |
| DMN15333LS | 99 | 0 | MaxTimeExcee | Timeout | N/A | 190 | 0 | 30.65s | 60.00s | 2.0x | BOTH_FAIL |
| DMN37142 | 66 | 4643 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 22.6ms | 0.0x | BOTH_FAIL |
| DMN37142LS | 66 | 0 | MaxTimeExcee | Timeout | N/A | 430 | 0 | 30.55s | 60.00s | 2.0x | BOTH_FAIL |
| DMN37143 | 99 | 4643 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 37.4ms | 0.0x | BOTH_FAIL |
| DMN37143LS | 99 | 0 | MaxTimeExcee | Timeout | N/A | 200 | 0 | 31.27s | 60.00s | 1.9x | BOTH_FAIL |
| DNIEPER | 61 | 24 | Optimal | Optimal | 1.34e-11 | 26 | 23 | 15.2ms | 6.3ms | 0.4x | PASS |
| DUAL1 | 85 | 1 | Optimal | Optimal | 1.11e-07 | 14 | 15 | 10.0ms | 8.5ms | 0.8x | PASS |
| DUAL2 | 96 | 1 | Optimal | Optimal | 2.00e-08 | 12 | 12 | 11.5ms | 8.0ms | 0.7x | PASS |
| DUAL4 | 75 | 1 | Optimal | Optimal | 6.54e-08 | 12 | 12 | 6.4ms | 6.0ms | 0.9x | PASS |
| DUALC1 | 9 | 215 | Optimal | Optimal | 1.71e-07 | 23 | 18 | 33.4ms | 18.9ms | 0.6x | PASS |
| DUALC2 | 7 | 229 | Optimal | Optimal | 1.36e-08 | 17 | 12 | 23.6ms | 13.1ms | 0.6x | PASS |
| DUALC5 | 8 | 278 | Optimal | Optimal | 5.61e-10 | 11 | 11 | 23.4ms | 14.5ms | 0.6x | PASS |
| DUALC8 | 8 | 503 | Optimal | Optimal | 2.45e-10 | 13 | 13 | 48.8ms | 27.5ms | 0.6x | PASS |
| ECKERLE4 | 3 | 35 | RestorationF | IpoptStatus( | N/A | 37 | 0 | 85.2ms | 108us | 0.0x | BOTH_FAIL |
| ECKERLE4LS | 3 | 0 | Optimal | Optimal | 4.34e-19 | 36 | 36 | 451us | 7.4ms | 16.4x | PASS |
| EG1 | 3 | 0 | Optimal | Optimal | 3.62e-09 | 8 | 8 | 93us | 1.9ms | 20.6x | PASS |
| EGGCRATE | 2 | 0 | Optimal | Optimal | 0.00e+00 | 5 | 5 | 31us | 1.1ms | 34.0x | PASS |
| EGGCRATEB | 2 | 0 | Optimal | Optimal | 5.62e-16 | 7 | 6 | 70us | 1.5ms | 21.9x | PASS |
| EGGCRATENE | 2 | 4 | RestorationF | IpoptStatus( | N/A | 22 | 0 | 1.6ms | 108us | 0.1x | BOTH_FAIL |
| ELATTAR | 7 | 102 | Optimal | Optimal | 9.98e-01 | 249 | 151 | 2.33s | 119.2ms | 0.1x | MISMATCH |
| ELATVIDU | 2 | 0 | Optimal | Optimal | 0.00e+00 | 11 | 11 | 53us | 1.6ms | 30.1x | PASS |
| ELATVIDUB | 2 | 0 | Optimal | Optimal | 2.60e-16 | 11 | 11 | 100us | 2.2ms | 21.6x | PASS |
| ELATVIDUNE | 2 | 3 | RestorationF | IpoptStatus( | N/A | 22 | 0 | 14.4ms | 110us | 0.0x | BOTH_FAIL |
| ENGVAL2 | 3 | 0 | Optimal | Optimal | 2.67e-26 | 21 | 21 | 119us | 3.7ms | 31.2x | PASS |
| ENGVAL2NE | 3 | 5 | Optimal | IpoptStatus( | N/A | 13 | 0 | 299us | 109us | 0.4x | ipopt_FAIL |
| ENSO | 9 | 168 | RestorationF | IpoptStatus( | N/A | 8 | 0 | 505.4ms | 185us | 0.0x | BOTH_FAIL |
| ENSOLS | 9 | 0 | Optimal | Optimal | 7.21e-16 | 7 | 7 | 1.3ms | 2.9ms | 2.2x | PASS |
| EQC | 9 | 3 | Acceptable | ErrorInStepC | N/A | 18 | 13 | 741us | 5.2ms | 7.0x | ipopt_FAIL |
| ERRINBAR | 18 | 9 | Optimal | Optimal | 1.30e-09 | 34 | 37 | 2.1ms | 9.7ms | 4.6x | PASS |
| EXP2 | 2 | 0 | Optimal | Optimal | 0.00e+00 | 7 | 7 | 50us | 1.3ms | 25.2x | PASS |
| EXP2B | 2 | 0 | Optimal | Optimal | 9.03e-18 | 8 | 7 | 98us | 1.5ms | 15.2x | PASS |
| EXP2NE | 2 | 10 | Optimal | IpoptStatus( | N/A | 24 | 0 | 650us | 109us | 0.2x | ipopt_FAIL |
| EXPFIT | 2 | 0 | Optimal | Optimal | 2.50e-16 | 8 | 8 | 57us | 1.6ms | 27.1x | PASS |
| EXPFITA | 5 | 22 | Optimal | Optimal | 1.53e-07 | 13 | 13 | 1.3ms | 3.5ms | 2.6x | PASS |
| EXPFITB | 5 | 102 | Optimal | Optimal | 8.10e-07 | 16 | 16 | 41.9ms | 8.2ms | 0.2x | PASS |
| EXPFITC | 5 | 502 | Optimal | Optimal | 4.44e-06 | 18 | 18 | 43.0ms | 33.0ms | 0.8x | PASS |
| EXPFITNE | 2 | 10 | RestorationF | IpoptStatus( | N/A | 15 | 0 | 3.7ms | 103us | 0.0x | BOTH_FAIL |
| EXTRASIM | 2 | 1 | Optimal | Optimal | 5.06e-09 | 4 | 3 | 108us | 830us | 7.7x | PASS |
| FBRAIN | 2 | 2211 | MaxTimeExcee | IpoptStatus( | N/A | 5 | 0 | 36.67s | 435us | 0.0x | BOTH_FAIL |
| FBRAIN2 | 4 | 2211 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 734us | 0.0x | BOTH_FAIL |
| FBRAIN2LS | 4 | 0 | Optimal | Optimal | 5.68e-09 | 20 | 10 | 29.7ms | 16.8ms | 0.6x | PASS |
| FBRAIN2NE | 4 | 2211 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 781us | 0.0x | BOTH_FAIL |
| FBRAIN3 | 6 | 2211 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 1.2ms | 0.0x | BOTH_FAIL |
| FBRAIN3LS | 6 | 0 | MaxIteration | MaxIteration | N/A | 2999 | 3000 | 7.09s | 7.86s | 1.1x | BOTH_FAIL |
| FBRAINLS | 2 | 0 | Optimal | Optimal | 1.11e-15 | 7 | 7 | 6.1ms | 6.4ms | 1.1x | PASS |
| FBRAINNE | 2 | 2211 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 431us | 0.0x | BOTH_FAIL |
| FCCU | 19 | 8 | Optimal | Optimal | 1.12e-15 | 9 | 9 | 457us | 2.1ms | 4.7x | PASS |
| FEEDLOC | 90 | 259 | Optimal | Optimal | 6.05e-09 | 347 | 23 | 2.01s | 31.2ms | 0.0x | PASS |
| FLETCHER | 4 | 4 | Optimal | Optimal | 5.45e-10 | 97 | 28 | 1.9ms | 6.9ms | 3.6x | PASS |
| FLT | 2 | 2 | Optimal | Optimal | 0.00e+00 | 6 | 5 | 81us | 1.4ms | 17.1x | PASS |
| GAUSS1 | 8 | 250 | RestorationF | IpoptStatus( | N/A | 8 | 0 | 460.3ms | 181us | 0.0x | BOTH_FAIL |
| GAUSS1LS | 8 | 0 | Optimal | Optimal | 0.00e+00 | 5 | 5 | 816us | 1.5ms | 1.9x | PASS |
| GAUSS2 | 8 | 250 | RestorationF | IpoptStatus( | N/A | 8 | 0 | 477.3ms | 175us | 0.0x | BOTH_FAIL |
| GAUSS2LS | 8 | 0 | Optimal | Optimal | 0.00e+00 | 5 | 5 | 793us | 1.5ms | 1.9x | PASS |
| GAUSS3 | 8 | 250 | RestorationF | IpoptStatus( | N/A | 11 | 0 | 538.2ms | 172us | 0.0x | BOTH_FAIL |
| GAUSS3LS | 8 | 0 | Optimal | Optimal | 0.00e+00 | 11 | 11 | 1.6ms | 4.0ms | 2.5x | PASS |
| GAUSSIAN | 3 | 0 | Optimal | Optimal | 0.00e+00 | 2 | 2 | 22us | 456us | 20.4x | PASS |
| GBRAIN | 2 | 2200 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 451us | 0.0x | BOTH_FAIL |
| GBRAINLS | 2 | 0 | Optimal | Optimal | 0.00e+00 | 6 | 6 | 5.5ms | 5.7ms | 1.0x | PASS |
| GENHS28 | 10 | 8 | Optimal | Optimal | 2.22e-16 | 1 | 1 | 71us | 416us | 5.9x | PASS |
| GIGOMEZ1 | 3 | 3 | Optimal | Optimal | 4.99e-09 | 26 | 13 | 448us | 2.9ms | 6.4x | PASS |
| GIGOMEZ2 | 3 | 3 | Optimal | Optimal | 6.99e-09 | 7 | 7 | 147us | 1.6ms | 10.8x | PASS |
| GIGOMEZ3 | 3 | 3 | Optimal | Optimal | 7.48e-09 | 8 | 8 | 168us | 1.8ms | 10.7x | PASS |
| GOFFIN | 51 | 50 | Optimal | Optimal | 3.16e-07 | 8 | 6 | 23.8ms | 5.1ms | 0.2x | PASS |
| GOTTFR | 2 | 2 | Optimal | Optimal | 0.00e+00 | 5 | 5 | 40us | 1.0ms | 24.8x | PASS |
| GOULDQP1 | 32 | 17 | Optimal | Optimal | 2.17e-10 | 15 | 15 | 2.3ms | 4.2ms | 1.8x | PASS |
| GROUPING | 100 | 125 | Optimal | IpoptStatus( | N/A | 5 | 0 | 102.7ms | 127us | 0.0x | ipopt_FAIL |
| GROWTH | 3 | 12 | RestorationF | IpoptStatus( | N/A | 64 | 0 | 7.8ms | 109us | 0.0x | BOTH_FAIL |
| GROWTHLS | 3 | 0 | Optimal | Optimal | 5.97e-15 | 71 | 71 | 553us | 12.7ms | 23.0x | PASS |
| GULF | 3 | 0 | Optimal | Optimal | 0.00e+00 | 28 | 28 | 1.7ms | 6.5ms | 3.9x | PASS |
| GULFNE | 3 | 99 | NumericalErr | IpoptStatus( | N/A | 32 | 0 | 116.9ms | 133us | 0.0x | BOTH_FAIL |
| HAHN1 | 7 | 236 | LocalInfeasi | IpoptStatus( | N/A | 37 | 0 | 4.60s | 154us | 0.0x | BOTH_FAIL |
| HAHN1LS | 7 | 0 | Optimal | Optimal | 1.70e-15 | 78 | 78 | 7.3ms | 22.6ms | 3.1x | PASS |
| HAIFAM | 99 | 150 | Acceptable | Optimal | 5.06e-09 | 95 | 40 | 319.6ms | 25.3ms | 0.1x | PASS |
| HAIFAS | 13 | 9 | NumericalErr | Optimal | N/A | 53 | 16 | 2.8ms | 4.2ms | 1.5x | ripopt_FAIL |
| HAIRY | 2 | 0 | Optimal | Optimal | 0.00e+00 | 59 | 59 | 319us | 11.1ms | 34.7x | PASS |
| HALDMADS | 6 | 42 | Optimal | Optimal | 9.63e-04 | 75 | 25 | 39.5ms | 10.1ms | 0.3x | MISMATCH |
| HART6 | 6 | 0 | Optimal | Optimal | 1.34e-16 | 8 | 7 | 122us | 1.8ms | 14.6x | PASS |
| HATFLDA | 4 | 0 | Optimal | Optimal | 2.43e-15 | 14 | 13 | 146us | 2.5ms | 17.4x | PASS |
| HATFLDANE | 4 | 4 | Optimal | Optimal | 0.00e+00 | 8 | 6 | 147us | 1.5ms | 10.0x | PASS |
| HATFLDB | 4 | 0 | Optimal | Optimal | 5.26e-09 | 9 | 8 | 102us | 1.7ms | 16.3x | PASS |
| HATFLDBNE | 4 | 4 | MaxIteration | Infeasible | N/A | 2999 | 13 | 120.7ms | 3.5ms | 0.0x | BOTH_FAIL |
| HATFLDC | 25 | 0 | Optimal | Optimal | 1.98e-16 | 5 | 5 | 200us | 1.3ms | 6.4x | PASS |
| HATFLDCNE | 25 | 25 | Optimal | Optimal | 0.00e+00 | 5 | 4 | 615us | 1.2ms | 2.0x | PASS |
| HATFLDD | 3 | 0 | Optimal | Optimal | 1.02e-21 | 21 | 21 | 156us | 3.4ms | 22.0x | PASS |
| HATFLDDNE | 3 | 10 | LocalInfeasi | IpoptStatus( | N/A | 1236 | 0 | 108.2ms | 104us | 0.0x | BOTH_FAIL |
| HATFLDE | 3 | 0 | Optimal | Optimal | 6.31e-19 | 20 | 20 | 193us | 3.3ms | 16.9x | PASS |
| HATFLDENE | 3 | 21 | LocalInfeasi | IpoptStatus( | N/A | 1985 | 0 | 580.1ms | 106us | 0.0x | BOTH_FAIL |
| HATFLDF | 3 | 3 | Optimal | Optimal | 0.00e+00 | 114 | 135 | 2.9ms | 28.5ms | 10.0x | PASS |
| HATFLDFL | 3 | 0 | Optimal | Optimal | 1.05e-08 | 498 | 1238 | 2.5ms | 202.6ms | 82.5x | PASS |
| HATFLDFLNE | 3 | 3 | RestorationF | Optimal | N/A | 413 | 15 | 7.1ms | 3.2ms | 0.5x | ripopt_FAIL |
| HATFLDFLS | 3 | 0 | Optimal | Optimal | 3.98e-25 | 36 | 36 | 186us | 6.4ms | 34.2x | PASS |
| HATFLDG | 25 | 25 | Optimal | Optimal | 0.00e+00 | 7 | 7 | 448us | 1.7ms | 3.8x | PASS |
| HATFLDGLS | 25 | 0 | Optimal | Optimal | 1.35e-31 | 14 | 14 | 351us | 2.8ms | 8.1x | PASS |
| HATFLDH | 4 | 7 | Optimal | Optimal | 6.90e-10 | 17 | 17 | 594us | 3.8ms | 6.4x | PASS |
| HEART6 | 6 | 6 | Optimal | Optimal | 0.00e+00 | 20 | 22 | 1.7ms | 7.3ms | 4.2x | PASS |
| HEART6LS | 6 | 0 | Optimal | Optimal | 3.14e-23 | 888 | 884 | 7.5ms | 177.4ms | 23.7x | PASS |
| HEART8 | 8 | 8 | Optimal | Optimal | 0.00e+00 | 13 | 12 | 587us | 2.8ms | 4.7x | PASS |
| HEART8LS | 8 | 0 | Optimal | Optimal | 3.65e-29 | 106 | 106 | 1.0ms | 21.9ms | 21.3x | PASS |
| HELIX | 3 | 0 | Optimal | Optimal | 1.08e-33 | 13 | 13 | 75us | 2.4ms | 31.7x | PASS |
| HELIXNE | 3 | 3 | Optimal | Optimal | 0.00e+00 | 7 | 7 | 59us | 1.4ms | 23.0x | PASS |
| HET-Z | 2 | 1002 | Optimal | Optimal | 8.98e-10 | 15 | 11 | 52.9ms | 39.3ms | 0.7x | PASS |
| HIELOW | 3 | 0 | Optimal | Optimal | 6.50e-16 | 8 | 8 | 16.4ms | 16.1ms | 1.0x | PASS |
| HIMMELBA | 2 | 2 | Optimal | Optimal | 0.00e+00 | 1 | 1 | 14us | 342us | 24.8x | PASS |
| HIMMELBB | 2 | 0 | Optimal | Optimal | 4.61e-25 | 18 | 18 | 90us | 3.3ms | 37.3x | PASS |
| HIMMELBC | 2 | 2 | Optimal | Optimal | 0.00e+00 | 6 | 6 | 44us | 1.1ms | 25.7x | PASS |
| HIMMELBCLS | 2 | 0 | Optimal | Optimal | 0.00e+00 | 6 | 6 | 32us | 1.2ms | 36.5x | PASS |
| HIMMELBD | 2 | 2 | RestorationF | Infeasible | N/A | 18 | 22 | 1.4ms | 6.4ms | 4.5x | BOTH_FAIL |
| HIMMELBE | 3 | 3 | Optimal | Optimal | 0.00e+00 | 2 | 2 | 25us | 467us | 18.6x | PASS |
| HIMMELBF | 4 | 0 | Optimal | Optimal | 8.92e-16 | 75 | 75 | 498us | 14.0ms | 28.1x | PASS |
| HIMMELBFNE | 4 | 7 | RestorationF | IpoptStatus( | N/A | 38 | 0 | 183.8ms | 114us | 0.0x | BOTH_FAIL |
| HIMMELBG | 2 | 0 | Optimal | Optimal | 1.72e-32 | 6 | 6 | 32us | 1.3ms | 41.8x | PASS |
| HIMMELBH | 2 | 0 | Optimal | Optimal | 0.00e+00 | 4 | 4 | 29us | 1.0ms | 35.7x | PASS |
| HIMMELBI | 100 | 12 | Optimal | Optimal | 2.74e-10 | 13 | 13 | 5.6ms | 5.3ms | 0.9x | PASS |
| HIMMELBJ | 45 | 14 | NumericalErr | ErrorInStepC | N/A | 181 | 619 | 29.1ms | 201.1ms | 6.9x | BOTH_FAIL |
| HIMMELBK | 24 | 14 | Optimal | Optimal | 9.18e-08 | 14 | 18 | 1.7ms | 5.0ms | 2.9x | PASS |
| HIMMELP1 | 2 | 0 | Optimal | Optimal | 3.66e-15 | 10 | 10 | 112us | 2.4ms | 21.4x | PASS |
| HIMMELP2 | 2 | 1 | Optimal | Optimal | 3.12e-10 | 19 | 17 | 339us | 4.9ms | 14.4x | PASS |
| HIMMELP3 | 2 | 2 | Optimal | Optimal | 1.74e-10 | 11 | 11 | 234us | 2.7ms | 11.5x | PASS |
| HIMMELP4 | 2 | 3 | NumericalErr | Optimal | N/A | 20 | 23 | 454us | 5.3ms | 11.7x | ripopt_FAIL |
| HIMMELP5 | 2 | 3 | Optimal | Optimal | 1.90e-10 | 69 | 46 | 1.2ms | 10.1ms | 8.5x | PASS |
| HIMMELP6 | 2 | 5 | Optimal | Optimal | 8.61e-01 | 14 | 31 | 328us | 7.4ms | 22.7x | MISMATCH |
| HONG | 4 | 1 | Optimal | Optimal | 7.87e-16 | 7 | 7 | 136us | 1.6ms | 11.8x | PASS |
| HS1 | 2 | 0 | Optimal | Optimal | 2.31e-15 | 25 | 28 | 202us | 5.5ms | 27.5x | PASS |
| HS10 | 2 | 1 | Optimal | Optimal | 2.40e-09 | 11 | 12 | 176us | 2.4ms | 13.9x | PASS |
| HS100 | 7 | 4 | Optimal | Optimal | 1.80e-11 | 9 | 9 | 291us | 2.2ms | 7.6x | PASS |
| HS100LNP | 7 | 2 | Optimal | Optimal | 0.00e+00 | 20 | 20 | 194us | 3.2ms | 16.5x | PASS |
| HS100MOD | 7 | 4 | Optimal | Optimal | 8.53e-12 | 24 | 14 | 636us | 3.2ms | 5.0x | PASS |
| HS101 | 7 | 5 | Optimal | Optimal | 4.07e-11 | 25 | 39 | 1.4ms | 15.6ms | 11.1x | PASS |
| HS102 | 7 | 5 | Optimal | Optimal | 1.22e-10 | 55 | 52 | 3.1ms | 13.3ms | 4.3x | PASS |
| HS103 | 7 | 5 | Optimal | Optimal | 3.04e-10 | 24 | 21 | 1.1ms | 5.3ms | 4.6x | PASS |
| HS104 | 8 | 5 | Optimal | Optimal | 6.19e-09 | 8 | 8 | 343us | 2.0ms | 5.8x | PASS |
| HS105 | 8 | 1 | Optimal | Optimal | 8.45e-12 | 18 | 23 | 3.9ms | 9.6ms | 2.4x | PASS |
| HS106 | 8 | 6 | Optimal | Optimal | 3.59e-11 | 11 | 18 | 424us | 4.0ms | 9.3x | PASS |
| HS107 | 9 | 6 | Optimal | Optimal | 1.04e-10 | 8 | 7 | 295us | 1.7ms | 5.8x | PASS |
| HS108 | 9 | 13 | Acceptable | Optimal | 1.75e-01 | 70 | 11 | 5.1ms | 3.2ms | 0.6x | MISMATCH |
| HS109 | 9 | 10 | Optimal | Optimal | 6.57e-13 | 16 | 14 | 836us | 3.2ms | 3.8x | PASS |
| HS11 | 2 | 1 | Optimal | Optimal | 5.60e-10 | 6 | 6 | 118us | 1.4ms | 11.8x | PASS |
| HS111 | 10 | 3 | Optimal | Optimal | 9.17e-12 | 15 | 15 | 511us | 3.6ms | 7.0x | PASS |
| HS111LNP | 10 | 3 | Optimal | Optimal | 2.98e-16 | 15 | 15 | 323us | 2.8ms | 8.6x | PASS |
| HS112 | 10 | 3 | Optimal | Optimal | 4.46e-16 | 9 | 10 | 319us | 2.3ms | 7.1x | PASS |
| HS113 | 10 | 8 | Optimal | Optimal | 1.52e-09 | 10 | 9 | 472us | 2.3ms | 4.8x | PASS |
| HS114 | 10 | 11 | Optimal | Optimal | 4.79e-11 | 13 | 13 | 761us | 3.1ms | 4.0x | PASS |
| HS116 | 13 | 14 | Optimal | Optimal | 7.71e-11 | 21 | 19 | 1.6ms | 4.8ms | 2.9x | PASS |
| HS117 | 15 | 5 | Optimal | Optimal | 2.51e-09 | 22 | 19 | 1.2ms | 4.8ms | 4.0x | PASS |
| HS118 | 15 | 17 | Optimal | Optimal | 1.10e-10 | 11 | 10 | 1.2ms | 2.6ms | 2.1x | PASS |
| HS119 | 16 | 8 | Optimal | Optimal | 2.07e-08 | 11 | 17 | 734us | 4.1ms | 5.6x | PASS |
| HS12 | 2 | 1 | Optimal | Optimal | 1.30e-10 | 7 | 6 | 139us | 1.4ms | 10.2x | PASS |
| HS13 | 2 | 1 | Optimal | Optimal | 7.22e-04 | 27 | 47 | 452us | 9.6ms | 21.2x | MISMATCH |
| HS14 | 2 | 2 | Optimal | Optimal | 3.59e-09 | 6 | 5 | 112us | 1.2ms | 10.7x | PASS |
| HS15 | 2 | 2 | Optimal | Optimal | 1.41e-10 | 15 | 13 | 227us | 2.8ms | 12.2x | PASS |
| HS16 | 2 | 2 | Optimal | Optimal | 1.58e-07 | 10 | 10 | 191us | 2.4ms | 12.5x | PASS |
| HS17 | 2 | 2 | Optimal | Optimal | 1.36e-07 | 16 | 22 | 288us | 4.7ms | 16.3x | PASS |
| HS18 | 2 | 2 | Optimal | Optimal | 1.04e-09 | 11 | 10 | 224us | 2.2ms | 9.7x | PASS |
| HS19 | 2 | 2 | Optimal | Optimal | 1.16e-11 | 12 | 12 | 230us | 2.8ms | 12.0x | PASS |
| HS1NE | 2 | 2 | Optimal | Optimal | 0.00e+00 | 15 | 30 | 166us | 9.1ms | 54.4x | PASS |
| HS2 | 2 | 0 | Optimal | Optimal | 2.44e-08 | 8 | 10 | 74us | 2.1ms | 28.5x | PASS |
| HS20 | 2 | 3 | Optimal | Optimal | 8.86e-09 | 5 | 5 | 137us | 1.2ms | 9.0x | PASS |
| HS21 | 2 | 1 | Optimal | Optimal | 5.30e-11 | 8 | 6 | 142us | 1.4ms | 9.8x | PASS |
| HS21MOD | 7 | 1 | Optimal | Optimal | 1.33e-10 | 14 | 13 | 381us | 3.1ms | 8.2x | PASS |
| HS22 | 2 | 2 | Optimal | Optimal | 1.02e-08 | 6 | 5 | 116us | 1.3ms | 11.2x | PASS |
| HS23 | 2 | 5 | Optimal | Optimal | 5.22e-09 | 10 | 9 | 283us | 2.0ms | 7.2x | PASS |
| HS24 | 2 | 3 | Optimal | Optimal | 7.03e-09 | 14 | 14 | 324us | 3.9ms | 12.1x | PASS |
| HS25 | 3 | 0 | Optimal | Optimal | 3.39e-15 | 23 | 27 | 1.1ms | 7.6ms | 7.1x | PASS |
| HS25NE | 3 | 99 | Optimal | IpoptStatus( | N/A | 126 | 0 | 331.8ms | 337us | 0.0x | ipopt_FAIL |
| HS26 | 3 | 1 | Optimal | Optimal | 2.69e-27 | 25 | 25 | 176us | 3.1ms | 17.8x | PASS |
| HS268 | 5 | 5 | Optimal | Optimal | 5.59e-07 | 14 | 14 | 383us | 3.2ms | 8.3x | PASS |
| HS27 | 3 | 1 | Optimal | Optimal | 1.28e-12 | 77 | 56 | 635us | 10.2ms | 16.0x | PASS |
| HS28 | 3 | 1 | Optimal | Optimal | 6.16e-32 | 1 | 1 | 38us | 1.0ms | 27.5x | PASS |
| HS29 | 3 | 1 | Optimal | Optimal | 2.21e-10 | 19 | 7 | 445us | 2.2ms | 4.9x | PASS |
| HS2NE | 2 | 2 | RestorationF | Infeasible | N/A | 128 | 12 | 4.6ms | 3.7ms | 0.8x | BOTH_FAIL |
| HS3 | 2 | 0 | Optimal | Optimal | 5.04e-09 | 5 | 4 | 51us | 1.1ms | 21.2x | PASS |
| HS30 | 3 | 1 | Optimal | Optimal | 1.18e-08 | 9 | 7 | 177us | 1.8ms | 10.3x | PASS |
| HS31 | 3 | 1 | Optimal | Optimal | 9.17e-10 | 7 | 6 | 157us | 1.6ms | 9.9x | PASS |
| HS32 | 3 | 2 | Optimal | Optimal | 6.92e-09 | 15 | 15 | 347us | 3.2ms | 9.2x | PASS |
| HS33 | 3 | 2 | Optimal | Optimal | 3.36e-09 | 9 | 9 | 208us | 2.0ms | 9.7x | PASS |
| HS34 | 3 | 2 | Optimal | Optimal | 1.05e-08 | 8 | 7 | 192us | 1.8ms | 9.5x | PASS |
| HS35 | 3 | 1 | Optimal | Optimal | 5.93e-09 | 7 | 7 | 144us | 2.5ms | 17.0x | PASS |
| HS35I | 3 | 1 | Optimal | Optimal | 5.79e-09 | 7 | 7 | 257us | 1.6ms | 6.3x | PASS |
| HS35MOD | 3 | 1 | Optimal | Optimal | 7.11e-10 | 15 | 14 | 285us | 3.1ms | 11.0x | PASS |
| HS36 | 3 | 1 | Optimal | Optimal | 4.17e-12 | 13 | 11 | 271us | 2.7ms | 10.1x | PASS |
| HS37 | 3 | 2 | Optimal | Optimal | 1.52e-12 | 10 | 11 | 239us | 2.7ms | 11.5x | PASS |
| HS38 | 4 | 0 | Optimal | Optimal | 8.34e-18 | 39 | 39 | 357us | 7.9ms | 22.2x | PASS |
| HS39 | 4 | 2 | Optimal | Optimal | 0.00e+00 | 13 | 13 | 116us | 2.1ms | 18.2x | PASS |
| HS3MOD | 2 | 0 | Optimal | Optimal | 5.04e-09 | 5 | 4 | 50us | 907us | 18.3x | PASS |
| HS4 | 2 | 0 | Optimal | Optimal | 4.10e-09 | 5 | 4 | 51us | 905us | 17.7x | PASS |
| HS40 | 4 | 3 | Optimal | Optimal | 0.00e+00 | 3 | 3 | 51us | 682us | 13.4x | PASS |
| HS41 | 4 | 1 | Optimal | Optimal | 2.62e-09 | 8 | 7 | 154us | 1.6ms | 10.3x | PASS |
| HS42 | 4 | 2 | Optimal | Optimal | 0.00e+00 | 4 | 4 | 53us | 734us | 13.9x | PASS |
| HS43 | 4 | 3 | Optimal | Optimal | 2.37e-10 | 9 | 8 | 221us | 1.8ms | 8.3x | PASS |
| HS44 | 4 | 6 | Optimal | Optimal | 1.41e-09 | 22 | 24 | 733us | 5.9ms | 8.1x | PASS |
| HS44NEW | 4 | 6 | Optimal | Optimal | 1.37e-09 | 18 | 18 | 620us | 4.8ms | 7.8x | PASS |
| HS45 | 5 | 0 | Optimal | Optimal | 4.84e-08 | 12 | 11 | 155us | 2.5ms | 16.4x | PASS |
| HS46 | 5 | 2 | Optimal | Optimal | 4.33e-24 | 19 | 19 | 175us | 2.5ms | 14.3x | PASS |
| HS47 | 5 | 3 | Optimal | Optimal | 3.60e-14 | 22 | 19 | 208us | 2.9ms | 14.0x | PASS |
| HS48 | 5 | 2 | Optimal | Optimal | 4.93e-32 | 1 | 1 | 39us | 394us | 10.1x | PASS |
| HS49 | 5 | 2 | Optimal | Optimal | 0.00e+00 | 19 | 19 | 162us | 2.9ms | 17.8x | PASS |
| HS5 | 2 | 0 | Optimal | Optimal | 0.00e+00 | 7 | 7 | 73us | 1.5ms | 20.1x | PASS |
| HS50 | 5 | 3 | Optimal | Optimal | 1.11e-31 | 9 | 9 | 101us | 1.5ms | 14.8x | PASS |
| HS51 | 5 | 3 | Optimal | Optimal | 0.00e+00 | 1 | 1 | 40us | 392us | 9.8x | PASS |
| HS52 | 5 | 3 | Optimal | Optimal | 4.17e-15 | 1 | 1 | 40us | 391us | 9.7x | PASS |
| HS53 | 5 | 3 | Optimal | Optimal | 2.17e-16 | 6 | 6 | 139us | 1.4ms | 9.7x | PASS |
| HS54 | 6 | 1 | Acceptable | Optimal | 4.28e-02 | 21 | 15 | 421us | 3.5ms | 8.3x | MISMATCH |
| HS55 | 6 | 6 | Optimal | Optimal | 2.49e-09 | 12 | 19 | 338us | 5.4ms | 16.1x | PASS |
| HS56 | 7 | 4 | Optimal | Optimal | 0.00e+00 | 10 | 10 | 134us | 2.0ms | 15.1x | PASS |
| HS57 | 2 | 1 | Optimal | Optimal | 2.78e-17 | 24 | 10 | 354us | 2.0ms | 5.6x | PASS |
| HS59 | 2 | 3 | NumericalErr | Optimal | N/A | 43 | 17 | 937us | 4.2ms | 4.5x | ripopt_FAIL |
| HS6 | 2 | 1 | Optimal | Optimal | 4.93e-32 | 5 | 5 | 56us | 1.2ms | 21.2x | PASS |
| HS60 | 3 | 1 | Optimal | Optimal | 2.66e-14 | 6 | 6 | 121us | 1.4ms | 11.4x | PASS |
| HS61 | 3 | 2 | Optimal | Optimal | 5.94e-16 | 28 | 10 | 225us | 1.6ms | 7.2x | PASS |
| HS62 | 3 | 1 | Optimal | Optimal | 1.38e-16 | 6 | 6 | 127us | 1.5ms | 11.6x | PASS |
| HS63 | 3 | 2 | Optimal | Optimal | 2.36e-16 | 6 | 5 | 125us | 1.2ms | 9.6x | PASS |
| HS64 | 3 | 1 | Optimal | Optimal | 1.14e-09 | 17 | 16 | 329us | 3.5ms | 10.5x | PASS |
| HS65 | 3 | 1 | Optimal | Optimal | 5.06e-09 | 17 | 16 | 322us | 3.9ms | 12.1x | PASS |
| HS66 | 3 | 2 | Optimal | Optimal | 1.22e-08 | 11 | 10 | 182us | 1.8ms | 9.8x | PASS |
| HS67 | 3 | 14 | Optimal | Optimal | 8.68e-12 | 10 | 9 | 648us | 2.3ms | 3.5x | PASS |
| HS68 | 4 | 2 | Optimal | Optimal | 1.80e-10 | 16 | 16 | 275us | 3.5ms | 12.8x | PASS |
| HS69 | 4 | 2 | Optimal | Optimal | 8.32e-16 | 10 | 10 | 202us | 2.4ms | 11.7x | PASS |
| HS7 | 2 | 1 | Optimal | Optimal | 1.28e-16 | 27 | 27 | 199us | 5.1ms | 25.8x | PASS |
| HS70 | 4 | 1 | Optimal | Optimal | 1.72e-01 | 16 | 46 | 606us | 10.6ms | 17.5x | MISMATCH |
| HS71 | 4 | 2 | Optimal | Optimal | 6.15e-10 | 9 | 8 | 223us | 2.0ms | 8.8x | PASS |
| HS72 | 4 | 2 | Optimal | Optimal | 1.49e-09 | 25 | 16 | 445us | 3.3ms | 7.4x | PASS |
| HS73 | 4 | 3 | Optimal | Optimal | 5.04e-10 | 9 | 8 | 246us | 1.8ms | 7.3x | PASS |
| HS74 | 4 | 5 | Optimal | Optimal | 1.77e-16 | 9 | 8 | 265us | 1.9ms | 7.1x | PASS |
| HS75 | 4 | 5 | Optimal | Optimal | 1.13e-12 | 9 | 8 | 267us | 1.9ms | 7.0x | PASS |
| HS76 | 4 | 3 | Optimal | Optimal | 2.90e-09 | 7 | 7 | 184us | 1.7ms | 9.1x | PASS |
| HS76I | 4 | 3 | Optimal | Optimal | 7.77e-10 | 7 | 6 | 191us | 1.5ms | 7.6x | PASS |
| HS77 | 5 | 2 | Optimal | Optimal | 1.51e-13 | 11 | 11 | 113us | 1.6ms | 14.3x | PASS |
| HS78 | 5 | 3 | Optimal | Optimal | 0.00e+00 | 4 | 4 | 64us | 848us | 13.3x | PASS |
| HS79 | 5 | 3 | Optimal | Optimal | 0.00e+00 | 4 | 4 | 81us | 811us | 10.0x | PASS |
| HS8 | 2 | 2 | Optimal | Optimal | 0.00e+00 | 5 | 5 | 51us | 950us | 18.5x | PASS |
| HS80 | 5 | 3 | Optimal | Optimal | 4.92e-15 | 6 | 5 | 155us | 1.2ms | 8.0x | PASS |
| HS81 | 5 | 3 | Optimal | Optimal | 3.46e-14 | 7 | 68 | 179us | 15.9ms | 89.1x | PASS |
| HS83 | 5 | 3 | Optimal | Optimal | 3.43e-12 | 9 | 9 | 259us | 2.1ms | 8.0x | PASS |
| HS84 | 5 | 3 | Optimal | Optimal | 8.80e-11 | 11 | 9 | 274us | 2.1ms | 7.8x | PASS |
| HS85 | 5 | 21 | Optimal | Optimal | 4.93e-09 | 14 | 13 | 3.3ms | 5.3ms | 1.6x | PASS |
| HS86 | 5 | 10 | Optimal | Optimal | 4.31e-10 | 11 | 10 | 541us | 2.4ms | 4.4x | PASS |
| HS87 | 6 | 4 | MaxIteration | MaxIteration | N/A | 2999 | 3000 | 38.0ms | 569.3ms | 15.0x | BOTH_FAIL |
| HS88 | 2 | 1 | Optimal | Optimal | 3.66e-09 | 20 | 18 | 3.2ms | 6.0ms | 1.9x | PASS |
| HS89 | 3 | 1 | Optimal | Optimal | 3.67e-09 | 16 | 15 | 4.2ms | 6.2ms | 1.5x | PASS |
| HS9 | 2 | 1 | Optimal | Optimal | 0.00e+00 | 3 | 3 | 68us | 768us | 11.3x | PASS |
| HS90 | 4 | 1 | Optimal | Optimal | 3.66e-09 | 12 | 16 | 3.9ms | 7.6ms | 1.9x | PASS |
| HS91 | 5 | 1 | NumericalErr | Optimal | N/A | 403 | 16 | 131.0ms | 9.6ms | 0.1x | ripopt_FAIL |
| HS92 | 6 | 1 | Optimal | Optimal | 3.66e-09 | 15 | 39 | 8.6ms | 27.3ms | 3.2x | PASS |
| HS93 | 6 | 2 | Optimal | Optimal | 1.04e-10 | 7 | 7 | 213us | 1.8ms | 8.6x | PASS |
| HS95 | 6 | 4 | Optimal | Optimal | 3.05e-08 | 9 | 9 | 287us | 2.2ms | 7.5x | PASS |
| HS96 | 6 | 4 | Optimal | Optimal | 4.84e-09 | 8 | 8 | 262us | 2.0ms | 7.5x | PASS |
| HS97 | 6 | 4 | Optimal | Optimal | 1.34e-08 | 19 | 24 | 570us | 5.7ms | 10.1x | PASS |
| HS98 | 6 | 4 | Optimal | Optimal | 1.22e-08 | 14 | 13 | 441us | 3.0ms | 6.7x | PASS |
| HS99 | 7 | 2 | Optimal | Optimal | 1.43e-16 | 5 | 5 | 149us | 1.3ms | 8.7x | PASS |
| HS99EXP | 31 | 21 | Optimal | Optimal | 9.21e-10 | 150 | 17 | 28.7ms | 4.5ms | 0.2x | PASS |
| HUBFIT | 2 | 1 | Optimal | Optimal | 4.95e-09 | 8 | 7 | 161us | 1.6ms | 9.7x | PASS |
| HUMPS | 2 | 0 | Optimal | Optimal | 3.60e-17 | 375 | 210 | 1.9ms | 38.3ms | 19.7x | PASS |
| HYDC20LS | 99 | 0 | Acceptable | Optimal | 3.40e-15 | 643 | 640 | 285.9ms | 250.9ms | 0.9x | PASS |
| HYDCAR20 | 99 | 99 | Optimal | Optimal | 0.00e+00 | 9 | 9 | 17.5ms | 3.9ms | 0.2x | PASS |
| HYDCAR6 | 29 | 29 | Optimal | Optimal | 0.00e+00 | 5 | 5 | 527us | 1.5ms | 2.9x | PASS |
| HYDCAR6LS | 29 | 0 | Optimal | Optimal | 1.73e-21 | 145 | 145 | 6.6ms | 35.5ms | 5.4x | PASS |
| HYPCIR | 2 | 2 | Optimal | Optimal | 0.00e+00 | 5 | 5 | 40us | 1.0ms | 26.2x | PASS |
| JENSMP | 2 | 0 | Optimal | Optimal | 0.00e+00 | 9 | 9 | 63us | 1.4ms | 21.8x | PASS |
| JENSMPNE | 2 | 10 | LocalInfeasi | IpoptStatus( | N/A | 45 | 0 | 5.8ms | 112us | 0.0x | BOTH_FAIL |
| JUDGE | 2 | 0 | Optimal | Optimal | 0.00e+00 | 9 | 9 | 63us | 1.4ms | 22.3x | PASS |
| JUDGEB | 2 | 0 | Optimal | Optimal | 0.00e+00 | 9 | 9 | 109us | 1.8ms | 17.0x | PASS |
| JUDGENE | 2 | 20 | RestorationF | IpoptStatus( | N/A | 14 | 0 | 14.7ms | 108us | 0.0x | BOTH_FAIL |
| KIRBY2 | 5 | 151 | RestorationF | IpoptStatus( | N/A | 34 | 0 | 1.16s | 141us | 0.0x | BOTH_FAIL |
| KIRBY2LS | 5 | 0 | Optimal | Optimal | 1.13e-14 | 11 | 11 | 516us | 2.6ms | 5.1x | PASS |
| KIWCRESC | 3 | 2 | Optimal | Optimal | 1.07e-08 | 8 | 8 | 183us | 2.0ms | 11.0x | PASS |
| KOEBHELB | 3 | 0 | Optimal | Optimal | 0.00e+00 | 72 | 71 | 2.8ms | 17.9ms | 6.3x | PASS |
| KOEBHELBNE | 3 | 156 | MaxTimeExcee | IpoptStatus( | N/A | 1380 | 0 | 30.11s | 127us | 0.0x | BOTH_FAIL |
| KOWOSB | 4 | 0 | Optimal | Optimal | 1.08e-18 | 8 | 8 | 62us | 1.7ms | 28.0x | PASS |
| KOWOSBNE | 4 | 11 | LocalInfeasi | IpoptStatus( | N/A | 14 | 0 | 6.4ms | 110us | 0.0x | BOTH_FAIL |
| KSIP | 20 | 1001 | Optimal | Optimal | 3.90e-09 | 32 | 22 | 310.7ms | 146.4ms | 0.5x | PASS |
| LAKES | 90 | 78 | Optimal | Optimal | 9.90e-14 | 11 | 11 | 9.3ms | 4.7ms | 0.5x | PASS |
| LANCZOS1 | 6 | 24 | Optimal | IpoptStatus( | N/A | 80 | 0 | 5.1ms | 114us | 0.0x | ipopt_FAIL |
| LANCZOS1LS | 6 | 0 | Optimal | Optimal | 2.83e-18 | 114 | 115 | 1.7ms | 22.5ms | 13.0x | PASS |
| LANCZOS2 | 6 | 24 | RestorationF | IpoptStatus( | N/A | 81 | 0 | 34.3ms | 113us | 0.0x | BOTH_FAIL |
| LANCZOS2LS | 6 | 0 | Optimal | Optimal | 4.51e-19 | 101 | 101 | 1.5ms | 19.4ms | 12.7x | PASS |
| LANCZOS3 | 6 | 24 | LocalInfeasi | IpoptStatus( | N/A | 51 | 0 | 33.2ms | 118us | 0.0x | BOTH_FAIL |
| LANCZOS3LS | 6 | 0 | Optimal | Optimal | 9.24e-16 | 167 | 163 | 2.5ms | 30.7ms | 12.3x | PASS |
| LAUNCH | 25 | 28 | Optimal | Optimal | 7.96e-09 | 27 | 12 | 6.3ms | 3.8ms | 0.6x | PASS |
| LEVYMONE10 | 10 | 20 | LocalInfeasi | IpoptStatus( | N/A | 175 | 0 | 58.8ms | 103us | 0.0x | BOTH_FAIL |
| LEVYMONE5 | 2 | 4 | Optimal | IpoptStatus( | N/A | 73 | 0 | 1.1ms | 104us | 0.1x | ipopt_FAIL |
| LEVYMONE6 | 3 | 6 | LocalInfeasi | IpoptStatus( | N/A | 27 | 0 | 2.9ms | 115us | 0.0x | BOTH_FAIL |
| LEVYMONE7 | 4 | 8 | LocalInfeasi | IpoptStatus( | N/A | 59 | 0 | 6.3ms | 109us | 0.0x | BOTH_FAIL |
| LEVYMONE8 | 5 | 10 | LocalInfeasi | IpoptStatus( | N/A | 113 | 0 | 10.5ms | 113us | 0.0x | BOTH_FAIL |
| LEVYMONE9 | 8 | 16 | LocalInfeasi | IpoptStatus( | N/A | 136 | 0 | 25.3ms | 109us | 0.0x | BOTH_FAIL |
| LEVYMONT10 | 10 | 0 | Optimal | Optimal | 3.47e-16 | 6 | 4 | 103us | 964us | 9.4x | PASS |
| LEVYMONT5 | 2 | 0 | Optimal | Optimal | 1.00e+00 | 12 | 10 | 128us | 2.3ms | 18.2x | MISMATCH |
| LEVYMONT6 | 3 | 0 | Optimal | Optimal | 2.84e-16 | 8 | 8 | 99us | 1.9ms | 19.4x | PASS |
| LEVYMONT7 | 4 | 0 | Optimal | Optimal | 0.00e+00 | 9 | 7 | 121us | 1.8ms | 15.0x | PASS |
| LEVYMONT8 | 5 | 0 | Optimal | Optimal | 1.64e-16 | 6 | 4 | 78us | 943us | 12.1x | PASS |
| LEVYMONT9 | 8 | 0 | Optimal | Optimal | 0.00e+00 | 6 | 4 | 140us | 961us | 6.9x | PASS |
| LEWISPOL | 6 | 9 | Optimal | IpoptStatus( | N/A | 13 | 0 | 477us | 112us | 0.2x | ipopt_FAIL |
| LHAIFAM | 99 | 150 | EvaluationEr | InvalidNumbe | N/A | 0 | 0 | 966us | 179us | 0.2x | BOTH_FAIL |
| LIN | 4 | 2 | Optimal | Optimal | 2.03e-03 | 7 | 7 | 157us | 1.5ms | 9.6x | MISMATCH |
| LINSPANH | 97 | 33 | Optimal | Optimal | 1.40e-10 | 19 | 19 | 5.2ms | 6.4ms | 1.2x | PASS |
| LOADBAL | 31 | 31 | Optimal | Optimal | 5.01e-08 | 13 | 13 | 3.9ms | 3.9ms | 1.0x | PASS |
| LOGHAIRY | 2 | 0 | MaxIteration | Optimal | N/A | 2999 | 2918 | 15.6ms | 521.1ms | 33.3x | ripopt_FAIL |
| LOGROS | 2 | 0 | Optimal | Optimal | 0.00e+00 | 50 | 49 | 653us | 10.3ms | 15.8x | PASS |
| LOOTSMA | 3 | 2 | Optimal | Optimal | 1.07e-08 | 20 | 13 | 399us | 3.1ms | 7.8x | PASS |
| LOTSCHD | 12 | 7 | Optimal | Optimal | 1.17e-11 | 11 | 9 | 417us | 2.1ms | 5.0x | PASS |
| LRCOVTYPE | 54 | 0 | Optimal | Optimal | 3.01e-03 | 25 | 33 | 9.78s | 12.38s | 1.3x | MISMATCH |
| LRIJCNN1 | 22 | 0 | Optimal | Optimal | 5.55e-17 | 11 | 11 | 346.9ms | 328.0ms | 0.9x | PASS |
| LSC1 | 3 | 6 | RestorationF | IpoptStatus( | N/A | 16 | 0 | 2.5ms | 121us | 0.0x | BOTH_FAIL |
| LSC1LS | 3 | 0 | Optimal | Optimal | 4.61e-16 | 16 | 16 | 90us | 3.3ms | 36.9x | PASS |
| LSC2 | 3 | 6 | RestorationF | IpoptStatus( | N/A | 21 | 0 | 3.6ms | 115us | 0.0x | BOTH_FAIL |
| LSC2LS | 3 | 0 | Acceptable | Optimal | 5.47e-04 | 39 | 41 | 227us | 6.9ms | 30.4x | MISMATCH |
| LSNNODOC | 5 | 4 | Optimal | Optimal | 2.23e-09 | 11 | 10 | 252us | 2.6ms | 10.3x | PASS |
| LSQFIT | 2 | 1 | Optimal | Optimal | 7.44e-09 | 7 | 7 | 141us | 1.7ms | 11.7x | PASS |
| MADSEN | 3 | 6 | Optimal | Optimal | 1.13e-08 | 16 | 18 | 496us | 4.2ms | 8.4x | PASS |
| MAKELA1 | 3 | 2 | Optimal | Optimal | 7.29e-09 | 15 | 12 | 275us | 3.0ms | 11.0x | PASS |
| MAKELA2 | 3 | 3 | Optimal | Optimal | 1.40e-09 | 7 | 6 | 145us | 1.6ms | 10.8x | PASS |
| MAKELA3 | 21 | 20 | Optimal | Optimal | 5.50e-08 | 30 | 11 | 3.7ms | 3.4ms | 0.9x | PASS |
| MAKELA4 | 21 | 40 | Optimal | Optimal | 2.22e-07 | 6 | 5 | 2.6ms | 1.7ms | 0.6x | PASS |
| MARATOS | 2 | 1 | Optimal | Optimal | 0.00e+00 | 4 | 4 | 47us | 816us | 17.2x | PASS |
| MARATOSB | 2 | 0 | Optimal | Optimal | 1.20e-14 | 673 | 671 | 3.0ms | 106.8ms | 35.8x | PASS |
| MATRIX2 | 6 | 2 | Optimal | Optimal | 8.37e-09 | 18 | 42 | 398us | 8.5ms | 21.4x | PASS |
| MAXLIKA | 8 | 0 | Optimal | Optimal | 8.43e-12 | 30 | 23 | 5.8ms | 9.5ms | 1.6x | PASS |
| MCONCON | 15 | 11 | Optimal | Optimal | 3.25e-12 | 8 | 7 | 403us | 1.8ms | 4.5x | PASS |
| MDHOLE | 2 | 0 | Optimal | Optimal | 1.86e-08 | 40 | 42 | 329us | 9.6ms | 29.0x | PASS |
| MESH | 41 | 48 | MaxIteration | IpoptStatus( | N/A | 2999 | 81 | 4.24s | 31.9ms | 0.0x | BOTH_FAIL |
| METHANB8 | 31 | 31 | Optimal | Optimal | 0.00e+00 | 3 | 3 | 519us | 1.1ms | 2.0x | PASS |
| METHANB8LS | 31 | 0 | Optimal | Optimal | 5.93e-26 | 8 | 8 | 411us | 2.0ms | 4.8x | PASS |
| METHANL8 | 31 | 31 | Optimal | Optimal | 0.00e+00 | 4 | 4 | 476us | 1.2ms | 2.6x | PASS |
| METHANL8LS | 31 | 0 | Optimal | Optimal | 9.50e-21 | 40 | 40 | 2.1ms | 11.2ms | 5.2x | PASS |
| MEXHAT | 2 | 0 | Optimal | Optimal | 6.94e-18 | 26 | 26 | 127us | 4.0ms | 31.2x | PASS |
| MEYER3 | 3 | 0 | Acceptable | Optimal | 3.12e-12 | 208 | 205 | 1.8ms | 37.5ms | 20.8x | PASS |
| MEYER3NE | 3 | 16 | LocalInfeasi | IpoptStatus( | N/A | 64 | 0 | 25.4ms | 129us | 0.0x | BOTH_FAIL |
| MGH09 | 4 | 11 | RestorationF | IpoptStatus( | N/A | 44 | 0 | 11.7ms | 117us | 0.0x | BOTH_FAIL |
| MGH09LS | 4 | 0 | Optimal | Optimal | 8.13e-19 | 71 | 72 | 504us | 13.4ms | 26.6x | PASS |
| MGH10 | 3 | 16 | LocalInfeasi | IpoptStatus( | N/A | 7 | 0 | 19.3ms | 111us | 0.0x | BOTH_FAIL |
| MGH10LS | 3 | 0 | Optimal | Optimal | 5.92e-12 | 2075 | 1899 | 21.0ms | 320.5ms | 15.3x | PASS |
| MGH10S | 3 | 16 | LocalInfeasi | IpoptStatus( | N/A | 7 | 0 | 7.1ms | 107us | 0.0x | BOTH_FAIL |
| MGH10SLS | 3 | 0 | Optimal | Optimal | 1.24e-12 | 355 | 352 | 2.9ms | 60.8ms | 20.9x | PASS |
| MGH17 | 5 | 33 | RestorationF | IpoptStatus( | N/A | 231 | 0 | 193.5ms | 117us | 0.0x | BOTH_FAIL |
| MGH17LS | 5 | 0 | Acceptable | Optimal | 1.00e+00 | 15 | 48 | 263us | 11.5ms | 43.7x | MISMATCH |
| MGH17S | 5 | 33 | RestorationF | IpoptStatus( | N/A | 39 | 0 | 75.5ms | 119us | 0.0x | BOTH_FAIL |
| MGH17SLS | 5 | 0 | Optimal | Optimal | 3.53e-07 | 40 | 40 | 665us | 9.2ms | 13.9x | PASS |
| MIFFLIN1 | 3 | 2 | Optimal | Optimal | 1.07e-08 | 6 | 5 | 141us | 1.2ms | 8.5x | PASS |
| MIFFLIN2 | 3 | 2 | Optimal | Optimal | 9.99e-09 | 15 | 11 | 279us | 2.6ms | 9.3x | PASS |
| MINMAXBD | 5 | 20 | Optimal | Optimal | 1.36e-10 | 37 | 25 | 8.3ms | 8.0ms | 1.0x | PASS |
| MINMAXRB | 3 | 4 | Optimal | Optimal | 1.82e-08 | 8 | 8 | 198us | 1.9ms | 9.5x | PASS |
| MINSURF | 64 | 0 | Optimal | Optimal | 0.00e+00 | 4 | 4 | 229us | 1.3ms | 5.7x | PASS |
| MISRA1A | 2 | 14 | RestorationF | IpoptStatus( | N/A | 28 | 0 | 8.5ms | 111us | 0.0x | BOTH_FAIL |
| MISRA1ALS | 2 | 0 | Optimal | Optimal | 1.90e-15 | 40 | 40 | 267us | 7.3ms | 27.2x | PASS |
| MISRA1B | 2 | 14 | RestorationF | IpoptStatus( | N/A | 27 | 0 | 8.6ms | 113us | 0.0x | BOTH_FAIL |
| MISRA1BLS | 2 | 0 | Optimal | Optimal | 1.25e-14 | 34 | 34 | 215us | 5.9ms | 27.4x | PASS |
| MISRA1C | 2 | 14 | LocalInfeasi | IpoptStatus( | N/A | 556 | 0 | 71.3ms | 104us | 0.0x | BOTH_FAIL |
| MISRA1CLS | 2 | 0 | Optimal | Optimal | 1.32e-14 | 14 | 14 | 94us | 3.7ms | 39.3x | PASS |
| MISRA1D | 2 | 14 | RestorationF | IpoptStatus( | N/A | 284 | 0 | 46.0ms | 127us | 0.0x | BOTH_FAIL |
| MISRA1DLS | 2 | 0 | Optimal | Optimal | 6.80e-16 | 30 | 30 | 189us | 5.3ms | 28.1x | PASS |
| MISTAKE | 9 | 13 | Acceptable | Optimal | 1.60e-08 | 32 | 16 | 2.5ms | 4.7ms | 1.9x | PASS |
| MRIBASIS | 36 | 55 | Optimal | Optimal | 2.20e-10 | 16 | 15 | 12.1ms | 5.9ms | 0.5x | PASS |
| MSS1 | 90 | 73 | StopAtTinySt | MaxIteration | N/A | 26 | 3000 | 271.6ms | 1.97s | 7.3x | BOTH_FAIL |
| MUONSINE | 1 | 512 | Timeout | IpoptStatus( | N/A | 0 | 0 | 60.00s | 165us | 0.0x | BOTH_FAIL |
| MUONSINELS | 1 | 0 | Optimal | Optimal | 0.00e+00 | 8 | 8 | 831us | 2.2ms | 2.7x | PASS |
| MWRIGHT | 5 | 3 | Optimal | Optimal | 0.00e+00 | 10 | 10 | 121us | 2.1ms | 17.1x | PASS |
| NASH | 72 | 24 | RestorationF | Infeasible | N/A | 38 | 45 | 95.3ms | 19.3ms | 0.2x | BOTH_FAIL |
| NELSON | 3 | 128 | RestorationF | IpoptStatus( | N/A | 32 | 0 | 920.7ms | 119us | 0.0x | BOTH_FAIL |
| NET1 | 48 | 57 | Optimal | Optimal | 3.55e-14 | 25 | 26 | 18.7ms | 8.1ms | 0.4x | PASS |
| NYSTROM5 | 18 | 20 | RestorationF | IpoptStatus( | N/A | 35 | 0 | 76.6ms | 109us | 0.0x | BOTH_FAIL |
| NYSTROM5C | 18 | 20 | RestorationF | IpoptStatus( | N/A | 35 | 0 | 77.4ms | 110us | 0.0x | BOTH_FAIL |
| ODFITS | 10 | 6 | Optimal | Optimal | 0.00e+00 | 10 | 8 | 279us | 2.1ms | 7.5x | PASS |
| OET1 | 3 | 1002 | Optimal | Optimal | 1.22e-08 | 16 | 33 | 57.4ms | 95.3ms | 1.7x | PASS |
| OET2 | 3 | 1002 | Optimal | Optimal | 1.18e-08 | 95 | 168 | 864.6ms | 591.6ms | 0.7x | PASS |
| OET3 | 4 | 1002 | Optimal | Optimal | 2.90e-09 | 14 | 13 | 54.2ms | 42.9ms | 0.8x | PASS |
| OET4 | 4 | 1002 | Optimal | Optimal | 2.28e-08 | 36 | 66 | 149.2ms | 220.9ms | 1.5x | PASS |
| OET5 | 5 | 1002 | Optimal | Optimal | 2.18e-08 | 42 | 65 | 257.0ms | 240.0ms | 0.9x | PASS |
| OET6 | 5 | 1002 | NumericalErr | Optimal | N/A | 20 | 182 | 10.53s | 904.8ms | 0.1x | ripopt_FAIL |
| OET7 | 7 | 1002 | NumericalErr | Optimal | N/A | 21 | 324 | 7.37s | 1.71s | 0.2x | ripopt_FAIL |
| OPTCNTRL | 32 | 20 | Optimal | Optimal | 7.74e-12 | 14 | 9 | 2.9ms | 3.3ms | 1.2x | PASS |
| OPTPRLOC | 30 | 30 | Optimal | Optimal | 1.68e-08 | 12 | 13 | 4.6ms | 4.7ms | 1.0x | PASS |
| ORTHREGB | 27 | 6 | Optimal | Optimal | 0.00e+00 | 2 | 2 | 212us | 849us | 4.0x | PASS |
| OSBORNE1 | 5 | 33 | RestorationF | IpoptStatus( | N/A | 32 | 0 | 80.4ms | 118us | 0.0x | BOTH_FAIL |
| OSBORNE2 | 11 | 65 | RestorationF | IpoptStatus( | N/A | 29 | 0 | 109.1ms | 183us | 0.0x | BOTH_FAIL |
| OSBORNEA | 5 | 0 | Optimal | Optimal | 1.36e-18 | 64 | 64 | 1.0ms | 12.6ms | 12.1x | PASS |
| OSBORNEB | 11 | 0 | Optimal | Optimal | 0.00e+00 | 19 | 19 | 1.1ms | 4.4ms | 3.9x | PASS |
| OSLBQP | 8 | 0 | Optimal | Optimal | 1.07e-09 | 16 | 15 | 290us | 3.2ms | 11.0x | PASS |
| PALMER1 | 4 | 0 | MaxIteration | Optimal | N/A | 2999 | 13 | 32.8ms | 3.0ms | 0.1x | ripopt_FAIL |
| PALMER1A | 6 | 0 | Optimal | Optimal | 8.56e-13 | 49 | 48 | 925us | 11.4ms | 12.3x | PASS |
| PALMER1ANE | 6 | 35 | RestorationF | IpoptStatus( | N/A | 93 | 0 | 44.7ms | 115us | 0.0x | BOTH_FAIL |
| PALMER1B | 4 | 0 | Optimal | Optimal | 2.10e-14 | 19 | 17 | 354us | 3.8ms | 10.9x | PASS |
| PALMER1BNE | 4 | 35 | LocalInfeasi | IpoptStatus( | N/A | 2999 | 0 | 2.45s | 133us | 0.0x | BOTH_FAIL |
| PALMER1C | 8 | 0 | Optimal | Optimal | 1.35e-13 | 1 | 1 | 47us | 364us | 7.7x | PASS |
| PALMER1D | 7 | 0 | Optimal | Optimal | 2.32e-13 | 1 | 1 | 30us | 503us | 17.0x | PASS |
| PALMER1E | 8 | 0 | Optimal | Optimal | 5.88e-09 | 35 | 55 | 904us | 13.2ms | 14.6x | PASS |
| PALMER1ENE | 8 | 35 | LocalInfeasi | IpoptStatus( | N/A | 2999 | 0 | 4.45s | 121us | 0.0x | BOTH_FAIL |
| PALMER1NE | 4 | 31 | RestorationF | IpoptStatus( | N/A | 81 | 0 | 59.9ms | 116us | 0.0x | BOTH_FAIL |
| PALMER2 | 4 | 0 | MaxIteration | Optimal | N/A | 2999 | 28 | 28.7ms | 7.8ms | 0.3x | ripopt_FAIL |
| PALMER2A | 6 | 0 | Optimal | Optimal | 7.62e-13 | 89 | 92 | 1.3ms | 22.1ms | 16.6x | PASS |
| PALMER2ANE | 6 | 23 | RestorationF | IpoptStatus( | N/A | 15 | 0 | 26.4ms | 112us | 0.0x | BOTH_FAIL |
| PALMER2B | 4 | 0 | Optimal | Optimal | 1.24e-14 | 15 | 15 | 225us | 3.8ms | 16.8x | PASS |
| PALMER2BNE | 4 | 23 | LocalInfeasi | IpoptStatus( | N/A | 2999 | 0 | 891.0ms | 107us | 0.0x | BOTH_FAIL |
| PALMER2C | 8 | 0 | Optimal | Optimal | 2.69e-15 | 1 | 1 | 27us | 308us | 11.3x | PASS |
| PALMER2E | 8 | 0 | Optimal | Optimal | 1.16e-01 | 33 | 91 | 725us | 23.1ms | 31.9x | MISMATCH |
| PALMER2ENE | 8 | 23 | RestorationF | IpoptStatus( | N/A | 471 | 0 | 636.1ms | 118us | 0.0x | BOTH_FAIL |
| PALMER2NE | 4 | 23 | RestorationF | IpoptStatus( | N/A | 36 | 0 | 162.2ms | 180us | 0.0x | BOTH_FAIL |
| PALMER3 | 4 | 0 | MaxIteration | Optimal | N/A | 2999 | 44 | 28.0ms | 9.2ms | 0.3x | ripopt_FAIL |
| PALMER3A | 6 | 0 | Optimal | Optimal | 9.19e-13 | 73 | 73 | 1.0ms | 16.8ms | 16.4x | PASS |
| PALMER3ANE | 6 | 23 | RestorationF | IpoptStatus( | N/A | 21 | 0 | 25.5ms | 120us | 0.0x | BOTH_FAIL |
| PALMER3B | 4 | 0 | Optimal | Optimal | 5.25e-15 | 14 | 15 | 208us | 3.8ms | 18.2x | PASS |
| PALMER3BNE | 4 | 23 | LocalInfeasi | IpoptStatus( | N/A | 2999 | 0 | 950.7ms | 105us | 0.0x | BOTH_FAIL |
| PALMER3C | 8 | 0 | Optimal | Optimal | 6.04e-15 | 1 | 1 | 23us | 338us | 14.6x | PASS |
| PALMER3E | 8 | 0 | Optimal | Optimal | 6.22e-10 | 30 | 32 | 574us | 7.3ms | 12.6x | PASS |
| PALMER3ENE | 8 | 23 | LocalInfeasi | IpoptStatus( | N/A | 2999 | 0 | 4.41s | 130us | 0.0x | BOTH_FAIL |
| PALMER3NE | 4 | 23 | RestorationF | IpoptStatus( | N/A | 22 | 0 | 153.5ms | 107us | 0.0x | BOTH_FAIL |
| PALMER4 | 4 | 0 | MaxIteration | Optimal | N/A | 2999 | 16 | 27.9ms | 4.1ms | 0.1x | ripopt_FAIL |
| PALMER4A | 6 | 0 | Optimal | Optimal | 5.61e-13 | 53 | 53 | 780us | 12.2ms | 15.7x | PASS |
| PALMER4ANE | 6 | 23 | RestorationF | IpoptStatus( | N/A | 65 | 0 | 28.0ms | 114us | 0.0x | BOTH_FAIL |
| PALMER4B | 4 | 0 | Optimal | Optimal | 6.50e-16 | 15 | 15 | 246us | 3.9ms | 15.9x | PASS |
| PALMER4BNE | 4 | 23 | LocalInfeasi | IpoptStatus( | N/A | 2999 | 0 | 926.5ms | 128us | 0.0x | BOTH_FAIL |
| PALMER4C | 8 | 0 | Optimal | Optimal | 3.18e-15 | 1 | 1 | 24us | 354us | 14.8x | PASS |
| PALMER4E | 8 | 0 | Optimal | Optimal | 2.29e-10 | 25 | 25 | 457us | 5.8ms | 12.8x | PASS |
| PALMER4ENE | 8 | 23 | LocalInfeasi | IpoptStatus( | N/A | 2999 | 0 | 4.32s | 110us | 0.0x | BOTH_FAIL |
| PALMER4NE | 4 | 23 | RestorationF | IpoptStatus( | N/A | 22 | 0 | 106.4ms | 107us | 0.0x | BOTH_FAIL |
| PALMER5A | 8 | 0 | MaxIteration | MaxIteration | N/A | 2999 | 3000 | 35.2ms | 756.2ms | 21.5x | BOTH_FAIL |
| PALMER5ANE | 8 | 12 | LocalInfeasi | IpoptStatus( | N/A | 14 | 0 | 13.8ms | 123us | 0.0x | BOTH_FAIL |
| PALMER5B | 9 | 0 | Optimal | Optimal | 1.88e-02 | 7 | 108 | 101us | 27.0ms | 267.4x | MISMATCH |
| PALMER5BNE | 9 | 12 | RestorationF | IpoptStatus( | N/A | 56 | 0 | 15.2ms | 127us | 0.0x | BOTH_FAIL |
| PALMER5C | 6 | 0 | Optimal | Optimal | 2.71e-15 | 1 | 1 | 15us | 335us | 21.8x | PASS |
| PALMER5D | 4 | 0 | Optimal | Optimal | 1.30e-15 | 1 | 1 | 13us | 415us | 31.2x | PASS |
| PALMER5E | 8 | 0 | MaxIteration | MaxIteration | N/A | 2999 | 3000 | 32.1ms | 561.3ms | 17.5x | BOTH_FAIL |
| PALMER5ENE | 8 | 12 | RestorationF | IpoptStatus( | N/A | 1760 | 0 | 100.5ms | 116us | 0.0x | BOTH_FAIL |
| PALMER6A | 6 | 0 | Optimal | Optimal | 1.36e-14 | 108 | 106 | 1.2ms | 22.8ms | 18.3x | PASS |
| PALMER6ANE | 6 | 13 | RestorationF | IpoptStatus( | N/A | 15 | 0 | 8.0ms | 110us | 0.0x | BOTH_FAIL |
| PALMER6C | 8 | 0 | Optimal | Optimal | 5.77e-14 | 1 | 1 | 21us | 327us | 15.5x | PASS |
| PALMER6E | 8 | 0 | Optimal | Optimal | 1.86e-11 | 32 | 30 | 490us | 7.0ms | 14.2x | PASS |
| PALMER6ENE | 8 | 13 | RestorationF | IpoptStatus( | N/A | 188 | 0 | 35.8ms | 114us | 0.0x | BOTH_FAIL |
| PALMER7A | 6 | 0 | MaxIteration | MaxIteration | N/A | 2999 | 3000 | 27.5ms | 570.7ms | 20.7x | BOTH_FAIL |
| PALMER7ANE | 6 | 13 | MaxIteration | IpoptStatus( | N/A | 2999 | 0 | 118.7ms | 105us | 0.0x | BOTH_FAIL |
| PALMER7C | 8 | 0 | Optimal | Optimal | 1.94e-13 | 1 | 1 | 20us | 302us | 14.8x | PASS |
| PALMER7E | 8 | 0 | MaxIteration | MaxIteration | N/A | 2999 | 3000 | 36.7ms | 741.3ms | 20.2x | BOTH_FAIL |
| PALMER7ENE | 8 | 13 | RestorationF | IpoptStatus( | N/A | 31 | 0 | 26.2ms | 113us | 0.0x | BOTH_FAIL |
| PALMER8A | 6 | 0 | Optimal | Optimal | 2.65e-13 | 36 | 36 | 480us | 9.2ms | 19.2x | PASS |
| PALMER8ANE | 6 | 12 | RestorationF | IpoptStatus( | N/A | 102 | 0 | 9.9ms | 113us | 0.0x | BOTH_FAIL |
| PALMER8C | 8 | 0 | Optimal | Optimal | 2.29e-14 | 1 | 1 | 20us | 317us | 15.5x | PASS |
| PALMER8E | 8 | 0 | Optimal | Optimal | 6.71e-13 | 17 | 23 | 250us | 5.7ms | 22.9x | PASS |
| PALMER8ENE | 8 | 12 | RestorationF | IpoptStatus( | N/A | 86 | 0 | 40.4ms | 106us | 0.0x | BOTH_FAIL |
| PARKCH | 15 | 0 | Optimal | Optimal | 5.60e-16 | 17 | 17 | 5.44s | 5.26s | 1.0x | PASS |
| PENTAGON | 6 | 15 | Optimal | Optimal | 2.00e-08 | 17 | 19 | 1.5ms | 5.3ms | 3.5x | PASS |
| PFIT1 | 3 | 3 | Optimal | Optimal | 0.00e+00 | 12 | 73 | 282us | 23.1ms | 82.1x | PASS |
| PFIT1LS | 3 | 0 | Optimal | Optimal | 1.85e-18 | 263 | 263 | 2.0ms | 53.1ms | 27.2x | PASS |
| PFIT2 | 3 | 3 | Optimal | RestorationF | N/A | 29 | 417 | 435us | 121.5ms | 279.4x | ipopt_FAIL |
| PFIT2LS | 3 | 0 | Optimal | Optimal | 1.35e-16 | 81 | 81 | 620us | 16.5ms | 26.5x | PASS |
| PFIT3 | 3 | 3 | RestorationF | Optimal | N/A | 2453 | 152 | 64.7ms | 49.2ms | 0.8x | ripopt_FAIL |
| PFIT3LS | 3 | 0 | Optimal | Optimal | 8.88e-17 | 132 | 131 | 946us | 26.4ms | 27.9x | PASS |
| PFIT4 | 3 | 3 | Optimal | RestorationF | N/A | 11 | 610 | 189us | 189.6ms | 1001.7x | ipopt_FAIL |
| PFIT4LS | 3 | 0 | Optimal | Optimal | 4.05e-17 | 215 | 215 | 1.6ms | 43.5ms | 27.8x | PASS |
| POLAK1 | 3 | 2 | Optimal | Optimal | 5.95e-09 | 5 | 5 | 121us | 1.3ms | 10.3x | PASS |
| POLAK2 | 11 | 2 | Optimal | Optimal | 7.19e-11 | 11 | 10 | 339us | 2.4ms | 7.1x | PASS |
| POLAK3 | 12 | 10 | Optimal | MaxIteration | N/A | 73 | 3000 | 6.1ms | 1.38s | 225.9x | ipopt_FAIL |
| POLAK4 | 3 | 3 | Optimal | Optimal | 1.18e-08 | 45 | 4 | 898us | 1.0ms | 1.2x | PASS |
| POLAK5 | 3 | 2 | Acceptable | Optimal | 2.00e-10 | 27 | 31 | 445us | 6.3ms | 14.1x | PASS |
| POLAK6 | 5 | 4 | Optimal | Optimal | 3.41e-10 | 153 | 654 | 3.8ms | 171.6ms | 45.1x | PASS |
| PORTFL1 | 12 | 1 | Optimal | Optimal | 3.00e-08 | 9 | 9 | 516us | 2.3ms | 4.5x | PASS |
| PORTFL2 | 12 | 1 | Optimal | Optimal | 3.55e-08 | 8 | 8 | 522us | 2.2ms | 4.2x | PASS |
| PORTFL3 | 12 | 1 | Optimal | Optimal | 3.02e-08 | 9 | 9 | 636us | 2.5ms | 3.9x | PASS |
| PORTFL4 | 12 | 1 | Optimal | Optimal | 3.02e-08 | 8 | 8 | 478us | 2.1ms | 4.4x | PASS |
| PORTFL6 | 12 | 1 | Optimal | Optimal | 3.02e-08 | 8 | 8 | 589us | 2.1ms | 3.6x | PASS |
| POWELLBS | 2 | 2 | Optimal | Optimal | 0.00e+00 | 11 | 11 | 70us | 1.7ms | 24.2x | PASS |
| POWELLBSLS | 2 | 0 | Optimal | Optimal | 1.58e-27 | 91 | 91 | 422us | 14.6ms | 34.7x | PASS |
| POWELLSQ | 2 | 2 | RestorationF | Infeasible | N/A | 15 | 29 | 2.2ms | 7.2ms | 3.3x | BOTH_FAIL |
| POWELLSQLS | 2 | 0 | Optimal | Optimal | 1.09e-30 | 10 | 10 | 55us | 2.2ms | 39.5x | PASS |
| PRICE3NE | 2 | 2 | Optimal | Optimal | 0.00e+00 | 7 | 7 | 88us | 1.2ms | 14.2x | PASS |
| PRICE4 | 2 | 0 | Optimal | Optimal | 0.00e+00 | 8 | 8 | 42us | 1.4ms | 34.3x | PASS |
| PRICE4B | 2 | 0 | Optimal | Optimal | 1.02e-19 | 8 | 8 | 83us | 1.9ms | 22.5x | PASS |
| PRICE4NE | 2 | 2 | Optimal | Acceptable | 0.00e+00 | 23 | 23 | 139us | 4.2ms | 30.4x | PASS |
| PRODPL0 | 60 | 29 | Optimal | Optimal | 3.28e-09 | 14 | 15 | 10.3ms | 4.9ms | 0.5x | PASS |
| PRODPL1 | 60 | 29 | Optimal | Optimal | 5.04e-09 | 19 | 28 | 12.6ms | 9.6ms | 0.8x | PASS |
| PSPDOC | 4 | 0 | Optimal | Optimal | 2.08e-09 | 5 | 5 | 61us | 1.3ms | 20.6x | PASS |
| PT | 2 | 501 | Optimal | Optimal | 9.73e-09 | 55 | 106 | 195.4ms | 167.5ms | 0.9x | PASS |
| QC | 9 | 4 | Optimal | Optimal | 1.61e-09 | 42 | 44 | 1.3ms | 12.3ms | 9.3x | PASS |
| QCNEW | 9 | 3 | Optimal | Optimal | 5.72e-09 | 5 | 6 | 196us | 1.5ms | 7.6x | PASS |
| QPCBLEND | 83 | 74 | Optimal | Optimal | 1.23e-07 | 16 | 19 | 15.8ms | 7.9ms | 0.5x | PASS |
| QPNBLEND | 83 | 74 | Optimal | Optimal | 1.54e-07 | 15 | 18 | 15.2ms | 7.6ms | 0.5x | PASS |
| RAT42 | 3 | 9 | RestorationF | IpoptStatus( | N/A | 20 | 0 | 4.2ms | 156us | 0.0x | BOTH_FAIL |
| RAT42LS | 3 | 0 | Optimal | Optimal | 5.95e-15 | 28 | 28 | 185us | 5.2ms | 28.0x | PASS |
| RAT43 | 4 | 15 | RestorationF | IpoptStatus( | N/A | 20 | 0 | 11.3ms | 116us | 0.0x | BOTH_FAIL |
| RAT43LS | 4 | 0 | Optimal | Optimal | 2.07e-16 | 34 | 34 | 423us | 6.6ms | 15.6x | PASS |
| RECIPE | 3 | 3 | Optimal | Optimal | 0.00e+00 | 16 | 16 | 113us | 2.9ms | 25.6x | PASS |
| RECIPELS | 3 | 0 | Optimal | Optimal | 2.45e-17 | 29 | 29 | 152us | 5.6ms | 37.1x | PASS |
| RES | 20 | 14 | Optimal | Optimal | 0.00e+00 | 8 | 10 | 675us | 2.4ms | 3.6x | PASS |
| RK23 | 17 | 11 | Optimal | Optimal | 3.76e-08 | 7 | 10 | 1.0ms | 3.3ms | 3.2x | PASS |
| ROBOT | 14 | 2 | Optimal | Optimal | 1.21e-15 | 15 | 18 | 530us | 4.9ms | 9.3x | PASS |
| ROSENBR | 2 | 0 | Optimal | Optimal | 0.00e+00 | 21 | 21 | 103us | 3.6ms | 35.2x | PASS |
| ROSENBRTU | 2 | 0 | Optimal | Optimal | 5.16e-22 | 87 | 85 | 396us | 15.0ms | 37.8x | PASS |
| ROSENMMX | 5 | 4 | Optimal | Optimal | 3.45e-10 | 16 | 13 | 414us | 3.3ms | 8.1x | PASS |
| ROSZMAN1 | 4 | 25 | RestorationF | IpoptStatus( | N/A | 24 | 0 | 34.8ms | 124us | 0.0x | BOTH_FAIL |
| ROSZMAN1LS | 4 | 0 | Optimal | Optimal | 7.59e-19 | 28 | 28 | 301us | 5.6ms | 18.6x | PASS |
| RSNBRNE | 2 | 2 | Optimal | Optimal | 0.00e+00 | 1 | 1 | 19us | 362us | 19.6x | PASS |
| S268 | 5 | 5 | Optimal | Optimal | 5.59e-07 | 14 | 14 | 389us | 3.1ms | 7.9x | PASS |
| S308 | 2 | 0 | Optimal | Optimal | 0.00e+00 | 9 | 9 | 51us | 1.7ms | 32.5x | PASS |
| S308NE | 2 | 3 | LocalInfeasi | IpoptStatus( | N/A | 21 | 0 | 2.7ms | 108us | 0.0x | BOTH_FAIL |
| S316-322 | 2 | 1 | Optimal | Optimal | 0.00e+00 | 7 | 7 | 67us | 1.6ms | 23.5x | PASS |
| S365 | 7 | 5 | EvaluationEr | RestorationF | N/A | 0 | 1 | 14us | 1.0ms | 70.7x | BOTH_FAIL |
| S365MOD | 7 | 5 | EvaluationEr | RestorationF | N/A | 0 | 1 | 14us | 996us | 69.3x | BOTH_FAIL |
| SANTA | 21 | 23 | RestorationF | IpoptStatus( | N/A | 53 | 0 | 49.7ms | 124us | 0.0x | BOTH_FAIL |
| SANTALS | 21 | 0 | Optimal | Optimal | 7.35e-18 | 31 | 31 | 1.3ms | 8.7ms | 6.8x | PASS |
| SIM2BQP | 2 | 0 | Optimal | Optimal | 4.06e-09 | 5 | 5 | 55us | 1.7ms | 30.0x | PASS |
| SIMBQP | 2 | 0 | Optimal | Optimal | 7.13e-09 | 5 | 5 | 84us | 1.2ms | 14.4x | PASS |
| SIMPLLPA | 2 | 2 | Optimal | Optimal | 9.41e-09 | 7 | 8 | 216us | 1.9ms | 9.0x | PASS |
| SIMPLLPB | 2 | 3 | Optimal | Optimal | 9.49e-09 | 8 | 7 | 296us | 1.6ms | 5.5x | PASS |
| SINEVAL | 2 | 0 | Optimal | Optimal | 3.35e-42 | 42 | 42 | 204us | 7.5ms | 36.9x | PASS |
| SINVALNE | 2 | 2 | Optimal | Optimal | 0.00e+00 | 1 | 1 | 17us | 375us | 21.8x | PASS |
| SIPOW1 | 2 | 2000 | Optimal | Optimal | 3.34e-09 | 151 | 79 | 1.53s | 440.2ms | 0.3x | PASS |
| SIPOW1M | 2 | 2000 | Optimal | Optimal | 6.48e-09 | 133 | 94 | 782.1ms | 546.1ms | 0.7x | PASS |
| SIPOW2 | 2 | 2000 | MaxIteration | Optimal | N/A | 2999 | 69 | 9.85s | 380.8ms | 0.0x | ripopt_FAIL |
| SIPOW2M | 2 | 2000 | Optimal | Optimal | 1.65e-09 | 81 | 73 | 735.1ms | 399.4ms | 0.5x | PASS |
| SIPOW3 | 4 | 2000 | Optimal | Optimal | 2.51e-08 | 12 | 12 | 144.2ms | 73.0ms | 0.5x | PASS |
| SIPOW4 | 4 | 2000 | Optimal | Optimal | 1.66e-08 | 12 | 11 | 172.2ms | 74.4ms | 0.4x | PASS |
| SISSER | 2 | 0 | Optimal | Optimal | 1.21e-27 | 18 | 18 | 88us | 2.7ms | 30.5x | PASS |
| SISSER2 | 2 | 0 | Optimal | Optimal | 1.01e-28 | 20 | 20 | 101us | 3.2ms | 31.9x | PASS |
| SNAIL | 2 | 0 | Optimal | Optimal | 5.33e-25 | 63 | 63 | 345us | 10.6ms | 30.8x | PASS |
| SNAKE | 2 | 2 | Optimal | Optimal | 1.59e-08 | 10 | 8 | 258us | 2.2ms | 8.4x | PASS |
| SPANHYD | 97 | 33 | Optimal | Optimal | 0.00e+00 | 285 | 23 | 136.8ms | 8.8ms | 0.1x | PASS |
| SPIRAL | 3 | 2 | NumericalErr | MaxIteration | N/A | 337 | 3000 | 5.9ms | 431.5ms | 72.9x | BOTH_FAIL |
| SSI | 3 | 0 | MaxIteration | MaxIteration | N/A | 2999 | 3000 | 14.5ms | 476.5ms | 32.9x | BOTH_FAIL |
| SSINE | 3 | 2 | MaxIteration | Optimal | N/A | 2999 | 224 | 47.0ms | 39.5ms | 0.8x | ripopt_FAIL |
| STANCMIN | 3 | 2 | Optimal | Optimal | 3.68e-09 | 10 | 9 | 223us | 2.0ms | 8.9x | PASS |
| STRATEC | 10 | 0 | Optimal | Optimal | 1.54e-14 | 24 | 24 | 3.07s | 2.95s | 1.0x | PASS |
| STREG | 4 | 0 | Acceptable | Optimal | 3.64e-01 | 10 | 13 | 55us | 2.5ms | 45.5x | MISMATCH |
| STREGNE | 4 | 2 | Optimal | Optimal | 0.00e+00 | 2 | 2 | 38us | 507us | 13.3x | PASS |
| SUPERSIM | 2 | 2 | Optimal | Optimal | 0.00e+00 | 5 | 1 | 57us | 481us | 8.5x | PASS |
| SWOPF | 83 | 92 | Optimal | Optimal | 1.51e-08 | 14 | 13 | 18.5ms | 5.4ms | 0.3x | PASS |
| SYNTHES1 | 6 | 6 | Optimal | Optimal | 2.51e-08 | 10 | 8 | 335us | 2.0ms | 5.9x | PASS |
| SYNTHES2 | 11 | 14 | Optimal | Optimal | 5.12e-08 | 12 | 14 | 1.6ms | 3.4ms | 2.2x | PASS |
| SYNTHES3 | 17 | 23 | Optimal | Optimal | 4.63e-09 | 13 | 13 | 2.9ms | 3.6ms | 1.2x | PASS |
| TAME | 2 | 1 | Optimal | Optimal | 0.00e+00 | 5 | 5 | 77us | 1.1ms | 14.5x | PASS |
| TAX13322 | 72 | 1261 | NumericalErr | Timeout | N/A | 38 | 0 | 557.6ms | 60.00s | 107.6x | BOTH_FAIL |
| TAXR13322 | 72 | 1261 | MaxTimeExcee | Optimal | N/A | 340 | 1991 | 30.46s | 12.18s | 0.4x | ripopt_FAIL |
| TENBARS1 | 18 | 9 | Optimal | Optimal | 3.12e-03 | 19 | 39 | 1.3ms | 10.7ms | 8.5x | MISMATCH |
| TENBARS2 | 18 | 8 | Optimal | Optimal | 1.45e-11 | 14 | 33 | 730us | 8.5ms | 11.7x | PASS |
| TENBARS3 | 18 | 8 | Optimal | Optimal | 1.40e-11 | 14 | 34 | 768us | 8.8ms | 11.4x | PASS |
| TENBARS4 | 18 | 9 | Optimal | Optimal | 1.16e-11 | 15 | 14 | 827us | 4.0ms | 4.9x | PASS |
| TFI1 | 3 | 101 | Optimal | Optimal | 1.04e-09 | 41 | 19 | 155.2ms | 10.9ms | 0.1x | PASS |
| TFI2 | 3 | 101 | Optimal | Optimal | 1.44e-08 | 8 | 8 | 22.4ms | 4.2ms | 0.2x | PASS |
| TFI3 | 3 | 101 | Optimal | Optimal | 3.36e-09 | 15 | 13 | 37.7ms | 6.7ms | 0.2x | PASS |
| THURBER | 7 | 37 | RestorationF | IpoptStatus( | N/A | 134 | 0 | 70.2ms | 110us | 0.0x | BOTH_FAIL |
| THURBERLS | 7 | 0 | Optimal | Optimal | 2.10e-15 | 19 | 22 | 371us | 5.3ms | 14.4x | PASS |
| TOINTGOR | 50 | 0 | Optimal | Optimal | 3.31e-16 | 7 | 7 | 435us | 1.4ms | 3.3x | PASS |
| TOINTPSP | 50 | 0 | Optimal | Optimal | 0.00e+00 | 17 | 20 | 1.1ms | 5.3ms | 5.0x | PASS |
| TOINTQOR | 50 | 0 | Optimal | Optimal | 0.00e+00 | 1 | 1 | 78us | 380us | 4.9x | PASS |
| TRIGGER | 7 | 6 | Optimal | Optimal | 0.00e+00 | 15 | 15 | 180us | 2.8ms | 15.5x | PASS |
| TRO3X3 | 30 | 13 | MaxIteration | Optimal | N/A | 2999 | 50 | 681.4ms | 18.9ms | 0.0x | ripopt_FAIL |
| TRO4X4 | 63 | 25 | MaxIteration | IpoptStatus( | N/A | 2999 | 255 | 4.02s | 113.4ms | 0.0x | BOTH_FAIL |
| TRO6X2 | 45 | 21 | MaxIteration | IpoptStatus( | N/A | 2999 | 506 | 2.14s | 200.3ms | 0.1x | BOTH_FAIL |
| TRUSPYR1 | 11 | 4 | Optimal | Optimal | 2.50e-09 | 16 | 10 | 705us | 2.2ms | 3.2x | PASS |
| TRUSPYR2 | 11 | 11 | Optimal | Optimal | 3.91e-09 | 12 | 13 | 669us | 3.5ms | 5.2x | PASS |
| TRY-B | 2 | 1 | Optimal | Optimal | 6.25e-18 | 23 | 23 | 308us | 5.3ms | 17.3x | PASS |
| TWOBARS | 2 | 2 | Optimal | Optimal | 3.49e-09 | 9 | 8 | 172us | 1.8ms | 10.5x | PASS |
| VESUVIA | 8 | 1025 | RestorationF | IpoptStatus( | N/A | 8 | 0 | 6.19s | 503us | 0.0x | BOTH_FAIL |
| VESUVIALS | 8 | 0 | Optimal | Optimal | 2.53e-10 | 71 | 48 | 35.4ms | 32.4ms | 0.9x | PASS |
| VESUVIO | 8 | 1025 | MaxTimeExcee | IpoptStatus( | N/A | 5 | 0 | 30.79s | 426us | 0.0x | BOTH_FAIL |
| VESUVIOLS | 8 | 0 | Optimal | Optimal | 0.00e+00 | 10 | 10 | 5.6ms | 7.5ms | 1.3x | PASS |
| VESUVIOU | 8 | 1025 | MaxTimeExcee | IpoptStatus( | N/A | 7 | 0 | 37.50s | 414us | 0.0x | BOTH_FAIL |
| VESUVIOULS | 8 | 0 | Optimal | Optimal | 0.00e+00 | 8 | 8 | 4.6ms | 5.7ms | 1.3x | PASS |
| VIBRBEAM | 8 | 0 | Optimal | Optimal | 3.72e-15 | 58 | 58 | 2.3ms | 10.6ms | 4.6x | PASS |
| VIBRBEAMNE | 8 | 30 | RestorationF | IpoptStatus( | N/A | 20 | 0 | 81.9ms | 146us | 0.0x | BOTH_FAIL |
| WACHBIEG | 3 | 2 | RestorationF | Infeasible | N/A | 14 | 15 | 1.9ms | 6.5ms | 3.5x | BOTH_FAIL |
| WATER | 31 | 10 | Optimal | Optimal | 1.94e-13 | 16 | 17 | 3.3ms | 4.3ms | 1.3x | PASS |
| WAYSEA1 | 2 | 0 | Optimal | Optimal | 0.00e+00 | 14 | 14 | 68us | 2.0ms | 29.5x | PASS |
| WAYSEA1B | 2 | 0 | Optimal | Optimal | 1.80e-13 | 14 | 14 | 130us | 2.7ms | 20.9x | PASS |
| WAYSEA1NE | 2 | 2 | Optimal | Optimal | 0.00e+00 | 7 | 7 | 47us | 1.2ms | 25.0x | PASS |
| WAYSEA2 | 2 | 0 | Optimal | Optimal | 3.54e-24 | 22 | 22 | 111us | 3.1ms | 27.8x | PASS |
| WAYSEA2B | 2 | 0 | Optimal | Optimal | 9.04e-21 | 22 | 22 | 182us | 4.4ms | 23.9x | PASS |
| WAYSEA2NE | 2 | 2 | Optimal | Optimal | 0.00e+00 | 11 | 11 | 76us | 2.0ms | 26.0x | PASS |
| WEEDS | 3 | 0 | MaxIteration | Optimal | N/A | 2999 | 28 | 57.1ms | 7.3ms | 0.1x | ripopt_FAIL |
| WEEDSNE | 3 | 12 | RestorationF | IpoptStatus( | N/A | 355 | 0 | 67.8ms | 204us | 0.0x | BOTH_FAIL |
| WOMFLET | 3 | 3 | Optimal | Optimal | 1.00e+00 | 37 | 8 | 525us | 1.8ms | 3.5x | MISMATCH |
| YFIT | 3 | 0 | Optimal | Optimal | 1.90e-14 | 36 | 36 | 411us | 7.9ms | 19.3x | PASS |
| YFITNE | 3 | 17 | MaxIteration | IpoptStatus( | N/A | 2999 | 0 | 480.7ms | 122us | 0.0x | BOTH_FAIL |
| YFITU | 3 | 0 | Optimal | Optimal | 3.06e-21 | 36 | 36 | 368us | 6.1ms | 16.6x | PASS |
| ZANGWIL2 | 2 | 0 | Optimal | Optimal | 0.00e+00 | 1 | 1 | 10us | 337us | 34.9x | PASS |
| ZANGWIL3 | 3 | 3 | Optimal | Optimal | 0.00e+00 | 1 | 1 | 16us | 327us | 20.5x | PASS |
| ZECEVIC2 | 2 | 2 | Optimal | Optimal | 2.18e-10 | 14 | 8 | 246us | 1.8ms | 7.1x | PASS |
| ZECEVIC3 | 2 | 2 | Optimal | Optimal | 5.20e-11 | 12 | 17 | 223us | 3.5ms | 15.9x | PASS |
| ZECEVIC4 | 2 | 2 | Optimal | Optimal | 9.00e-10 | 10 | 10 | 194us | 2.4ms | 12.3x | PASS |
| ZY2 | 3 | 2 | Optimal | Optimal | 8.30e-09 | 10 | 14 | 209us | 3.6ms | 17.5x | PASS |

## Performance Comparison (where both solve)

### Iteration Comparison

| Metric | ripopt | Ipopt |
|--------|--------|-------|
| Mean   | 38.6 | 37.1 |
| Median | 13 | 12 |
| Total  | 20864 | 20072 |

- ripopt fewer iterations: 84/541
- Ipopt fewer iterations: 171/541
- Tied: 286/541

### Timing Comparison

| Metric | ripopt | Ipopt |
|--------|--------|-------|
| Mean   | 56.5ms | 53.5ms |
| Median | 308us | 3.1ms |
| Total  | 30.57s | 28.93s |

- Geometric mean speedup (Ipopt_time/ripopt_time): **7.44x**
  - \>1 means ripopt is faster, <1 means Ipopt is faster
- ripopt faster: 478/541 problems
- Ipopt faster: 63/541 problems
- Overall speedup (total time): 0.95x

## Failure Analysis

### Problems where only ripopt fails (20)

| Problem | n | m | ripopt status | Ipopt obj |
|---------|---|---|---------------|-----------|
| DECONVBNE | 63 | 40 | Timeout | 0.000000e+00 |
| DISCS | 36 | 66 | Timeout | 1.528822e+01 |
| HAIFAS | 13 | 9 | NumericalError | -4.500000e-01 |
| HATFLDFLNE | 3 | 3 | RestorationFailed | 0.000000e+00 |
| HIMMELP4 | 2 | 3 | NumericalError | -5.901318e+01 |
| HS59 | 2 | 3 | NumericalError | -6.749505e+00 |
| HS91 | 5 | 1 | NumericalError | 1.362646e+00 |
| LOGHAIRY | 2 | 0 | MaxIterations | 1.823216e-01 |
| OET6 | 5 | 1002 | NumericalError | 2.069727e-03 |
| OET7 | 7 | 1002 | NumericalError | 4.446419e-05 |
| PALMER1 | 4 | 0 | MaxIterations | 1.175460e+04 |
| PALMER2 | 4 | 0 | MaxIterations | 3.651098e+03 |
| PALMER3 | 4 | 0 | MaxIterations | 2.416980e+03 |
| PALMER4 | 4 | 0 | MaxIterations | 2.285383e+03 |
| PFIT3 | 3 | 3 | RestorationFailed | 0.000000e+00 |
| SIPOW2 | 2 | 2000 | MaxIterations | -1.000000e+00 |
| SSINE | 3 | 2 | MaxIterations | 0.000000e+00 |
| TAXR13322 | 72 | 1261 | MaxTimeExceeded | -6.449419e+04 |
| TRO3X3 | 30 | 13 | MaxIterations | 8.967215e+00 |
| WEEDS | 3 | 0 | MaxIterations | 2.587277e+00 |

### Problems where only Ipopt fails (20)

| Problem | n | m | Ipopt status | ripopt obj |
|---------|---|---|--------------|------------|
| AVION2 | 49 | 15 | MaxIterations | 9.468013e+07 |
| BEALENE | 2 | 3 | IpoptStatus(-10) | 0.000000e+00 |
| BIGGS6NE | 6 | 13 | IpoptStatus(-10) | 0.000000e+00 |
| BOX3NE | 3 | 10 | IpoptStatus(-10) | 0.000000e+00 |
| BROWNBSNE | 2 | 3 | IpoptStatus(-10) | 0.000000e+00 |
| DECONVB | 63 | 0 | MaxIterations | 3.318319e-03 |
| DENSCHNBNE | 2 | 3 | IpoptStatus(-10) | 0.000000e+00 |
| DEVGLA1NE | 4 | 24 | IpoptStatus(-10) | 0.000000e+00 |
| DEVGLA2NE | 5 | 16 | IpoptStatus(-10) | 0.000000e+00 |
| ENGVAL2NE | 3 | 5 | IpoptStatus(-10) | 0.000000e+00 |
| EQC | 9 | 3 | ErrorInStepComputation | -8.293542e+02 |
| EXP2NE | 2 | 10 | IpoptStatus(-10) | 0.000000e+00 |
| GROUPING | 100 | 125 | IpoptStatus(-10) | 1.385040e+01 |
| HS25NE | 3 | 99 | IpoptStatus(-10) | 0.000000e+00 |
| LANCZOS1 | 6 | 24 | IpoptStatus(-10) | 0.000000e+00 |
| LEVYMONE5 | 2 | 4 | IpoptStatus(-10) | 0.000000e+00 |
| LEWISPOL | 6 | 9 | IpoptStatus(-10) | 2.999995e+00 |
| PFIT2 | 3 | 3 | RestorationFailed | 0.000000e+00 |
| PFIT4 | 3 | 3 | RestorationFailed | 0.000000e+00 |
| POLAK3 | 12 | 10 | MaxIterations | 5.933003e+00 |

### Problems where both fail (146)

| Problem | n | m | ripopt status | Ipopt status |
|---------|---|---|---------------|--------------|
| ARGAUSS | 3 | 15 | RestorationFailed | IpoptStatus(-10) |
| BARDNE | 3 | 15 | RestorationFailed | IpoptStatus(-10) |
| BENNETT5 | 3 | 154 | RestorationFailed | IpoptStatus(-10) |
| BLEACHNG | 17 | 0 | Timeout | Timeout |
| BOXBOD | 2 | 6 | RestorationFailed | IpoptStatus(-10) |
| BROWNDENE | 4 | 20 | RestorationFailed | IpoptStatus(-10) |
| BURKEHAN | 1 | 1 | LocalInfeasibility | Infeasible |
| CERI651A | 7 | 61 | RestorationFailed | IpoptStatus(-10) |
| CERI651B | 7 | 66 | RestorationFailed | IpoptStatus(-10) |
| CERI651C | 7 | 56 | LocalInfeasibility | IpoptStatus(-10) |
| CERI651D | 7 | 67 | RestorationFailed | IpoptStatus(-10) |
| CERI651E | 7 | 64 | RestorationFailed | IpoptStatus(-10) |
| CHWIRUT1 | 3 | 214 | RestorationFailed | IpoptStatus(-10) |
| CHWIRUT2 | 3 | 54 | RestorationFailed | IpoptStatus(-10) |
| CRESC100 | 6 | 200 | MaxTimeExceeded | Infeasible |
| CRESC132 | 6 | 2654 | MaxTimeExceeded | Timeout |
| CRESC50 | 6 | 100 | NumericalError | Infeasible |
| DANIWOOD | 2 | 6 | RestorationFailed | IpoptStatus(-10) |
| DANWOOD | 2 | 6 | RestorationFailed | IpoptStatus(-10) |
| DENSCHNENE | 3 | 3 | LocalInfeasibility | Infeasible |
| DIAMON2D | 66 | 4643 | Timeout | IpoptStatus(-10) |
| DIAMON2DLS | 66 | 0 | MaxTimeExceeded | Timeout |
| DIAMON3D | 99 | 4643 | Timeout | IpoptStatus(-10) |
| DIAMON3DLS | 99 | 0 | MaxTimeExceeded | Timeout |
| DMN15102 | 66 | 4643 | Timeout | IpoptStatus(-10) |
| DMN15102LS | 66 | 0 | MaxTimeExceeded | Timeout |
| DMN15103 | 99 | 4643 | Timeout | IpoptStatus(-10) |
| DMN15103LS | 99 | 0 | MaxTimeExceeded | Timeout |
| DMN15332 | 66 | 4643 | Timeout | IpoptStatus(-10) |
| DMN15332LS | 66 | 0 | MaxTimeExceeded | Timeout |
| DMN15333 | 99 | 4643 | Timeout | IpoptStatus(-10) |
| DMN15333LS | 99 | 0 | MaxTimeExceeded | Timeout |
| DMN37142 | 66 | 4643 | Timeout | IpoptStatus(-10) |
| DMN37142LS | 66 | 0 | MaxTimeExceeded | Timeout |
| DMN37143 | 99 | 4643 | Timeout | IpoptStatus(-10) |
| DMN37143LS | 99 | 0 | MaxTimeExceeded | Timeout |
| ECKERLE4 | 3 | 35 | RestorationFailed | IpoptStatus(-10) |
| EGGCRATENE | 2 | 4 | RestorationFailed | IpoptStatus(-10) |
| ELATVIDUNE | 2 | 3 | RestorationFailed | IpoptStatus(-10) |
| ENSO | 9 | 168 | RestorationFailed | IpoptStatus(-10) |
| EXPFITNE | 2 | 10 | RestorationFailed | IpoptStatus(-10) |
| FBRAIN | 2 | 2211 | MaxTimeExceeded | IpoptStatus(-10) |
| FBRAIN2 | 4 | 2211 | Timeout | IpoptStatus(-10) |
| FBRAIN2NE | 4 | 2211 | Timeout | IpoptStatus(-10) |
| FBRAIN3 | 6 | 2211 | Timeout | IpoptStatus(-10) |
| FBRAIN3LS | 6 | 0 | MaxIterations | MaxIterations |
| FBRAINNE | 2 | 2211 | Timeout | IpoptStatus(-10) |
| GAUSS1 | 8 | 250 | RestorationFailed | IpoptStatus(-10) |
| GAUSS2 | 8 | 250 | RestorationFailed | IpoptStatus(-10) |
| GAUSS3 | 8 | 250 | RestorationFailed | IpoptStatus(-10) |
| GBRAIN | 2 | 2200 | Timeout | IpoptStatus(-10) |
| GROWTH | 3 | 12 | RestorationFailed | IpoptStatus(-10) |
| GULFNE | 3 | 99 | NumericalError | IpoptStatus(-10) |
| HAHN1 | 7 | 236 | LocalInfeasibility | IpoptStatus(-10) |
| HATFLDBNE | 4 | 4 | MaxIterations | Infeasible |
| HATFLDDNE | 3 | 10 | LocalInfeasibility | IpoptStatus(-10) |
| HATFLDENE | 3 | 21 | LocalInfeasibility | IpoptStatus(-10) |
| HIMMELBD | 2 | 2 | RestorationFailed | Infeasible |
| HIMMELBFNE | 4 | 7 | RestorationFailed | IpoptStatus(-10) |
| HIMMELBJ | 45 | 14 | NumericalError | ErrorInStepComputation |
| HS2NE | 2 | 2 | RestorationFailed | Infeasible |
| HS87 | 6 | 4 | MaxIterations | MaxIterations |
| JENSMPNE | 2 | 10 | LocalInfeasibility | IpoptStatus(-10) |
| JUDGENE | 2 | 20 | RestorationFailed | IpoptStatus(-10) |
| KIRBY2 | 5 | 151 | RestorationFailed | IpoptStatus(-10) |
| KOEBHELBNE | 3 | 156 | MaxTimeExceeded | IpoptStatus(-10) |
| KOWOSBNE | 4 | 11 | LocalInfeasibility | IpoptStatus(-10) |
| LANCZOS2 | 6 | 24 | RestorationFailed | IpoptStatus(-10) |
| LANCZOS3 | 6 | 24 | LocalInfeasibility | IpoptStatus(-10) |
| LEVYMONE10 | 10 | 20 | LocalInfeasibility | IpoptStatus(-10) |
| LEVYMONE6 | 3 | 6 | LocalInfeasibility | IpoptStatus(-10) |
| LEVYMONE7 | 4 | 8 | LocalInfeasibility | IpoptStatus(-10) |
| LEVYMONE8 | 5 | 10 | LocalInfeasibility | IpoptStatus(-10) |
| LEVYMONE9 | 8 | 16 | LocalInfeasibility | IpoptStatus(-10) |
| LHAIFAM | 99 | 150 | EvaluationError | InvalidNumberDetected |
| LSC1 | 3 | 6 | RestorationFailed | IpoptStatus(-10) |
| LSC2 | 3 | 6 | RestorationFailed | IpoptStatus(-10) |
| MESH | 41 | 48 | MaxIterations | IpoptStatus(4) |
| MEYER3NE | 3 | 16 | LocalInfeasibility | IpoptStatus(-10) |
| MGH09 | 4 | 11 | RestorationFailed | IpoptStatus(-10) |
| MGH10 | 3 | 16 | LocalInfeasibility | IpoptStatus(-10) |
| MGH10S | 3 | 16 | LocalInfeasibility | IpoptStatus(-10) |
| MGH17 | 5 | 33 | RestorationFailed | IpoptStatus(-10) |
| MGH17S | 5 | 33 | RestorationFailed | IpoptStatus(-10) |
| MISRA1A | 2 | 14 | RestorationFailed | IpoptStatus(-10) |
| MISRA1B | 2 | 14 | RestorationFailed | IpoptStatus(-10) |
| MISRA1C | 2 | 14 | LocalInfeasibility | IpoptStatus(-10) |
| MISRA1D | 2 | 14 | RestorationFailed | IpoptStatus(-10) |
| MSS1 | 90 | 73 | StopAtTinyStep | MaxIterations |
| MUONSINE | 1 | 512 | Timeout | IpoptStatus(-10) |
| NASH | 72 | 24 | RestorationFailed | Infeasible |
| NELSON | 3 | 128 | RestorationFailed | IpoptStatus(-10) |
| NYSTROM5 | 18 | 20 | RestorationFailed | IpoptStatus(-10) |
| NYSTROM5C | 18 | 20 | RestorationFailed | IpoptStatus(-10) |
| OSBORNE1 | 5 | 33 | RestorationFailed | IpoptStatus(-10) |
| OSBORNE2 | 11 | 65 | RestorationFailed | IpoptStatus(-10) |
| PALMER1ANE | 6 | 35 | RestorationFailed | IpoptStatus(-10) |
| PALMER1BNE | 4 | 35 | LocalInfeasibility | IpoptStatus(-10) |
| PALMER1ENE | 8 | 35 | LocalInfeasibility | IpoptStatus(-10) |
| PALMER1NE | 4 | 31 | RestorationFailed | IpoptStatus(-10) |
| PALMER2ANE | 6 | 23 | RestorationFailed | IpoptStatus(-10) |
| PALMER2BNE | 4 | 23 | LocalInfeasibility | IpoptStatus(-10) |
| PALMER2ENE | 8 | 23 | RestorationFailed | IpoptStatus(-10) |
| PALMER2NE | 4 | 23 | RestorationFailed | IpoptStatus(-10) |
| PALMER3ANE | 6 | 23 | RestorationFailed | IpoptStatus(-10) |
| PALMER3BNE | 4 | 23 | LocalInfeasibility | IpoptStatus(-10) |
| PALMER3ENE | 8 | 23 | LocalInfeasibility | IpoptStatus(-10) |
| PALMER3NE | 4 | 23 | RestorationFailed | IpoptStatus(-10) |
| PALMER4ANE | 6 | 23 | RestorationFailed | IpoptStatus(-10) |
| PALMER4BNE | 4 | 23 | LocalInfeasibility | IpoptStatus(-10) |
| PALMER4ENE | 8 | 23 | LocalInfeasibility | IpoptStatus(-10) |
| PALMER4NE | 4 | 23 | RestorationFailed | IpoptStatus(-10) |
| PALMER5A | 8 | 0 | MaxIterations | MaxIterations |
| PALMER5ANE | 8 | 12 | LocalInfeasibility | IpoptStatus(-10) |
| PALMER5BNE | 9 | 12 | RestorationFailed | IpoptStatus(-10) |
| PALMER5E | 8 | 0 | MaxIterations | MaxIterations |
| PALMER5ENE | 8 | 12 | RestorationFailed | IpoptStatus(-10) |
| PALMER6ANE | 6 | 13 | RestorationFailed | IpoptStatus(-10) |
| PALMER6ENE | 8 | 13 | RestorationFailed | IpoptStatus(-10) |
| PALMER7A | 6 | 0 | MaxIterations | MaxIterations |
| PALMER7ANE | 6 | 13 | MaxIterations | IpoptStatus(-10) |
| PALMER7E | 8 | 0 | MaxIterations | MaxIterations |
| PALMER7ENE | 8 | 13 | RestorationFailed | IpoptStatus(-10) |
| PALMER8ANE | 6 | 12 | RestorationFailed | IpoptStatus(-10) |
| PALMER8ENE | 8 | 12 | RestorationFailed | IpoptStatus(-10) |
| POWELLSQ | 2 | 2 | RestorationFailed | Infeasible |
| RAT42 | 3 | 9 | RestorationFailed | IpoptStatus(-10) |
| RAT43 | 4 | 15 | RestorationFailed | IpoptStatus(-10) |
| ROSZMAN1 | 4 | 25 | RestorationFailed | IpoptStatus(-10) |
| S308NE | 2 | 3 | LocalInfeasibility | IpoptStatus(-10) |
| S365 | 7 | 5 | EvaluationError | RestorationFailed |
| S365MOD | 7 | 5 | EvaluationError | RestorationFailed |
| SANTA | 21 | 23 | RestorationFailed | IpoptStatus(-10) |
| SPIRAL | 3 | 2 | NumericalError | MaxIterations |
| SSI | 3 | 0 | MaxIterations | MaxIterations |
| TAX13322 | 72 | 1261 | NumericalError | Timeout |
| THURBER | 7 | 37 | RestorationFailed | IpoptStatus(-10) |
| TRO4X4 | 63 | 25 | MaxIterations | IpoptStatus(4) |
| TRO6X2 | 45 | 21 | MaxIterations | IpoptStatus(4) |
| VESUVIA | 8 | 1025 | RestorationFailed | IpoptStatus(-10) |
| VESUVIO | 8 | 1025 | MaxTimeExceeded | IpoptStatus(-10) |
| VESUVIOU | 8 | 1025 | MaxTimeExceeded | IpoptStatus(-10) |
| VIBRBEAMNE | 8 | 30 | RestorationFailed | IpoptStatus(-10) |
| WACHBIEG | 3 | 2 | RestorationFailed | Infeasible |
| WEEDSNE | 3 | 12 | RestorationFailed | IpoptStatus(-10) |
| YFITNE | 3 | 17 | MaxIterations | IpoptStatus(-10) |

### Objective mismatches (19)

Both solvers converged but found different objective values (rel diff > 1e-4).

- **Different local minimum** (both Optimal): 14
- **Convergence gap** (one Acceptable): 5
- **Better objective found by**: ripopt 7, Ipopt 12

| Problem | ripopt obj | Ipopt obj | Rel Diff | r_status | i_status | Better |
|---------|-----------|-----------|----------|----------|----------|--------|
| WOMFLET | -7.272776e-09 | 6.050000e+00 | 1.00e+00 | Optimal | Optimal | ripopt |
| LEVYMONT5 | 7.773811e+00 | 1.239502e-25 | 1.00e+00 | Optimal | Optimal | ipopt |
| MGH17LS | 1.022414e+00 | 7.895088e-05 | 1.00e+00 | Acceptable | Optimal | ipopt |
| ELATTAR | 1.427080e-01 | 7.420618e+01 | 9.98e-01 | Optimal | Optimal | ripopt |
| HIMMELP6 | -8.198044e+00 | -5.901318e+01 | 8.61e-01 | Optimal | Optimal | ipopt |
| STREG | 4.526251e-01 | 8.901950e-02 | 3.64e-01 | Acceptable | Optimal | ipopt |
| HS108 | -6.749814e-01 | -5.000000e-01 | 1.75e-01 | Acceptable | Optimal | ripopt |
| HS70 | 1.798079e-01 | 7.498464e-03 | 1.72e-01 | Optimal | Optimal | ipopt |
| PALMER2E | 2.065047e-04 | 1.163043e-01 | 1.16e-01 | Optimal | Optimal | ripopt |
| ANTWERP | 3.485094e+03 | 3.245241e+03 | 6.88e-02 | Optimal | Optimal | ipopt |
| HS54 | -8.652402e-01 | -9.080748e-01 | 4.28e-02 | Acceptable | Optimal | ipopt |
| PALMER5B | 2.856378e-02 | 9.752496e-03 | 1.88e-02 | Optimal | Optimal | ipopt |
| TENBARS1 | 2.302548e+03 | 2.295373e+03 | 3.12e-03 | Optimal | Optimal | ipopt |
| LRCOVTYPE | 5.753182e-01 | 5.723072e-01 | 3.01e-03 | Optimal | Optimal | ipopt |
| DECONVC | 4.179247e-09 | 2.569475e-03 | 2.57e-03 | Optimal | Optimal | ripopt |
| LIN | -1.960628e-02 | -1.757754e-02 | 2.03e-03 | Optimal | Optimal | ripopt |
| HALDMADS | 3.465928e-02 | 3.369586e-02 | 9.63e-04 | Optimal | Optimal | ipopt |
| HS13 | 9.938566e-01 | 9.945785e-01 | 7.22e-04 | Optimal | Optimal | ripopt |
| LSC2LS | 1.334125e+01 | 1.333395e+01 | 5.47e-04 | Acceptable | Optimal | ipopt |

---
*Generated by benchmarks/cutest/compare.py*