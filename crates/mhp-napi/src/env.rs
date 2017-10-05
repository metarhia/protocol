use std::ffi::CStr;
use std::mem;

use sys;
use result::NapiError;

#[derive(Clone)]
pub struct NapiEnv {
    env: sys::napi_env,
}

impl From<sys::napi_env> for NapiEnv {
    fn from(env: sys::napi_env) -> Self {
        Self { env }
    }
}

impl NapiEnv {
    fn decode_error(&self, status: sys::napi_status) -> Option<NapiError> {
        if status == sys::napi_status::napi_ok {
            return None;
        }

        let mut error_message = String::new();

        unsafe {
            let mut extended_error_info = mem::uninitialized();
            sys::napi_get_last_error_info(self.env, &mut extended_error_info);

            let raw_error_message = (*extended_error_info).error_message;
            if raw_error_message.is_null() {
                error_message.push_str("(error message is nullptr)");
            } else {
                let c_string = CStr::from_ptr(raw_error_message);
                error_message = c_string.to_string_lossy().into_owned();
            }
        }

        Some(NapiError::new(status, error_message))
    }
}
