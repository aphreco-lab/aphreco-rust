pub trait RecModelSimTrait<const LEN_Y: usize, const LEN_P: usize> {
  fn new() -> Self;
  fn init(&self) -> (f64, [f64; LEN_Y]);
  fn cre(&self, t: &f64, y: &mut [f64; LEN_Y]);
}

pub trait RecModelOptTrait<const LEN_Y: usize, const LEN_P: usize, const LEN_X: usize> {
  fn getx(&self);
}
