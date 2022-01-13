mod beat;
mod model;

pub mod prelude {
    // macro
    pub use crate::beat;

    // model
    pub use crate::model::{FixModelOptTrait, FlexModelOptTrait, OdeModelOptTrait};
    pub use crate::model::{FixModelSimTrait, FlexModelSimTrait, OdeModelSimTrait};
    pub use core::str::FromStr;
    pub use rust_decimal::Decimal;
}
