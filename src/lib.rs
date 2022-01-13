mod beat;
mod simulate;

pub mod prelude {
    // macro
    pub use crate::beat;

    // model
    pub use crate::simulate::FixSimModelTrait;
    pub use core::str::FromStr;
    pub use rust_decimal::Decimal;
}
