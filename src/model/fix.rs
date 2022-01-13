use rust_decimal::Decimal;

pub trait FixModelSimTrait<const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize> {
  fn new() -> Self;
  fn init(&self) -> (f64, [f64; LEN_Y]);
  fn ode(&self, t: &f64, y: &[f64; LEN_Y], deriv_y: &mut [f64; LEN_Y]);
  fn rec(&self, t: &f64, y: &[f64; LEN_Y], delta_y: &mut [f64; LEN_Y], act: &[bool; LEN_B]);
  fn cond(
    &self,
    dec_t: &Decimal,
    act: &mut [bool; LEN_B],
    next_t: &[Decimal; LEN_B],
    y: &[f64; LEN_Y],
  );
  fn beat(&self, t: &f64, y: &[f64; LEN_Y]) -> [[Decimal; 3]; LEN_B];
  fn cre(&self, t: &f64, y: &mut [f64; LEN_Y]);

  fn getp(&self) -> &[f64; LEN_P] {
    unimplemented!(
      "\nplease implement setp function in OptModelTrait:\n
fn setp(&mut self, index: usize, value: f64) {{
  self.p[index] = value;
}}\n
"
    );
  }
}

pub trait FixModelOptTrait<
  const LEN_Y: usize,
  const LEN_P: usize,
  const LEN_B: usize,
  const LEN_X: usize,
>: FixModelSimTrait<LEN_Y, LEN_P, LEN_B> + Clone + Send + 'static
{
  // getx(&self) -> (x_index, x_bounds) {}
  // x_index indicates which parameter will be optimized and it is used when optimizer renews parameter values.
  // x_bounds are for generating initial value cluster of x in methods not using initial values.
  // In methods using initial values will use the values in p as initial values instead of the bounds.
  fn getx(&self) -> (Vec<usize>, Option<Vec<(f64, f64)>>);

  // setx(&self, index: usize, value: f64) {}
  // set a value to p[index] in a model.
  fn setp(&mut self, _index: usize, _value: f64) {
    unimplemented!(
      "\nplease implement setp function in OptModelTrait:\n
fn setp(&mut self, index: usize, value: f64) {{
self.p[index] = value;
}}\n
"
    );
  }
}
