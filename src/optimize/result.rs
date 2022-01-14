use chrono::{DateTime, Local};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use ndarray::Array1;

pub struct OptResult {
  pub opt_x: Array1<f64>,
  pub index: Vec<usize>,
  pub opt_f: f64,
}

impl OptResult {
  pub fn new(x: Array1<f64>, index: Vec<usize>, f: f64) -> Self {
    Self { x, index, f }
  }

  pub fn save(&self, dir: &str) {
    let save_dir = Path::new(dir);
    let mut str_result = String::new();

    for (index, value) in self.index.iter().zip(self.opt_x.iter()) {
      str_result.push_str(&index.to_string());
      str_result.push(',');
      str_result.push_str(&value.to_string());
      str_result.push('\n');
    }

    let datetime: DateTime<Local> = Local::now();
    let now_str = datetime.format("%Y%m%d_%H%M%S%.3f").to_string();
    let file_name = String::from("Opt_") + &now_str + ".csv";
    let save_path = save_dir.join(file_name);

    // Write string into a file
    let mut file = File::create(save_path).unwrap();
    write!(file, "{}", str_result).unwrap();
    file.flush().unwrap();
  }
}
