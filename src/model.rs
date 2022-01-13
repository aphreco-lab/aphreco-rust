mod fix;
mod flex;
mod ode;

pub use crate::model::fix::{FixModelOptTrait, FixModelSimTrait};
pub use crate::model::flex::{FlexModelOptTrait, FlexModelSimTrait};
pub use crate::model::ode::{OdeModelOptTrait, OdeModelSimTrait};
