use super::genetic_algorithm::GeneticAlgorithm;
use super::neldermead::NelderMead;
use crate::model::OptModelTrait;
use crate::objective::Objective;
use crate::optimizer::OptResult;

pub enum OptOptions {
  Default,

  NelderMead {
    max_iter: u64,
    adaptive: bool,
    verbose: bool,
  },

  GeneticAlgorithm {
    max_gen: u64,
    n_pop: usize,
    mutation_rate: f64,
    verbose: bool,
  },
}

pub enum Optimizer {
  NelderMead(OptOptions),
  GeneticAlgorithm(OptOptions),
}

impl Optimizer {
  pub fn run<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize, const LEN_X: usize>(
    &self,
    objective: &mut Objective<M, LEN_Y, LEN_P, LEN_B, LEN_X>,
  ) -> OptResult
  where
    M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>,
  {
    match self {
      Optimizer::NelderMead(options) => {
        let opt = NelderMead::new(objective.len_x, options);
        opt.run(objective)
      }

      Optimizer::GeneticAlgorithm(options) => {
        let opt = GeneticAlgorithm::new(objective.len_x, options);
        opt.run(objective)
      }
    }
  }
}

pub trait ConcreteOptimizer {
  fn new(len_x: usize, options: &OptOptions) -> Self;

  fn run<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize, const LEN_X: usize>(
    &self,
    objective: &mut Objective<M, LEN_Y, LEN_P, LEN_B, LEN_X>,
  ) -> OptResult
  where
    M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>;
}
