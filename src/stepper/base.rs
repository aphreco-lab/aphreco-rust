use super::dopri45::Dopri45;
use super::rk4::Rk4;

#[derive(Clone)]
pub enum StepOptions {
  Default,

  Rk4 {
    h: f64,
  },

  Dopri45 {
    h0: f64,
    abstol: f64,
    reltol: f64,
    hmin: f64,
    hmax: f64,
  },
}

#[derive(Clone)]
pub enum Stepper {
  Rk4(StepOptions),
  Dopri45(StepOptions),
}

impl Stepper {
  pub fn new<Ode, const LEN_Y: usize>(&self, ode: Ode) -> ConcreteStepper<Ode, LEN_Y>
  where
    Ode: Fn(&f64, &[f64; LEN_Y], &mut [f64; LEN_Y]),
  {
    match self {
      Stepper::Rk4(options) => ConcreteStepper::Rk4 {
        concrete_stepper: Rk4::new(ode, options),
      },

      Stepper::Dopri45(options) => ConcreteStepper::Dopri45 {
        concrete_stepper: Dopri45::new(ode, options),
      },
    }
  }
}

pub enum ConcreteStepper<Ode, const LEN_Y: usize>
where
  Ode: Fn(&f64, &[f64; LEN_Y], &mut [f64; LEN_Y]),
{
  Rk4 {
    concrete_stepper: Rk4<Ode, LEN_Y>,
  },
  Dopri45 {
    concrete_stepper: Dopri45<Ode, LEN_Y>,
  },
}

impl<Ode, const LEN_Y: usize> ConcreteStepper<Ode, LEN_Y>
where
  Ode: Fn(&f64, &[f64; LEN_Y], &mut [f64; LEN_Y]),
{
  pub fn run(&mut self, t: &f64, y: &mut [f64; LEN_Y], dy: &mut [f64; LEN_Y]) -> f64 {
    match self {
      ConcreteStepper::Rk4 { concrete_stepper } => concrete_stepper.run(t, y, dy),
      ConcreteStepper::Dopri45 { concrete_stepper } => concrete_stepper.run(t, y, dy),
    }
  }
}
