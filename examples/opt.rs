use aphreco::prelude::*;

fn main() {
  let model = Model::new();
  let st_options = StepOptions::Dopri45 {
    h0: 1e-4,
    abstol: 1e-6,
    reltol: 1e-9,
    hmin: 1e-6,
    hmax: 1e-2,
  };
  let stepper = Stepper::Dopri45(st_options);
  let simulator = Simulator::new(model, stepper);

  let data = Data::new(obs());
  let mut objective = Objective::new(simulator, data);

  let ga_options = OptOptions::GeneticAlgorithm {
    max_gen: 30,
    n_pop: 100,
    mutation_rate: 0.5,
    verbose: false,
  };
  let optimizer = Optimizer::GeneticAlgorithm(ga_options);
  let optres = optimizer.run(&mut objective);

  objective.setx(&optres.x);

  let nm_options = OptOptions::NelderMead {
    max_iter: 0,
    adaptive: true,
    verbose: true,
  };
  let optimizer = Optimizer::NelderMead(nm_options);
  let optres = optimizer.run(&mut objective);

  optres.save("./");
}

const LEN_Y: usize = 4;
const LEN_P: usize = 11;
const LEN_B: usize = 2;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Model {
  pub p: [f64; LEN_P],
}

#[allow(dead_code)]
impl SimModelTrait<LEN_Y, LEN_P, LEN_B> for Model {
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
  fn beat(&self, t: &f64, y: &[f64; LEN_Y]) -> [[Decimal; 3]; LEN_B] {
    [
      beat![self.p[2], self.p[3], self.p[4]],
      beat![self.p[5], self.p[6], self.p[7]],
    ]
  }

  #[allow(unused_variables)]
  fn cre(&self, t: &f64, y: &mut [f64; LEN_Y]) {
    y[3] = self.p[8] * t;
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

const LEN_X: usize = 2;

impl OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X> for Model {
  fn getx(&self) -> (Vec<usize>, Option<Vec<(f64, f64)>>) {
    let x_index = vec![0, 1];
    let x_bounds = Some(vec![(1e-4, 1e0), (1e-4, 1e0)]);
    // let x_bounds = None;
    (x_index, x_bounds)
  }

  fn getp(&self) -> &[f64; LEN_P] {
    &self.p
  }

  fn setp(&mut self, index: usize, value: f64) {
    self.p[index] = value;
  }
}

#[allow(dead_code)]
fn obs() -> Vec<(usize, f64, f64, Option<f64>, Option<f64>)> {
  vec![
    // A <=> B
    // ddt_A = -k12*A + k21*B
    // ddt_B = k12*A - k21*B
    // k12 = 0.2
    // k21 = 0.05
    // A0 = 100
    // B0 = 0
    // index, t, y, terr, yerr

    // A
    (0, 0.0, 100.000, None, None),  // d[0]
    (0, 0.1, 98.0248, None, None),  // d[1]
    (0, 0.2, 96.0984, None, None),  // d[2]
    (0, 0.5, 90.5998, None, None),  // d[3]
    (0, 1.0, 82.3041, None, None),  // d[4]
    (0, 2.0, 68.5225, None, None),  // d[5]
    (0, 5.0, 42.9204, None, None),  // d[6]
    (0, 10.0, 26.5668, None, None), // d[7]
    (0, 20.0, 20.5390, None, None), // d[8]
    (0, 50.0, 20.0003, None, None), // d[9]
    // B
    (1, 0.0, 0.00000, None, None),  // d[10]
    (1, 0.1, 1.9752, None, None),   // d[11]
    (1, 0.2, 3.9016, None, None),   // d[12]
    (1, 0.5, 9.4002, None, None),   // d[13]
    (1, 1.0, 17.6959, None, None),  // d[14]
    (1, 2.0, 31.4775, None, None),  // d[15]
    (1, 5.0, 57.0796, None, None),  // d[16]
    (1, 10.0, 73.4332, None, None), // d[17]
    (1, 20.0, 79.4610, None, None), // d[18]
    (1, 50.0, 79.9997, None, None), // d[19]
  ]
}
