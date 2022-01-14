use super::genetic_algorithm::GeneticAlgorithm;
use super::neldermead::NelderMead;
use crate::model::OptModelTrait;
use crate::objective::Objective;
use crate::optimizer::OptResult;

pub enum Optimizer {
  NelderMead,
  GeneticAlgorithm,
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
      Optimizer::NelderMead => {
        let nm = NelderMead::new(objective.len_x);
        nm.run(objective)
      }

      Optimizer::GeneticAlgorithm => {
        let nm = GeneticAlgorithm::new(objective.len_x);
        nm.run(objective)
      }
    }
  }
}

pub trait ConcreteOptimizer {
  fn new(len_x: usize) -> Self;

  fn run<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize, const LEN_X: usize>(
    &self,
    objective: &mut Objective<M, LEN_Y, LEN_P, LEN_B, LEN_X>,
  ) -> OptResult
  where
    M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>;
}
