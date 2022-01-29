use super::base::{ConcreteOptimizer, OptOptions};
use super::result::OptResult;

use crate::model::OptModelTrait;
use crate::objective::Objective;

use ndarray::Array1;

type Vertex = (f64, Array1<f64>);
type Simplex = Vec<Vertex>;

#[allow(dead_code)]
pub struct NelderMead {
  max_iter: u64,
  rho: f64,
  chi: f64,
  psi: f64,
  sigma: f64,
  nonzero_delta: f64,
  zero_delta: f64,
  len_x: usize,
  x_abstol: f64,
  f_abstol: f64,
  verbose: bool,
}

impl ConcreteOptimizer for NelderMead {
  fn new(len_x: usize, options: &OptOptions) -> Self {
    let (max_iter, adaptive, x_abstol, f_abstol, verbose) = match options {
      OptOptions::Default => (
        0,     // max_iter
        true,  // adaptive
        1e-4,  // x_abstol
        1e-4,  // f_abstol
        false, // verbose
      ),

      OptOptions::NelderMead {
        max_iter,
        adaptive,
        x_abstol,
        f_abstol,
        verbose,
      } => (*max_iter, *adaptive, *x_abstol, *f_abstol, *verbose),

      _ => panic!("Invalid OptOptions variant."),
    };

    let rho;
    let chi;
    let psi;
    let sigma;

    if adaptive {
      rho = 1.0;
      chi = 1.0 + 2.0 / (len_x as f64);
      psi = 0.75 - 1.0 / (2.0 * len_x as f64);
      sigma = 1.0 - 1.0 / (len_x as f64);
    } else {
      rho = 1.0;
      chi = 2.0;
      psi = 0.5;
      sigma = 0.5;
    }

    let nonzero_delta = 0.05;
    let zero_delta = 0.00025;

    // set max_iter
    let max_iter = if max_iter == 0 {
      (len_x * 200) as u64
    } else {
      max_iter
    };

    Self {
      max_iter,
      rho,
      chi,
      psi,
      sigma,
      nonzero_delta,
      zero_delta,
      len_x,
      x_abstol,
      f_abstol,
      verbose,
    }
  }

  fn run<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize, const LEN_X: usize>(
    &self,
    objective: &mut Objective<M, LEN_Y, LEN_P, LEN_B, LEN_X>,
  ) -> OptResult
  where
    M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>,
  {
    let mut simplex: Simplex = Vec::new();
    let mut fcall: u64 = 0;
    let mut str_proc = "--";

    // make initial simplex
    let x_initial = self.make_initial_x(objective);
    simplex.push((objective.obj(&x_initial), x_initial.clone()));
    fcall += 1;

    for k in 0..self.len_x {
      let mut x = x_initial.clone();
      if x[k] != 0.0 {
        x[k] = (1.0 + self.nonzero_delta) * x[k];
      } else {
        x[k] = self.zero_delta;
      }

      simplex.push((objective.obj(&x), x));
      fcall += 1;
    }

    if self.verbose {
      println!(
        "   {}:   f:{:.4e}    x{:10.8}",
        str_proc, &simplex[0].0, &simplex[0].1
      );
    }

    // optimization
    for _ in 0..self.max_iter {
      // sort simplex in ascending order
      simplex.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
      if self.verbose {
        println!(
          "   {}:   f:{:.4e}    x{:10.8}",
          str_proc, &simplex[0].0, &simplex[0].1
        );
      }
      let f_best = simplex[0].0;

      // judge convergence
      if self.is_converged(&simplex) {
        println!("Converged. fcall={}", fcall);
        break;
      }

      // centroid
      let x_centroid = self.centroid(&simplex);

      // reflect
      let (f_reflect, x_reflect) = self.reflect(&x_centroid, &simplex[self.len_x].1, objective);
      fcall += 1;
      if f_best <= f_reflect && f_reflect < simplex[self.len_x - 1].0 {
        simplex[self.len_x] = (f_reflect, x_reflect);
        str_proc = "Re";
        continue;
      }

      // expand
      if f_reflect < f_best {
        let (f_expand, x_expand) = self.expand(&x_centroid, &simplex[self.len_x].1, objective);
        fcall += 1;
        if f_expand < f_reflect {
          simplex[self.len_x] = (f_expand, x_expand);
          str_proc = "Ex";
          continue;
        } else {
          simplex[self.len_x] = (f_reflect, x_reflect);
          str_proc = "Rx";
          continue;
        }
      }

      // outside contraction
      if simplex[self.len_x - 1].0 <= f_reflect && f_reflect < simplex[self.len_x].0 {
        let (f_outside, x_outside) = self.outside(&x_centroid, &simplex[self.len_x].1, objective);
        fcall += 1;
        if f_outside <= f_reflect {
          simplex[self.len_x] = (f_outside, x_outside);
          str_proc = "Oc";
          continue;
        } else {
          simplex[self.len_x] = (f_reflect, x_reflect);
          str_proc = "Ro";
          continue;
        }
      }

      // inside contraction
      if simplex[self.len_x].0 <= f_reflect {
        let (f_inside, x_inside) = self.inside(&x_centroid, &simplex[self.len_x].1, objective);
        fcall += 1;
        if f_inside < simplex[self.len_x].0 {
          simplex[self.len_x] = (f_inside, x_inside);
          str_proc = "Ic";
          continue;
        }
      }

      // shrink
      self.shrink(&mut simplex, objective);
      fcall += self.len_x as u64;
      str_proc = "Sh";
    }

    // last sort
    simplex.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    OptResult::new(
      simplex[0].1.clone(),
      objective.x_index.clone(),
      simplex[0].0,
    )
  }
}

impl NelderMead {
  fn make_initial_x<
    M,
    const LEN_Y: usize,
    const LEN_P: usize,
    const LEN_B: usize,
    const LEN_X: usize,
  >(
    &self,
    objective: &Objective<M, LEN_Y, LEN_P, LEN_B, LEN_X>,
  ) -> Array1<f64>
  where
    M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>,
  {
    let mut ini_x: Array1<f64> = Array1::zeros(objective.len_x);

    for (i, &x_index) in objective.x_index.iter().enumerate() {
      let p = objective.simulator.model.getp();
      ini_x[i] = p[x_index];
    }

    ini_x
  }

  fn centroid(&self, simplex: &Simplex) -> Array1<f64> {
    let mut sum_x = simplex[0].1.clone();
    for i in 1..self.len_x {
      sum_x += &simplex[i].1;
    }
    &sum_x / (self.len_x as f64)
  }

  fn reflect<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize, const LEN_X: usize>(
    &self,
    x_centroid: &Array1<f64>,
    x_worst: &Array1<f64>,
    objective: &mut Objective<M, LEN_Y, LEN_P, LEN_B, LEN_X>,
  ) -> Vertex
  where
    M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>,
  {
    let x_reflect = self.rho * (x_centroid - x_worst) + x_centroid;
    let f_reflect = objective.obj(&x_reflect);
    (f_reflect, x_reflect)
  }

  fn expand<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize, const LEN_X: usize>(
    &self,
    x_centroid: &Array1<f64>,
    x_worst: &Array1<f64>,
    objective: &mut Objective<M, LEN_Y, LEN_P, LEN_B, LEN_X>,
  ) -> Vertex
  where
    M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>,
  {
    let x_expand = self.rho * self.chi * (x_centroid - x_worst);
    let f_expand = objective.obj(&x_expand);
    (f_expand, x_expand)
  }

  fn outside<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize, const LEN_X: usize>(
    &self,
    x_centroid: &Array1<f64>,
    x_worst: &Array1<f64>,
    objective: &mut Objective<M, LEN_Y, LEN_P, LEN_B, LEN_X>,
  ) -> Vertex
  where
    M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>,
  {
    let x_outside = self.psi * self.rho * (x_centroid - x_worst) + x_centroid;
    let f_outside = objective.obj(&x_outside);
    (f_outside, x_outside)
  }

  fn inside<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize, const LEN_X: usize>(
    &self,
    x_centroid: &Array1<f64>,
    x_worst: &Array1<f64>,
    objective: &mut Objective<M, LEN_Y, LEN_P, LEN_B, LEN_X>,
  ) -> Vertex
  where
    M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>,
  {
    let x_inside = self.psi * (x_worst - x_centroid);
    let f_inside = objective.obj(&x_inside);
    (f_inside, x_inside)
  }

  fn shrink<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize, const LEN_X: usize>(
    &self,
    simplex: &mut Simplex,
    objective: &mut Objective<M, LEN_Y, LEN_P, LEN_B, LEN_X>,
  ) where
    M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>,
  {
    for i in 1..self.len_x + 1 {
      let x_shrink = &simplex[0].1 + &(self.sigma * (&simplex[i].1 - &simplex[0].1));
      let f_shrink = objective.obj(&x_shrink);
      simplex[i] = (f_shrink, x_shrink);
    }
  }

  fn is_converged(&self, simplex: &Simplex) -> bool {
    let mut max_f_dif = 0.0;
    let mut max_x_dif = 0.0;

    // calculate a maximum value of the absolute differences from the best.
    let best_f = &simplex[0].0;
    let best_x = &simplex[0].1;
    for (not_best_f, not_best_x) in simplex.iter().skip(1) {
      let f_dif = (best_f - not_best_f).abs();
      let x_dif = *&(best_x - not_best_x)
        .mapv(f64::abs)
        .iter()
        .fold(0.0, |a, b| b.max(a));

      if f_dif > max_f_dif {
        max_f_dif = f_dif;
      }
      if x_dif > max_x_dif {
        max_x_dif = x_dif;
      }
    }

    // converged or not
    if max_x_dif <= self.x_abstol && max_f_dif <= self.f_abstol {
      true
    } else {
      false
    }
  }
}
