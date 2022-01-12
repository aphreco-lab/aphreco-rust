use aphrecors::prelude::*;

fn main() {
  let _model = Model::new();
}

#[allow(dead_code)]
pub struct Model {}

impl SimModelTrait for Model {
  fn new() -> Self {
    Self {}
  }
}
