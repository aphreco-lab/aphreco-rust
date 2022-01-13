use aphrecors::prelude::*;

fn main() {
  let _model = Model::new();
}

#[allow(dead_code)]
pub struct Model {}

const LEN_Y: usize = 0;
const LEN_P: usize = 0;
const LEN_B: usize = 0;

impl FixModelSimTrait<LEN_Y, LEN_P, LEN_B> for Model {
  fn new() -> Self {
    Self {}
  }
}
