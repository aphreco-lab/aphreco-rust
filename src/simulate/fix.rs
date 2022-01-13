use super::result::SimResult;
use crate::step::Stepper;

use rust_decimal::Decimal;

pub trait SimModelTraitFix<const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize> {
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

#[derive(Clone)]
pub struct SimulatorFix<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize>
where
  M: SimModelTraitFix<LEN_Y, LEN_P, LEN_B>,
{
  pub model: M,
}

impl<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize>
  SimulatorFix<M, LEN_Y, LEN_P, LEN_B>
where
  M: SimModelTraitFix<LEN_Y, LEN_P, LEN_B>,
{
  pub fn new(model: M, stepper: Stepper) -> Self {
    Self { model }
  }

  pub fn run(&self, smp_t: &Vec<f64>) -> SimResult<LEN_Y> {
    let (ini_t, ini_y) = self.model.init();
    let beats = self.model.beats(&ini_t, &ini_y);
    // let (end_t, mut vdq_smp_t, mut dec_times) = self.initialize_times(&ini_t, smp_t, &beats);

    let mut res_y: Vec<[f64; LEN_Y]> = Vec::new();

    let mut cur_t = ini_t;
    let mut cur_y = ini_y;

    SimResult::new(smp_t.clone(), res_y)
  }
}
