pub enum Optimizer {
  GeneticAlgorithm,
  NelderMead,
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
      Optimizer::NelderMead => NelderMead::new(LEN_X).run(objective),
      Optimizer::GeneticAlgorithm => GeneticAlgorithm::new(LEN_X).run(objective),
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
