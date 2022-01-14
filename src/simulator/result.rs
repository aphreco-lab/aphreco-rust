use chrono::{DateTime, Local};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct SimResult<const LEN_Y: usize> {
  pub t: Vec<f64>,
  pub y: Vec<[f64; LEN_Y]>,
}

impl<const LEN_Y: usize> SimResult<LEN_Y> {
  pub fn new(t: Vec<f64>, y: Vec<[f64; LEN_Y]>) -> Self {
    Self { t, y }
  }

  pub fn save(&self, dir: &str) {
    let save_dir = Path::new(dir);
    let mut str_result = String::new();

    for (t, y) in self.t.iter().zip(self.y.iter()) {
      str_result.push_str(&t.to_string());
      for i in 0..LEN_Y {
        str_result.push(',');
        str_result.push_str(&y[i].to_string());
      }
      str_result.push('\n');
    }

    let datetime: DateTime<Local> = Local::now();
    let now_str = datetime.format("%Y%m%d_%H%M%S%.3f").to_string();
    let file_name = String::from("Sim_") + &now_str + ".csv";
    let save_path = save_dir.join(file_name);

    // Write string into a file
    let mut file = File::create(save_path).unwrap();
    write!(file, "{}", str_result).unwrap();
    file.flush().unwrap();
  }
}
