mod model;

pub mod prelude {
    pub use crate::model::{
        FixModelOptTrait, FlexModelOptTrait, OdeModelOptTrait, RecModelOptTrait,
    };
    pub use crate::model::{
        FixModelSimTrait, FlexModelSimTrait, OdeModelSimTrait, RecModelSimTrait,
    };
}
