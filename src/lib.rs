mod beat;
mod simulate;
mod step;

pub mod prelude {
    // model
    pub use crate::beat;
    pub use core::str::FromStr;
    pub use rust_decimal::Decimal;

    // simulate
    pub use crate::simulate::{SimModelTraitFix, SimulatorFix};
    pub use crate::step::Stepper;
}
