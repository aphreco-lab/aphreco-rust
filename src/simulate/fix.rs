use super::result::SimResult;
use crate::step::{ConcreteStepper, Stepper};

use core::str::FromStr;
use num_traits::ToPrimitive;
use rust_decimal::Decimal;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

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
  pub stepper: Stepper,
}

impl<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize>
  SimulatorFix<M, LEN_Y, LEN_P, LEN_B>
where
  M: SimModelTraitFix<LEN_Y, LEN_P, LEN_B>,
{
  pub fn new(model: M, stepper: Stepper) -> Self {
    Self { model, stepper }
  }

  pub fn run(&self, smp_t: &Vec<f64>) -> SimResult<LEN_Y> {
    // ***time measurement***
    let mut time_ini = Duration::new(0, 0);
    let mut time_rec = Duration::new(0, 0);
    let mut time_ode = Duration::new(0, 0);
    let mut time_step = Duration::new(0, 0);
    let mut time_push = Duration::new(0, 0);

    let clock_start = Instant::now(); // ***time measurement***

    // initialize
    // ini_t: initial time of simulation (t0)
    // act: bool array indicating whether or not the recursive equations
    //      for each beat should be calculated at the cur_t.
    let (ini_t, ini_y) = self.model.init();
    let ini_beats = self.model.beats(&ini_t, &ini_y);
    let (end_t, mut vdq_smp_t, mut dec_times, beats, mut act) =
      self.initialize_times(&ini_t, smp_t, ini_beats);

    // for storing results
    let mut res_y: Vec<[f64; LEN_Y]> = Vec::new();

    // set the current state
    let mut cur_t = ini_t;
    let mut cur_y = ini_y;

    // construct ConcreteStepper instance
    let mut stepper = self.stepper.new(|t, y, dy| self.model.ode(t, y, dy));

    // derivative of y for ODE
    // difference of y for REC
    let mut deriv_y = [0f64; LEN_Y];
    let mut delta_y = [0f64; LEN_Y];

    // next_t indicates the next earliest discrete time point
    // for determining the end time of ODE solving in each loop.
    let mut next_t: f64;

    let clock_end = Instant::now(); // ***time measurement***
    time_ini += clock_end.duration_since(clock_start); // ***time measurement***

    loop {
      let clock_start = Instant::now(); // ***time measurement***

      // update act to be used in REC calculation
      // update dec_next_t in dec_times for next loop
      // next_t is the end of the ODE solving
      next_t = self.evaluate_condition(&cur_t, &cur_y, &beats, &mut act, &mut dec_times);

      // calculate REC
      self.solve_rec(&cur_t, &mut cur_y, &mut delta_y, &act);
      let clock_end = Instant::now(); // ***time measurement***
      time_rec += clock_end.duration_since(clock_start); // ***time measurement***

      // judge end
      if cur_t >= end_t {
        break;
      }

      let clock_start = Instant::now(); // ***time measurement***

      // integrate ODE and store the calculated y to the res_y
      let (t_step, t_push) = self.solve_ode(
        &mut stepper,
        &cur_t,
        &next_t,
        &mut vdq_smp_t,
        &mut cur_y,
        &mut deriv_y,
        &mut res_y,
      );

      let clock_end = Instant::now(); // ***time measurement***
      time_ode += clock_end.duration_since(clock_start); // ***time measurement***
      time_step += t_step; // ***time measurement***
      time_push += t_push; // ***time measurement***

      // make a progress to the next loop
      cur_t = next_t;
    }

    // store the last values
    res_y.push(cur_y);

    // ***time measurement***
    if false {
      println!("Ini  : {:.10}", time_ini.as_secs_f32());
      println!("Rec  : {:.10}", time_rec.as_secs_f32());
      println!("Ode  : {:.10}", time_ode.as_secs_f32());
      println!("Step : {:.10}", time_step.as_secs_f32());
      println!("Push : {:.10}", time_push.as_secs_f32());
    }

    SimResult::new(smp_t.clone(), res_y)
  }

  fn initialize_times(
    &self,
    ini_t: &f64,
    smp_t: &Vec<f64>,
    ini_beats: [(Decimal, Decimal, Decimal, bool); LEN_B],
  ) -> (
    f64,
    VecDeque<f64>,
    (Decimal, Decimal, Decimal, [Decimal; LEN_B]),
    [[Decimal; 3]; LEN_B],
    [bool; LEN_B],
  ) {
    let mut vec_smp_t = smp_t.clone();
    // sort
    vec_smp_t.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // remove duplication
    vec_smp_t.dedup();
    // remove smp_t earlier than ini_t
    vec_smp_t.retain(|&x| x >= *ini_t);

    // end time of simulation
    let end_t = vec_smp_t[vec_smp_t.len() - 1];

    // create VecDeq of sampling time,
    // because Simulator calls VecDeq::pop_front()
    // when it stores the calculated results.
    let vdq_smp_t = VecDeque::from(vec_smp_t);

    // convert ini/end time from f64 into Decimal
    // and make a Decimal meaning stopped (timeout) for beats
    let dec_ini_t = Decimal::from_str(&ini_t.to_string()).unwrap();
    let dec_end_t = Decimal::from_str(&end_t.to_string()).unwrap();
    let dec_stopped = dec_end_t + Decimal::from_str("1").unwrap();

    // set the first discrete time point for each beat.
    // and create act (is_active) at t=0.
    let mut dec_first_t = [dec_ini_t; LEN_B];
    let mut ini_act = [false; LEN_B];
    let mut beats = [[Decimal::from_str("0.0").unwrap(); 3]; LEN_B];

    for i in 0..LEN_B {
      // t0 of simulatjion < beat starting time
      if dec_ini_t < ini_beats[i].0 {
        dec_first_t[i] = ini_beats[i].0;
      }

      // is_active at t=0 is true
      ini_act[i] = ini_beats[i].3;

      // beats
      beats[i][0] = ini_beats[i].0;
      beats[i][1] = ini_beats[i].1;
      beats[i][2] = ini_beats[i].2;
    }

    (
      end_t,
      vdq_smp_t,
      (dec_ini_t, dec_end_t, dec_stopped, dec_first_t),
      beats,
      ini_act,
    )
  }

  fn evaluate_condition(
    &self,
    cur_t: &f64,
    cur_y: &[f64; LEN_Y],
    beats: &[[Decimal; 3]; LEN_B],
    act: &mut [bool; LEN_B],
    (_, dec_end_t, dec_stopped, dec_next_t): &mut (Decimal, Decimal, Decimal, [Decimal; LEN_B]),
  ) -> f64 {
    let dec_cur_t = Decimal::from_str(&cur_t.to_string()).unwrap();

    self.model.cond(&dec_cur_t, act, dec_next_t, cur_y);

    let mut tmp_dec_next_t: Decimal;
    for (i, &is_active) in act.iter().enumerate() {
      if is_active {
        // cur_time + interval
        tmp_dec_next_t = dec_cur_t + beats[i][2];

        // next_time = cur_time + interval if next_time <= end_of_beat.
        // otherwise, next_time is set to be end_time + 1 (stopped),
        // so that the corresponding beat will never beat again.
        if tmp_dec_next_t <= beats[i][1] {
          dec_next_t[i] = dec_next_t[i] + beats[i][2];
        } else {
          dec_next_t[i] = *dec_stopped;
        }
      }
    }

    // the earliest next discrete time point will be used
    // as the next end of the ODE solving.
    let mut dec_earliest = dec_next_t[0];
    for &next_t in dec_next_t.iter().skip(1) {
      if next_t < dec_earliest {
        dec_earliest = next_t;
      }
    }

    // if the next earliest discrete time is greater than end_time
    // next_t will be end_t, meaning this is the last rec solving.
    if dec_earliest < *dec_end_t {
      dec_earliest.to_string().parse::<f64>().unwrap()
    } else {
      dec_end_t.to_f64().unwrap()
    }
  }

  fn solve_rec(
    &self,
    cur_t: &f64,
    cur_y: &mut [f64; LEN_Y],
    delta_y: &mut [f64; LEN_Y],
    act: &[bool; LEN_B],
  ) {
    self.model.rec(cur_t, cur_y, delta_y, act);
    for i in 0..LEN_Y {
      cur_y[i] += delta_y[i];
      delta_y[i] = 0.0;
    }
    self.model.cre(cur_t, cur_y);
  }

  fn solve_ode<ODE>(
    &self,
    stepper: &mut ConcreteStepper<ODE, LEN_Y>,
    ini_t: &f64,
    end_t: &f64,
    vdq_smp_t: &mut VecDeque<f64>,
    cur_y: &mut [f64; LEN_Y],
    deriv_y: &mut [f64; LEN_Y],
    res_y: &mut Vec<[f64; LEN_Y]>,
  ) -> (Duration, Duration)
  where
    ODE: Fn(&f64, &[f64; LEN_Y], &mut [f64; LEN_Y]),
  {
    let mut time_step = Duration::new(0, 0); // ***time measurement***
    let mut time_push = Duration::new(0, 0); // ***time measurement***

    let mut cur_t = ini_t.clone();

    let mut new_t: f64;
    let mut new_y = cur_y.clone();

    let mut out_t: f64;
    let mut out_y = [0f64; LEN_Y];

    loop {
      let clock_start = Instant::now(); // ***time measurement***

      // evaluate derivative
      new_t = stepper.run(&cur_t, &mut new_y, deriv_y);
      let clock_end = Instant::now(); // ***time measurement***
      time_step += clock_end.duration_since(clock_start); // ***time measurement***

      // keep constant relation (cre)
      self.model.cre(&cur_t, &mut new_y);

      // store results
      let clock_start = Instant::now(); // ***time measurement***
      loop {
        if cur_t < 1e-10 && cur_t != 0.0 {
          println!("cur_t; {}", &cur_t);
          println!("cur_y; {:?}", cur_y);
          // println!("{}", self.stepper.h);
          panic!();
        }

        if vdq_smp_t.len() == 0 {
          println!("All sample points have been collected.");
          break;
        }

        if vdq_smp_t[0] < new_t && vdq_smp_t[0] < *end_t {
          // get output time point.
          out_t = vdq_smp_t.pop_front().unwrap();

          // interpolate the value at out_t.
          for i in 0..LEN_Y {
            out_y[i] = cur_y[i] + (out_t - cur_t) * deriv_y[i];
          }

          // keep constant relation (cre).
          self.model.cre(&out_t, &mut out_y);

          // store results
          res_y.push(out_y);

          // print for debug.
          // self._print_current_state(&out_t, &out_y);
        } else {
          break;
        }
      }
      let clock_end = Instant::now(); // ***time measurement***
      time_push += clock_end.duration_since(clock_start); // ***time measurement***

      // make progress to the next loop
      cur_t = new_t;
      *cur_y = new_y;

      if new_t > *end_t {
        break;
      }

      // store results at the end_t
      for i in 0..LEN_Y {
        cur_y[i] = cur_y[i] + (end_t - cur_t) * deriv_y[i];
      }
    }

    // store results at the end_t
    for i in 0..LEN_Y {
      cur_y[i] = cur_y[i] + (end_t - cur_t) * deriv_y[i];
    }

    (time_step, time_push) // ***time measurement***
  }
}
