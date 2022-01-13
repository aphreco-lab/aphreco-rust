mod model;

pub mod prelude {
    pub use crate::model::{FixModelOptTrait, FlexModelOptTrait, OdeModelOptTrait};
    pub use crate::model::{FixModelSimTrait, FlexModelSimTrait, OdeModelSimTrait};

    pub use core::str::FromStr;
    pub use rust_decimal::Decimal;
}
