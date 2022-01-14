pub struct Rk4<Ode, const LEN_Y: usize>
where
  Ode: Fn(&f64, &[f64; LEN_Y], &mut [f64; LEN_Y]),
{
  ode: Ode,
  h: f64,
  k1: [f64; LEN_Y],
  k2: [f64; LEN_Y],
  k3: [f64; LEN_Y],
  k4: [f64; LEN_Y],
  wk: [f64; LEN_Y],
}

impl<Ode, const LEN_Y: usize> Rk4<Ode, LEN_Y>
where
  Ode: Fn(&f64, &[f64; LEN_Y], &mut [f64; LEN_Y]),
{
  pub fn new(ode: Ode) -> Self {
    Self {
      ode: ode,
      h: 1e-3,
      k1: [0f64; LEN_Y],
      k2: [0f64; LEN_Y],
      k3: [0f64; LEN_Y],
      k4: [0f64; LEN_Y],
      wk: [0f64; LEN_Y],
    }
  }

  pub fn run(&mut self, t: &f64, y: &mut [f64; LEN_Y], dy: &mut [f64; LEN_Y]) -> f64 {
    self.step(t, y, dy);
    t + self.h
  }

  pub fn step(&mut self, t: &f64, y: &mut [f64; LEN_Y], dy: &mut [f64; LEN_Y]) {
    (self.ode)(t, y, &mut self.k1);

    for i in 0..LEN_Y {
      self.wk[i] = y[i] + self.k1[i] * self.h / 2.0;
    }
    (self.ode)(&(t + 0.5 * self.h), &self.wk, &mut self.k2);

    for i in 0..LEN_Y {
      self.wk[i] = y[i] + self.k2[i] * self.h / 2.0;
    }
    (self.ode)(&(t + 0.5 * self.h), &self.wk, &mut self.k3);

    for i in 0..LEN_Y {
      self.wk[i] = y[i] + self.k3[i] * self.h;
    }
    (self.ode)(&(t + self.h), &self.wk, &mut self.k4);

    for i in 0..LEN_Y {
      dy[i] = (self.k1[i] + 2.0 * self.k2[i] + 2.0 * self.k3[i] + self.k4[i]) / 6.0;
      y[i] += dy[i] * self.h;
    }
  }
}