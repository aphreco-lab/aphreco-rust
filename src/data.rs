use ndarray::Array1;

#[derive(Clone)]
pub struct Data {
  pub obs: Vec<(usize, f64, f64, Option<f64>, Option<f64>)>,
}

#[allow(dead_code)]
impl Data {
  pub fn new(obs: Vec<(usize, f64, f64, Option<f64>, Option<f64>)>) -> Self {
    Self { obs }
  }

  pub fn make_sampling_time(&self) -> Vec<f64> {
    // collect all of the unique observation times in data.
    let mut vec_smp_t = Vec::new();
    for &(_, t, _, _, _) in self.obs.iter() {
      vec_smp_t.push(t);
    }
    vec_smp_t.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vec_smp_t.dedup();
    vec_smp_t
  }

  pub fn make_arr_obs_y(&self) -> Array1<f64> {
    let mut vec_obs_y = Vec::new();
    for &(_, _, y, _, _) in self.obs.iter() {
      vec_obs_y.push(y);
    }
    Array1::from(vec_obs_y)
  }

  pub fn make_ty_index(&self, vec_smp_t: &Vec<f64>) -> Vec<(usize, usize)> {
    let mut ty_index = Vec::new();
    for &(y_index, obs_t, _, _, _) in self.obs.iter() {
      let mut t_index = 0;
      for (j, &smp_t) in vec_smp_t.iter().enumerate() {
        if obs_t == smp_t {
          t_index = j;
        }
      }
      ty_index.push((t_index, y_index));
    }
    ty_index
  }
}
