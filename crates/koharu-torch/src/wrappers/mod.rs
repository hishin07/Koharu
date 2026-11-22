pub(crate) mod device;
pub(crate) mod generator;
pub mod kind;
pub(crate) mod layout;
pub(crate) mod scalar;
pub(crate) mod tensor;
<<<<<<< HEAD
// The generated tensor wrappers stay byte-identical to the upstream tch-rs
// codegen, whose `allow(clippy::all)` silences only clippy: its `std::f64::NAN`
// defaults trip rustc's `deprecated` under CI's `-D warnings`. Exempting the
// module here keeps the generated files untouched for the next upstream sync.
#[allow(deprecated)]
pub(crate) mod tensor_fallible_generated;
pub(crate) mod tensor_generated;
=======
pub(crate) mod utils;

// A method or function for each operator, which build.rs generates from
// koharu-torch-sys's bindings. A dunder operator, such as `__and__`, keeps
// its doubled underscores.
#[allow(clippy::all, non_snake_case)]
mod tensor_generated {
    use std::borrow::Borrow;

    use koharu_torch_sys::*;

    use super::utils::{IntList, IntListOption, check};
    use crate::{Device, Generator, Kind, Layout, MemoryFormat, QScheme, Result, Scalar, Tensor};

    include!(concat!(env!("OUT_DIR"), "/tensor_generated.rs"));
}
>>>>>>> upstream/main
