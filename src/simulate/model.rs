use rust_decimal::Decimal;

pub trait FixSimModelTrait<const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize> {
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
  fn beats(&self, t: &f64, y: &[f64; LEN_Y]) -> [(Decimal, Decimal, Decimal, bool); LEN_B];
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
