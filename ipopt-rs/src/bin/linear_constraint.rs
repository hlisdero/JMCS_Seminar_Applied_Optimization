use approx::assert_relative_eq;
use ipopt::{BasicProblem, ConstrainedProblem, Number, Ipopt, SolveStatus, Index};

// Minimize:
//     f(x, y) = -x^2 + y^2
//
// Subject to:
//     -2 <= x <= 4
//     y <= -1
//
// Notes:
// - Ipopt expresses general constraints in the form g_L <= g(x) <= g_U.
// - Strict inequalities like y < -1 are not supported directly, so we model it as y <= -1.
// - Here we demonstrate both styles of constraints:
//   1. x is handled as a variable bound.
//   2. y <= -1 is handled as one explicit constraint g(x, y) = y with upper bound -1.
//
// Expected optimizer solution:
//     x = 4, y = -1
//
// Objective value there:
//     f(4, -1) = -(4^2) + (-1)^2 = -16 + 1 = -15

struct Nlp {}

impl BasicProblem for Nlp {
    // Two decision variables: x and y.
    fn num_variables(&self) -> usize {
        2
    }

    // Variable bounds:
    //   -2 <= x <= 4
    //   y is left unbounded here because we enforce y <= -1 as a general constraint below.
    fn bounds(&self, x_l: &mut [Number], x_u: &mut [Number]) -> bool {
        x_l[0] = -2.0;
        x_u[0] =  4.0;

        x_l[1] = -1e20;
        x_u[1] =  1e20;
        true
    }

    // Initial guess. It is usually better to start from a feasible point for constrained problems.
    fn initial_point(&self, x: &mut [Number]) -> bool {
        x[0] = 0.0;
        x[1] = -2.0;
        true
    }

    // Objective:
    //   f(x, y) = -x^2 + y^2
    //
    // Minimizing this pushes:
    // - x toward the largest magnitude allowed on the positive side, because of -x^2,
    // - y toward the smallest magnitude feasible value, which under y <= -1 is y = -1.
    fn objective(&self, x: &[Number], _new_x: bool, obj: &mut Number) -> bool {
        *obj = -x[0] * x[0] + x[1] * x[1];
        true
    }

    // Gradient of f(x, y) = -x^2 + y^2:
    //   df/dx = -2x
    //   df/dy =  2y
    fn objective_grad(&self, x: &[Number], _new_x: bool, grad_f: &mut [Number]) -> bool {
        grad_f[0] = -2.0 * x[0];
        grad_f[1] =  2.0 * x[1];
        true
    }
}

impl ConstrainedProblem for Nlp {
    // One scalar constraint:
    //   g(x, y) = y
    // with upper bound -1, i.e. y <= -1.
    fn num_constraints(&self) -> usize {
        1
    }

    // Constraint bounds for g(x, y):
    //   -infinity <= g(x, y) <= -1
    //
    // Since g(x, y) = y, this is exactly y <= -1.
    fn constraint_bounds(&self, g_l: &mut [Number], g_u: &mut [Number]) -> bool {
        g_l[0] = -1e20;
        g_u[0] = -1.0;
        true
    }

    // Evaluate the constraint function:
    //   g(x, y) = y
    fn constraint(&self, x: &[Number], _new_x: bool, g: &mut [Number]) -> bool {
        g[0] = x[1];
        true
    }

    // Jacobian sparsity structure for the single constraint g with respect to [x, y].
    //
    // Since g(x, y) = y, only dg/dy = 1 is nonzero, so the Jacobian has one nonzero entry
    // at row 0, col 1.
    fn num_constraint_jacobian_non_zeros(&self) -> usize {
        1
    }

    fn constraint_jacobian_indices(&self, rows: &mut [Index], cols: &mut [Index]) -> bool {
        rows[0] = 0;
        cols[0] = 1;
        true
    }

    // Jacobian of g(x, y) = y:
    //   dg/dx = 0
    //   dg/dy = 1
    //
    // With the sparse structure above, we only provide the nonzero value dg/dy = 1.
    fn constraint_jacobian_values(
        &self,
        _x: &[Number],
        _new_x: bool,
        values: &mut [Number],
    ) -> bool {
        values[0] = 1.0;
        true
    }

    // Ipopt asks for the Hessian of the Lagrangian:
    //   L(x, y) = obj_factor * f(x, y) + lambda[0] * g(x, y)
    //
    // Here:
    //   f(x, y) = -x^2 + y^2
    //   g(x, y) = y
    //
    // The constraint is linear, so its Hessian is zero. Therefore the Hessian of the
    // Lagrangian is just obj_factor times the Hessian of the objective:
    //
    //   d²f/dx²   = -2
    //   d²f/dxdy  =  0
    //   d²f/dy²   =  2
    //
    // We provide the diagonal part only: (0,0), (1,1).
    fn num_hessian_non_zeros(&self) -> usize {
        2
    }

    fn hessian_indices(&self, rows: &mut [Index], cols: &mut [Index]) -> bool {
        rows[0] = 0;
        cols[0] = 0;

        rows[1] = 1;
        cols[1] = 1;

        true
    }

    fn hessian_values(
        &self,
        _x: &[Number],
        _new_x: bool,
        obj_factor: Number,
        lambda: &[Number],
        values: &mut [Number],
    ) -> bool {
        let _ = lambda;

        values[0] = obj_factor * -2.0;
        values[1] = obj_factor * 2.0;

        true
    }
}

fn main() {
    let nlp = Nlp {};
    let mut ipopt = Ipopt::new(nlp).unwrap();

    ipopt.set_option("tol", 1e-9);
    ipopt.set_option("print_level", 5);

    let solve_result = ipopt.solve();

    assert_eq!(solve_result.status, SolveStatus::SolveSucceeded);

    let solution = solve_result.solver_data.solution;

    assert_relative_eq!(solution.primal_variables[0], 4.0, epsilon = 1e-7);
    assert_relative_eq!(solution.primal_variables[1], -1.0, epsilon = 1e-7);
    assert_relative_eq!(solve_result.objective_value, -15.0, epsilon = 1e-6);
}