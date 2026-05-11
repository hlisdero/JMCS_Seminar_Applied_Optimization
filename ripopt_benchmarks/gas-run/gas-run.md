# gas-run

## Command

```bash
make gas-run
```

## Output

```bash
make -C benchmarks gas-run
make[1]: Entering directory '/home/orazio/Projects/ripopt/benchmarks'
=== gaslib11_dynamic.nl ===
   Compiling ripopt v0.8.0 (/home/orazio/Projects/ripopt)
    Finished `release` profile [optimized + debuginfo] target(s) in 1m 22s
     Running `/home/orazio/Projects/ripopt/target/release/ripopt /home/orazio/Projects/ripopt/benchmarks/gas/gaslib11_dynamic.nl print_level=5`
ripopt: Detected 1192/2492 linear constraints (Hessian contribution skipped)

Number of nonzeros in equality constraint Jacobian...:        6594
Number of nonzeros in inequality constraint Jacobian.:           0
Number of nonzeros in Lagrangian Hessian.............:        1350

Total number of variables............................:        2542
                     variables with only lower bounds:         200
                variables with lower and upper bounds:        1042
                     variables with only upper bounds:           0
Total number of equality constraints.................:        2492
Total number of inequality constraints...............:           0
        inequality constraints with only lower bounds:           0
   inequality constraints with lower and upper bounds:           0
        inequality constraints with only upper bounds:           0

iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
ripopt: Starting main loop (n=2542, m=2492)
   0     1.7279983e0      3.49e1      2.60e0      5.50e1        -1.0    0.00e0    0.00e0    0
ripopt: iter0-probe: |y_c|=0.00e0 |y_d|=0.00e0 |z_L|=1.00e0 |z_U|=1.00e0  |dy_c|=0.00e0 |dy_d|=0.00e0 |dz_L|=0.00e0 |dz_U|=0.00e0  ftb_du=-@-1 z=NaN dz=NaN ratio=inf  dw_last=0.00e0 dc_last=0.00e0
ripopt: iter0-probe: |grad_f|_inf=3.600e0@var1350, |J^T y|_inf=0.000e0, |y|_inf=0.000e0, |z_L|_inf=1.000e0, |z_U|_inf=1.000e0, sum|y|=0.000e0, sum|z_L|=1.242e3, sum|z_U|=1.042e3, |x|_inf=5.500e1, n=2542 m=2492
ripopt: iter0-probe: |grad_lag|_inf=2.600e0@var1350 (grad_f=3.600e0, J^T y=0.000e0, z_L=1.000e0, z_U=0.000e0, x_l_fin=true, x_u_fin=false, obj_scaling=1.000e0)
   1     1.7303582e0      3.49e1      1.35e1      5.45e1        -1.0   7.06e-4   9.57e-3    0
ripopt: iter1-probe: |y_c|=3.05e0 |y_d|=0.00e0 |z_L|=1.45e1 |z_U|=1.00e0  |dy_c|=4.32e3 |dy_d|=0.00e0 |dz_L|=1.41e3 |dz_U|=2.47e0  ftb_du=zL@500 z=1.00e-2 dz=-1.03e2 ratio=9.57e-5  dw_last=0.00e0 dc_last=0.00e0
   2     1.7229200e0      3.43e1      1.06e1      5.39e1        -1.0   1.50e-2   1.16e-2    0
ripopt: iter2-probe: |y_c|=3.96e1 |y_d|=0.00e0 |z_L|=2.37e1 |z_U|=1.00e0  |dy_c|=2.44e3 |dy_d|=0.00e0 |dz_L|=7.94e2 |dz_U|=1.31e0  ftb_du=zL@300 z=5.45e-4 dz=-4.66e0 ratio=1.16e-4  dw_last=0.00e0 dc_last=0.00e0
   3     1.6062342e0      2.85e1      4.55e0      5.24e1        -1.0   1.72e-1   2.90e-2    0
ripopt: iter3-probe: |y_c|=6.07e1 |y_d|=0.00e0 |z_L|=2.46e1 |z_U|=1.00e0  |dy_c|=1.23e2 |dy_d|=0.00e0 |dz_L|=2.98e1 |dz_U|=1.28e0  ftb_du=zL@424 z=4.80e-4 dz=-1.64e0 ratio=2.90e-4  dw_last=0.00e0 dc_last=0.00e0
   4     1.0777767e0      1.61e2      1.05e2      4.01e1        -1.0    1.00e0   2.57e-1    0
ripopt: iter4-probe: |y_c|=6.16e1 |y_d|=0.00e0 |z_L|=3.95e1 |z_U|=9.94e-1  |dy_c|=9.50e0 |dy_d|=0.00e0 |dz_L|=1.47e2 |dz_U|=1.10e0  ftb_du=zL@201 z=1.68e-4 dz=-6.46e-2 ratio=2.57e-3  dw_last=0.00e0 dc_last=0.00e0
   5     1.8563923e0      2.29e0      3.91e1      2.89e1        -1.0    1.00e0   2.93e-1    0
ripopt: iter5-probe: |y_c|=1.75e1 |y_d|=0.00e0 |z_L|=2.17e1 |z_U|=8.75e-1  |dy_c|=6.46e1 |dy_d|=0.00e0 |dz_L|=6.08e1 |dz_U|=1.23e0  ftb_du=zL@96 z=1.59e-1 dz=-5.39e1 ratio=2.93e-3  dw_last=0.00e0 dc_last=0.00e0
   6     1.9812644e0     4.40e-1      1.23e0      1.46e0        -1.0    1.00e0   9.48e-1    0
ripopt: iter6-probe: |y_c|=1.31e1 |y_d|=0.00e0 |z_L|=1.28e1 |z_U|=7.32e-1  |dy_c|=6.64e0 |dy_d|=0.00e0 |dz_L|=2.24e1 |dz_U|=6.69e-1  ftb_du=zL@76 z=1.83e-1 dz=-1.91e1 ratio=9.48e-3  dw_last=0.00e0 dc_last=0.00e0
   7     2.0232153e0     3.83e-1     8.02e-2     1.33e-1        -1.0    1.00e0    1.00e0    0
ripopt: iter7-probe: |y_c|=1.04e1 |y_d|=0.00e0 |z_L|=1.09e1 |z_U|=3.55e-1  |dy_c|=3.77e0 |dy_d|=0.00e0 |dz_L|=3.60e0 |dz_U|=4.29e-1  ftb_du=zL@675 z=1.76e-3 dz=-2.40e-2 ratio=7.26e-2  dw_last=0.00e0 dc_last=0.00e0
   8     2.0179741e0     4.34e-2     2.44e-2     2.67e-2        -1.7    1.00e0    1.00e0    0
ripopt: iter8-probe: |y_c|=2.73e0 |y_d|=0.00e0 |z_L|=2.14e0 |z_U|=8.77e-2  |dy_c|=8.81e0 |dy_d|=0.00e0 |dz_L|=8.81e0 |dz_U|=2.67e-1  ftb_du=zU@425 z=5.11e-3 dz=-4.48e-2 ratio=1.13e-1  dw_last=0.00e0 dc_last=0.00e0
   9     1.9590214e0     8.36e-1     6.52e-2     7.88e-3        -2.5    1.00e0   9.63e-1    0
ripopt: iter9-probe: |y_c|=3.47e0 |y_d|=0.00e0 |z_L|=1.01e0 |z_U|=3.03e-2  |dy_c|=1.74e0 |dy_d|=0.00e0 |dz_L|=1.74e0 |dz_U|=7.44e-2  ftb_du=zU@375 z=1.45e-5 dz=-5.30e-3 ratio=2.72e-3  dw_last=0.00e0 dc_last=0.00e0
  10     1.8269838e0     5.75e-1     4.80e-3     4.12e-3        -2.5    1.00e0    1.00e0    0
ripopt: iter10-probe: |y_c|=3.47e0 |y_d|=0.00e0 |z_L|=6.12e-1 |z_U|=3.54e-2  |dy_c|=4.88e-1 |dy_d|=0.00e0 |dz_L|=5.14e-1 |dz_U|=5.07e-3  ftb_du=zL@525 z=5.00e-1 dz=-5.14e-1 ratio=9.69e-1  dw_last=0.00e0 dc_last=0.00e0
  11     1.8376613e0     2.18e-2     2.30e-4     3.26e-3        -2.5    1.00e0    1.00e0    0
ripopt: iter11-probe: |y_c|=3.47e0 |y_d|=0.00e0 |z_L|=6.26e-1 |z_U|=3.95e-2  |dy_c|=1.74e-1 |dy_d|=0.00e0 |dz_L|=1.74e-1 |dz_U|=4.14e-3  ftb_du=zL@526 z=3.75e-1 dz=-1.74e-1 ratio=2.14e0  dw_last=0.00e0 dc_last=0.00e0
  12     1.7380658e0     4.97e-2     4.16e-2     8.84e-4        -3.8    1.00e0   8.75e-1    0
ripopt: iter12-probe: |y_c|=3.59e0 |y_d|=0.00e0 |z_L|=6.42e-1 |z_U|=4.05e-2  |dy_c|=3.32e-1 |dy_d|=0.00e0 |dz_L|=3.32e-1 |dz_U|=1.14e-2  ftb_du=zU@396 z=1.62e-7 dz=-1.23e-3 ratio=1.32e-4  dw_last=0.00e0 dc_last=0.00e0
  13     1.7050040e0     1.29e-2     8.69e-3     3.28e-4        -3.8   8.92e-1    1.00e0    0
ripopt: iter13-probe: |y_c|=3.59e0 |y_d|=0.00e0 |z_L|=6.52e-1 |z_U|=5.25e-2  |dy_c|=8.05e-2 |dy_d|=0.00e0 |dz_L|=1.18e-1 |dz_U|=1.20e-2  ftb_du=zL@526 z=2.94e-2 dz=-1.13e-1 ratio=2.61e-1  dw_last=0.00e0 dc_last=0.00e0
  14     1.7062553e0     5.63e-4     1.97e-5     1.78e-4        -3.8    1.00e0    1.00e0    0
ripopt: iter14-probe: |y_c|=3.59e0 |y_d|=0.00e0 |z_L|=6.51e-1 |z_U|=5.44e-2  |dy_c|=1.76e-2 |dy_d|=0.00e0 |dz_L|=2.90e-2 |dz_U|=3.43e-3  ftb_du=zL@81 z=1.40e-2 dz=-2.83e-2 ratio=4.95e-1  dw_last=0.00e0 dc_last=0.00e0
  15     1.6998569e0     3.88e-4     4.92e-3     4.46e-5        -5.7   8.25e-1   9.93e-1    0
ripopt: iter15-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=6.53e-1 |z_U|=5.70e-2  |dy_c|=2.86e-2 |dy_d|=0.00e0 |dz_L|=2.95e-2 |dz_U|=3.23e-3  ftb_du=zU@397 z=8.11e-11 dz=-4.42e-5 ratio=1.83e-6  dw_last=0.00e0 dc_last=0.00e0
  16     1.6987263e0     7.75e-5     2.53e-4     3.56e-6        -5.7    1.00e0   9.88e-1    0
ripopt: iter16-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=6.53e-1 |z_U|=5.72e-2  |dy_c|=7.15e-3 |dy_d|=0.00e0 |dz_L|=2.13e-2 |dz_U|=3.30e-3  ftb_du=zL@80 z=3.89e-8 dz=-2.13e-2 ratio=1.82e-6  dw_last=0.00e0 dc_last=0.00e0
  17     1.6987290e0     5.57e-5     2.45e-9     1.85e-6        -5.7    1.00e0    1.00e0    0
ripopt: iter17-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=6.53e-1 |z_U|=5.72e-2  |dy_c|=2.90e-4 |dy_d|=0.00e0 |dz_L|=8.27e-4 |dz_U|=1.16e-4  ftb_du=zL@97 z=2.22e-4 dz=-1.19e-4 ratio=1.86e0  dw_last=0.00e0 dc_last=0.00e0
  18     1.6987290e0    1.27e-11    1.84e-11     1.84e-6        -5.7    1.00e0    1.00e0    0
ripopt: iter18-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=6.53e-1 |z_U|=5.72e-2  |dy_c|=3.99e-7 |dy_d|=0.00e0 |dz_L|=1.62e-6 |dz_U|=2.15e-7  ftb_du=zL@80 z=8.25e-4 dz=-1.62e-6 ratio=5.09e2  dw_last=0.00e0 dc_last=0.00e0
  19     1.6986408e0     8.30e-8     3.13e-6     1.26e-8        -8.6   9.96e-1    1.00e0    0
ripopt: iter19-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=6.53e-1 |z_U|=5.72e-2  |dy_c|=3.63e-4 |dy_d|=0.00e0 |dz_L|=8.22e-4 |dz_U|=1.08e-4  ftb_du=zU@397 z=5.98e-10 dz=-5.29e-7 ratio=1.13e-3  dw_last=0.00e0 dc_last=0.00e0
  20     1.6986405e0    1.82e-12    2.70e-12     2.51e-9        -8.6    1.00e0    1.00e0    0
ripopt: iter20-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=6.53e-1 |z_U|=5.72e-2  |dy_c|=1.48e-6 |dy_d|=0.00e0 |dz_L|=2.03e-6 |dz_U|=3.34e-7  ftb_du=zL@80 z=1.12e-6 dz=-2.03e-6 ratio=5.54e-1  dw_last=0.00e0 dc_last=0.00e0

Phase breakdown (21 iterations):
  Problem eval            0.099s (  8.1%)
  KKT assembly            0.000s (  0.0%)
  Factorization           0.000s (  0.0%)
  Direction solve         1.102s ( 90.2%)
  Line search             0.004s (  0.4%)
  Other                   0.017s (  1.4%)
  Total                   1.222s

Number of Iterations....: 20

                                   (scaled)                 (unscaled)
Objective...............:    1.6986405294099947e0     1.6986405294099947e0
Dual infeasibility......:  2.6957325260923426e-12   2.6957325260923426e-12
Constraint violation....:  1.8189894035458565e-12   1.8189894035458565e-12
Complementarity.........:   2.5059732792805587e-9    2.5059732792805587e-9
Overall NLP error.......:   2.5059732792805587e-9    2.5059732792805587e-9

Number of objective function evaluations             = 41
Number of objective gradient evaluations             = 21
Number of equality constraint evaluations            = 41
Number of inequality constraint evaluations          = 41
Number of equality constraint Jacobian evaluations   = 21
Number of inequality constraint Jacobian evaluations = 21
Number of Lagrangian Hessian evaluations             = 21
Total seconds in ripopt                              = 1.265

EXIT: Optimal Solution Found.
ripopt 0.8.0: Optimal after 20 iterations
Objective: 1.698640529409995e0

--- ripopt diagnostics ---
status: Optimal
iterations: 20
wall_time: 1.265s
final_mu: 2.51e-9
final_primal_inf: 1.82e-12
final_dual_inf: 2.70e-12
final_compl: 2.51e-9
restoration_count: 0
nlp_restoration_count: 0
mu_mode_switches: 0
filter_rejects: 0
watchdog_activations: 0
soc_corrections: 0
--- end diagnostics ---
=== gaslib11_steady.nl ===
    Finished `release` profile [optimized + debuginfo] target(s) in 0.05s
     Running `/home/orazio/Projects/ripopt/target/release/ripopt /home/orazio/Projects/ripopt/benchmarks/gas/gaslib11_steady.nl print_level=5`
ripopt: Detected 96/200 linear constraints (Hessian contribution skipped)

Number of nonzeros in equality constraint Jacobian...:         500
Number of nonzeros in inequality constraint Jacobian.:           0
Number of nonzeros in Lagrangian Hessian.............:         108

Total number of variables............................:         204
                     variables with only lower bounds:          16
                variables with lower and upper bounds:          84
                     variables with only upper bounds:           0
Total number of equality constraints.................:         200
Total number of inequality constraints...............:           0
        inequality constraints with only lower bounds:           0
   inequality constraints with lower and upper bounds:           0
        inequality constraints with only upper bounds:           0

iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
ripopt: Starting main loop (n=204, m=200)
   0    7.1999928e-2      3.49e1     8.12e-2      5.50e1        -1.0    0.00e0    0.00e0    0
ripopt: iter0-probe: |y_c|=3.00e0 |y_d|=0.00e0 |z_L|=1.00e0 |z_U|=1.00e0  |dy_c|=0.00e0 |dy_d|=0.00e0 |dz_L|=0.00e0 |dz_U|=0.00e0  ftb_du=-@-1 z=NaN dz=NaN ratio=inf  dw_last=0.00e0 dc_last=0.00e0
ripopt: iter0-probe: |grad_f|_inf=3.600e0@var108, |J^T y|_inf=2.600e0, |y|_inf=3.000e0, |z_L|_inf=1.000e0, |z_U|_inf=1.000e0, sum|y|=9.232e1, sum|z_L|=1.000e2, sum|z_U|=8.400e1, |x|_inf=5.500e1, n=204 m=200
ripopt: iter0-probe: |grad_lag|_inf=8.124e-2@var158 (grad_f=0.000e0, J^T y=9.188e-1, z_L=1.000e0, z_U=0.000e0, x_l_fin=true, x_u_fin=false, obj_scaling=1.000e0)
   1    7.1032796e-2      3.39e1     7.68e-1      5.40e1        -1.0   2.93e-2   1.64e-2    0
ripopt: iter1-probe: |y_c|=2.32e0 |y_d|=0.00e0 |z_L|=1.71e0 |z_U|=9.94e-1  |dy_c|=1.67e2 |dy_d|=0.00e0 |dz_L|=6.02e1 |dz_U|=4.86e0  ftb_du=zL@16 z=1.00e-2 dz=-6.02e1 ratio=1.64e-4  dw_last=0.00e0 dc_last=0.00e0
   2    7.1019451e-2      3.39e1      4.66e2      5.09e1        -1.0   3.51e-4   7.96e-2    0
ripopt: iter2-probe: |y_c|=2.32e0 |y_d|=0.00e0 |z_L|=4.70e2 |z_U|=9.63e-1  |dy_c|=1.20e2 |dy_d|=0.00e0 |dz_L|=5.88e3 |dz_U|=4.02e0  ftb_du=zL@9 z=9.48e-3 dz=-1.18e1 ratio=7.96e-4  dw_last=0.00e0 dc_last=0.00e0
   3    7.0070222e-2      3.30e1      4.04e2      5.00e1        -1.0   2.45e-2   1.61e-2    0
ripopt: iter3-probe: |y_c|=4.98e0 |y_d|=0.00e0 |z_L|=5.68e2 |z_U|=9.57e-1  |dy_c|=1.39e2 |dy_d|=0.00e0 |dz_L|=6.08e3 |dz_U|=2.57e0  ftb_du=zL@90 z=1.84e-3 dz=-1.13e1 ratio=1.61e-4  dw_last=0.00e0 dc_last=0.00e0
   4    7.0038496e-2      3.30e1      5.60e2      4.89e1        -1.0   8.10e-4   2.31e-2    0
ripopt: iter4-probe: |y_c|=5.09e0 |y_d|=0.00e0 |z_L|=7.30e2 |z_U|=9.49e-1  |dy_c|=1.68e2 |dy_d|=0.00e0 |dz_L|=7.06e3 |dz_U|=1.10e0  ftb_du=zL@32 z=5.41e-3 dz=-2.32e1 ratio=2.31e-4  dw_last=0.00e0 dc_last=0.00e0
   5    3.3242978e-2      7.91e1      2.48e2      4.61e1        -1.0   9.90e-1   3.15e-2    0
ripopt: iter5-probe: |y_c|=1.43e1 |y_d|=0.00e0 |z_L|=7.24e2 |z_U|=9.37e-1  |dy_c|=1.28e1 |dy_d|=0.00e0 |dz_L|=2.22e2 |dz_U|=1.06e0  ftb_du=zL@97 z=1.20e-4 dz=-3.76e-1 ratio=3.15e-4  dw_last=0.00e0 dc_last=0.00e0
   6    3.3978247e-2      1.24e1      1.40e3      3.41e1        -1.0   9.90e-1   2.71e-1    0
ripopt: iter6-probe: |y_c|=1.05e3 |y_d|=0.00e0 |z_L|=2.89e2 |z_U|=7.65e-1  |dy_c|=1.05e3 |dy_d|=0.00e0 |dz_L|=1.95e3 |dz_U|=1.75e0  ftb_du=zL@7 z=1.68e0 dz=-6.16e2 ratio=2.71e-3  dw_last=0.00e0 dc_last=0.00e0
   7    4.2500379e-2      1.44e0      1.68e4      5.27e0        -1.0    1.00e0   8.41e-1    0
ripopt: iter7-probe: |y_c|=1.06e5 |y_d|=0.00e0 |z_L|=8.92e4 |z_U|=2.59e-1  |dy_c|=1.05e5 |dy_d|=0.00e0 |dz_L|=1.06e5 |dz_U|=6.77e-1  ftb_du=zL@4 z=1.97e0 dz=-2.32e2 ratio=8.41e-3  dw_last=0.00e0 dc_last=0.00e0
   8    4.8053756e-2     4.45e-1      3.23e3      1.05e0        -1.0    1.00e0   8.08e-1    0
ripopt: iter8-probe: |y_c|=1.06e5 |y_d|=0.00e0 |z_L|=1.03e5 |z_U|=1.27e-1  |dy_c|=1.19e1 |dy_d|=0.00e0 |dz_L|=1.68e4 |dz_U|=2.38e-1  ftb_du=zL@58 z=2.37e-3 dz=-2.90e-1 ratio=8.08e-3  dw_last=0.00e0 dc_last=0.00e0
   9    5.1472984e-2     6.63e-2     1.84e-2     1.14e-1        -1.0    1.00e0    1.00e0    0
ripopt: iter9-probe: |y_c|=1.06e5 |y_d|=0.00e0 |z_L|=1.06e5 |z_U|=1.10e-1  |dy_c|=3.15e0 |dy_d|=0.00e0 |dz_L|=3.23e3 |dz_U|=5.62e-2  ftb_du=zL@4 z=2.22e0 dz=-3.10e1 ratio=7.09e-2  dw_last=0.00e0 dc_last=0.00e0
  10    5.0445209e-2     2.59e-3     2.53e-3     5.95e-3        -2.5    1.00e0    1.00e0    0
ripopt: iter10-probe: |y_c|=1.06e5 |y_d|=0.00e0 |z_L|=1.06e5 |z_U|=3.19e-3  |dy_c|=1.69e1 |dy_d|=0.00e0 |dz_L|=1.69e1 |dz_U|=1.07e-1  ftb_du=zU@34 z=1.15e-4 dz=-1.48e-2 ratio=7.76e-3  dw_last=0.00e0 dc_last=0.00e0
  11    3.6138123e-2     6.65e-1     4.35e-2     2.38e-3        -3.8   6.93e-1   8.05e-1    0
ripopt: iter11-probe: |y_c|=1.06e5 |y_d|=0.00e0 |z_L|=1.06e5 |z_U|=7.59e-4  |dy_c|=3.91e-1 |dy_d|=0.00e0 |dz_L|=3.91e-1 |dz_U|=3.02e-3  ftb_du=zU@42 z=1.83e-8 dz=-1.51e-4 ratio=1.21e-4  dw_last=0.00e0 dc_last=0.00e0
  12    3.3031598e-2     4.13e-1     2.57e-1     5.85e-4        -3.8   4.70e-1    1.00e0    0
ripopt: iter12-probe: |y_c|=1.06e5 |y_d|=0.00e0 |z_L|=1.06e5 |z_U|=1.69e-4  |dy_c|=4.86e-1 |dy_d|=0.00e0 |dz_L|=4.42e-1 |dz_U|=5.91e-4  ftb_du=zL@42 z=2.59e-2 dz=-4.42e-1 ratio=5.86e-2  dw_last=0.00e0 dc_last=0.00e0
  13    3.3186524e-2     2.16e-1     1.56e-4     1.71e-4        -3.8    1.00e0    1.00e0    0
ripopt: iter13-probe: |y_c|=1.06e5 |y_d|=0.00e0 |z_L|=1.06e5 |z_U|=1.69e-4  |dy_c|=2.54e-1 |dy_d|=0.00e0 |dz_L|=8.70e-2 |dz_U|=1.46e-5  ftb_du=zL@40 z=3.58e-2 dz=-8.70e-2 ratio=4.11e-1  dw_last=0.00e0 dc_last=0.00e0
  14    3.3163858e-2     8.06e-5     2.01e-8     1.50e-4        -3.8    1.00e0    1.00e0    0
ripopt: iter14-probe: |y_c|=1.06e5 |y_d|=0.00e0 |z_L|=1.06e5 |z_U|=1.69e-4  |dy_c|=9.19e-4 |dy_d|=0.00e0 |dz_L|=9.19e-4 |dz_U|=1.65e-7  ftb_du=zL@7 z=2.56e-3 dz=-8.96e-5 ratio=2.85e1  dw_last=0.00e0 dc_last=0.00e0
  15    3.2858532e-2     3.81e-4     7.49e-4     4.95e-6        -5.7   9.79e-1    1.00e0    0
ripopt: iter15-probe: |y_c|=1.06e5 |y_d|=0.00e0 |z_L|=1.06e5 |z_U|=2.07e-6  |dy_c|=3.58e-2 |dy_d|=0.00e0 |dz_L|=3.58e-2 |dz_U|=1.67e-4  ftb_du=zU@30 z=1.11e-7 dz=-1.46e-5 ratio=7.62e-3  dw_last=0.00e0 dc_last=0.00e0
  16    3.2859284e-2     2.18e-8     1.06e-9     1.85e-6        -5.7    1.00e0    1.00e0    0
ripopt: iter16-probe: |y_c|=1.06e5 |y_d|=0.00e0 |z_L|=1.06e5 |z_U|=2.07e-6  |dy_c|=1.14e-3 |dy_d|=0.00e0 |dz_L|=3.90e-4 |dz_U|=1.29e-7  ftb_du=zL@40 z=4.55e-4 dz=-3.90e-4 ratio=1.16e0  dw_last=0.00e0 dc_last=0.00e0
  17    3.2855596e-2     5.52e-8    1.65e-10     2.74e-9        -8.6    1.00e0    1.00e0    0
ripopt: iter17-probe: |y_c|=1.06e5 |y_d|=0.00e0 |z_L|=1.06e5 |z_U|=2.81e-9  |dy_c|=4.54e-4 |dy_d|=0.00e0 |dz_L|=4.54e-4 |dz_U|=2.07e-6  ftb_du=zU@30 z=2.34e-10 dz=-1.79e-7 ratio=1.30e-3  dw_last=0.00e0 dc_last=0.00e0
  18    3.2855597e-2     1.00e-8    1.46e-11     2.51e-9        -8.6    1.00e0    1.00e0    0
ripopt: iter18-probe: |y_c|=1.06e5 |y_d|=0.00e0 |z_L|=1.06e5 |z_U|=2.81e-9  |dy_c|=5.74e-8 |dy_d|=0.00e0 |dz_L|=5.74e-8 |dz_U|=1.84e-11  ftb_du=zL@40 z=6.18e-7 dz=-5.74e-8 ratio=1.08e1  dw_last=0.00e0 dc_last=0.00e0

Phase breakdown (19 iterations):
  Problem eval            0.002s ( 13.7%)
  KKT assembly            0.000s (  0.0%)
  Factorization           0.000s (  0.0%)
  Direction solve         0.008s ( 70.9%)
  Line search             0.000s (  3.2%)
  Other                   0.001s ( 12.2%)
  Total                   0.012s

Number of Iterations....: 18

                                   (scaled)                 (unscaled)
Objective...............:   3.2855597129387175e-2    3.2855597129387175e-2
Dual infeasibility......:  1.1979694210692010e-13   1.4551915228366852e-11
Constraint violation....:   9.9999759584079584e-9    9.9999759584079584e-9
Complementarity.........:   2.5059036512254293e-9    2.5059036512254293e-9
Overall NLP error.......:   9.9999759584079584e-9    9.9999759584079584e-9

Number of objective function evaluations             = 37
Number of objective gradient evaluations             = 19
Number of equality constraint evaluations            = 37
Number of inequality constraint evaluations          = 37
Number of equality constraint Jacobian evaluations   = 19
Number of inequality constraint Jacobian evaluations = 19
Number of Lagrangian Hessian evaluations             = 19
Total seconds in ripopt                              = 0.013

EXIT: Optimal Solution Found.
ripopt 0.8.0: Optimal after 18 iterations
Objective: 3.285559712938717e-2

--- ripopt diagnostics ---
status: Optimal
iterations: 18
wall_time: 0.013s
final_mu: 2.51e-9
final_primal_inf: 1.00e-8
final_dual_inf: 1.46e-11
final_compl: 2.51e-9
restoration_count: 0
nlp_restoration_count: 0
mu_mode_switches: 0
filter_rejects: 0
watchdog_activations: 0
soc_corrections: 0
--- end diagnostics ---
=== gaslib40_dynamic.nl ===
    Finished `release` profile [optimized + debuginfo] target(s) in 0.05s
     Running `/home/orazio/Projects/ripopt/target/release/ripopt /home/orazio/Projects/ripopt/benchmarks/gas/gaslib40_dynamic.nl print_level=5`
ripopt: Detected 8908/20908 linear constraints (Hessian contribution skipped)

Number of nonzeros in equality constraint Jacobian...:       59331
Number of nonzeros in inequality constraint Jacobian.:           0
Number of nonzeros in Lagrangian Hessian.............:       12150

Total number of variables............................:       21058
                     variables with only lower bounds:         300
                variables with lower and upper bounds:        8758
                     variables with only upper bounds:           0
Total number of equality constraints.................:       20908
Total number of inequality constraints...............:           0
        inequality constraints with only lower bounds:           0
   inequality constraints with lower and upper bounds:           0
        inequality constraints with only upper bounds:           0

iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
ripopt: Starting main loop (n=21058, m=20908)
   0     5.1839948e0      1.58e2      1.00e0      1.58e2        -1.0    0.00e0    0.00e0    0
ripopt: iter0-probe: |y_c|=4.55e0 |y_d|=0.00e0 |z_L|=1.00e0 |z_U|=1.00e0  |dy_c|=0.00e0 |dy_d|=0.00e0 |dz_L|=0.00e0 |dz_U|=0.00e0  ftb_du=-@-1 z=NaN dz=NaN ratio=inf  dw_last=0.00e0 dc_last=0.00e0
ripopt: iter0-probe: |grad_f|_inf=3.600e0@var12150, |J^T y|_inf=2.600e0, |y|_inf=4.546e0, |z_L|_inf=1.000e0, |z_U|_inf=1.000e0, sum|y|=3.885e3, sum|z_L|=9.058e3, sum|z_U|=8.758e3, |x|_inf=1.581e2, n=21058 m=20908
ripopt: iter0-probe: |grad_lag|_inf=9.998e-1@var15274 (grad_f=0.000e0, J^T y=1.850e-4, z_L=1.000e0, z_U=0.000e0, x_l_fin=true, x_u_fin=false, obj_scaling=1.000e0)
   1     5.1755208e0      5.65e3      9.81e3      1.59e2        -1.0   5.17e-5   1.90e-4    0
ripopt: iter1-probe: |y_c|=2.14e5 |y_d|=0.00e0 |z_L|=4.64e0 |z_U|=1.00e0  |dy_c|=4.14e9 |dy_d|=0.00e0 |dz_L|=1.91e4 |dz_U|=3.12e0  ftb_du=zL@5700 z=1.00e-2 dz=-5.21e3 ratio=1.90e-6  dw_last=0.00e0 dc_last=0.00e0
   2     5.1753567e0      5.65e3      9.81e3      1.59e2        -1.0   2.11e-4   2.45e-4    0
ripopt: iter2-probe: |y_c|=2.12e5 |y_d|=0.00e0 |z_L|=9.97e0 |z_U|=1.00e0  |dy_c|=1.02e7 |dy_d|=0.00e0 |dz_L|=2.17e4 |dz_U|=3.24e0  ftb_du=zL@5275 z=1.17e-3 dz=-4.72e2 ratio=2.45e-6  dw_last=1.00e-2 dc_last=0.00e0
   3     5.1753564e0      5.65e3      9.81e3      1.59e2        -1.0   1.06e-6   1.77e-4    1
ripopt: iter3-probe: |y_c|=2.11e5 |y_d|=0.00e0 |z_L|=8.35e2 |z_U|=1.00e0  |dy_c|=8.67e8 |dy_d|=0.00e0 |dz_L|=4.66e6 |dz_U|=4.65e0  ftb_du=zL@701 z=1.69e-3 dz=-9.44e2 ratio=1.77e-6  dw_last=3.33e-3 dc_last=0.00e0
   4     5.1753562e0      5.65e3      1.42e5      8.53e2        -1.0   2.67e-7   1.82e-4    2
ripopt: iter4-probe: |y_c|=1.78e5 |y_d|=0.00e0 |z_L|=1.42e5 |z_U|=1.00e0  |dy_c|=1.44e11 |dy_d|=0.00e0 |dz_L|=7.74e8 |dz_U|=4.14e0  ftb_du=zL@776 z=2.77e-5 dz=-1.50e1 ratio=1.82e-6  dw_last=8.89e-3 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=2.82e5, mu=1.00e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     5.0254639e8      2.82e0      9.93e1      5.65e3         3.8    0.00e0    0.00e0    0
   1     4.3081174e8      1.17e2      9.92e1      6.13e3         2.4   2.82e-3   5.17e-3    0
ripopt: NLP restoration result: status=Optimal, theta_new=1.96e5 (was 2.82e5), phi_new=3.92e2, sum_p=6.80e4, sum_n=2.26e5, iters=2
   5     3.9153197e2      1.95e3      2.60e0      8.76e1        -1.0    0.00e0   1.82e-4    2
ripopt: iter5-probe: |y_c|=0.00e0 |y_d|=0.00e0 |z_L|=1.00e0 |z_U|=1.00e0  |dy_c|=3.26e13 |dy_d|=0.00e0 |dz_L|=1.75e11 |dz_U|=4.04e0  ftb_du=zL@702 z=1.00e0 dz=-8.19e2 ratio=1.21e-3  dw_last=2.96e-3 dc_last=0.00e0
   6     3.9147257e2      1.95e3      5.49e2      8.75e1        -1.0   1.59e-4   6.30e-4    0
ripopt: iter6-probe: |y_c|=1.15e3 |y_d|=0.00e0 |z_L|=4.92e0 |z_U|=1.00e0  |dy_c|=7.25e6 |dy_d|=0.00e0 |dz_L|=6.22e3 |dz_U|=2.69e0  ftb_du=zL@500 z=1.00e-2 dz=-1.57e3 ratio=6.30e-6  dw_last=7.90e-3 dc_last=0.00e0
   7     3.9147197e2      1.95e3      3.25e3      8.99e1        -1.0   1.59e-6   1.06e-3    2
ripopt: iter7-probe: |y_c|=3.57e5 |y_d|=0.00e0 |z_L|=6.51e3 |z_U|=1.00e0  |dy_c|=3.35e8 |dy_d|=0.00e0 |dz_L|=3.06e6 |dz_U|=2.72e0  ftb_du=zL@500 z=3.30e-10 dz=-9.30e0 ratio=3.51e-11  dw_last=2.63e-3 dc_last=0.00e0
   8     3.9147197e2      1.95e3      5.04e9      7.41e3        -1.0   1.59e-8   1.25e-2    0
ripopt: iter8-probe: |y_c|=5.53e11 |y_d|=0.00e0 |z_L|=1.01e10 |z_U|=9.99e-1  |dy_c|=4.44e13 |dy_d|=0.00e0 |dz_L|=4.05e11 |dz_U|=3.01e0  ftb_du=zL@701 z=1.26e-11 dz=-6.29e1 ratio=1.99e-13  dw_last=8.78e-4 dc_last=0.00e0
   9     3.9078365e2      1.95e3      4.75e9      7.19e3        -1.0   1.84e-3   1.13e-2    0
ripopt: iter9-probe: |y_c|=5.47e11 |y_d|=0.00e0 |z_L|=9.74e9 |z_U|=9.98e-1  |dy_c|=2.97e12 |dy_d|=0.00e0 |dz_L|=2.98e10 |dz_U|=2.70e0  ftb_du=zL@728 z=1.80e-3 dz=-1.57e1 ratio=1.13e-4  dw_last=1.87e-2 dc_last=0.00e0
  10     3.9077648e2      1.95e3      4.35e9      6.90e3        -1.0   1.92e-5   1.23e-2    0
ripopt: iter10-probe: |y_c|=5.47e11 |y_d|=0.00e0 |z_L|=9.35e9 |z_U|=9.97e-1  |dy_c|=2.91e12 |dy_d|=0.00e0 |dz_L|=3.21e10 |dz_U|=2.80e0  ftb_du=zL@706 z=1.80e-3 dz=-1.44e1 ratio=1.23e-4  dw_last=4.99e-2 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=1.96e5, mu=1.00e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     2.7130473e8      1.50e0      9.93e1      1.95e3         3.3    0.00e0    0.00e0    0
   1     2.2548942e8      6.62e1      9.94e1      2.68e3         1.9   3.65e-3   1.18e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=1.35e5 (was 1.96e5), phi_new=4.65e2, sum_p=3.18e4, sum_n=1.43e5, iters=2
  11     4.6512167e2      8.62e2      2.60e0      6.54e1        -1.0    0.00e0   1.23e-2    0
ripopt: iter11-probe: |y_c|=0.00e0 |y_d|=0.00e0 |z_L|=1.00e0 |z_U|=1.00e0  |dy_c|=2.90e12 |dy_d|=0.00e0 |dz_L|=3.17e10 |dz_U|=2.41e0  ftb_du=zL@5250 z=1.00e0 dz=-3.17e10 ratio=3.13e-11  dw_last=1.66e-2 dc_last=0.00e0
  12     4.6505553e2      8.62e2      4.81e7      6.53e1        -1.0   1.44e-4   9.69e-4    0
ripopt: iter12-probe: |y_c|=1.75e7 |y_d|=0.00e0 |z_L|=7.65e0 |z_U|=1.00e0  |dy_c|=1.21e11 |dy_d|=0.00e0 |dz_L|=6.86e3 |dz_U|=6.88e0  ftb_du=zL@500 z=1.00e-2 dz=-1.02e3 ratio=9.69e-6  dw_last=1.16e6 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=1.35e5, mu=1.00e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.6835525e8     4.35e-1      9.93e1      8.63e2         2.9    0.00e0    0.00e0    0
   1     1.6070395e8      2.54e0      9.94e1      1.71e3         1.5   1.01e-3   1.21e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=8.41e4 (was 1.35e5), phi_new=4.67e2, sum_p=1.69e4, sum_n=8.36e4, iters=2
  13     4.6650541e2      3.63e2      2.62e0      6.45e1        -1.0    0.00e0   9.69e-4    1
ripopt: iter13-probe: |y_c|=0.00e0 |y_d|=0.00e0 |z_L|=1.07e0 |z_U|=9.86e-1  |dy_c|=2.00e11 |dy_d|=0.00e0 |dz_L|=7.89e6 |dz_U|=6.80e0  ftb_du=zL@500 z=5.74e-2 dz=-1.03e1 ratio=5.51e-3  dw_last=3.88e5 dc_last=0.00e0
  14     4.6646602e2      3.63e2      2.74e4      6.45e1        -1.0   8.57e-5   2.84e-5    0
ripopt: iter14-probe: |y_c|=2.59e6 |y_d|=0.00e0 |z_L|=1.09e0 |z_U|=9.86e-1  |dy_c|=3.03e10 |dy_d|=0.00e0 |dz_L|=2.00e3 |dz_U|=9.31e0  ftb_du=zL@500 z=5.74e-4 dz=-2.00e3 ratio=2.84e-7  dw_last=1.29e5 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=8.41e4, mu=1.00e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     9.7813093e7     4.99e-1      9.93e1      3.63e2         2.6    0.00e0    0.00e0    0
   1     9.5244493e7     5.18e-1      9.93e1      7.65e2         1.2   6.06e-4   1.23e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=5.52e4 (was 8.41e4), phi_new=3.95e2, sum_p=1.09e4, sum_n=5.14e4, iters=2
  15     3.9489559e2      1.71e2      1.34e1      4.54e1        -1.0    0.00e0   2.84e-5    1
ripopt: iter15-probe: |y_c|=0.00e0 |y_d|=0.00e0 |z_L|=1.34e1 |z_U|=7.37e-1  |dy_c|=1.01e10 |dy_d|=0.00e0 |dz_L|=1.17e5 |dz_U|=9.33e0  ftb_du=zL@3500 z=9.31e-3 dz=-1.21e1 ratio=7.63e-4  dw_last=4.31e4 dc_last=0.00e0
  16     3.9487093e2      1.71e2      1.54e3      4.54e1        -1.0   6.35e-5   9.22e-5    0
ripopt: iter16-probe: |y_c|=1.62e5 |y_d|=0.00e0 |z_L|=3.27e1 |z_U|=7.37e-1  |dy_c|=2.56e9 |dy_d|=0.00e0 |dz_L|=2.09e5 |dz_U|=8.47e0  ftb_du=zL@500 z=5.68e-2 dz=-6.11e4 ratio=9.22e-7  dw_last=1.44e4 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=5.52e4, mu=1.00e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     6.1548994e7     6.01e-1      9.93e1      1.72e2         2.2    0.00e0    0.00e0    0
   1     6.0166057e7     6.01e-1      9.92e1      3.88e2         0.8   5.34e-4   1.37e-2    0
   2     4.8325394e7      5.11e0      9.85e1      3.35e2         0.8   7.76e-3   7.10e-3    0
   3     3.6515656e7      5.09e2      9.73e1      2.73e2         0.8   1.15e-2   1.01e-2    0
   4     3.3281821e7      5.06e2      9.67e1      3.08e2         0.8   6.48e-3   1.44e-2    0
   5     2.7189000e7      4.96e2      9.49e1      2.64e2         0.8   1.93e-2   1.87e-2    0
   6     2.3162567e7      4.84e2      9.27e1      2.20e2         0.8   2.48e-2   2.01e-2    0
   7     2.1423137e7      4.75e2      9.42e1      1.93e2         0.8   1.83e-2   9.49e-3    0
   8     2.0433192e7      4.69e2      9.35e1      1.85e2         0.8   1.32e-2   1.11e-2    0
   9     1.9308024e7      4.60e2      9.58e1      1.67e2         0.8   1.78e-2   9.33e-3    0
  10     1.8737804e7      4.55e2      9.62e1      1.64e2         0.8   1.08e-2   1.08e-2    0
  11     1.8242119e7      4.51e2      8.84e1      1.85e2         0.8   1.06e-2   3.41e-2    0
  12     1.7508289e7      4.42e2      8.70e1      1.73e2         0.8   1.88e-2   1.08e-2    0
  13     1.7128679e7      4.37e2      8.56e1      1.79e2         0.8   1.10e-2   2.25e-2    0
  14     1.6515875e7      4.29e2      1.22e2      1.57e2         0.8   1.98e-2   1.13e-3    0
  15     1.5164021e7      3.71e2      7.23e2      1.39e2         0.8   1.33e-1   2.98e-1    0
  16     1.4801770e7      3.64e2      6.93e2      1.32e2         0.8   1.93e-2   1.41e-2    0
  17     1.4232810e7      3.52e2      7.79e2      1.15e2         0.8   3.38e-2   1.73e-2    0
  18     1.4046089e7      3.47e2      7.67e2      1.36e2         0.8   1.32e-2   6.10e-2    0
  19     1.3852836e7      3.41e2      7.52e2      1.32e2         0.8   1.76e-2   1.54e-2    0
  20     1.3740609e7      3.37e2      9.07e2      1.27e2         0.8   1.25e-2   4.58e-3    0
  21     1.3507840e7      3.29e2      9.29e2      1.19e2         0.8   2.23e-2   7.59e-3    0
  22     1.3300885e7      3.22e2      8.93e2      1.17e2         0.8   2.19e-2   2.56e-2    0
  23     1.3240860e7      3.19e2      8.13e2      1.18e2         0.8   8.93e-3   1.67e-2    0
  24     1.3098839e7      3.13e2      1.03e3      1.12e2         0.8   2.08e-2   6.76e-3    0
iter       objective      inf_pr      inf_du       compl      lg(mu)  alpha_pr  alpha_du   ls
  25     1.2997987e7      3.08e2      7.81e2      1.17e2         0.8   1.52e-2   4.55e-2    0
  26     1.2778166e7      2.96e2      7.51e2      1.16e2         0.8   3.82e-2   5.71e-2    0
  27     1.2572527e7      2.84e2      7.19e2      1.07e2         0.8   4.23e-2   2.10e-2    0
  28     1.2497969e7      2.79e2      1.10e3      1.04e2         0.8   1.80e-2   1.05e-2    0
  29     1.2251681e7      2.62e2      1.94e3      9.14e1         0.8   5.85e-2   1.44e-2    0
  30     1.2170373e7      2.56e2      1.90e3      9.42e1         0.8   2.55e-2   6.86e-2    0
  31     1.1914697e7      2.32e2      1.72e3      7.77e1         0.8   9.14e-2   1.75e-2    0
  32     1.1832015e7      2.23e2      1.65e3      8.04e1         0.8   4.05e-2   1.14e-1    0
  33     1.1670700e7      2.00e2      1.48e3      6.87e1         0.8   1.04e-1   5.77e-2    0
  34     1.1595241e7      1.87e2      1.39e3      6.48e1         0.8   6.32e-2   7.07e-2    0
  35     1.1442983e7      1.59e2      1.18e3      5.33e1         0.8   1.48e-1   1.07e-1    0
ripopt: NLP restoration result: status=Optimal, theta_new=9.26e3 (was 5.52e4), phi_new=6.89e1, sum_p=6.00e3, sum_n=4.34e3, iters=36
  17     6.8946085e1      1.46e2      2.07e1      5.30e1        -1.0    0.00e0   9.22e-5    0
ripopt: iter17-probe: |y_c|=0.00e0 |y_d|=0.00e0 |z_L|=2.07e1 |z_U|=7.37e-1  |dy_c|=7.65e10 |dy_d|=0.00e0 |dz_L|=7.07e7 |dz_U|=8.46e0  ftb_du=zL@500 z=5.68e-4 dz=-2.59e2 ratio=2.17e-6  dw_last=4.79e3 dc_last=0.00e0
  18     6.8936974e1      1.46e2      1.01e2      5.25e1        -1.0   1.91e-4   1.09e-2    0
ripopt: iter18-probe: |y_c|=2.56e5 |y_d|=0.00e0 |z_L|=6.61e1 |z_U|=7.52e-1  |dy_c|=1.34e9 |dy_d|=0.00e0 |dz_L|=5.99e3 |dz_U|=8.01e0  ftb_du=zL@5251 z=8.12e-3 dz=-7.41e1 ratio=1.09e-4  dw_last=1.60e3 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=9.26e3, mu=1.00e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.4867010e7     5.24e-1      9.93e1      1.46e2         2.2    0.00e0    0.00e0    0
   1     1.4381251e7     5.24e-1      9.99e1      6.47e2         0.8   8.23e-4   3.88e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=8.45e3 (was 9.26e3), phi_new=7.18e1, sum_p=6.90e3, sum_n=5.62e3, iters=2
  19     6.8936974e1      1.46e2      1.01e2      5.25e1        -1.7   1.91e-4   1.09e-2    3
ripopt: iter19-probe: |y_c|=2.56e5 |y_d|=0.00e0 |z_L|=6.61e1 |z_U|=7.52e-1  |dy_c|=4.90e8 |dy_d|=0.00e0 |dz_L|=2.85e7 |dz_U|=6.84e0  ftb_du=zL@5251 z=8.12e-3 dz=-4.76e-1 ratio=1.69e-2  dw_last=0.00e0 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=9.26e3, mu=2.00e-2)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.4867010e7     5.24e-1      9.93e1      1.46e2         2.2    0.00e0    0.00e0    0
   1     1.4381251e7     5.24e-1      9.99e1      6.47e2         0.8   8.23e-4   3.88e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=8.45e3 (was 9.26e3), phi_new=7.18e1, sum_p=6.90e3, sum_n=5.62e3, iters=2
  20     6.8936974e1      1.46e2      1.01e2      5.25e1        -0.7   1.91e-4   1.09e-2    3
ripopt: iter20-probe: |y_c|=2.56e5 |y_d|=0.00e0 |z_L|=6.61e1 |z_U|=7.52e-1  |dy_c|=1.77e8 |dy_d|=0.00e0 |dz_L|=2.14e7 |dz_U|=6.78e0  ftb_du=zL@5251 z=8.12e-3 dz=-6.54e-1 ratio=1.23e-2  dw_last=0.00e0 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=9.26e3, mu=2.00e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.4867010e7     5.24e-1      9.93e1      1.46e2         2.2    0.00e0    0.00e0    0
   1     1.4381251e7     5.24e-1      9.99e1      6.47e2         0.8   8.23e-4   3.88e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=8.45e3 (was 9.26e3), phi_new=7.18e1, sum_p=6.90e3, sum_n=5.62e3, iters=2
  21     6.8936974e1      1.46e2      1.01e2      5.25e1        -1.7   1.91e-4   1.09e-2    4
ripopt: iter21-probe: |y_c|=2.56e5 |y_d|=0.00e0 |z_L|=6.61e1 |z_U|=7.52e-1  |dy_c|=6.50e7 |dy_d|=0.00e0 |dz_L|=1.21e7 |dz_U|=6.37e0  ftb_du=zL@248 z=1.53e0 dz=-4.32e1 ratio=3.52e-2  dw_last=0.00e0 dc_last=0.00e0
  22     6.8936340e1      1.46e2      5.41e4      5.19e1        -1.7   1.32e-5   1.09e-2    0
ripopt: iter22-probe: |y_c|=2.56e5 |y_d|=0.00e0 |z_L|=5.41e4 |z_U|=7.91e-1  |dy_c|=7.45e6 |dy_d|=0.00e0 |dz_L|=4.98e6 |dz_U|=6.28e0  ftb_du=zL@249 z=3.21e-2 dz=-2.93e2 ratio=1.09e-4  dw_last=1.97e1 dc_last=0.00e0
  23     6.8858806e1      1.46e2      6.64e4      5.14e1        -1.7   1.62e-3   1.06e-2    0
ripopt: iter23-probe: |y_c|=1.70e5 |y_d|=0.00e0 |z_L|=6.88e4 |z_U|=8.26e-1  |dy_c|=5.28e7 |dy_d|=0.00e0 |dz_L|=1.39e6 |dz_U|=4.59e0  ftb_du=zL@248 z=4.23e-3 dz=-3.97e1 ratio=1.06e-4  dw_last=6.57e0 dc_last=0.00e0
  24     6.8739839e1      1.45e2      6.79e4      5.07e1        -1.7   2.59e-3   1.38e-2    0
ripopt: iter24-probe: |y_c|=1.22e5 |y_d|=0.00e0 |z_L|=7.09e4 |z_U|=8.75e-1  |dy_c|=1.84e7 |dy_d|=0.00e0 |dz_L|=5.03e5 |dz_U|=3.81e0  ftb_du=zL@1249 z=2.37e-2 dz=-1.70e2 ratio=1.38e-4  dw_last=2.19e0 dc_last=0.00e0
iter       objective      inf_pr      inf_du       compl      lg(mu)  alpha_pr  alpha_du   ls
  25     6.8529780e1      1.45e2      6.71e4      5.01e1        -1.7   4.67e-3   1.11e-2    0
ripopt: iter25-probe: |y_c|=8.64e4 |y_d|=0.00e0 |z_L|=7.01e4 |z_U|=9.18e-1  |dy_c|=7.73e6 |dy_d|=0.00e0 |dz_L|=2.82e5 |dz_U|=3.93e0  ftb_du=zL@1249 z=2.37e-4 dz=-2.12e0 ratio=1.11e-4  dw_last=7.30e-1 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=9.19e3, mu=2.00e-2)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.4742816e7     5.50e-1      9.93e1      1.45e2         2.2    0.00e0    0.00e0    0
   1     1.4259244e7     5.49e-1      9.99e1      6.46e2         0.8   8.22e-4   3.92e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=8.36e3 (was 9.19e3), phi_new=7.14e1, sum_p=6.82e3, sum_n=5.56e3, iters=2
  26     6.8529780e1      1.45e2      6.71e4      5.01e1        -2.5   4.67e-3   1.11e-2    2
ripopt: iter26-probe: |y_c|=8.64e4 |y_d|=0.00e0 |z_L|=7.01e4 |z_U|=9.18e-1  |dy_c|=2.27e12 |dy_d|=0.00e0 |dz_L|=1.51e7 |dz_U|=3.43e0  ftb_du=zL@4149 z=1.28e-1 dz=-6.66e0 ratio=1.92e-2  dw_last=0.00e0 dc_last=0.00e0
  27     6.8278568e1      1.44e2      6.49e4      4.92e1        -2.5   5.70e-3   1.84e-2    0
ripopt: iter27-probe: |y_c|=7.22e4 |y_d|=0.00e0 |z_L|=6.73e4 |z_U|=9.97e-1  |dy_c|=2.48e6 |dy_d|=0.00e0 |dz_L|=2.95e5 |dz_U|=4.28e0  ftb_du=zL@4149 z=3.62e-4 dz=-6.93e0 ratio=5.21e-5  dw_last=6.49e-1 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=9.14e3, mu=2.83e-3)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.4668193e7     5.66e-1      9.93e1      1.44e2         2.2    0.00e0    0.00e0    0
   1     1.4185599e7     5.66e-1      9.99e1      6.47e2         0.8   8.22e-4   3.95e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=8.31e3 (was 9.14e3), phi_new=7.11e1, sum_p=6.77e3, sum_n=5.53e3, iters=2
  28     6.8278568e1      1.44e2      6.49e4      4.92e1        -3.8   5.70e-3   1.84e-2    2
ripopt: iter28-probe: |y_c|=7.22e4 |y_d|=0.00e0 |z_L|=6.73e4 |z_U|=9.97e-1  |dy_c|=2.28e12 |dy_d|=0.00e0 |dz_L|=1.50e7 |dz_U|=3.76e0  ftb_du=zL@196 z=9.47e1 dz=-4.26e3 ratio=2.22e-2  dw_last=0.00e0 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=9.14e3, mu=1.50e-4)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.4668193e7     5.66e-1      9.93e1      1.44e2         2.2    0.00e0    0.00e0    0
   1     1.4185599e7     5.66e-1      9.99e1      6.47e2         0.8   8.22e-4   3.95e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=8.31e3 (was 9.14e3), phi_new=7.11e1, sum_p=6.77e3, sum_n=5.53e3, iters=2
  29     6.8278568e1      1.44e2      6.49e4      4.92e1        -2.8   5.70e-3   1.84e-2    2
ripopt: iter29-probe: |y_c|=7.22e4 |y_d|=0.00e0 |z_L|=6.73e4 |z_U|=9.97e-1  |dy_c|=2.48e12 |dy_d|=0.00e0 |dz_L|=1.50e7 |dz_U|=4.76e0  ftb_du=zL@1249 z=1.11e-4 dz=-5.38e-3 ratio=2.06e-2  dw_last=0.00e0 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=9.14e3, mu=1.50e-3)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.4668193e7     5.66e-1      9.93e1      1.44e2         2.2    0.00e0    0.00e0    0
   1     1.4185599e7     5.66e-1      9.99e1      6.47e2         0.8   8.22e-4   3.95e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=8.31e3 (was 9.14e3), phi_new=7.11e1, sum_p=6.77e3, sum_n=5.53e3, iters=2
  30     6.8278568e1      1.44e2      6.49e4      4.92e1        -3.8   5.70e-3   1.84e-2    2
ripopt: iter30-probe: |y_c|=7.22e4 |y_d|=0.00e0 |z_L|=6.73e4 |z_U|=9.97e-1  |dy_c|=2.26e12 |dy_d|=0.00e0 |dz_L|=1.50e7 |dz_U|=3.57e0  ftb_du=zL@196 z=9.47e1 dz=-4.54e3 ratio=2.08e-2  dw_last=0.00e0 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=9.14e3, mu=1.50e-4)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.4668193e7     5.66e-1      9.93e1      1.44e2         2.2    0.00e0    0.00e0    0
   1     1.4185599e7     5.66e-1      9.99e1      6.47e2         0.8   8.22e-4   3.95e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=8.31e3 (was 9.14e3), phi_new=7.11e1, sum_p=6.77e3, sum_n=5.53e3, iters=2
  31     6.8278568e1      1.44e2      6.49e4      4.92e1        -1.8   5.70e-3   1.84e-2    2
ripopt: iter31-probe: |y_c|=7.22e4 |y_d|=0.00e0 |z_L|=6.73e4 |z_U|=9.97e-1  |dy_c|=2.45e12 |dy_d|=0.00e0 |dz_L|=1.50e7 |dz_U|=4.68e0  ftb_du=zL@1249 z=1.11e-4 dz=-5.36e-3 ratio=2.04e-2  dw_last=0.00e0 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=9.14e3, mu=1.50e-2)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.4668193e7     5.66e-1      9.93e1      1.44e2         2.2    0.00e0    0.00e0    0
   1     1.4185599e7     5.66e-1      9.99e1      6.47e2         0.8   8.22e-4   3.95e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=8.31e3 (was 9.14e3), phi_new=7.11e1, sum_p=6.77e3, sum_n=5.53e3, iters=2
  32     6.8278568e1      1.44e2      6.49e4      4.92e1        -3.8   5.70e-3   1.84e-2    2
ripopt: iter32-probe: |y_c|=7.22e4 |y_d|=0.00e0 |z_L|=6.73e4 |z_U|=9.97e-1  |dy_c|=2.25e12 |dy_d|=0.00e0 |dz_L|=1.50e7 |dz_U|=3.37e0  ftb_du=zL@299 z=2.69e1 dz=-2.27e3 ratio=1.19e-2  dw_last=0.00e0 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=9.14e3, mu=1.50e-4)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.4668193e7     5.66e-1      9.93e1      1.44e2         2.2    0.00e0    0.00e0    0
   1     1.4185599e7     5.66e-1      9.99e1      6.47e2         0.8   8.22e-4   3.95e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=8.31e3 (was 9.14e3), phi_new=7.11e1, sum_p=6.77e3, sum_n=5.53e3, iters=2
  33     6.8278568e1      1.44e2      6.49e4      4.92e1        -0.8   5.70e-3   1.84e-2    2
ripopt: iter33-probe: |y_c|=7.22e4 |y_d|=0.00e0 |z_L|=6.73e4 |z_U|=9.97e-1  |dy_c|=2.41e12 |dy_d|=0.00e0 |dz_L|=1.50e7 |dz_U|=4.58e0  ftb_du=zL@1249 z=1.11e-4 dz=-5.34e-3 ratio=2.05e-2  dw_last=0.00e0 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=9.14e3, mu=1.50e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.4668193e7     5.66e-1      9.93e1      1.44e2         2.2    0.00e0    0.00e0    0
   1     1.4185599e7     5.66e-1      9.99e1      6.47e2         0.8   8.22e-4   3.95e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=8.31e3 (was 9.14e3), phi_new=7.11e1, sum_p=6.77e3, sum_n=5.53e3, iters=2

Number of Iterations....: 33

                                   (scaled)                 (unscaled)
Objective...............:    6.8278568369133197e1     6.8278568369133197e1
Dual infeasibility......:    6.4859339378786717e4     6.4859339378786717e4
Constraint violation....:    1.4379309802098115e2     1.4379309802098115e2
Complementarity.........:    4.9180434370753908e1     4.9180434370753908e1
Overall NLP error.......:    6.4859339378786717e4     6.4859339378786717e4

Number of objective function evaluations             = 93
Number of objective gradient evaluations             = 42
Number of equality constraint evaluations            = 93
Number of inequality constraint evaluations          = 93
Number of equality constraint Jacobian evaluations   = 42
Number of inequality constraint Jacobian evaluations = 42
Number of Lagrangian Hessian evaluations             = 24
Total seconds in ripopt                              = 1193.701

EXIT: Restoration Failed.
ripopt 0.8.0: RestorationFailed after 33 iterations
Objective: 6.827856836913320e1

--- ripopt diagnostics ---
status: RestorationFailed
iterations: 33
wall_time: 1193.701s
final_mu: 1.50e-1
final_primal_inf: 1.44e2
final_dual_inf: 6.49e4
final_compl: 4.92e1
restoration_count: 0
nlp_restoration_count: 16
mu_mode_switches: 0
filter_rejects: 16
watchdog_activations: 0
soc_corrections: 0
--- end diagnostics ---
=== gaslib40_steady.nl ===
    Finished `release` profile [optimized + debuginfo] target(s) in 0.08s
     Running `/home/orazio/Projects/ripopt/target/release/ripopt /home/orazio/Projects/ripopt/benchmarks/gas/gaslib40_steady.nl print_level=5`
ripopt: Detected 722/1682 linear constraints (Hessian contribution skipped)

Number of nonzeros in equality constraint Jacobian...:        4344
Number of nonzeros in inequality constraint Jacobian.:           0
Number of nonzeros in Lagrangian Hessian.............:         972

Total number of variables............................:        1694
                     variables with only lower bounds:          24
                variables with lower and upper bounds:         710
                     variables with only upper bounds:           0
Total number of equality constraints.................:        1682
Total number of inequality constraints...............:           0
        inequality constraints with only lower bounds:           0
   inequality constraints with lower and upper bounds:           0
        inequality constraints with only upper bounds:           0

iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
ripopt: Starting main loop (n=1694, m=1682)
   0    2.1599978e-1      1.58e2      1.00e0      1.58e2        -1.0    0.00e0    0.00e0    0
ripopt: iter0-probe: |y_c|=2.60e0 |y_d|=0.00e0 |z_L|=1.00e0 |z_U|=1.00e0  |dy_c|=0.00e0 |dy_d|=0.00e0 |dz_L|=0.00e0 |dz_U|=0.00e0  ftb_du=-@-1 z=NaN dz=NaN ratio=inf  dw_last=0.00e0 dc_last=0.00e0
ripopt: iter0-probe: |grad_f|_inf=3.600e0@var972, |J^T y|_inf=2.600e0, |y|_inf=2.600e0, |z_L|_inf=1.000e0, |z_U|_inf=1.000e0, sum|y|=3.516e2, sum|z_L|=7.340e2, sum|z_U|=7.100e2, |x|_inf=1.581e2, n=1694 m=1682
ripopt: iter0-probe: |grad_lag|_inf=9.998e-1@var20 (grad_f=0.000e0, J^T y=1.855e-4, z_L=1.000e0, z_U=0.000e0, x_l_fin=true, x_u_fin=false, obj_scaling=1.000e0)
   1    2.1722066e-1      1.56e2      4.55e1      1.59e2        -1.0   1.22e-2   2.60e-4    0
ripopt: iter1-probe: |y_c|=4.41e2 |y_d|=0.00e0 |z_L|=1.02e0 |z_U|=1.00e0  |dy_c|=3.62e4 |dy_d|=0.00e0 |dz_L|=3.81e3 |dz_U|=3.08e0  ftb_du=zL@41 z=1.00e-2 dz=-3.81e3 ratio=2.60e-6  dw_last=0.00e0 dc_last=0.00e0
   2    2.1722568e-1      1.56e2      4.55e1      1.59e2        -1.0   2.32e-4   3.75e-4    0
ripopt: iter2-probe: |y_c|=4.41e2 |y_d|=0.00e0 |z_L|=3.06e0 |z_U|=1.00e0  |dy_c|=3.23e4 |dy_d|=0.00e0 |dz_L|=5.42e3 |dz_U|=4.34e0  ftb_du=zL@669 z=9.97e-3 dz=-2.63e3 ratio=3.75e-6  dw_last=0.00e0 dc_last=0.00e0
   3    2.1722601e-1      1.56e2      1.61e2      1.59e2        -1.0   1.21e-5   4.67e-4    0
ripopt: iter3-probe: |y_c|=4.41e2 |y_d|=0.00e0 |z_L|=1.69e2 |z_U|=1.00e0  |dy_c|=5.60e4 |dy_d|=0.00e0 |dz_L|=3.55e5 |dz_U|=3.33e0  ftb_du=zL@673 z=4.31e-3 dz=-9.14e2 ratio=4.67e-6  dw_last=0.00e0 dc_last=0.00e0
   4    2.1724708e-1      1.56e2      1.38e2      1.59e2        -1.0   6.50e-4   5.03e-4    0
ripopt: iter4-probe: |y_c|=4.41e2 |y_d|=0.00e0 |z_L|=2.50e2 |z_U|=9.99e-1  |dy_c|=1.56e4 |dy_d|=0.00e0 |dz_L|=1.60e5 |dz_U|=3.09e0  ftb_du=zL@90 z=3.32e-4 dz=-6.54e1 ratio=5.03e-6  dw_last=0.00e0 dc_last=0.00e0
   5    2.1724887e-1      1.56e2      1.01e3      1.58e2        -1.0   4.47e-5   1.16e-3    0
ripopt: iter5-probe: |y_c|=4.41e2 |y_d|=0.00e0 |z_L|=1.15e3 |z_U|=9.99e-1  |dy_c|=1.49e4 |dy_d|=0.00e0 |dz_L|=7.77e5 |dz_U|=3.07e0  ftb_du=zL@236 z=2.37e-3 dz=-2.01e2 ratio=1.16e-5  dw_last=0.00e0 dc_last=0.00e0
   6    2.1728671e-1      1.56e2      2.50e3      1.58e2        -1.0   6.28e-4   1.34e-3    0
ripopt: iter6-probe: |y_c|=4.52e2 |y_d|=0.00e0 |z_L|=3.97e3 |z_U|=9.99e-1  |dy_c|=4.03e4 |dy_d|=0.00e0 |dz_L|=2.10e6 |dz_U|=3.03e0  ftb_du=zL@239 z=1.88e-3 dz=-1.39e2 ratio=1.34e-5  dw_last=0.00e0 dc_last=0.00e0
   7    2.1741883e-1      1.56e2      2.28e3      1.58e2        -1.0   7.37e-4   8.40e-4    0
ripopt: iter7-probe: |y_c|=4.37e2 |y_d|=0.00e0 |z_L|=2.19e3 |z_U|=9.99e-1  |dy_c|=4.04e4 |dy_d|=0.00e0 |dz_L|=2.11e6 |dz_U|=3.01e0  ftb_du=zL@656 z=5.97e-4 dz=-7.04e1 ratio=8.40e-6  dw_last=0.00e0 dc_last=0.00e0
   8    2.1741954e-1      1.56e2      1.87e3      1.58e2        -1.0   4.52e-6   6.39e-6    1
ripopt: iter8-probe: |y_c|=4.27e2 |y_d|=0.00e0 |z_L|=7.94e2 |z_U|=9.99e-1  |dy_c|=4.21e6 |dy_d|=0.00e0 |dz_L|=2.20e8 |dz_U|=2.99e0  ftb_du=zL@21 z=6.89e0 dz=-1.07e8 ratio=6.39e-8  dw_last=0.00e0 dc_last=0.00e0
   9    2.1768751e-1      1.56e2      6.23e3      1.58e2        -1.0   1.71e-3   7.35e-6    0
ripopt: iter9-probe: |y_c|=3.98e2 |y_d|=0.00e0 |z_L|=7.75e2 |z_U|=9.99e-1  |dy_c|=4.92e4 |dy_d|=0.00e0 |dz_L|=2.57e6 |dz_U|=2.99e0  ftb_du=zL@20 z=9.71e-2 dz=-1.31e6 ratio=7.35e-8  dw_last=0.00e0 dc_last=0.00e0
  10    2.1982173e-1      1.53e2      7.30e3      1.59e2        -1.0   1.70e-2   3.64e-3    0
ripopt: iter10-probe: |y_c|=3.93e2 |y_d|=0.00e0 |z_L|=6.57e2 |z_U|=9.99e-1  |dy_c|=5.27e3 |dy_d|=0.00e0 |dz_L|=8.82e4 |dz_U|=2.96e0  ftb_du=zL@286 z=1.24e-1 dz=-3.36e3 ratio=3.64e-5  dw_last=0.00e0 dc_last=0.00e0
  11    2.2042660e-1      1.52e2      8.33e3      1.59e2        -1.0   3.29e-3   9.76e-4    0
ripopt: iter11-probe: |y_c|=3.92e2 |y_d|=0.00e0 |z_L|=3.77e2 |z_U|=9.99e-1  |dy_c|=8.74e3 |dy_d|=0.00e0 |dz_L|=4.60e5 |dz_U|=2.81e0  ftb_du=zL@16 z=4.53e0 dz=-4.60e5 ratio=9.76e-6  dw_last=0.00e0 dc_last=0.00e0
  12    2.3091409e-1      1.44e2      2.26e4      1.74e2        -1.0   5.63e-2   2.91e-4    0
ripopt: iter12-probe: |y_c|=4.37e2 |y_d|=0.00e0 |z_L|=2.86e2 |z_U|=9.99e-1  |dy_c|=5.91e3 |dy_d|=0.00e0 |dz_L|=3.11e5 |dz_U|=2.78e0  ftb_du=zL@16 z=4.53e-2 dz=-1.54e4 ratio=2.91e-6  dw_last=0.00e0 dc_last=0.00e0
  13    2.3106317e-1      1.44e2      3.08e4      1.57e2        -1.0   3.56e-4   2.94e-2    0
ripopt: iter13-probe: |y_c|=4.34e2 |y_d|=0.00e0 |z_L|=8.66e3 |z_U|=1.01e0  |dy_c|=6.06e3 |dy_d|=0.00e0 |dz_L|=2.85e5 |dz_U|=2.52e0  ftb_du=zL@686 z=8.52e-3 dz=-2.87e1 ratio=2.94e-4  dw_last=0.00e0 dc_last=0.00e0
  14    2.3175858e-1      1.44e2      3.06e4      1.57e2        -1.0   1.65e-3   8.99e-4    0
ripopt: iter14-probe: |y_c|=4.26e2 |y_d|=0.00e0 |z_L|=8.87e3 |z_U|=1.01e0  |dy_c|=4.93e3 |dy_d|=0.00e0 |dz_L|=2.26e5 |dz_U|=2.32e0  ftb_du=zL@664 z=6.23e-3 dz=-6.86e2 ratio=8.99e-6  dw_last=0.00e0 dc_last=0.00e0
  15    2.4836108e-1      1.38e2      2.17e4      1.65e2        -1.0   3.88e-2   3.00e-3    0
ripopt: iter15-probe: |y_c|=3.68e2 |y_d|=0.00e0 |z_L|=9.52e3 |z_U|=1.01e0  |dy_c|=4.76e3 |dy_d|=0.00e0 |dz_L|=2.18e5 |dz_U|=2.31e0  ftb_du=zL@63 z=1.85e-3 dz=-6.11e1 ratio=3.00e-5  dw_last=0.00e0 dc_last=0.00e0
  16    3.5814187e-1      1.25e2      6.85e3      1.89e2        -1.0   1.88e-1   4.66e-2    0
ripopt: iter16-probe: |y_c|=3.04e2 |y_d|=0.00e0 |z_L|=1.35e4 |z_U|=1.02e0  |dy_c|=2.05e3 |dy_d|=0.00e0 |dz_L|=8.49e4 |dz_U|=2.16e0  ftb_du=zL@59 z=6.05e-4 dz=-1.29e0 ratio=4.66e-4  dw_last=0.00e0 dc_last=0.00e0
  17    4.7246061e-1      1.26e2      6.26e3      1.65e2        -1.0   8.75e-2   9.79e-2    0
ripopt: iter17-probe: |y_c|=2.72e2 |y_d|=0.00e0 |z_L|=1.03e4 |z_U|=1.14e0  |dy_c|=5.13e2 |dy_d|=0.00e0 |dz_L|=3.27e4 |dz_U|=1.51e0  ftb_du=zL@104 z=8.30e-2 dz=-8.39e1 ratio=9.79e-4  dw_last=0.00e0 dc_last=0.00e0
  18    4.7395147e-1      1.26e2      6.25e3      1.53e2        -1.0   9.30e-4   3.78e-2    0
ripopt: iter18-probe: |y_c|=2.71e2 |y_d|=0.00e0 |z_L|=9.44e3 |z_U|=1.23e0  |dy_c|=1.46e4 |dy_d|=0.00e0 |dz_L|=2.20e4 |dz_U|=2.23e0  ftb_du=zL@105 z=1.26e-2 dz=-3.31e1 ratio=3.78e-4  dw_last=0.00e0 dc_last=0.00e0
  19    4.7396643e-1      1.26e2      2.98e6      1.24e2        -1.0   9.32e-6   4.97e-2    5
ripopt: iter19-probe: |y_c|=2.98e6 |y_d|=0.00e0 |z_L|=5.96e6 |z_U|=1.46e0  |dy_c|=5.99e7 |dy_d|=0.00e0 |dz_L|=5.99e7 |dz_U|=2.33e0  ftb_du=zL@107 z=1.46e-10 dz=-3.23e0 ratio=4.48e-11  dw_last=0.00e0 dc_last=0.00e0
  20    4.7397447e-1      1.26e2     4.44e10      4.71e2        -1.0   5.02e-6   3.78e-2    4
ripopt: iter20-probe: |y_c|=4.44e10 |y_d|=0.00e0 |z_L|=8.88e10 |z_U|=1.66e0  |dy_c|=1.17e12 |dy_d|=0.00e0 |dz_L|=1.17e12 |dz_U|=5.42e0  ftb_du=zL@105 z=1.65e-9 dz=-3.82e1 ratio=4.29e-11  dw_last=0.00e0 dc_last=0.00e0
  21    4.7397458e-1      1.26e2     4.44e10      4.71e2        -1.0   6.52e-8   1.20e-6    3
ripopt: iter21-probe: |y_c|=4.44e10 |y_d|=0.00e0 |z_L|=8.88e10 |z_U|=1.66e0  |dy_c|=5.19e12 |dy_d|=0.00e0 |dz_L|=3.43e10 |dz_U|=5.44e5  ftb_du=zL@125 z=3.13e-5 dz=-2.58e3 ratio=1.20e-8  dw_last=0.00e0 dc_last=0.00e0
  22    4.7397458e-1      1.26e2     4.44e10      4.71e2        -1.0   3.14e-9   1.54e-6   11
ripopt: iter22-probe: |y_c|=4.44e10 |y_d|=0.00e0 |z_L|=8.88e10 |z_U|=1.66e0  |dy_c|=6.74e11 |dy_d|=0.00e0 |dz_L|=7.75e11 |dz_U|=1.49e5  ftb_du=zL@125 z=3.13e-7 dz=-2.01e1 ratio=1.54e-8  dw_last=0.00e0 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=1.82e4, mu=1.00e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.8548546e7     5.37e-1      9.93e1      2.72e2         2.1    0.00e0    0.00e0    0
   1     1.6742196e7      1.33e0      9.88e1      2.80e2         0.7   1.66e-3   6.02e-3    0
ripopt: NLP restoration result: status=Optimal, theta_new=1.34e4 (was 1.82e4), phi_new=1.01e0, sum_p=1.86e3, sum_n=1.18e4, iters=2
  23     1.0055958e0      8.12e1      2.60e0      2.67e2        -1.0    0.00e0   1.54e-6   17
ripopt: iter23-probe: |y_c|=0.00e0 |y_d|=0.00e0 |z_L|=1.00e0 |z_U|=1.00e0  |dy_c|=5.45e11 |dy_d|=0.00e0 |dz_L|=7.82e11 |dz_U|=1.49e5  ftb_du=zL@341 z=1.00e0 dz=-4.37e10 ratio=2.26e-11  dw_last=0.00e0 dc_last=0.00e0
  24     1.0102237e0      8.08e1      3.18e7      2.67e2        -1.0   3.87e-3   1.81e-4    0
ripopt: iter24-probe: |y_c|=3.78e8 |y_d|=0.00e0 |z_L|=1.05e0 |z_U|=1.00e0  |dy_c|=9.75e10 |dy_d|=0.00e0 |dz_L|=5.46e3 |dz_U|=4.30e0  ftb_du=zL@17 z=1.00e-2 dz=-5.46e3 ratio=1.81e-6  dw_last=1.00e6 dc_last=0.00e0
iter       objective      inf_pr      inf_du       compl      lg(mu)  alpha_pr  alpha_du   ls
  25     1.0102705e0      8.08e1      3.18e7      2.61e2        -1.0   3.89e-5   1.90e-2    0
ripopt: iter25-probe: |y_c|=3.79e8 |y_d|=0.00e0 |z_L|=5.86e2 |z_U|=1.08e0  |dy_c|=2.65e10 |dy_d|=0.00e0 |dz_L|=3.08e4 |dz_U|=4.42e0  ftb_du=zL@16 z=9.86e-3 dz=-5.14e1 ratio=1.90e-4  dw_last=3.33e5 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=1.34e4, mu=1.00e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     1.3602474e7     3.85e-1      9.93e1      2.67e2         1.9    0.00e0    0.00e0    0
   1     1.1493174e7      2.60e0      9.87e1      2.72e2         1.2   2.60e-3   1.22e-2    0
ripopt: NLP restoration result: status=Optimal, theta_new=8.25e3 (was 1.34e4), phi_new=2.78e0, sum_p=1.12e3, sum_n=7.27e3, iters=2
  26     2.7796100e0      5.38e1      4.84e2      2.61e2        -1.0    0.00e0   1.90e-2    4
ripopt: iter26-probe: |y_c|=0.00e0 |y_d|=0.00e0 |z_L|=4.85e2 |z_U|=1.08e0  |dy_c|=8.24e9 |dy_d|=0.00e0 |dz_L|=1.28e8 |dz_U|=4.77e0  ftb_du=zL@17 z=9.15e-1 dz=-1.72e2 ratio=5.25e-3  dw_last=1.11e5 dc_last=0.00e0
  27     2.7802182e0      5.37e1      3.38e4      2.58e2        -1.0   9.17e-4   1.28e-2    0
ripopt: iter27-probe: |y_c|=1.78e6 |y_d|=0.00e0 |z_L|=7.82e2 |z_U|=1.66e1  |dy_c|=1.95e9 |dy_d|=0.00e0 |dz_L|=2.32e4 |dz_U|=1.21e3  ftb_du=zL@17 z=9.15e-3 dz=-7.08e1 ratio=1.28e-4  dw_last=3.70e4 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=8.25e3, mu=1.00e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     8.3888153e6     2.96e-1      9.93e1      2.67e2         1.7    0.00e0    0.00e0    0
   1     8.1863868e6     2.96e-1      9.99e1      2.67e2         1.0   3.69e-4   3.41e-3    0
ripopt: NLP restoration result: status=Optimal, theta_new=7.27e3 (was 8.25e3), phi_new=2.98e0, sum_p=9.88e2, sum_n=6.40e3, iters=2
  28     2.9766968e0      4.75e1      2.60e0      2.66e2        -1.0    0.00e0   1.28e-2    5
ripopt: iter28-probe: |y_c|=0.00e0 |y_d|=0.00e0 |z_L|=1.00e0 |z_U|=1.00e0  |dy_c|=6.40e8 |dy_d|=0.00e0 |dz_L|=3.90e4 |dz_U|=1.79e6  ftb_du=zL@16 z=1.00e0 dz=-2.19e1 ratio=4.52e-2  dw_last=1.23e4 dc_last=0.00e0
  29     2.9768187e0      4.75e1      6.46e1      2.66e2        -1.0   1.83e-4   5.72e-4    0
ripopt: iter29-probe: |y_c|=2.67e4 |y_d|=0.00e0 |z_L|=1.04e0 |z_U|=4.24e0  |dy_c|=1.46e8 |dy_d|=0.00e0 |dz_L|=1.73e3 |dz_U|=5.67e3  ftb_du=zL@16 z=1.00e-2 dz=-1.73e3 ratio=5.72e-6  dw_last=4.12e3 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=7.27e3, mu=1.00e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     7.3983835e6     4.11e-1      9.93e1      2.66e2         1.7    0.00e0    0.00e0    0
   1     7.2057262e6     4.11e-1      9.99e1      2.66e2         1.0   3.94e-4   3.39e-3    0
ripopt: NLP restoration result: status=Optimal, theta_new=6.40e3 (was 7.27e3), phi_new=3.11e0, sum_p=8.61e2, sum_n=5.64e3, iters=2
  30     3.1125785e0      4.19e1      2.60e0      2.66e2        -1.0    0.00e0   5.72e-4    2
ripopt: iter30-probe: |y_c|=0.00e0 |y_d|=0.00e0 |z_L|=1.00e0 |z_U|=1.00e0  |dy_c|=4.93e7 |dy_d|=0.00e0 |dz_L|=4.14e2 |dz_U|=2.32e6  ftb_du=zL@17 z=1.00e0 dz=-2.82e2 ratio=3.52e-3  dw_last=1.37e3 dc_last=0.00e0
  31     3.1129076e0      4.19e1      1.78e1      2.66e2        -1.0   4.72e-4   1.71e-4    0
ripopt: iter31-probe: |y_c|=4.48e3 |y_d|=0.00e0 |z_L|=1.01e0 |z_U|=1.37e0  |dy_c|=9.49e6 |dy_d|=0.00e0 |dz_L|=5.79e3 |dz_U|=2.17e3  ftb_du=zL@16 z=1.00e-2 dz=-5.79e3 ratio=1.71e-6  dw_last=4.57e2 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=6.40e3, mu=1.00e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     6.5119892e6     4.05e-1      9.93e1      2.66e2         1.6    0.00e0    0.00e0    0
   1     6.3230685e6     4.05e-1      9.99e1      2.65e2         0.9   4.31e-4   5.20e-3    0
ripopt: NLP restoration result: status=Optimal, theta_new=5.42e3 (was 6.40e3), phi_new=3.24e0, sum_p=7.29e2, sum_n=4.74e3, iters=2
  32     3.2391923e0      3.72e1      2.60e0      2.66e2        -1.0    0.00e0   1.71e-4    4
ripopt: iter32-probe: |y_c|=0.00e0 |y_d|=0.00e0 |z_L|=1.00e0 |z_U|=1.00e0  |dy_c|=3.16e6 |dy_d|=0.00e0 |dz_L|=4.13e2 |dz_U|=2.95e5  ftb_du=zL@17 z=1.00e0 dz=-3.84e2 ratio=2.58e-3  dw_last=1.52e2 dc_last=0.00e0
  33     3.2395381e0      3.72e1      2.60e0      2.66e2        -1.0   4.23e-4   1.48e-4    0
ripopt: iter33-probe: |y_c|=2.53e2 |y_d|=0.00e0 |z_L|=1.01e0 |z_U|=1.36e0  |dy_c|=5.98e5 |dy_d|=0.00e0 |dz_L|=6.70e3 |dz_U|=2.41e3  ftb_du=zL@16 z=1.00e-2 dz=-6.70e3 ratio=1.48e-6  dw_last=5.08e1 dc_last=0.00e0
ripopt: Entering NLP restoration (theta=5.42e3, mu=1.00e-1)
iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
   0     5.5163908e6     4.02e-1      9.93e1      2.66e2         1.6    0.00e0    0.00e0    0
   1     5.3245448e6     4.02e-1      9.99e1      2.62e2         0.9   5.07e-4   8.78e-3    0
ripopt: NLP restoration result: status=Optimal, theta_new=4.21e3 (was 5.42e3), phi_new=3.39e0, sum_p=5.75e2, sum_n=3.54e3, iters=2
  34     3.3853021e0      2.98e1      2.60e0      2.65e2        -1.0    0.00e0   1.48e-4    4
ripopt: iter34-probe: |y_c|=0.00e0 |y_d|=0.00e0 |z_L|=1.00e0 |z_U|=1.00e0  |dy_c|=2.00e5 |dy_d|=0.00e0 |dz_L|=4.56e2 |dz_U|=3.22e5  ftb_du=zL@17 z=1.00e0 dz=-4.06e2 ratio=2.44e-3  dw_last=1.69e1 dc_last=0.00e0
  35     3.3858175e0      2.98e1      2.78e0      2.65e2        -1.0   4.42e-4   1.16e-4    0
ripopt: iter35-probe: |y_c|=1.65e1 |y_d|=0.00e0 |z_L|=1.01e0 |z_U|=1.27e0  |dy_c|=3.73e4 |dy_d|=0.00e0 |dz_L|=8.53e3 |dz_U|=2.30e3  ftb_du=zL@16 z=1.00e-2 dz=-8.53e3 ratio=1.16e-6  dw_last=5.65e0 dc_last=0.00e0
  36     3.3475218e0      4.84e1      6.29e1      2.66e2        -1.0   1.31e-2   2.40e-4    0
ripopt: iter36-probe: |y_c|=1.37e1 |y_d|=0.00e0 |z_L|=1.00e0 |z_U|=1.45e0  |dy_c|=1.09e3 |dy_d|=0.00e0 |dz_L|=4.57e2 |dz_U|=4.76e3  ftb_du=zU@19 z=1.15e-2 dz=-4.76e3 ratio=2.40e-6  dw_last=5.65e0 dc_last=0.00e0
  37     3.3525177e0      4.84e1      6.30e1      2.59e2        -1.0   1.59e-3   2.18e-2    0
ripopt: iter37-probe: |y_c|=1.40e1 |y_d|=0.00e0 |z_L|=1.43e0 |z_U|=2.69e1  |dy_c|=2.01e2 |dy_d|=0.00e0 |dz_L|=3.88e1 |dz_U|=1.17e3  ftb_du=zL@17 z=8.54e-3 dz=-3.88e1 ratio=2.18e-4  dw_last=5.65e0 dc_last=0.00e0
  38     3.3724393e0      4.83e1      6.29e1      2.49e2        -1.0   7.84e-3   3.51e-2    0
ripopt: iter38-probe: |y_c|=1.55e1 |y_d|=0.00e0 |z_L|=2.01e0 |z_U|=5.69e1  |dy_c|=1.88e2 |dy_d|=0.00e0 |dz_L|=1.79e1 |dz_U|=9.03e2  ftb_du=zL@105 z=4.55e-3 dz=-1.28e1 ratio=3.51e-4  dw_last=5.65e0 dc_last=0.00e0
  39     3.3712760e0      4.82e1      7.25e1      2.38e2        -1.0   1.94e-3   4.16e-2    0
ripopt: iter39-probe: |y_c|=1.58e1 |y_d|=0.00e0 |z_L|=2.69e0 |z_U|=8.98e1  |dy_c|=1.81e2 |dy_d|=0.00e0 |dz_L|=2.07e1 |dz_U|=7.92e2  ftb_du=zL@17 z=6.02e-3 dz=-1.43e1 ratio=4.16e-4  dw_last=5.65e0 dc_last=0.00e0
  40     3.2986258e0      1.67e2      5.46e1      2.26e2        -1.0   1.22e-1   5.69e-2    0
ripopt: iter40-probe: |y_c|=3.66e1 |y_d|=0.00e0 |z_L|=3.76e0 |z_U|=1.22e2  |dy_c|=1.70e2 |dy_d|=0.00e0 |dz_L|=1.92e1 |dz_U|=6.80e2  ftb_du=zL@145 z=1.04e-2 dz=-1.81e1 ratio=5.69e-4  dw_last=5.65e0 dc_last=0.00e0
  41     3.2679420e0      1.55e2      5.42e1      1.83e2        -1.0   7.25e-2   1.81e-1    0
ripopt: iter41-probe: |y_c|=4.45e1 |y_d|=0.00e0 |z_L|=7.45e0 |z_U|=1.59e2  |dy_c|=1.10e2 |dy_d|=0.00e0 |dz_L|=2.97e1 |dz_U|=2.09e2  ftb_du=zL@717 z=1.60e-3 dz=-8.77e-1 ratio=1.81e-3  dw_last=5.65e0 dc_last=0.00e0
  42     3.2380501e0      1.39e2      5.56e1      1.29e2        -1.0   1.07e-1   2.81e-1    0
ripopt: iter42-probe: |y_c|=5.20e1 |y_d|=0.00e0 |z_L|=3.69e1 |z_U|=2.11e2  |dy_c|=6.94e1 |dy_d|=0.00e0 |dz_L|=1.05e2 |dz_U|=1.84e2  ftb_du=zL@151 z=4.31e-4 dz=-1.52e-1 ratio=2.81e-3  dw_last=5.65e0 dc_last=0.00e0
  43     3.1151142e0      1.93e2      6.95e1      7.40e1        -1.0   9.36e-1   4.29e-1    0
ripopt: iter43-probe: |y_c|=7.71e1 |y_d|=0.00e0 |z_L|=4.92e1 |z_U|=2.51e2  |dy_c|=2.68e1 |dy_d|=0.00e0 |dz_L|=3.19e1 |dz_U|=9.31e1  ftb_du=zL@271 z=4.36e-4 dz=-1.01e-1 ratio=4.29e-3  dw_last=5.65e0 dc_last=0.00e0
  44     3.2479844e0      3.18e1      5.11e1      3.99e1        -1.0   7.72e-1   4.61e-1    0
ripopt: iter44-probe: |y_c|=2.73e1 |y_d|=0.00e0 |z_L|=4.15e1 |z_U|=1.63e2  |dy_c|=6.45e1 |dy_d|=0.00e0 |dz_L|=1.67e1 |dz_U|=1.91e2  ftb_du=zL@419 z=5.81e-4 dz=-1.25e-1 ratio=4.61e-3  dw_last=5.65e0 dc_last=0.00e0
  45     3.2172017e0      5.30e0      1.15e2      3.08e1        -1.0    1.00e0   1.97e-1    0
ripopt: iter45-probe: |y_c|=3.44e1 |y_d|=0.00e0 |z_L|=3.46e1 |z_U|=1.35e2  |dy_c|=2.44e1 |dy_d|=0.00e0 |dz_L|=3.50e1 |dz_U|=1.42e2  ftb_du=zL@721 z=3.26e-2 dz=-1.64e1 ratio=1.97e-3  dw_last=5.65e0 dc_last=0.00e0
  46     3.1153515e0     1.39e-1      2.11e1      5.03e0        -1.0    1.00e0   8.44e-1    0
ripopt: iter46-probe: |y_c|=4.90e1 |y_d|=0.00e0 |z_L|=4.38e1 |z_U|=2.09e1  |dy_c|=1.49e1 |dy_d|=0.00e0 |dz_L|=3.61e1 |dz_U|=1.35e2  ftb_du=zL@356 z=2.90e-2 dz=-3.40e0 ratio=8.44e-3  dw_last=5.65e0 dc_last=0.00e0
  47     2.9791425e0      9.30e0      1.57e1      3.60e0        -1.0    1.00e0   2.68e-1    0
ripopt: iter47-probe: |y_c|=2.86e1 |y_d|=0.00e0 |z_L|=3.99e1 |z_U|=1.52e1  |dy_c|=2.04e1 |dy_d|=0.00e0 |dz_L|=1.48e1 |dz_U|=2.14e1  ftb_du=zL@720 z=5.02e-3 dz=-1.85e0 ratio=2.68e-3  dw_last=5.65e0 dc_last=0.00e0
  48     2.7119024e0      4.53e0      1.30e1      2.05e0        -1.0    1.00e0   4.34e-1    0
ripopt: iter48-probe: |y_c|=1.64e1 |y_d|=0.00e0 |z_L|=3.02e1 |z_U|=8.40e0  |dy_c|=1.22e1 |dy_d|=0.00e0 |dz_L|=2.30e1 |dz_U|=1.56e1  ftb_du=zU@18 z=1.12e-2 dz=-2.55e0 ratio=4.34e-3  dw_last=5.65e0 dc_last=0.00e0
  49     1.6924212e0      2.93e1      4.62e0     7.17e-1        -1.0   8.41e-1   7.09e-1    0
ripopt: iter49-probe: |y_c|=1.12e1 |y_d|=0.00e0 |z_L|=1.68e1 |z_U|=2.38e0  |dy_c|=6.24e0 |dy_d|=0.00e0 |dz_L|=1.92e1 |dz_U|=8.73e0  ftb_du=zU@632 z=2.41e-4 dz=-3.37e-2 ratio=7.09e-3  dw_last=5.65e0 dc_last=0.00e0
iter       objective      inf_pr      inf_du       compl      lg(mu)  alpha_pr  alpha_du   ls
  50     2.2230231e0      7.11e1      1.64e0     6.28e-1        -1.0    1.00e0   7.81e-1    0
ripopt: iter50-probe: |y_c|=8.47e0 |y_d|=0.00e0 |z_L|=1.09e1 |z_U|=4.38e-1  |dy_c|=4.58e0 |dy_d|=0.00e0 |dz_L|=7.52e0 |dz_U|=3.01e0  ftb_du=zU@10 z=2.38e-2 dz=-3.01e0 ratio=7.81e-3  dw_last=5.65e0 dc_last=0.00e0
  51     1.8333041e0      3.85e1     4.44e-1     6.19e-1        -1.0    1.00e0    1.00e0    0
ripopt: iter51-probe: |y_c|=1.05e1 |y_d|=0.00e0 |z_L|=1.39e1 |z_U|=1.23e0  |dy_c|=4.58e0 |dy_d|=0.00e0 |dz_L|=2.93e0 |dz_U|=1.21e0  ftb_du=zU@18 z=8.21e-2 dz=-1.44e-1 ratio=5.63e-1  dw_last=5.65e0 dc_last=0.00e0
  52     1.7950889e0      5.74e0     6.51e-2     2.08e-1        -1.0    1.00e0    1.00e0    0
ripopt: iter52-probe: |y_c|=1.04e1 |y_d|=0.00e0 |z_L|=1.18e1 |z_U|=6.72e-1  |dy_c|=2.03e0 |dy_d|=0.00e0 |dz_L|=2.03e0 |dz_U|=5.59e-1  ftb_du=zU@10 z=6.72e-1 dz=-5.59e-1 ratio=1.19e0  dw_last=5.65e0 dc_last=0.00e0
  53     1.8087751e0     5.96e-1     6.92e-3     1.13e-1        -1.0    1.00e0    1.00e0    0
ripopt: iter53-probe: |y_c|=1.04e1 |y_d|=0.00e0 |z_L|=1.14e1 |z_U|=4.69e-1  |dy_c|=4.55e-1 |dy_d|=0.00e0 |dz_L|=4.55e-1 |dz_U|=2.11e-1  ftb_du=zU@10 z=4.61e-1 dz=-2.11e-1 ratio=2.17e0  dw_last=5.65e0 dc_last=0.00e0
  54     1.6167595e0     8.64e-1     2.80e-2     3.53e-2        -1.7    1.00e0    1.00e0    0
ripopt: iter54-probe: |y_c|=3.49e0 |y_d|=0.00e0 |z_L|=3.16e0 |z_U|=9.38e-2  |dy_c|=8.31e0 |dy_d|=0.00e0 |dz_L|=8.31e0 |dz_U|=4.28e-1  ftb_du=zU@10 z=3.30e-2 dz=-4.28e-1 ratio=7.64e-2  dw_last=5.65e0 dc_last=0.00e0
  55     1.5417391e0      1.23e0     2.11e-3     2.28e-2        -1.7    1.00e0    1.00e0    0
ripopt: iter55-probe: |y_c|=3.48e0 |y_d|=0.00e0 |z_L|=3.24e0 |z_U|=9.38e-2  |dy_c|=3.81e-1 |dy_d|=0.00e0 |dz_L|=3.81e-1 |dz_U|=2.86e-2  ftb_du=zL@336 z=8.77e-1 dz=-3.81e-1 ratio=2.28e0  dw_last=5.65e0 dc_last=0.00e0
  56     1.5482378e0     5.90e-3     2.03e-4     2.01e-2        -1.7    1.00e0    1.00e0    0
ripopt: iter56-probe: |y_c|=3.48e0 |y_d|=0.00e0 |z_L|=3.22e0 |z_U|=9.38e-2  |dy_c|=1.28e-1 |dy_d|=0.00e0 |dz_L|=1.28e-1 |dz_U|=5.73e-3  ftb_du=zL@346 z=1.19e0 dz=-1.28e-1 ratio=9.16e0  dw_last=5.65e0 dc_last=0.00e0
  57     1.4520882e0      2.27e0     3.02e-1     7.33e-3        -3.8   8.80e-1   7.33e-1    0
ripopt: iter57-probe: |y_c|=3.59e0 |y_d|=0.00e0 |z_L|=1.78e0 |z_U|=2.55e-2  |dy_c|=2.06e0 |dy_d|=0.00e0 |dz_L|=2.06e0 |dz_U|=9.31e-2  ftb_du=zU@10 z=8.41e-6 dz=-7.62e-2 ratio=1.10e-4  dw_last=5.65e0 dc_last=0.00e0
  58     1.3715427e0      1.64e1     2.81e-2     2.59e-3        -3.8   7.98e-1   8.58e-1    0
ripopt: iter58-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=1.23e0 |z_U|=4.24e-3  |dy_c|=6.69e-1 |dy_d|=0.00e0 |dz_L|=7.58e-1 |dz_U|=2.48e-2  ftb_du=zL@0 z=1.76e-8 dz=-1.37e-4 ratio=1.29e-4  dw_last=5.65e0 dc_last=0.00e0
  59     1.2947845e0      2.88e1     2.45e-1     8.65e-4        -3.8   7.14e-1    1.00e0    0
ripopt: iter59-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=1.25e0 |z_U|=7.05e-4  |dy_c|=8.58e-1 |dy_d|=0.00e0 |dz_L|=8.73e-1 |dz_U|=3.53e-3  ftb_du=zL@346 z=8.14e-4 dz=-9.39e-2 ratio=8.67e-3  dw_last=5.65e0 dc_last=0.00e0
  60     1.2934089e0      4.96e0     7.64e-3     2.17e-3        -3.8    1.00e0    1.00e0    0
ripopt: iter60-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=1.25e0 |z_U|=7.05e-4  |dy_c|=4.55e-1 |dy_d|=0.00e0 |dz_L|=2.14e-1 |dz_U|=7.30e-5  ftb_du=zL@286 z=9.55e-5 dz=-3.17e-4 ratio=3.01e-1  dw_last=5.65e0 dc_last=0.00e0
  61     1.2906462e0     3.22e-1     1.15e-3     1.69e-4        -3.8   9.41e-1    1.00e0    0
ripopt: iter61-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=1.25e0 |z_U|=7.05e-4  |dy_c|=4.04e-3 |dy_d|=0.00e0 |dz_L|=3.03e-2 |dz_U|=1.14e-5  ftb_du=zL@720 z=5.90e-4 dz=-5.56e-4 ratio=1.06e0  dw_last=5.65e0 dc_last=0.00e0
  62     1.2908258e0     7.15e-5     1.60e-6     1.52e-4        -3.8    1.00e0    1.00e0    0
ripopt: iter62-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=1.25e0 |z_U|=7.05e-4  |dy_c|=1.63e-3 |dy_d|=0.00e0 |dz_L|=2.19e-3 |dz_U|=1.10e-6  ftb_du=zL@356 z=1.98e-3 dz=-1.34e-4 ratio=1.48e1  dw_last=5.65e0 dc_last=0.00e0
  63     1.2900718e0     8.99e-2     1.31e-2     4.19e-5        -5.7   8.31e-1    1.00e0    0
ripopt: iter63-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=1.25e0 |z_U|=8.65e-6  |dy_c|=7.76e-2 |dy_d|=0.00e0 |dz_L|=7.76e-2 |dz_U|=6.97e-4  ftb_du=zU@52 z=1.78e-7 dz=-2.22e-5 ratio=7.99e-3  dw_last=5.65e0 dc_last=0.00e0
  64     1.2900582e0     6.11e-6     4.40e-1     1.24e-5        -5.7    1.00e0   7.32e-1    0
ripopt: iter64-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=1.52e0 |z_U|=8.65e-6  |dy_c|=7.82e-1 |dy_d|=0.00e0 |dz_L|=1.64e0 |dz_U|=3.23e-7  ftb_du=zL@356 z=4.37e-9 dz=-3.24e-3 ratio=1.35e-6  dw_last=1.88e0 dc_last=0.00e0
  65     1.2899987e0     9.67e-5      1.15e0     4.17e-6        -5.7   3.01e-1    1.00e0    0
ripopt: iter65-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=1.25e0 |z_U|=8.65e-6  |dy_c|=7.79e-1 |dy_d|=0.00e0 |dz_L|=1.20e0 |dz_U|=8.61e-8  ftb_du=zL@720 z=1.85e-5 dz=-7.21e-5 ratio=2.57e-1  dw_last=1.88e0 dc_last=0.00e0
  66     1.2900014e0     9.74e-5     2.80e-4     7.47e-6        -5.7    1.00e0    1.00e0    0
ripopt: iter66-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=1.25e0 |z_U|=8.65e-6  |dy_c|=5.48e-1 |dy_d|=0.00e0 |dz_L|=2.31e-3 |dz_U|=9.96e-10  ftb_du=zL@720 z=1.34e-5 dz=-5.18e-6 ratio=2.58e0  dw_last=1.88e0 dc_last=0.00e0
  67     1.2899974e0     3.77e-5     1.93e-7     2.45e-6        -5.7    1.00e0    1.00e0    0
ripopt: iter67-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=1.25e0 |z_U|=8.65e-6  |dy_c|=6.17e-4 |dy_d|=0.00e0 |dz_L|=4.01e-4 |dz_U|=3.43e-11  ftb_du=zL@356 z=2.66e-3 dz=-4.01e-4 ratio=6.65e0  dw_last=1.88e0 dc_last=0.00e0
  68     1.2899968e0     8.55e-7    1.84e-11     1.84e-6        -5.7    1.00e0    1.00e0    0
ripopt: iter68-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=1.25e0 |z_U|=8.65e-6  |dy_c|=2.01e-7 |dy_d|=0.00e0 |dz_L|=3.01e-7 |dz_U|=4.11e-12  ftb_du=zL@356 z=2.66e-3 dz=-2.94e-7 ratio=9.06e3  dw_last=1.88e0 dc_last=0.00e0
  69     1.2899876e0     7.86e-6     3.58e-6     1.15e-8        -8.6   9.96e-1    1.00e0    0
ripopt: iter69-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=1.25e0 |z_U|=1.18e-8  |dy_c|=1.00e-3 |dy_d|=0.00e0 |dz_L|=1.00e-3 |dz_U|=8.64e-6  ftb_du=zU@52 z=3.49e-10 dz=-2.73e-7 ratio=1.28e-3  dw_last=1.88e0 dc_last=0.00e0
  70     1.2899875e0    1.32e-11    1.68e-13     2.51e-9        -8.6    1.00e0    1.00e0    0
ripopt: iter70-probe: |y_c|=3.60e0 |y_d|=0.00e0 |z_L|=1.25e0 |z_U|=1.18e-8  |dy_c|=3.86e-6 |dy_d|=0.00e0 |dz_L|=2.80e-7 |dz_U|=4.17e-11  ftb_du=zL@720 z=1.86e-8 dz=-6.63e-8 ratio=2.80e-1  dw_last=1.88e0 dc_last=0.00e0

Phase breakdown (71 iterations):
  Problem eval            0.151s (  8.7%)
  KKT assembly            0.000s (  0.0%)
  Factorization           0.000s (  0.0%)
  Direction solve         1.087s ( 62.9%)
  Line search             0.018s (  1.0%)
  Other                   0.472s ( 27.3%)
  Total                   1.727s

Number of Iterations....: 70

                                   (scaled)                 (unscaled)
Objective...............:    1.2899875389723998e0     1.2899875389723998e0
Dual infeasibility......:  1.6780095563959776e-13   1.6780095563959776e-13
Constraint violation....:  1.3187673175707459e-11   1.3187673175707459e-11
Complementarity.........:   2.5059651073292035e-9    2.5059651073292035e-9
Overall NLP error.......:   2.5059651073292035e-9    2.5059651073292035e-9

Number of objective function evaluations             = 201
Number of objective gradient evaluations             = 79
Number of equality constraint evaluations            = 201
Number of inequality constraint evaluations          = 201
Number of equality constraint Jacobian evaluations   = 79
Number of inequality constraint Jacobian evaluations = 79
Number of Lagrangian Hessian evaluations             = 71
Total seconds in ripopt                              = 1.743

EXIT: Optimal Solution Found.
ripopt 0.8.0: Optimal after 70 iterations
Objective: 1.289987538972400e0

--- ripopt diagnostics ---
status: Optimal
iterations: 70
wall_time: 1.743s
final_mu: 2.51e-9
final_primal_inf: 1.32e-11
final_dual_inf: 1.68e-13
final_compl: 2.51e-9
restoration_count: 0
nlp_restoration_count: 6
mu_mode_switches: 0
filter_rejects: 6
watchdog_activations: 0
soc_corrections: 0
--- end diagnostics ---
Gas benchmark complete. Solver output is in benchmarks/gas/*.sol.
make[1]: Leaving directory '/home/orazio/Projects/ripopt/benchmarks'
```
