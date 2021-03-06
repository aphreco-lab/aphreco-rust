use super::base::StepOptions;

pub struct Dopri45<Ode, const LEN_Y: usize>
where
  Ode: Fn(&f64, &[f64; LEN_Y], &mut [f64; LEN_Y]),
{
  ode: Ode,
  h: f64,
  k1: [f64; LEN_Y],
  k2: [f64; LEN_Y],
  k3: [f64; LEN_Y],
  k4: [f64; LEN_Y],
  k5: [f64; LEN_Y],
  k6: [f64; LEN_Y],
  k7: [f64; LEN_Y],
  wk: [f64; LEN_Y],
  y4: [f64; LEN_Y],
  y5: [f64; LEN_Y],
  total_tols: [f64; LEN_Y],
  abstol: f64,
  reltol: f64,
  hmin: f64,
  hmax: f64,
}

impl<Ode, const LEN_Y: usize> Dopri45<Ode, LEN_Y>
where
  Ode: Fn(&f64, &[f64; LEN_Y], &mut [f64; LEN_Y]),
{
  const ORDER: f64 = 5.0;

  const C2: f64 = 1.0 / 5.0;
  const C3: f64 = 3.0 / 10.0;
  const C4: f64 = 4.0 / 5.0;
  const C5: f64 = 8.0 / 9.0;
  const C6: f64 = 1.0;
  const C7: f64 = 1.0;

  const A21: f64 = 1.0 / 5.0;
  const A31: f64 = 3.0 / 40.0;
  const A32: f64 = 9.0 / 40.0;
  const A41: f64 = 44.0 / 45.0;
  const A42: f64 = -56.0 / 15.0;
  const A43: f64 = 32.0 / 9.0;
  const A51: f64 = 19372.0 / 6561.0;
  const A52: f64 = -25360.0 / 2187.0;
  const A53: f64 = 64448.0 / 6561.0;
  const A54: f64 = -212.0 / 729.0;
  const A61: f64 = -9017.0 / 3168.0;
  const A62: f64 = -355.0 / 33.0;
  const A63: f64 = 46732.0 / 5247.0;
  const A64: f64 = 49.0 / 176.0;
  const A65: f64 = -5103.0 / 18656.0;
  const A71: f64 = 35.0 / 384.0;
  const A72: f64 = 0.0;
  const A73: f64 = 500.0 / 1113.0;
  const A74: f64 = 125.0 / 192.0;
  const A75: f64 = -2187.0 / 6784.0;
  const A76: f64 = 11.0 / 84.0;

  const B41: f64 = 5179.0 / 57600.0;
  const B42: f64 = 0.0;
  const B43: f64 = 7571.0 / 16695.0;
  const B44: f64 = 393.0 / 640.0;
  const B45: f64 = -92097.0 / 339200.0;
  const B46: f64 = 187.0 / 2100.0;
  const B47: f64 = 1.0 / 40.0;
  const B51: f64 = Self::A71;
  const B52: f64 = Self::A72;
  const B53: f64 = Self::A73;
  const B54: f64 = Self::A74;
  const B55: f64 = Self::A75;
  const B56: f64 = Self::A76;

  pub fn new(ode: Ode, options: &StepOptions) -> Self {
    let (h0, abstol, reltol, hmin, hmax) = match options {
      StepOptions::Default => (
        1e-8, // default h0
        1e-4, // default abstol
        1e-4, // default reltol
        1e-8, // default hmin
        1e-3, // default hmax
      ),

      StepOptions::Dopri45 {
        h0,
        abstol,
        reltol,
        hmin,
        hmax,
      } => (*h0, *abstol, *reltol, *hmin, *hmax),

      _ => panic!("Invalid StepOptions variant."),
    };

    Self {
      ode: ode,
      h: h0,
      k1: [0f64; LEN_Y],
      k2: [0f64; LEN_Y],
      k3: [0f64; LEN_Y],
      k4: [0f64; LEN_Y],
      k5: [0f64; LEN_Y],
      k6: [0f64; LEN_Y],
      k7: [0f64; LEN_Y],
      wk: [0f64; LEN_Y],
      y4: [0f64; LEN_Y],
      y5: [0f64; LEN_Y],
      total_tols: [0f64; LEN_Y],
      abstol: abstol,
      reltol: reltol,
      hmin: hmin,
      hmax: hmax,
    }
  }

  pub fn run(&mut self, t: &f64, y: &mut [f64; LEN_Y], dy: &mut [f64; LEN_Y]) -> f64 {
    let next_t;
    loop {
      // calculate self.y4 and self.y5 (and renew dy)
      // using the current self.h.
      let rms_err = self.step(t, y, dy);

      // if results are accepted, break the loop,
      // else, calculate step again after shortening step size.
      if rms_err <= 1.0 {
        // renew t and y
        next_t = t + self.h.clone();
        *y = self.y5;
        // extend step size
        self.update_stepsize(rms_err);
        break;
      } else {
        // shrink step size
        self.update_stepsize(rms_err);
      }
    }

    next_t
  }

  pub fn step(&mut self, t: &f64, y: &mut [f64; LEN_Y], dy: &mut [f64; LEN_Y]) -> f64 {
    (self.ode)(t, y, &mut self.k1);

    let ha21 = self.h * Self::A21;
    for i in 0..LEN_Y {
      self.wk[i] = y[i] + ha21 * self.k1[i];
    }
    (self.ode)(&(t + self.h * Self::C2), &self.wk, &mut self.k2);

    let ha31 = self.h * Self::A31;
    let ha32 = self.h * Self::A32;
    for i in 0..LEN_Y {
      self.wk[i] = y[i] + ha31 * self.k1[i] + ha32 * self.k2[i];
    }
    (self.ode)(&(t + self.h * Self::C3), &self.wk, &mut self.k3);

    let ha41 = self.h * Self::A41;
    let ha42 = self.h * Self::A42;
    let ha43 = self.h * Self::A43;
    for i in 0..LEN_Y {
      self.wk[i] = y[i] + ha41 * self.k1[i] + ha42 * self.k2[i] + ha43 * self.k3[i];
    }
    (self.ode)(&(t + self.h * Self::C4), &self.wk, &mut self.k4);

    let ha51 = self.h * Self::A51;
    let ha52 = self.h * Self::A52;
    let ha53 = self.h * Self::A53;
    let ha54 = self.h * Self::A54;
    for i in 0..LEN_Y {
      self.wk[i] =
        y[i] + ha51 * self.k1[i] + ha52 * self.k2[i] + ha53 * self.k3[i] + ha54 * self.k4[i];
    }
    (self.ode)(&(t + self.h * Self::C5), &self.wk, &mut self.k5);

    let ha61 = self.h * Self::A61;
    let ha62 = self.h * Self::A62;
    let ha63 = self.h * Self::A63;
    let ha64 = self.h * Self::A64;
    let ha65 = self.h * Self::A65;
    for i in 0..LEN_Y {
      self.wk[i] = y[i]
        + ha61 * self.k1[i]
        + ha62 * self.k2[i]
        + ha63 * self.k3[i]
        + ha64 * self.k4[i]
        + ha65 * self.k5[i];
    }
    (self.ode)(&(t + self.h * Self::C6), &self.wk, &mut self.k6);

    let ha71 = self.h * Self::A71;
    let ha72 = self.h * Self::A72;
    let ha73 = self.h * Self::A73;
    let ha74 = self.h * Self::A74;
    let ha75 = self.h * Self::A75;
    let ha76 = self.h * Self::A76;
    for i in 0..LEN_Y {
      self.wk[i] = y[i]
        + ha71 * self.k1[i]
        + ha72 * self.k2[i]
        + ha73 * self.k3[i]
        + ha74 * self.k4[i]
        + ha75 * self.k5[i]
        + ha76 * self.k6[i];
    }
    (self.ode)(&(t + self.h * Self::C7), &self.wk, &mut self.k7);

    let mut sum_of_squared_err = 0.0;

    let hb41 = self.h * Self::B41;
    let hb42 = self.h * Self::B42;
    let hb43 = self.h * Self::B43;
    let hb44 = self.h * Self::B44;
    let hb45 = self.h * Self::B45;
    let hb46 = self.h * Self::B46;
    let hb47 = self.h * Self::B47;
    for i in 0..LEN_Y {
      // y4
      self.y4[i] = y[i]
        + hb41 * self.k1[i]
        + hb42 * self.k2[i]
        + hb43 * self.k3[i]
        + hb44 * self.k4[i]
        + hb45 * self.k5[i]
        + hb46 * self.k6[i]
        + hb47 * self.k7[i];

      // y5
      dy[i] = Self::B51 * self.k1[i]
        + Self::B52 * self.k2[i]
        + Self::B53 * self.k3[i]
        + Self::B54 * self.k4[i]
        + Self::B55 * self.k5[i]
        + Self::B56 * self.k6[i];
      self.y5[i] = y[i] + self.h * dy[i];

      // error
      self.total_tols[i] = self.abstol + self.reltol * self.y5[i].abs();
      sum_of_squared_err += ((self.y5[i] - self.y4[i]) / self.total_tols[i]).powf(2.0);
    }

    let rms_err = (sum_of_squared_err / LEN_Y as f64).sqrt();

    rms_err
  }

  fn update_stepsize(&mut self, rms_err: f64) {
    let ratio = 0.8 * ((1.0 / rms_err).powf(1.0 / Self::ORDER));

    if ratio < 0.25 {
      self.h *= 0.25;
    } else if ratio < 4.0 {
      self.h *= ratio;
    } else {
      self.h *= 4.0;
    }

    if self.h < self.hmin {
      self.h = self.hmin
    } else if self.hmax < self.h {
      self.h = self.hmax
    }
  }
}
