#include <node_api.h>

extern void mhp_initialize(void);

#define DECLARE_NAPI_PROPERTY(name, func) \
  { (name), 0, (func), 0, 0, 0, napi_default, 0 }

napi_value initialize(napi_env env, napi_callback_info info) {
  mhp_initialize();

  napi_value result;
  napi_get_undefined(env, &result);

  return result;
}

napi_value init(napi_env env, napi_value exports) {
  napi_property_descriptor desc = DECLARE_NAPI_PROPERTY("initialize", initialize);
  napi_define_properties(env, exports, 1, &desc);
  return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, init)
