extern crate mhp_napi_sys;

mod result;
mod env;

pub use result::{NapiError, NapiResult};
pub use env::NapiEnv;

pub mod sys {
    pub use mhp_napi_sys::*;
}
