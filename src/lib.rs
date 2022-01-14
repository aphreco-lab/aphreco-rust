mod beat;
pub mod data;
pub mod model;
pub mod objective;
pub mod optimizer;
pub mod simulator;
pub mod stepper;
mod utils;

pub mod prelude {
  // macro
  pub use crate::beat;
  pub use crate::clock;

  // modeling
  pub use crate::model::{OptModelTrait, SimModelTrait};
  pub use core::str::FromStr;
  pub use rust_decimal::Decimal;

  // simulation
  pub use crate::simulator::SimResult;
  pub use crate::simulator::Simulator;
  pub use crate::stepper::Stepper;

  // optimization
  pub use crate::data::Data;
  pub use crate::objective::Objective;
  pub use crate::optimizer::Optimizer;
}
