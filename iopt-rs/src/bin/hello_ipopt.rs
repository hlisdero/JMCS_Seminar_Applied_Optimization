use approx::assert_relative_eq;
use ipopt::{BasicProblem, Number, Ipopt, SolveStatus};

struct NLP {
}

impl BasicProblem for NLP {
    // There are two independent variables: x and y.
    fn num_variables(&self) -> usize {
        2
    }    
    // The variables are unbounded. Any lower bound lower than -10^9 and upper bound higher
    // than 10^9 is treated effectively as infinity. These absolute infinity limits can be
    // changed via the `nlp_lower_bound_inf` and `nlp_upper_bound_inf` Ipopt options.
    fn bounds(&self, x_l: &mut [Number], x_u: &mut [Number]) -> bool {
        x_l.swap_with_slice(vec![-1e20; 2].as_mut_slice());
        x_u.swap_with_slice(vec![1e20; 2].as_mut_slice());
        true
    }

    // Set the initial conditions for the solver.
    fn initial_point(&self, x: &mut [Number]) -> bool {
        x.swap_with_slice(vec![0.0, 0.0].as_mut_slice());
        true
    }

    // The objective to be minimized.
    fn objective(&self, x: &[Number], _new_x: bool, obj: &mut Number) -> bool {
        *obj = (x[0] - 1.0)*(x[0] - 1.0) + (x[1] - 1.0)*(x[1] - 1.0);
        true
    }

    // Objective gradient is used to find a new search direction to find the critical point.
    fn objective_grad(&self, x: &[Number], _new_x: bool, grad_f: &mut [Number]) -> bool {
        grad_f[0] = 2.0*(x[0] - 1.0);
        grad_f[1] = 2.0*(x[1] - 1.0);
        true
    }
}

fn main() {
    let nlp = NLP { };
    let mut ipopt = Ipopt::new_unconstrained(nlp).unwrap();

    // Set Ipopt specific options here a list of all options is available at
    // https://www.coin-or.org/Ipopt/documentation/node40.html
    ipopt.set_option("tol", 1e-9); // set error tolerance
    ipopt.set_option("print_level", 5); // set the print level (5 is the default)

    let solve_result = ipopt.solve();

    assert_eq!(solve_result.status, SolveStatus::SolveSucceeded);
    assert_relative_eq!(solve_result.objective_value, 0.0, epsilon = 1e-10);
    let solution = solve_result.solver_data.solution;
    assert_relative_eq!(solution.primal_variables[0], 1.0, epsilon = 1e-10);
    assert_relative_eq!(solution.primal_variables[1], 1.0, epsilon = 1e-10);
}