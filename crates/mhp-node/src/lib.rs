extern crate mhp_napi_sys;

use mhp_napi_sys::{napi_callback_info, napi_env, napi_value,
                   napi_get_undefined};

use std::mem;

#[no_mangle]
pub extern "C" fn mhp_initialize(
    env: napi_env,
    _info: napi_callback_info,
) -> napi_value {
    println!("Hello from the Rust land!");

    unsafe {
        let mut result: napi_value = mem::uninitialized();
        napi_get_undefined(env, &mut result);

        result
    }
}
