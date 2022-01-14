use crate::data::Data;
use crate::model::OptModelTrait;
use crate::simulator::Simulator;

use ndarray::Array1;

#[derive(Clone)]
pub struct Objective<
  M,
  const LEN_Y: usize,
  const LEN_P: usize,
  const LEN_B: usize,
  const LEN_X: usize,
> where
  M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>,
{
  pub simulator: Simulator<M, LEN_Y, LEN_P, LEN_B>,
  pub data: Data,
  pub len_x: usize,
  vec_smp_t: Vec<f64>,
  arr_obs_y: Array1<f64>,
  ty_index: Vec<(usize, usize)>,
  pub x_index: Vec<usize>,
  pub x_bounds: Option<Vec<(f64, f64)>>,
}

impl<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize, const LEN_X: usize>
  Objective<M, LEN_Y, LEN_P, LEN_B, LEN_X>
where
  M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>,
{
  pub fn new(simulator: Simulator<M, LEN_Y, LEN_P, LEN_B>, data: Data) -> Self {
    let vec_smp_t = data.make_sampling_time();
    let arr_obs_y = data.make_arr_obs_y();
    let ty_index = data.make_ty_index(&vec_smp_t);
    let (x_index, x_bounds) = simulator.model.getx();
    let len_x = x_index.len();

    Self {
      simulator,
      data,
      len_x,
      vec_smp_t,
      arr_obs_y,
      ty_index,
      x_index,
      x_bounds,
    }
  }

  pub fn obj(&mut self, new_x: &Array1<f64>) -> f64 {
    // assign x to the corresponding parameter in a model.
    self.setx(new_x);

    // simulate
    let simres = self.simulator.run(&mut self.vec_smp_t.clone());

    // get arr_sim_y from simulation results
    let mut vec_sim_y = Vec::new();
    for &(t_index, y_index) in self.ty_index.iter() {
      vec_sim_y.push(simres.y[t_index][y_index]);
    }
    let arr_sim_y = Array1::from(vec_sim_y);

    // calculate SSR (sum of squared residuals)
    let ssr = (&self.arr_obs_y - &arr_sim_y).mapv(|a| a.powi(2)).sum();
    ssr
  }

  pub fn setx(&mut self, new_x: &Array1<f64>) {
    for (&x_index, &x_value) in self.x_index.iter().zip(new_x.iter()) {
      self.simulator.model.setp(x_index, x_value);
    }
  }
}
