# test-c

## Command

```bash
make test-c
```

## Output

```bash
make test-c
cargo build --release
    Finished `release` profile [optimized + debuginfo] target(s) in 0.05s
=== Compiling and running C API examples ===
cc examples/c_api_test.c -I. -Ltarget/release -lripopt \
        -Wl,-rpath,/home/orazio/Projects/ripopt/target/release -o target/release/c_api_test -lm
target/release/c_api_test

Number of nonzeros in equality constraint Jacobian...:           4
Number of nonzeros in inequality constraint Jacobian.:           4
Number of nonzeros in Lagrangian Hessian.............:          10

Total number of variables............................:           4
                     variables with only lower bounds:           0
                variables with lower and upper bounds:           4
                     variables with only upper bounds:           0
Total number of equality constraints.................:           1
Total number of inequality constraints...............:           1
        inequality constraints with only lower bounds:           1
   inequality constraints with lower and upper bounds:           0
        inequality constraints with only upper bounds:           0

iter       objective      inf_pr      inf_du       compl          mu  alpha_pr  alpha_du   ls
ripopt: Starting main loop (n=4, m=2)
   0     1.6109693e1      1.12e1     5.28e-1      3.99e0        -1.0    0.00e0    0.00e0    0
ripopt: iter0-probe: |y_c|=8.79e-2 |y_d|=4.72e-1 |z_L|=1.00e0 |z_U|=1.00e0  |dy_c|=0.00e0 |dy_d|=0.00e0 |dz_L|=0.00e0 |dz_U|=0.00e0  ftb_du=-@-1 z=NaN dz=NaN ratio=inf  dw_last=0.00e0 dc_last=0.00e0
ripopt: iter0-probe: |grad_f|_inf=1.206e1@var0, |J^T y|_inf=1.156e1, |y|_inf=4.724e-1, |z_L|_inf=1.000e0, |z_U|_inf=1.000e0, sum|y|=5.603e-1, sum|z_L|=4.000e0, sum|z_U|=4.000e0, |x|_inf=4.960e0, n=4 m=2
ripopt: iter0-probe: |grad_lag|_inf=5.211e-1@var3 (grad_f=1.104e1, J^T y=-1.156e1, z_L=1.000e0, z_U=1.000e0, x_l_fin=true, x_u_fin=true, obj_scaling=1.000e0)
   1     1.6982239e1     7.30e-1      1.02e1      3.61e0        -1.0    1.00e0   7.19e-2    0
ripopt: iter1-probe: |y_c|=1.50e0 |y_d|=7.48e-1 |z_L|=9.41e-1 |z_U|=9.32e-1  |dy_c|=1.41e0 |dy_d|=2.76e-1 |dz_L|=3.81e0 |dz_U|=1.38e1  ftb_du=zU@2 z=1.00e-2 dz=-1.38e1 ratio=7.19e-4  dw_last=0.00e0 dc_last=0.00e0
   2     1.7318411e1     3.60e-2     5.05e-1     2.39e-1        -1.0    1.00e0    1.00e0    0
ripopt: iter2-probe: |y_c|=1.63e-1 |y_d|=5.47e-1 |z_L|=9.13e-1 |z_U|=1.71e-1  |dy_c|=1.34e0 |dy_d|=2.01e-1 |dz_L|=9.28e-1 |dz_U|=9.10e-1  ftb_du=zL@1 z=1.27e-2 dz=-9.28e-1 ratio=1.36e-2  dw_last=0.00e0 dc_last=0.00e0
   3     1.6849424e1     2.78e-1     6.68e-2     7.48e-2        -1.7    1.00e0   7.94e-1    0
ripopt: iter3-probe: |y_c|=1.52e-1 |y_d|=5.48e-1 |z_L|=1.00e0 |z_U|=1.27e-1  |dy_c|=1.06e-2 |dy_d|=4.18e-4 |dz_L|=3.88e-1 |dz_U|=1.80e-1  ftb_du=zL@3 z=3.11e-3 dz=-3.88e-1 ratio=7.94e-3  dw_last=0.00e0 dc_last=0.00e0
   4     1.7051199e1     4.71e-3     2.78e-3     2.20e-2        -1.7    1.00e0    1.00e0    0
ripopt: iter4-probe: |y_c|=1.53e-1 |y_d|=5.47e-1 |z_L|=1.22e0 |z_U|=7.85e-2  |dy_c|=6.94e-4 |dy_d|=8.92e-4 |dz_L|=2.14e-1 |dz_U|=4.88e-2  ftb_du=zL@2 z=7.43e-3 dz=-1.83e-2 ratio=4.01e-1  dw_last=0.00e0 dc_last=0.00e0
   5     1.7011979e1     7.19e-3     8.50e-3     2.17e-3        -3.8   9.98e-1   9.45e-1    0
ripopt: iter5-probe: |y_c|=1.61e-1 |y_d|=5.52e-1 |z_L|=1.10e0 |z_U|=8.37e-3  |dy_c|=8.29e-3 |dy_d|=5.20e-3 |dz_L|=1.22e-1 |dz_U|=7.42e-2  ftb_du=zL@3 z=8.97e-6 dz=-6.32e-2 ratio=1.42e-4  dw_last=0.00e0 dc_last=0.00e0
   6     1.7014271e1     1.74e-5     9.78e-6     1.70e-4        -3.8    1.00e0    1.00e0    0
ripopt: iter6-probe: |y_c|=1.61e-1 |y_d|=5.52e-1 |z_L|=1.09e0 |z_U|=6.03e-4  |dy_c|=3.09e-4 |dy_d|=7.47e-5 |dz_L|=1.10e-2 |dz_U|=7.76e-3  ftb_du=zU@1 z=6.03e-4 dz=-7.76e-3 ratio=7.77e-2  dw_last=0.00e0 dc_last=0.00e0
   7     1.7014021e1     1.23e-7     1.82e-7     2.00e-6        -5.7    1.00e0    1.00e0    0
ripopt: iter7-probe: |y_c|=1.61e-1 |y_d|=5.52e-1 |z_L|=1.09e0 |z_U|=7.42e-6  |dy_c|=6.45e-5 |dy_d|=3.86e-5 |dz_L|=9.90e-4 |dz_U|=5.96e-4  ftb_du=zL@3 z=4.61e-6 dz=-3.96e-4 ratio=1.17e-2  dw_last=0.00e0 dc_last=0.00e0
   8     1.7014017e1    1.77e-11    2.52e-11     2.53e-9        -8.6    1.00e0    1.00e0    0
ripopt: iter8-probe: |y_c|=1.61e-1 |y_d|=5.52e-1 |z_L|=1.09e0 |z_U|=9.79e-9  |dy_c|=7.90e-7 |dy_d|=4.63e-7 |dz_L|=1.19e-5 |dz_U|=7.41e-6  ftb_du=zU@1 z=9.79e-9 dz=-7.41e-6 ratio=1.32e-3  dw_last=0.00e0 dc_last=0.00e0

Phase breakdown (9 iterations):
  Problem eval            0.000s (  1.2%)
  KKT assembly            0.000s (  0.0%)
  Factorization           0.000s (  0.0%)
  Direction solve         0.000s ( 12.9%)
  Line search             0.000s (  8.7%)
  Other                   0.000s ( 77.3%)
  Total                   0.000s

Number of Iterations....: 8

                                   (scaled)                 (unscaled)
Objective...............:    1.7014017145179164e1     1.7014017145179164e1
Dual infeasibility......:  2.5164268330523022e-11   2.5164268330523022e-11
Constraint violation....:  1.7706724975141697e-11   1.7706724975141697e-11
Complementarity.........:   2.5277100427932946e-9    2.5277100427932946e-9
Overall NLP error.......:   2.5277100427932946e-9    2.5277100427932946e-9

Number of objective function evaluations             = 17
Number of objective gradient evaluations             = 9
Number of equality constraint evaluations            = 17
Number of inequality constraint evaluations          = 17
Number of equality constraint Jacobian evaluations   = 9
Number of inequality constraint Jacobian evaluations = 9
Number of Lagrangian Hessian evaluations             = 9
Total seconds in ripopt                              = 0.000

EXIT: Optimal Solution Found.

=== HS071 Result ===
Status : 0  (0 = Optimal)
Obj    : 17.0140171452  (expected ~17.0140173)
x      : [1.000000, 4.743000, 3.821150, 1.379408]
         (expected ~[1.0, 4.743, 3.821, 1.379])
Test   : PASSED
cc examples/c_rosenbrock.c -I. -Ltarget/release -lripopt \
        -Wl,-rpath,/home/orazio/Projects/ripopt/target/release -o target/release/c_rosenbrock -lm
target/release/c_rosenbrock
=== Rosenbrock (unconstrained) ===
Status : 0  (0 = Optimal)
Obj    : 6.3882005409e-26  (expected 0)
x      : [1.00000000, 1.00000000]  (expected [1, 1])
Test   : PASSED
cc examples/c_hs035.c -I. -Ltarget/release -lripopt \
        -Wl,-rpath,/home/orazio/Projects/ripopt/target/release -o target/release/c_hs035 -lm
target/release/c_hs035
=== HS035 (inequality + bounds) ===
Status      : 0  (0 = Optimal)
Obj         : 0.1111111097  (expected 0.1111111111)
x           : [1.333333, 0.777778, 0.444444]
              (expected [1.3333, 0.7778, 0.4444])
g(x)        : [3.000000]  (should be <= 3)
mult_g      : [0.222222]  (constraint multiplier)
mult_x_L    : [0.000000, 0.000000, 0.000000]  (bound multipliers)
Test        : PASSED
cc examples/c_example_with_options.c -I. -Ltarget/release -lripopt \
        -Wl,-rpath,/home/orazio/Projects/ripopt/target/release -o target/release/c_example_with_options -lm
target/release/c_example_with_options
ripopt C API version 0.8.0

=== HS071 with default options ===
Options: tol=1e-08, max_iter=3000, mu_strategy=adaptive
Status:  0 (SOLVE_SUCCEEDED)
Obj:     17.0140171529
x:       [1.000000, 4.743000, 3.821150, 1.379408]
g(x):    [25.000000, 40.000000]
  g[0] = x1*x2*x3*x4 = 25.000000  (>= 25)
  g[1] = sum(xi^2)   = 40.000000  (== 40)
Constraint multipliers (lambda):
  mult_g = [-5.522937e-01, 1.614686e-01]
Bound multipliers:
  z_L = [1.087871e+00, 1.702305e-09, 2.258563e-09, 1.679360e-08]
  z_U = [1.592931e-09, 2.479301e-08, 5.405017e-09, 1.759862e-09]
Active bounds:
  x[0] = 1.0000 at LOWER bound (z_L=1.0879e+00)
  x[1] = 4.7430 (free, z_L=1.7e-09, z_U=2.5e-08)
  x[2] = 3.8211 (free, z_L=2.3e-09, z_U=5.4e-09)
  x[3] = 1.3794 (free, z_L=1.7e-08, z_U=1.8e-09)

=== HS071 with tight tolerance ===
Options: tol=1e-10, max_iter=3000, mu_strategy=adaptive
Status:  0 (SOLVE_SUCCEEDED)
Obj:     17.0140171403
x:       [1.000000, 4.743000, 3.821150, 1.379408]
g(x):    [25.000000, 40.000000]
  g[0] = x1*x2*x3*x4 = 25.000000  (>= 25)
  g[1] = sum(xi^2)   = 40.000000  (== 40)
Constraint multipliers (lambda):
  mult_g = [-5.522937e-01, 1.614686e-01]
Bound multipliers:
  z_L = [1.087871e+00, 1.633875e-11, 2.167767e-11, 1.611877e-10]
  z_U = [1.528899e-11, 2.379606e-10, 5.187763e-11, 1.689115e-11]
Active bounds:
  x[0] = 1.0000 at LOWER bound (z_L=1.0879e+00)
  x[1] = 4.7430 (free, z_L=1.6e-11, z_U=2.4e-10)
  x[2] = 3.8211 (free, z_L=2.2e-11, z_U=5.2e-11)
  x[3] = 1.3794 (free, z_L=1.6e-10, z_U=1.7e-11)

=== HS071 with monotone mu ===
Options: tol=1e-08, max_iter=3000, mu_strategy=monotone
Status:  0 (SOLVE_SUCCEEDED)
Obj:     17.0140171452
x:       [1.000000, 4.743000, 3.821150, 1.379408]
g(x):    [25.000000, 40.000000]
  g[0] = x1*x2*x3*x4 = 25.000000  (>= 25)
  g[1] = sum(xi^2)   = 40.000000  (== 40)
Constraint multipliers (lambda):
  mult_g = [-5.522937e-01, 1.614686e-01]
Bound multipliers:
  z_L = [1.087871e+00, 6.693166e-10, 8.887657e-10, 6.570873e-09]
  z_U = [6.262653e-10, 9.788835e-09, 2.122849e-09, 6.925198e-10]
Active bounds:
  x[0] = 1.0000 at LOWER bound (z_L=1.0879e+00)
  x[1] = 4.7430 (free, z_L=6.7e-10, z_U=9.8e-09)
  x[2] = 3.8211 (free, z_L=8.9e-10, z_U=2.1e-09)
  x[3] = 1.3794 (free, z_L=6.6e-09, z_U=6.9e-10)

=== Overall: ALL PASSED ===
=== All C API examples passed ===
```
