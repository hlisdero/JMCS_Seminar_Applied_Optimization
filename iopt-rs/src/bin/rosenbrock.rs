use approx::assert_relative_eq;
use ipopt::{BasicProblem, Ipopt, Number, SolveStatus};

// Minimize the classical Rosenbrock function:
//
//     f(x, y) = (1 - x)^2 + 100 * (y - x^2)^2
//
// Global minimizer:
//     x = 1, y = 1
//
// Objective value there:
//     f(1, 1) = 0

struct Nlp {}

impl BasicProblem for Nlp {
    fn num_variables(&self) -> usize {
        2
    }

    /// Unbounded variables.
    fn bounds(&self, x_l: &mut [Number], x_u: &mut [Number]) -> bool {
        x_l[0] = -1e20;
        x_u[0] =  1e20;

        x_l[1] = -1e20;
        x_u[1] =  1e20;

        true
    }

    /// Rosenbrock starting point arbitrarily chosen
    fn initial_point(&self, x: &mut [Number]) -> bool {
        x[0] = -1.2;
        x[1] =  1.0;
        true
    }

    // f(x, y) = (1 - x)^2 + 100 (y - x^2)^2
    fn objective(&self, x: &[Number], _new_x: bool, obj: &mut Number) -> bool {
        let x0 = x[0];
        let x1 = x[1];
        *obj = (1.0 - x0).powi(2) + 100.0 * (x1 - x0 * x0).powi(2);
        true
    }

    // Gradient:
    // df/dx = 2(x - 1) - 400x(y - x^2)
    // df/dy = 200(y - x^2)
    fn objective_grad(&self, x: &[Number], _new_x: bool, grad_f: &mut [Number]) -> bool {
        let x0 = x[0];
        let x1 = x[1];
        let t = x1 - x0 * x0;

        grad_f[0] = 2.0 * (x0 - 1.0) - 400.0 * x0 * t;
        grad_f[1] = 200.0 * t;

        true
    }
}

fn main() {
    let nlp = Nlp {};
    let mut ipopt = Ipopt::new_unconstrained(nlp).unwrap();

    ipopt.set_option("tol", 1e-9);
    ipopt.set_option("print_level", 5);
    ipopt.set_option("max_iter", 200);

    let solve_result = ipopt.solve();

    assert_eq!(solve_result.status, SolveStatus::SolveSucceeded);

    let solution = solve_result.solver_data.solution;

    assert_relative_eq!(solution.primal_variables[0], 1.0, epsilon = 1e-5);
    assert_relative_eq!(solution.primal_variables[1], 1.0, epsilon = 1e-5);
    assert_relative_eq!(solve_result.objective_value, 0.0, epsilon = 1e-8);
}