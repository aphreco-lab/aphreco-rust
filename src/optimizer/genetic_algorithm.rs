use super::base::{ConcreteOptimizer, OptOptions};
use super::result::OptResult;

use crate::model::OptModelTrait;
use crate::objective::Objective;

use ndarray::Array1;
use rand::distributions::{Distribution, WeightedIndex};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::thread;

type Individual = (f64, Array1<f64>);
type Population = Vec<Individual>;

#[allow(dead_code)]
pub struct GeneticAlgorithm {
  max_gen: u64,
  n_pop: usize,
  n_elite: usize,
  mutation_rate: f64,
  len_x: usize,
  verbose: bool,
}

impl ConcreteOptimizer for GeneticAlgorithm {
  fn new(len_x: usize, options: &OptOptions) -> Self {
    let (max_gen, n_pop, mutation_rate, verbose) = match options {
      OptOptions::Default => (100, 10, 0.8, false),

      OptOptions::GeneticAlgorithm {
        max_gen,
        n_pop,
        mutation_rate,
        verbose,
      } => (*max_gen, *n_pop, *mutation_rate, *verbose),

      _ => panic!("Invalid OptOptions variant."),
    };

    let n_elite = if n_pop / 10 == 0 { 1 } else { n_pop / 10 };

    Self {
      max_gen,
      n_pop,
      n_elite,
      mutation_rate,
      len_x,
      verbose,
    }
  }

  fn run<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize, const LEN_X: usize>(
    &self,
    objective: &mut Objective<M, LEN_Y, LEN_P, LEN_B, LEN_X>,
  ) -> OptResult
  where
    M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>,
  {
    let mut fcall: u64 = 0;
    let mut rng = thread_rng();

    // bounds
    let log10_bounds = self.get_bounds(objective);

    // make initial population
    let mut pop = self.make_initial_pop(&log10_bounds, &mut rng);
    let mut next_pop = pop.clone();

    for n_gen in 0..self.max_gen {
      // vector for join-handles
      let mut handles = Vec::new();

      for ind in pop.iter() {
        let thread_ind = ind.clone();
        let mut thread_objective = objective.clone();

        // ===== FORK =====
        let handle = thread::spawn(move || {
          // if individual is an elite, the fitness has already been
          // evaluated in the previous generation.
          if thread_ind.0 == f64::INFINITY {
            let thread_f = thread_objective.obj(&thread_ind.1);
            thread_f
          } else {
            thread_ind.0
          }
        });
        // ================

        fcall += 1;
        handles.push(handle);
      }

      // ===== JOIN =====
      for (ind, handle) in pop.iter_mut().zip(handles) {
        ind.0 = handle.join().unwrap();
      }
      // ================

      // sort individuals in descending order (fmax objective)
      pop.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

      // print
      if self.verbose {
        println!("{:6}:   f:{:.4e}   x:{:10.5}", n_gen, pop[0].0, pop[0].1);
      }

      // choose elite
      for i in 0..self.n_elite {
        next_pop[i] = pop[i].clone();
      }

      // crossover
      for i in self.n_elite..self.n_pop {
        // current WeightedIndex
        let dist = self.weight_distribution(&pop);
        let (index_p1, index_p2) = self.select(dist, &mut rng);
        next_pop[i] = self.crossover(&pop[index_p1], &pop[index_p2], &mut rng);
      }

      // mutate
      // TODO: in parallel
      for i in 1..self.n_pop {
        self.mutate(&mut next_pop[i], &log10_bounds, &mut rng);
      }

      // alternate
      pop = next_pop.clone();
    }

    println!("Finished. fcall = {}", fcall);
    OptResult::new(pop[0].1.clone(), objective.x_index.clone(), pop[0].0)
  }
}

impl GeneticAlgorithm {
  fn get_bounds<M, const LEN_Y: usize, const LEN_P: usize, const LEN_B: usize, const LEN_X: usize>(
    &self,
    objective: &Objective<M, LEN_Y, LEN_P, LEN_B, LEN_X>,
  ) -> Vec<(f64, f64)>
  where
    M: OptModelTrait<LEN_Y, LEN_P, LEN_B, LEN_X>,
  {
    let mut log10_bounds: Vec<(f64, f64)> = Vec::new();
    let x_bounds = objective
      .x_bounds
      .as_ref()
      .expect("please define lower and upper bounds.");

    for &(lb, ub) in x_bounds.iter() {
      log10_bounds.push((f64::log10(lb), f64::log10(ub)));
    }

    log10_bounds
  }

  fn make_initial_pop(&self, log10_bounds: &Vec<(f64, f64)>, rng: &mut ThreadRng) -> Population {
    let mut pop = Vec::new();
    let mut ind: Individual = (f64::INFINITY, Array1::zeros(self.len_x));

    for _ in 0..self.n_pop {
      for (i, &(log10_lb, log10_ub)) in log10_bounds.iter().enumerate() {
        ind.1[i] = 10f64.powf(rng.gen_range(log10_lb..log10_ub));
      }
      pop.push(ind.clone());
    }
    pop
  }

  fn weight_distribution(&self, pop: &Population) -> WeightedIndex<f64> {
    // make a roulette (weights) for selection
    let f_worst = pop[self.n_pop - 1].0;
    let f_best = pop[0].0;

    let weights: Vec<f64> = pop
      .iter()
      .map(|x| (x.0 - f_worst) / (f_best - f_worst))
      .collect();

    // distribution
    WeightedIndex::new(&weights).unwrap()
  }

  fn crossover(
    &self,
    parent1: &Individual,
    parent2: &Individual,
    rng: &mut ThreadRng,
  ) -> Individual {
    let mut child: Individual = (f64::INFINITY, Array1::from(vec![0.0; self.len_x]));
    let mut i_rand_0_100: isize;

    for i in 0..self.len_x {
      i_rand_0_100 = rng.gen_range(0..=100);
      if i_rand_0_100 % 2 == 0 {
        child.1[i] = parent1.1[i];
      } else {
        child.1[i] = parent2.1[i];
      }
    }
    child
  }

  fn mutate(&self, ind: &mut Individual, log10_bounds: &Vec<(f64, f64)>, rng: &mut ThreadRng) {
    let mut f_rand_0_1: f64;
    let mut new_x: f64;

    for i in 0..self.len_x {
      f_rand_0_1 = rng.gen_range(0.0..1.0);

      if f_rand_0_1 < self.mutation_rate {
        new_x = ind.1[i] * rng.gen_range(0.8..1.25);
        let lb = 10f64.powf(log10_bounds[i].0);
        let ub = 10f64.powf(log10_bounds[i].1);

        if new_x < lb {
          ind.1[i] = lb;
        } else if new_x > ub {
          ind.1[i] = ub;
        } else {
          ind.1[i] = new_x;
        }
      }
    }
  }

  fn select(&self, dist: WeightedIndex<f64>, rng: &mut ThreadRng) -> (usize, usize) {
    let index_p1 = dist.sample(rng);
    let index_p2 = dist.sample(rng);
    (index_p1, index_p2)
  }
}
