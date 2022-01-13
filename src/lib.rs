mod beat;
mod optimize;
mod simulate;

pub mod prelude {
    // macro
    pub use crate::beat;

    // model
    // pub use crate::optimizer::OptTraitFix;
    pub use crate::simulate::SimModelTraitFix;
    pub use core::str::FromStr;
    pub use rust_decimal::Decimal;
}
