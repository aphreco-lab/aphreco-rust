use aphrecors::prelude::*;

fn main() {
  let _model = Model::new();
}

#[allow(dead_code)]
pub struct Model {
  pub p: [f64; LEN_P],
}

const LEN_Y: usize = 4;
const LEN_P: usize = 11;
const LEN_B: usize = 2;

#[allow(dead_code)]
impl FixModelSimTrait<LEN_Y, LEN_P, LEN_B> for Model {
  fn new() -> Self {
    let p = [
      0.1,   // p[0] k12
      0.1,   // p[1] k21
      0.0,   // p[2] ini_b1
      1e12,  // p[3] end_b1
      0.1,   // p[4] tau_b1
      1.0,   // p[5] ini_b2
      1e12,  // p[6] end_b2
      0.2,   // p[7] tau_b2
      2.0,   // p[8] R_cre
      10.0,  // p[9] X_dose_A
      500.0, // p[10] MW_A
    ];
    Self { p }
  }

  fn init(&self) -> (f64, [f64; LEN_Y]) {
    let t0 = 0.0;
    let y0 = [
      100.0, // y[p] A
      0.0,   // y[1] B
      10.0,  // y[2] C
      0.0,   // y[3] D
    ];
    (t0, y0)
  }
  #[allow(unused_variables)]
  fn ode(&self, t: &f64, y: &[f64; LEN_Y], deriv_y: &mut [f64; LEN_Y]) {
    deriv_y[0] = -self.p[0] * y[0] + self.p[1] * y[1];
    deriv_y[1] = self.p[0] * y[0] - self.p[1] * y[1];
  }

  #[allow(unused_variables)]
  fn rec(&self, t: &f64, y: &[f64; LEN_Y], delta_y: &mut [f64; LEN_Y], act: &[bool; LEN_B]) {
    if act[0] {
      delta_y[2] += self.p[8];
    }
    if act[1] {
      delta_y[2] -= self.p[8];
    }
  }

  #[allow(unused_variables)]
  fn cond(
    &self,
    dec_t: &Decimal,
    act: &mut [bool; LEN_B],
    next_t: &[Decimal; LEN_B],
    y: &[f64; LEN_Y],
  ) {
    act[0] = if *dec_t == next_t[0] { true } else { false };
    act[1] = if *dec_t == next_t[1] { true } else { false };
  }

  #[allow(unused_variables)]
  fn beats(&self, t: &f64, y: &[f64; LEN_Y]) -> [(Decimal, Decimal, Decimal, bool); LEN_B] {
    [
      beat!(self.p[2], self.p[3], self.p[4], false),
      beat!(self.p[5], self.p[6], self.p[7], false),
    ]
  }

  #[allow(unused_variables)]
  fn cre(&self, t: &f64, y: &mut [f64; LEN_Y]) {
    y[3] = self.p[8] * t;
  }

  fn getp(&self) -> &[f64; LEN_P] {
    &self.p
  }
}

#[allow(dead_code)]
fn sampling_time() -> Vec<f64> {
  let mut vec_smp_t = Vec::new();
  for i in 0..=5000 {
    vec_smp_t.push(i as f64 / 100.0);
  }
  vec_smp_t
}
