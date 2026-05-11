# cutest-smoke

## Command

```bash
make cutest-smoke
```

## Output

```bash
make -C benchmarks cutest-smoke
make[1]: Entering directory '/home/orazio/Projects/ripopt/benchmarks'
bash /home/orazio/Projects/ripopt/benchmarks/cutest/prepare.sh ROSENBR BEALE CUBE DENSCHNB BROWNBS HS6 HS10 HS35 HS71 HS106 MARATOS BT1 HS13 HS57 S316-322 BEALENE BROWNBSNE ENGVAL2NE PALMER1D PALMER5D
MASTSIF: /home/orazio/.local/cutest/mastsif (1542 SIF files)
Preparing 20 CUTEst problems...
  OK   ROSENBR (already prepared)
  OK   BEALE (already prepared)
  OK   CUBE (already prepared)
  OK   DENSCHNB (already prepared)
  OK   BROWNBS (already prepared)
  OK   HS6 (already prepared)
  OK   HS10 (already prepared)
  OK   HS35 (already prepared)
  OK   HS71 (already prepared)
  OK   HS106 (already prepared)
  OK   MARATOS (already prepared)
  OK   BT1 (already prepared)
  OK   HS13 (already prepared)
  OK   HS57 (already prepared)
  OK   S316-322 (already prepared)
  OK   BEALENE (already prepared)
  OK   BROWNBSNE (already prepared)
  OK   ENGVAL2NE (already prepared)
  OK   PALMER1D (already prepared)
  OK   PALMER5D (already prepared)

Done: 20 prepared, 0 failed, 0 skipped
RESULTS_FILE=/home/orazio/Projects/ripopt/benchmarks/cutest/smoke_results.json \
cargo run --manifest-path /home/orazio/Projects/ripopt/Cargo.toml --bin cutest_suite --features cutest,ipopt-native --release -- \
        ROSENBR BEALE CUBE DENSCHNB BROWNBS HS6 HS10 HS35 HS71 HS106 MARATOS BT1 HS13 HS57 S316-322 BEALENE BROWNBSNE ENGVAL2NE PALMER1D PALMER5D \
        2> >(tee /home/orazio/Projects/ripopt/benchmarks/cutest/smoke_stderr.txt >&2)
   Compiling ripopt v0.8.0 (/home/orazio/Projects/ripopt)
    Finished `release` profile [optimized + debuginfo] target(s) in 1m 25s
     Running `/home/orazio/Projects/ripopt/target/release/cutest_suite ROSENBR BEALE CUBE DENSCHNB BROWNBS HS6 HS10 HS35 HS71 HS106 MARATOS BT1 HS13 HS57 S316-322 BEALENE BROWNBSNE ENGVAL2NE PALMER1D PALMER5D`
=== System Information ===
  OS:           linux
  Arch:         x86_64
  CPU:          Intel(R) Core(TM) i5-10400H CPU @ 2.60GHz
  RAM:          30 GB
  Rust version: 0.8.0
  Profile:      release
=========================
CUTEst benchmark: 20 problems, 3 timing runs, max_n=100, timeout=60s
Streaming results to /home/orazio/Projects/ripopt/benchmarks/cutest/smoke_results.jsonl
  ROSENBR (n=2, m=0) ... ripopt: Optimal (obj=3.743976e-21, 0.1ms) ipopt: Optimal (obj=3.743976e-21, 3.5ms) 
  BEALE (n=2, m=0) ... ripopt: Optimal (obj=4.342571e-18, 0.0ms) ipopt: Optimal (obj=4.342571e-18, 1.8ms) 
  CUBE (n=2, m=0) ... ripopt: Optimal (obj=1.753568e-24, 0.1ms) ipopt: Optimal (obj=1.753568e-24, 4.6ms) 
  DENSCHNB (n=2, m=0) ... ripopt: Optimal (obj=9.860761e-32, 0.0ms) ipopt: Optimal (obj=9.860761e-32, 1.3ms) 
  BROWNBS (n=2, m=0) ... ripopt: Optimal (obj=0.000000e0, 0.1ms) ipopt: Optimal (obj=0.000000e0, 1.2ms) 
  HS6 (n=2, m=1) ... ripopt: Optimal (obj=4.930381e-32, 0.1ms) ipopt: Optimal (obj=0.000000e0, 1.3ms) 
  HS10 (n=2, m=1) ... ripopt: Optimal (obj=-1.000000e0, 0.2ms) ipopt: Optimal (obj=-1.000000e0, 2.6ms) 
  HS35 (n=3, m=1) ... ripopt: Optimal (obj=1.111111e-1, 0.1ms) ipopt: Optimal (obj=1.111111e-1, 1.7ms) 
  HS71 (n=4, m=2) ... ripopt: Optimal (obj=1.701402e1, 0.2ms) ipopt: Optimal (obj=1.701402e1, 2.0ms) 
  HS106 (n=8, m=6) ... ripopt: Optimal (obj=7.049248e3, 0.4ms) ipopt: Optimal (obj=7.049248e3, 4.1ms) 
  MARATOS (n=2, m=1) ... ripopt: Optimal (obj=-1.000000e0, 0.0ms) ipopt: Optimal (obj=-1.000000e0, 0.8ms) 
  BT1 (n=2, m=1) ... ripopt: Optimal (obj=-1.000000e0, 0.1ms) ipopt: Optimal (obj=-1.000000e0, 1.5ms) 
  HS13 (n=2, m=1) ... ripopt: Optimal (obj=9.938566e-1, 0.4ms) ipopt: Optimal (obj=9.945785e-1, 10.2ms) 
  HS57 (n=2, m=1) ... ripopt: Optimal (obj=3.064762e-2, 0.4ms) ipopt: Optimal (obj=3.064762e-2, 2.0ms) 
  S316-322 (n=2, m=1) ... ripopt: Optimal (obj=3.343146e2, 0.1ms) ipopt: Optimal (obj=3.343146e2, 1.6ms) 
  BEALENE (n=2, m=3) ... ripopt: Optimal (obj=0.000000e0, 0.4ms) ipopt: IpoptStatus(-10) (obj=0.000000e0, 0.1ms) 
  BROWNBSNE (n=2, m=3) ... ripopt: Optimal (obj=0.000000e0, 0.4ms) ipopt: IpoptStatus(-10) (obj=0.000000e0, 0.1ms) 
  ENGVAL2NE (n=3, m=5) ... ripopt: Optimal (obj=0.000000e0, 0.3ms) ipopt: IpoptStatus(-10) (obj=0.000000e0, 0.1ms) 
  PALMER1D (n=7, m=0) ... ripopt: Optimal (obj=6.526826e-1, 0.0ms) ipopt: Optimal (obj=6.526826e-1, 0.3ms) 
  PALMER5D (n=4, m=0) ... ripopt: Optimal (obj=8.733940e1, 0.0ms) ipopt: Optimal (obj=8.733940e1, 0.3ms) 
[
  {
    "name": "ROSENBR",
    "solver": "ripopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 3.743975643139474e-21,
    "x": [
      0.9999999999400668,
      0.9999999998789006
    ],
    "constraint_violation": 0.0,
    "iterations": 21,
    "solve_time": 0.00010945,
    "final_primal_inf": 0.0,
    "final_dual_inf": 1.731215665429828e-10,
    "final_dual_inf_scaled": 1.731215665429828e-10,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "ROSENBR",
    "solver": "ipopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 3.743975643139474e-21,
    "x": [
      0.9999999999400668,
      0.9999999998789006
    ],
    "constraint_violation": 0.0,
    "iterations": 21,
    "solve_time": 0.003535942
  },
  {
    "name": "BEALE",
    "solver": "ripopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 4.342571473026132e-18,
    "x": [
      2.9999999951069873,
      0.49999999893546615
    ],
    "constraint_violation": 0.0,
    "iterations": 8,
    "solve_time": 0.000046222,
    "final_primal_inf": 0.0,
    "final_dual_inf": 6.862211412312909e-9,
    "final_dual_inf_scaled": 6.862211412312909e-9,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "BEALE",
    "solver": "ipopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 4.342571473026132e-18,
    "x": [
      2.9999999951069873,
      0.49999999893546615
    ],
    "constraint_violation": 0.0,
    "iterations": 8,
    "solve_time": 0.00175437
  },
  {
    "name": "CUBE",
    "solver": "ripopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 1.753567842530729e-24,
    "x": [
      0.9999999999992156,
      0.9999999999975402
    ],
    "constraint_violation": 0.0,
    "iterations": 27,
    "solve_time": 0.000124015,
    "final_primal_inf": 0.0,
    "final_dual_inf": 2.6444874237733827e-12,
    "final_dual_inf_scaled": 2.6444874237733827e-12,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "CUBE",
    "solver": "ipopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 1.753567842530729e-24,
    "x": [
      0.9999999999992156,
      0.9999999999975402
    ],
    "constraint_violation": 0.0,
    "iterations": 27,
    "solve_time": 0.004605572
  },
  {
    "name": "DENSCHNB",
    "solver": "ripopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 9.860761315262648e-32,
    "x": [
      2.0,
      -1.0
    ],
    "constraint_violation": 0.0,
    "iterations": 7,
    "solve_time": 0.000039839,
    "final_primal_inf": 0.0,
    "final_dual_inf": 8.881784197001252e-16,
    "final_dual_inf_scaled": 8.881784197001252e-16,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "DENSCHNB",
    "solver": "ipopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 9.860761315262648e-32,
    "x": [
      2.0,
      -1.0
    ],
    "constraint_violation": 0.0,
    "iterations": 7,
    "solve_time": 0.001321022
  },
  {
    "name": "BROWNBS",
    "solver": "ripopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 0.0,
    "x": [
      1000000.0,
      2e-6
    ],
    "constraint_violation": 0.0,
    "iterations": 7,
    "solve_time": 0.000050266,
    "final_primal_inf": 0.0,
    "final_dual_inf": 0.0,
    "final_dual_inf_scaled": 0.0,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "BROWNBS",
    "solver": "ipopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 0.0,
    "x": [
      1000000.0,
      2e-6
    ],
    "constraint_violation": 0.0,
    "iterations": 7,
    "solve_time": 0.001210126
  },
  {
    "name": "HS6",
    "solver": "ripopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": 4.930380657631324e-32,
    "x": [
      1.0000000000000002,
      1.0000000000000004
    ],
    "constraint_violation": 0.0,
    "iterations": 5,
    "solve_time": 0.000055367,
    "final_primal_inf": 0.0,
    "final_dual_inf": 3.05311331771918e-16,
    "final_dual_inf_scaled": 3.05311331771918e-16,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "HS6",
    "solver": "ipopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": 0.0,
    "x": [
      1.0,
      1.0000000000000002
    ],
    "constraint_violation": 2.220446049250313e-15,
    "iterations": 5,
    "solve_time": 0.001289828
  },
  {
    "name": "HS10",
    "solver": "ripopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": -1.0000000025855558,
    "x": [
      3.244019349363757e-9,
      1.000000005829575
    ],
    "constraint_violation": 5.171111316926158e-9,
    "iterations": 11,
    "solve_time": 0.000173944,
    "final_primal_inf": 0.0,
    "final_dual_inf": 8.736918077900668e-9,
    "final_dual_inf_scaled": 8.736918077900668e-9,
    "final_compl": 2.414444329864688e-9,
    "final_mu": 5.0053011497836054e-9
  },
  {
    "name": "HS10",
    "solver": "ipopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": -1.00000000499,
    "x": [
      -5.053851031114222e-18,
      1.00000000499
    ],
    "constraint_violation": 9.979999937570485e-9,
    "iterations": 12,
    "solve_time": 0.002592867
  },
  {
    "name": "HS35",
    "solver": "ripopt",
    "n": 3,
    "m": 1,
    "status": "Optimal",
    "objective": 0.1111111148277284,
    "x": [
      1.333333337635707,
      0.7777777746974488,
      0.4444444354710347
    ],
    "constraint_violation": 0.0,
    "iterations": 7,
    "solve_time": 0.000149397,
    "final_primal_inf": 0.0,
    "final_dual_inf": 5.936917624183026e-14,
    "final_dual_inf_scaled": 5.936917624183026e-14,
    "final_compl": 5.938839145005231e-9,
    "final_mu": 5.938695574051523e-9
  },
  {
    "name": "HS35",
    "solver": "ipopt",
    "n": 3,
    "m": 1,
    "status": "Optimal",
    "objective": 0.11111110889889032,
    "x": [
      1.333333330012859,
      0.7777777799910702,
      0.44444444997553295
    ],
    "constraint_violation": 9.954995050520664e-9,
    "iterations": 7,
    "solve_time": 0.001688327
  },
  {
    "name": "HS71",
    "solver": "ripopt",
    "n": 4,
    "m": 2,
    "status": "Optimal",
    "objective": 17.014017283235166,
    "x": [
      0.99999999481699,
      4.742999632374415,
      3.8211499891774943,
      1.3794082999137776
    ],
    "constraint_violation": 5.11843012418467e-10,
    "iterations": 9,
    "solve_time": 0.000217911,
    "final_primal_inf": 1.3322676295501878e-15,
    "final_dual_inf": 5.229150445984487e-14,
    "final_dual_inf_scaled": 5.229150445984487e-14,
    "final_compl": 5.2402650350132254e-9,
    "final_mu": 5.240260031488142e-9
  },
  {
    "name": "HS71",
    "solver": "ipopt",
    "n": 4,
    "m": 2,
    "status": "Optimal",
    "objective": 17.01401727277449,
    "x": [
      0.9999999900092812,
      4.742999636070476,
      3.8211499832593807,
      1.3794083070844954
    ],
    "constraint_violation": 9.982336734992714e-9,
    "iterations": 8,
    "solve_time": 0.002006114
  },
  {
    "name": "HS106",
    "solver": "ripopt",
    "n": 8,
    "m": 6,
    "status": "Optimal",
    "objective": 7049.247897934314,
    "x": [
      579.3066922675137,
      1359.9706654076724,
      5109.970540259128,
      182.01770186941067,
      295.6011763490138,
      217.98230212200716,
      286.41652951716196,
      395.6011773481892
    ],
    "constraint_violation": 9.991912852669316e-9,
    "iterations": 11,
    "solve_time": 0.000420637,
    "final_primal_inf": 0.0,
    "final_dual_inf": 4.547473508864641e-13,
    "final_dual_inf_scaled": 4.547473508864641e-13,
    "final_compl": 4.2142228903471065e-8,
    "final_mu": 4.2139350392093646e-8
  },
  {
    "name": "HS106",
    "solver": "ipopt",
    "n": 8,
    "m": 6,
    "status": "Optimal",
    "objective": 7049.247897681538,
    "x": [
      579.3066922569045,
      1359.9706652123832,
      5109.970540212251,
      182.01770187556184,
      295.6011763475186,
      217.98230212443613,
      286.4165295280425,
      395.6011773475184
    ],
    "constraint_violation": 9.999998162868453e-9,
    "iterations": 18,
    "solve_time": 0.004129962
  },
  {
    "name": "MARATOS",
    "solver": "ripopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": -1.0000000000000009,
    "x": [
      1.0000000000000009,
      1.5355409586601045e-15
    ],
    "constraint_violation": 1.776356839400253e-15,
    "iterations": 4,
    "solve_time": 0.000045598,
    "final_primal_inf": 1.776356839400253e-15,
    "final_dual_inf": 1.535540958660102e-15,
    "final_dual_inf_scaled": 1.535540958660102e-15,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "MARATOS",
    "solver": "ipopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": -1.0000000000000009,
    "x": [
      1.0000000000000009,
      1.5355409520426594e-15
    ],
    "constraint_violation": 1.776356839400253e-15,
    "iterations": 4,
    "solve_time": 0.000812118
  },
  {
    "name": "BT1",
    "solver": "ripopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": -0.9999999987917932,
    "x": [
      1.0000000000060714,
      6.661142929950003e-12
    ],
    "constraint_violation": 1.2142731264974633e-11,
    "iterations": 7,
    "solve_time": 0.000067665,
    "final_primal_inf": 1.2142731264974633e-11,
    "final_dual_inf": 9.919176591211e-12,
    "final_dual_inf_scaled": 9.919176591211e-12,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "BT1",
    "solver": "ipopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": -0.9999999999974848,
    "x": [
      1.0000000000000129,
      -7.512430487170066e-15
    ],
    "constraint_violation": 2.5313084961453623e-14,
    "iterations": 7,
    "solve_time": 0.001459239
  },
  {
    "name": "HS13",
    "solver": "ripopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": 0.9938565778817288,
    "x": [
      1.003076443310858,
      -9.999943327321849e-9
    ],
    "constraint_violation": 1.911706498640668e-8,
    "iterations": 27,
    "solve_time": 0.000438545,
    "final_primal_inf": 9.117064986406682e-9,
    "final_dual_inf": 4.048532750237102e-6,
    "final_dual_inf_scaled": 4.048532750237102e-6,
    "final_compl": 3.9796648117480754e-9,
    "final_mu": 3.979664811715033e-9
  },
  {
    "name": "HS13",
    "solver": "ipopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": 0.994578532848864,
    "x": [
      1.0027144176070408,
      -9.999999889173629e-9
    ],
    "constraint_violation": 9.99999989964128e-9,
    "iterations": 47,
    "solve_time": 0.01023441
  },
  {
    "name": "HS57",
    "solver": "ripopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": 0.03064761904761923,
    "x": [
      0.4219047619064043,
      60917.82116610664
    ],
    "constraint_violation": 0.0,
    "iterations": 24,
    "solve_time": 0.000388246,
    "final_primal_inf": 0.0,
    "final_dual_inf": 2.547657016406969e-12,
    "final_dual_inf_scaled": 2.547657016406969e-12,
    "final_compl": 5.0162663332585e-9,
    "final_mu": 5.016265769892318e-9
  },
  {
    "name": "HS57",
    "solver": "ipopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": 0.0306476190476192,
    "x": [
      0.4219047619145534,
      1091.1402007759573
    ],
    "constraint_violation": 0.0,
    "iterations": 10,
    "solve_time": 0.002014334
  },
  {
    "name": "S316-322",
    "solver": "ripopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": 334.31457505076196,
    "x": [
      7.0710678118654755,
      -7.0710678118654755
    ],
    "constraint_violation": 2.220446049250313e-16,
    "iterations": 7,
    "solve_time": 0.000065126,
    "final_primal_inf": 2.220446049250313e-16,
    "final_dual_inf": 7.30331350951019e-11,
    "final_dual_inf_scaled": 7.30331350951019e-11,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "S316-322",
    "solver": "ipopt",
    "n": 2,
    "m": 1,
    "status": "Optimal",
    "objective": 334.31457505076196,
    "x": [
      7.0710678118654755,
      -7.071067811865475
    ],
    "constraint_violation": 5.551115123125783e-17,
    "iterations": 7,
    "solve_time": 0.001567455
  },
  {
    "name": "BEALENE",
    "solver": "ripopt",
    "n": 2,
    "m": 3,
    "status": "Optimal",
    "objective": 0.0,
    "x": [
      3.0000000000065947,
      0.5000000000017821
    ],
    "constraint_violation": 2.049027614248189e-12,
    "iterations": 25,
    "solve_time": 0.000380567,
    "final_primal_inf": 2.049027614248189e-12,
    "final_dual_inf": 7.16227077646181e-12,
    "final_dual_inf_scaled": 7.16227077646181e-12,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "BEALENE",
    "solver": "ipopt",
    "n": 2,
    "m": 3,
    "status": "IpoptStatus(-10)",
    "objective": 0.0,
    "x": [
      1.0,
      1.0
    ],
    "constraint_violation": 2.625,
    "iterations": 0,
    "solve_time": 0.000109393
  },
  {
    "name": "BROWNBSNE",
    "solver": "ripopt",
    "n": 2,
    "m": 3,
    "status": "Optimal",
    "objective": 0.0,
    "x": [
      1000000.0,
      2e-6
    ],
    "constraint_violation": 0.0,
    "iterations": 4,
    "solve_time": 0.000428565,
    "final_primal_inf": 0.0,
    "final_dual_inf": 1.011145580784821e-20,
    "final_dual_inf_scaled": 1.011145580784821e-20,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "BROWNBSNE",
    "solver": "ipopt",
    "n": 2,
    "m": 3,
    "status": "IpoptStatus(-10)",
    "objective": 0.0,
    "x": [
      1.0,
      1.0
    ],
    "constraint_violation": 999999.0,
    "iterations": 0,
    "solve_time": 0.000107914
  },
  {
    "name": "ENGVAL2NE",
    "solver": "ripopt",
    "n": 3,
    "m": 5,
    "status": "Optimal",
    "objective": 0.0,
    "x": [
      2.4797948255012323e-8,
      -2.7342982440175914e-8,
      1.0000000049461448
    ],
    "constraint_violation": 9.892291097379768e-9,
    "iterations": 13,
    "solve_time": 0.000298886,
    "final_primal_inf": 9.892291097379768e-9,
    "final_dual_inf": 4.274638542825179e-7,
    "final_dual_inf_scaled": 4.274638542825179e-7,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "ENGVAL2NE",
    "solver": "ipopt",
    "n": 3,
    "m": 5,
    "status": "IpoptStatus(-10)",
    "objective": 0.0,
    "x": [
      1.0,
      2.0,
      0.0
    ],
    "constraint_violation": 23.0,
    "iterations": 0,
    "solve_time": 0.000108943
  },
  {
    "name": "PALMER1D",
    "solver": "ripopt",
    "n": 7,
    "m": 0,
    "status": "Optimal",
    "objective": 0.6526826233882761,
    "x": [
      88.11201815997583,
      -159.25178600435163,
      123.0391086325404,
      -64.74352023226935,
      24.845515768233795,
      -4.974174135880725,
      0.39618744471687695
    ],
    "constraint_violation": 0.0,
    "iterations": 1,
    "solve_time": 0.000025109,
    "final_primal_inf": 0.0,
    "final_dual_inf": 5.872468671993395e-14,
    "final_dual_inf_scaled": 5.872468671993395e-14,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "PALMER1D",
    "solver": "ipopt",
    "n": 7,
    "m": 0,
    "status": "Optimal",
    "objective": 0.6526826233885079,
    "x": [
      88.11201815969842,
      -159.25178600012092,
      123.03910862146444,
      -64.74352022111057,
      24.84551576298056,
      -4.974174134714392,
      0.39618744461813016
    ],
    "constraint_violation": 0.0,
    "iterations": 1,
    "solve_time": 0.000329986
  },
  {
    "name": "PALMER5D",
    "solver": "ripopt",
    "n": 4,
    "m": 0,
    "status": "Optimal",
    "objective": 87.33939945055276,
    "x": [
      80.25131781794943,
      -132.1059487582921,
      51.6401308923556,
      0.6953143228832808
    ],
    "constraint_violation": 0.0,
    "iterations": 1,
    "solve_time": 0.000012927,
    "final_primal_inf": 0.0,
    "final_dual_inf": 3.772939436783206e-13,
    "final_dual_inf_scaled": 3.772939436783206e-13,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "PALMER5D",
    "solver": "ipopt",
    "n": 4,
    "m": 0,
    "status": "Optimal",
    "objective": 87.33939945055265,
    "x": [
      80.25131781794853,
      -132.10594875828502,
      51.64013089234816,
      0.6953143228852396
    ],
    "constraint_violation": 0.0,
    "iterations": 1,
    "solve_time": 0.000333401
  }
Results written to /home/orazio/Projects/ripopt/benchmarks/cutest/smoke_results.json
]

Summary: 20 problems
  ripopt solved: 20/20
  ipopt  solved: 17/20
Smoke test complete.
make[1]: Leaving directory '/home/orazio/Projects/ripopt/benchmarks'
```
