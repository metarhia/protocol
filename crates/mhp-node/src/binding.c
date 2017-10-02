#include <node_api.h>

extern void mhp_initialize(void);

napi_value initialize(napi_env env, napi_callback_info info) {
  mhp_initialize();

  napi_value result;
  napi_get_undefined(env, &result);

  return result;
}

#define MHP_PROPERTY_MAP(V)                                                   \
  V(initialize, initialize)

#ifdef __GNUC__
#define UNLIKELY(expr) __builtin_expect(!!(expr), 0)
#else
#define UNLIKELY(expr) (expr)
#endif

#define ASSERT_NAPI_OK(expr)                                                  \
  if (UNLIKELY((expr) != napi_ok)) {                                          \
    napi_throw_error(env, NULL, "Assertion failed: (" #expr ") != napi_ok");  \
    return NULL;                                                              \
  }

napi_value init(napi_env env, napi_value exports) {
#define V(name, func)                                                         \
  { #name, NULL, func, NULL, NULL, NULL, napi_default, NULL },

  napi_property_descriptor descriptors[] = {
    MHP_PROPERTY_MAP(V)
  };
#undef V

  napi_status status = napi_define_properties(
      env,
      exports,
      sizeof(descriptors) / sizeof(descriptors[0]),
      descriptors);

  ASSERT_NAPI_OK(status);

  return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, init)
