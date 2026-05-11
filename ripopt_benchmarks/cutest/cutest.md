# benchmark-name

## Command

```bash
make cutest
```

## Output

(partial)

```bash
      5.577999282016008e-6,
      0.13051001843681195,
      0.00016652885902267088,
      0.000030132258606539617
    ],
    "constraint_violation": 0.0,
    "iterations": 10,
    "solve_time": 0.005617241,
    "final_primal_inf": 0.0,
    "final_dual_inf": 2.93515936692684e-13,
    "final_dual_inf_scaled": 2.93515936692684e-13,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "VESUVIOLS",
    "solver": "ipopt",
    "n": 8,
    "m": 0,
    "status": "Optimal",
    "objective": 991.4100028650448,
    "x": [
      0.0017497256830076627,
      -3.50272326140283,
      0.11392104043512072,
      0.00037332597705256145,
      5.577999282016008e-6,
      0.13051001843681195,
      0.00016652885902267088,
      0.000030132258606539617
    ],
    "constraint_violation": 0.0,
    "iterations": 10,
    "solve_time": 0.007462556
  },
  {
    "name": "VESUVIOU",
    "solver": "ripopt",
    "n": 8,
    "m": 1025,
    "status": "MaxTimeExceeded",
    "objective": 0.0,
    "x": [
      0.00172735532364122,
      -2.450948290108935,
      0.10267077167354022,
      0.00037381223765888775,
      -6.105219321875387e-6,
      0.1550441448468259,
      0.00016744333151843226,
      0.000028387434358691963
    ],
    "constraint_violation": 0.07609192183933296,
    "iterations": 7,
    "solve_time": 37.501752294,
    "final_primal_inf": 0.07609192183933296,
    "final_dual_inf": 3021883702.7445073,
    "final_dual_inf_scaled": 3021883702.7445073,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "VESUVIOU",
    "solver": "ipopt",
    "n": 8,
    "m": 1025,
    "status": "IpoptStatus(-10)",
    "objective": 0.0,
    "x": [
      0.0,
      0.0,
      0.01,
      0.00037,
      0.00001,
      0.0979798,
      0.000167,
      0.00001
    ],
    "constraint_violation": 0.13224011936635657,
    "iterations": 0,
    "solve_time": 0.000414425
  },
  {
    "name": "VESUVIOULS",
    "solver": "ripopt",
    "n": 8,
    "m": 0,
    "status": "Optimal",
    "objective": 0.4771137689335303,
    "x": [
      0.0006800850198856823,
      -0.6584397431519465,
      0.11392631887755472,
      0.0003733353119661844,
      -5.585096677508359e-6,
      0.13091022865220414,
      0.00016645753575675302,
      0.000030316094774538697
    ],
    "constraint_violation": 0.0,
    "iterations": 8,
    "solve_time": 0.004556624,
    "final_primal_inf": 0.0,
    "final_dual_inf": 1.508816916531929e-13,
    "final_dual_inf_scaled": 1.508816916531929e-13,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "VESUVIOULS",
    "solver": "ipopt",
    "n": 8,
    "m": 0,
    "status": "Optimal",
    "objective": 0.4771137689335303,
    "x": [
      0.0006800850198856852,
      -0.6584397431519545,
      0.11392631887755472,
      0.0003733353119661844,
      -5.585096677508359e-6,
      0.13091022865220414,
      0.00016645753575675302,
      0.000030316094774538697
    ],
    "constraint_violation": 0.0,
    "iterations": 8,
    "solve_time": 0.005718716
  },
  {
    "name": "VIBRBEAM",
    "solver": "ripopt",
    "n": 8,
    "m": 0,
    "status": "Optimal",
    "objective": 0.3322376046050448,
    "x": [
      -4.081487513384368,
      0.8219290749866638,
      -0.03768147409011422,
      0.0004706805997463164,
      1.7862710155962125,
      0.011851579922587226,
      -0.0016836555129235086,
      0.00004047292330590413
    ],
    "constraint_violation": 0.0,
    "iterations": 58,
    "solve_time": 0.002313399,
    "final_primal_inf": 0.0,
    "final_dual_inf": 1.1209718056260704e-12,
    "final_dual_inf_scaled": 1.1209718056260704e-12,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "VIBRBEAM",
    "solver": "ipopt",
    "n": 8,
    "m": 0,
    "status": "Optimal",
    "objective": 0.33223760460504853,
    "x": [
      -4.081487513384386,
      0.8219290749866668,
      -0.03768147409011436,
      0.00047068059974631794,
      1.7862710155962116,
      0.01185157992258786,
      -0.001683655512923555,
      0.00004047292330590479
    ],
    "constraint_violation": 0.0,
    "iterations": 58,
    "solve_time": 0.010581397
  },
  {
    "name": "VIBRBEAMNE",
    "solver": "ripopt",
    "n": 8,
    "m": 30,
    "status": "RestorationFailed",
    "objective": 0.0,
    "x": [
      -4.164948321675595,
      0.8254060151005156,
      -0.03744217048884973,
      0.0004646843417448797,
      1.8775329636752476,
      -0.0429844675460497,
      0.002721603675005412,
      -0.00003783216168123313
    ],
    "constraint_violation": 0.16446895115131,
    "iterations": 20,
    "solve_time": 0.081850209,
    "final_primal_inf": 0.0008967178655366007,
    "final_dual_inf": 0.0,
    "final_dual_inf_scaled": 0.0,
    "final_compl": 0.0,
    "final_mu": 20.871073829658854
  },
  {
    "name": "VIBRBEAMNE",
    "solver": "ipopt",
    "n": 8,
    "m": 30,
    "status": "IpoptStatus(-10)",
    "objective": 0.0,
    "x": [
      -3.5,
      1.0,
      0.0,
      0.0,
      1.7,
      0.0,
      0.0,
      0.0
    ],
    "constraint_violation": 29.45825745470384,
    "iterations": 0,
    "solve_time": 0.000146494
  },
  {
    "name": "WACHBIEG",
    "solver": "ripopt",
    "n": 3,
    "m": 2,
    "status": "RestorationFailed",
    "objective": -0.9999999951667344,
    "x": [
      -0.9999999951667344,
      -8.999593726703689e-9,
      -9.49979681872902e-9
    ],
    "constraint_violation": 1.4999999856669377,
    "iterations": 14,
    "solve_time": 0.00186327,
    "final_primal_inf": 1.4999999856669377,
    "final_dual_inf": 1.0,
    "final_dual_inf_scaled": 1.0,
    "final_compl": 1.0004062732963112e-9,
    "final_mu": 6.002437818269164e-7
  },
  {
    "name": "WACHBIEG",
    "solver": "ipopt",
    "n": 3,
    "m": 2,
    "status": "Infeasible",
    "objective": -0.9999999950003332,
    "x": [
      -0.9999999950003332,
      -9.998000000015344e-9,
      -9.999e-9
    ],
    "constraint_violation": 1.4999999850013332,
    "iterations": 15,
    "solve_time": 0.006514976
  },
  {
    "name": "WATER",
    "solver": "ripopt",
    "n": 31,
    "m": 10,
    "status": "Optimal",
    "objective": 10549.379465195289,
    "x": [
      864.5678998564149,
      180.01660692985035,
      584.5512928679321,
      80.01660693234618,
      28.98659022628884,
      435.5647026313508,
      281.9919524207775,
      81.9919524218445,
      255.4321001338421,
      3.913615454508987e-8,
      -9.843855775527071e-9,
      -9.524134035114692e-9,
      -9.916714271610505e-9,
      -9.760579815236456e-9,
      -9.837290321826112e-9,
      -9.9307899984368e-9,
      -7.364620185255771e-9,
      -9.863555882276624e-9,
      79.00485038005289,
      176.42724977963977,
      -8.435012073195225e-9,
      -9.742570228062905e-9,
      -9.839026015920742e-9,
      -9.468558805965978e-9,
      -9.821986363948161e-9,
      -9.899477841635913e-9,
      -9.521739919243433e-9,
      -9.829847411680014e-9,
      -9.494966088069424e-9,
      -9.74257016469808e-9,
      255.43210012409912
    ],
    "constraint_violation": 3.8495095268851886e-12,
    "iterations": 16,
    "solve_time": 0.003250823,
    "final_primal_inf": 3.8495095268851886e-12,
    "final_dual_inf": 1.9078920191568203e-11,
    "final_dual_inf_scaled": 1.9078920191568203e-11,
    "final_compl": 1.0984231785372975e-9,
    "final_mu": 9.090909090909091e-10
  },
  {
    "name": "WATER",
    "solver": "ipopt",
    "n": 31,
    "m": 10,
    "status": "Optimal",
    "objective": 10549.379465197331,
    "x": [
      864.5678998564032,
      180.0166069298525,
      584.5512928679557,
      80.01660693234588,
      28.98659022628877,
      435.5647026313443,
      281.99195242078014,
      81.99195242184435,
      255.43210013385507,
      3.909999477617454e-8,
      -9.840211946473366e-9,
      -9.492848031554471e-9,
      -9.915713346926865e-9,
      -9.751947410965354e-9,
      -9.833327376176677e-9,
      -9.930789998437822e-9,
      -7.364620189780831e-9,
      -9.863555882278237e-9,
      79.0048503800532,
      176.42724977965634,
      -8.4350120733958e-9,
      -9.74257022804732e-9,
      -9.8390260159095e-9,
      -9.468558806003009e-9,
      -9.821986363966784e-9,
      -9.899477841636144e-9,
      -9.521739919257415e-9,
      -9.829847411680538e-9,
      -9.494966088057324e-9,
      -9.741034917353196e-9,
      255.4321001241148
    ],
    "constraint_violation": 6.981581431014929e-12,
    "iterations": 17,
    "solve_time": 0.004296072
  },
  {
    "name": "WAYSEA1",
    "solver": "ripopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 2.694997714910621e-15,
    "x": [
      0.9999999738865948,
      2.000000005610164
    ],
    "constraint_violation": 0.0,
    "iterations": 14,
    "solve_time": 0.000068213,
    "final_primal_inf": 0.0,
    "final_dual_inf": 2.247655113727608e-10,
    "final_dual_inf_scaled": 2.247655113727608e-10,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "WAYSEA1",
    "solver": "ipopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 2.694997714910621e-15,
    "x": [
      0.9999999738865948,
      2.000000005610164
    ],
    "constraint_violation": 0.0,
    "iterations": 14,
    "solve_time": 0.002014553
  },
  {
    "name": "WAYSEA1B",
    "solver": "ripopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 1.8023696967876355e-13,
    "x": [
      0.9999997701576612,
      2.0000000409148817
    ],
    "constraint_violation": 0.0,
    "iterations": 14,
    "solve_time": 0.00013031,
    "final_primal_inf": 0.0,
    "final_dual_inf": 8.160397331620669e-11,
    "final_dual_inf_scaled": 8.160397331620669e-11,
    "final_compl": 5.0001575571315665e-9,
    "final_mu": 5.0001575559317054e-9
  },
  {
    "name": "WAYSEA1B",
    "solver": "ipopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 4.542681561752429e-16,
    "x": [
      0.999999989015249,
      2.0000000023174587
    ],
    "constraint_violation": 0.0,
    "iterations": 14,
    "solve_time": 0.002724055
  },
  {
    "name": "WAYSEA1NE",
    "solver": "ripopt",
    "n": 2,
    "m": 2,
    "status": "Optimal",
    "objective": 0.0,
    "x": [
      0.999999999532062,
      2.0000000009358767
    ],
    "constraint_violation": 2.7140425906679866e-8,
    "iterations": 7,
    "solve_time": 0.000047326,
    "final_primal_inf": 5.4280851813359745e-9,
    "final_dual_inf": 0.0,
    "final_dual_inf_scaled": 0.0,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "WAYSEA1NE",
    "solver": "ipopt",
    "n": 2,
    "m": 2,
    "status": "Optimal",
    "objective": 0.0,
    "x": [
      0.999999999532062,
      2.0000000009358767
    ],
    "constraint_violation": 2.7140425906679866e-8,
    "iterations": 7,
    "solve_time": 0.001183431
  },
  {
    "name": "WAYSEA2",
    "solver": "ripopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 9.846529917707052e-18,
    "x": [
      0.4248610409743775,
      1.0000000024247158
    ],
    "constraint_violation": 0.0,
    "iterations": 22,
    "solve_time": 0.000111438,
    "final_primal_inf": 0.0,
    "final_dual_inf": 6.067814570663623e-10,
    "final_dual_inf_scaled": 6.067814570663623e-10,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "WAYSEA2",
    "solver": "ipopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 9.846533455862597e-18,
    "x": [
      0.4248610409743777,
      1.0000000024247158
    ],
    "constraint_violation": 0.0,
    "iterations": 22,
    "solve_time": 0.003101247
  },
  {
    "name": "WAYSEA2B",
    "solver": "ripopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 9.86373822201904e-18,
    "x": [
      0.4248610409860642,
      1.0000000024263518
    ],
    "constraint_violation": 0.0,
    "iterations": 22,
    "solve_time": 0.000181995,
    "final_primal_inf": 0.0,
    "final_dual_inf": 6.075463860771093e-10,
    "final_dual_inf_scaled": 6.075463860771093e-10,
    "final_compl": 5.000000007009432e-9,
    "final_mu": 5.000000007009431e-9
  },
  {
    "name": "WAYSEA2B",
    "solver": "ipopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": 9.872775148955867e-18,
    "x": [
      0.42486104099560645,
      1.00000000242802
    ],
    "constraint_violation": 0.0,
    "iterations": 22,
    "solve_time": 0.004354196
  },
  {
    "name": "WAYSEA2NE",
    "solver": "ripopt",
    "n": 2,
    "m": 2,
    "status": "Optimal",
    "objective": 0.0,
    "x": [
      0.424861025618432,
      1.0
    ],
    "constraint_violation": 3.1210367623657476e-10,
    "iterations": 11,
    "solve_time": 0.000076256,
    "final_primal_inf": 3.1210367623657476e-10,
    "final_dual_inf": 0.0,
    "final_dual_inf_scaled": 0.0,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "WAYSEA2NE",
    "solver": "ipopt",
    "n": 2,
    "m": 2,
    "status": "Optimal",
    "objective": 0.0,
    "x": [
      0.424861025618432,
      1.0
    ],
    "constraint_violation": 3.1210367623657476e-10,
    "iterations": 11,
    "solve_time": 0.001981058
  },
  {
    "name": "WEEDS",
    "solver": "ripopt",
    "n": 3,
    "m": 0,
    "status": "MaxIterations",
    "objective": 2.5872773952841386,
    "x": [
      196.18626203108644,
      49.09163947272053,
      0.3135697297082181
    ],
    "constraint_violation": 0.0,
    "iterations": 2999,
    "solve_time": 0.057060067,
    "final_primal_inf": 0.0,
    "final_dual_inf": 0.000014022388814944954,
    "final_dual_inf_scaled": 0.000014022388814944954,
    "final_compl": 4.000812279558871e-9,
    "final_mu": 4.000812275194674e-9
  },
  {
    "name": "WEEDS",
    "solver": "ipopt",
    "n": 3,
    "m": 0,
    "status": "Optimal",
    "objective": 2.587277395284193,
    "x": [
      196.18626177481735,
      49.0916394570863,
      0.31356972993429266
    ],
    "constraint_violation": 0.0,
    "iterations": 28,
    "solve_time": 0.007320499
  },
  {
    "name": "WEEDSNE",
    "solver": "ripopt",
    "n": 3,
    "m": 12,
    "status": "RestorationFailed",
    "objective": 0.0,
    "x": [
      196.25222261772285,
      49.09854498857045,
      0.31353228735343946
    ],
    "constraint_violation": 1.105859660412687,
    "iterations": 355,
    "solve_time": 0.067835419,
    "final_primal_inf": 1.105859660412687,
    "final_dual_inf": 49309726.018066406,
    "final_dual_inf_scaled": 49309726.018066406,
    "final_compl": 1.0000009740966408e-11,
    "final_mu": 1e-8
  },
  {
    "name": "WEEDSNE",
    "solver": "ipopt",
    "n": 3,
    "m": 12,
    "status": "IpoptStatus(-10)",
    "objective": 0.0,
    "x": [
      1.0,
      1.0,
      1.0
    ],
    "constraint_violation": 90.9720061441746,
    "iterations": 0,
    "solve_time": 0.000204437
  },
  {
    "name": "WOMFLET",
    "solver": "ripopt",
    "n": 3,
    "m": 3,
    "status": "Optimal",
    "objective": -7.272776322770975e-9,
    "x": [
      -1.7534008214478695e-11,
      3.4603105818917164e-8,
      -7.272776322770975e-9
    ],
    "constraint_violation": 8.158242527317084e-9,
    "iterations": 37,
    "solve_time": 0.000525338,
    "final_primal_inf": 0.0,
    "final_dual_inf": 3.5616679008734566e-10,
    "final_dual_inf_scaled": 3.5616679008734566e-10,
    "final_compl": 9.34366893789883e-10,
    "final_mu": 9.090909090909091e-10
  },
  {
    "name": "WOMFLET",
    "solver": "ipopt",
    "n": 3,
    "m": 3,
    "status": "Optimal",
    "objective": 6.049999999078937,
    "x": [
      -1.1000000115450983,
      1.3994485269384076e-14,
      6.049999999078937
    ],
    "constraint_violation": 9.116702059941416e-9,
    "iterations": 8,
    "solve_time": 0.001848505
  },
  {
    "name": "YFIT",
    "solver": "ripopt",
    "n": 3,
    "m": 0,
    "status": "Optimal",
    "objective": 6.860001974905794e-13,
    "x": [
      0.1499999809769365,
      -0.2499999678869094,
      140.0000187092698
    ],
    "constraint_violation": 0.0,
    "iterations": 36,
    "solve_time": 0.000410924,
    "final_primal_inf": 0.0,
    "final_dual_inf": 2.6725337345796134e-9,
    "final_dual_inf_scaled": 2.6725337345796134e-9,
    "final_compl": 5.0000000018929285e-9,
    "final_mu": 5.00000000190983e-9
  },
  {
    "name": "YFIT",
    "solver": "ipopt",
    "n": 3,
    "m": 0,
    "status": "Optimal",
    "objective": 6.669721919626666e-13,
    "x": [
      0.1500000019944904,
      -0.2500000027458305,
      139.99999868942055
    ],
    "constraint_violation": 0.0,
    "iterations": 36,
    "solve_time": 0.007938674
  },
  {
    "name": "YFITNE",
    "solver": "ripopt",
    "n": 3,
    "m": 17,
    "status": "MaxIterations",
    "objective": 0.0,
    "x": [
      0.1500000020213069,
      -0.2500000027902992,
      139.9999986644895
    ],
    "constraint_violation": 4.463694089906767e-7,
    "iterations": 2999,
    "solve_time": 0.480710746,
    "final_primal_inf": 4.463694089906767e-7,
    "final_dual_inf": 0.003959357738494873,
    "final_dual_inf_scaled": 0.003959357738494873,
    "final_compl": 0.0,
    "final_mu": 1e-11
  },
  {
    "name": "YFITNE",
    "solver": "ipopt",
    "n": 3,
    "m": 17,
    "status": "IpoptStatus(-10)",
    "objective": 0.0,
    "x": [
      0.6,
      -0.6,
      20.0
    ],
    "constraint_violation": 22.065132833166153,
    "iterations": 0,
    "solve_time": 0.00012188
  },
  {
    "name": "YFITU",
    "solver": "ripopt",
    "n": 3,
    "m": 0,
    "status": "Optimal",
    "objective": 6.669720635059138e-13,
    "x": [
      0.15000000202536493,
      -0.2500000027970322,
      139.99999866043768
    ],
    "constraint_violation": 0.0,
    "iterations": 36,
    "solve_time": 0.000367571,
    "final_primal_inf": 0.0,
    "final_dual_inf": 1.0872365658427715e-9,
    "final_dual_inf_scaled": 1.0872365658427715e-9,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "YFITU",
    "solver": "ipopt",
    "n": 3,
    "m": 0,
    "status": "Optimal",
    "objective": 6.669720604442095e-13,
    "x": [
      0.15000000202536512,
      -0.2500000027970325,
      139.9999986604375
    ],
    "constraint_violation": 0.0,
    "iterations": 36,
    "solve_time": 0.006119555
  },
  {
    "name": "ZANGWIL2",
    "solver": "ripopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": -18.200000000091,
    "x": [
      4.0,
      9.0
    ],
    "constraint_violation": 0.0,
    "iterations": 1,
    "solve_time": 9.649e-6,
    "final_primal_inf": 0.0,
    "final_dual_inf": 0.0,
    "final_dual_inf_scaled": 0.0,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "ZANGWIL2",
    "solver": "ipopt",
    "n": 2,
    "m": 0,
    "status": "Optimal",
    "objective": -18.200000000091,
    "x": [
      4.0,
      9.0
    ],
    "constraint_violation": 0.0,
    "iterations": 1,
    "solve_time": 0.000336597
  },
  {
    "name": "ZANGWIL3",
    "solver": "ripopt",
    "n": 3,
    "m": 3,
    "status": "Optimal",
    "objective": 0.0,
    "x": [
      0.0,
      0.0,
      0.0
    ],
    "constraint_violation": 0.0,
    "iterations": 1,
    "solve_time": 0.000015957,
    "final_primal_inf": 0.0,
    "final_dual_inf": 0.0,
    "final_dual_inf_scaled": 0.0,
    "final_compl": 0.0,
    "final_mu": 1.0
  },
  {
    "name": "ZANGWIL3",
    "solver": "ipopt",
    "n": 3,
    "m": 3,
    "status": "Optimal",
    "objective": 0.0,
    "x": [
      0.0,
      0.0,
      0.0
    ],
    "constraint_violation": 0.0,
    "iterations": 1,
    "solve_time": 0.000327023
  },
  {
    "name": "ZECEVIC2",
    "solver": "ripopt",
    "n": 2,
    "m": 2,
    "status": "Optimal",
    "objective": -4.125000019090912,
    "x": [
      1.7500000093074255,
      0.2500000002380304
    ],
    "constraint_violation": 9.545455870174637e-9,
    "iterations": 14,
    "solve_time": 0.000246349,
    "final_primal_inf": 0.0,
    "final_dual_inf": 9.547918011776346e-15,
    "final_dual_inf_scaled": 9.547918011776346e-15,
    "final_compl": 9.091069567223559e-10,
    "final_mu": 9.090909090909091e-10
  },
  {
    "name": "ZECEVIC2",
    "solver": "ipopt",
    "n": 2,
    "m": 2,
    "status": "Optimal",
    "objective": -4.125000019990001,
    "x": [
      1.7500000099922766,
      0.25000000000272343
    ],
    "constraint_violation": 9.994999994322741e-9,
    "iterations": 8,
    "solve_time": 0.001759847
  },
  {
    "name": "ZECEVIC3",
    "solver": "ripopt",
    "n": 2,
    "m": 2,
    "status": "Optimal",
    "objective": 97.30945006679929,
    "x": [
      2.7955451546977117,
      1.0885436589380173
    ],
    "constraint_violation": 9.36801947126753e-9,
    "iterations": 12,
    "solve_time": 0.000222876,
    "final_primal_inf": 0.0,
    "final_dual_inf": 5.1514348342607263e-14,
    "final_dual_inf_scaled": 5.1514348342607263e-14,
    "final_compl": 5.070950633516679e-9,
    "final_mu": 5.0709463084196885e-9
  },
  {
    "name": "ZECEVIC3",
    "solver": "ipopt",
    "n": 2,
    "m": 2,
    "status": "Optimal",
    "objective": 97.30945006173832,
    "x": [
      2.7955451549306134,
      1.0885436586296051
    ],
    "constraint_violation": 9.998753602857846e-9,
    "iterations": 17,
    "solve_time": 0.003545624
  },
  {
    "name": "ZECEVIC4",
    "solver": "ripopt",
    "n": 2,
    "m": 2,
    "status": "Optimal",
    "objective": 7.557507761903931,
    "x": [
      4.970952880057176,
      1.2518287260721752
    ],
    "constraint_violation": 5.077433584688151e-9,
    "iterations": 10,
    "solve_time": 0.000193711,
    "final_primal_inf": 0.0,
    "final_dual_inf": 6.813493406423995e-14,
    "final_dual_inf_scaled": 6.813493406423995e-14,
    "final_compl": 6.813505998112235e-9,
    "final_mu": 6.813493406914223e-9
  },
  {
    "name": "ZECEVIC4",
    "solver": "ipopt",
    "n": 2,
    "m": 2,
    "status": "Optimal",
    "objective": 7.557507755101343,
    "x": [
      4.970952879782224,
      1.251828727327263
    ],
    "constraint_violation": 9.992087157684182e-9,
    "iterations": 10,
    "solve_time": 0.002383476
  },
  {
    "name": "ZY2",
    "solver": "ripopt",
    "n": 3,
    "m": 2,
    "status": "Optimal",
    "objective": 1.9999998941299355,
    "x": [
      -9.496062571917054e-9,
      -4.456689987187844e-9,
      2.0000000030433145
    ],
    "constraint_violation": 0.0,
    "iterations": 10,
    "solve_time": 0.00020863,
    "final_primal_inf": 0.0,
    "final_dual_inf": 5.5455640080026576e-14,
    "final_dual_inf_scaled": 5.5455640080026576e-14,
    "final_compl": 5.543314496232948e-9,
    "final_mu": 5.543312222662732e-9
  },
  {
    "name": "ZY2",
    "solver": "ipopt",
    "n": 3,
    "m": 2,
    "status": "Optimal",
    "objective": 1.999999877529895,
    "x": [
      -9.999091057861949e-9,
      -9.990010505556176e-9,
      1.999999997509908
    ],
    "constraint_violation": 9.960368529959853e-9,
    "iterations": 14,
    "solve_time": 0.003645104
  }
Results written to /home/orazio/Projects/ripopt/benchmarks/cutest/results.json
]

Summary: 727 problems
  ripopt solved: 561/727
  ipopt  solved: 561/727
python /home/orazio/Projects/ripopt/benchmarks/cutest/compare.py /home/orazio/Projects/ripopt/benchmarks/cutest/results.json
Report written to /home/orazio/Projects/ripopt/benchmarks/cutest/CUTEST_REPORT.md

Summary:
  Total: 727
  ripopt solved: 561/727
  Ipopt solved: 561/727
  Both solved: 541/727
  Matching (rel diff < 1e-4): 522/541
CUTEst benchmark complete. Report: benchmarks/cutest/CUTEST_REPORT.md
make[1]: Leaving directory '/home/orazio/Projects/ripopt/benchmarks'
```
