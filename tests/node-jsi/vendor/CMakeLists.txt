set(NAPI_JSI_DIR ../../../vendor/v8-jsi/src)

add_library(napi-jsi
  ${NAPI_JSI_DIR}/etw/tracing.cpp
  ${NAPI_JSI_DIR}/napi/js_native_api_v8.cc
  ${NAPI_JSI_DIR}/napi/js_native_ext_api_v8.cpp
  ${NAPI_JSI_DIR}/public/NapiJsiRuntime.cpp
  ${CMAKE_JS_SRC}
)

target_link_libraries(napi-jsi
  jsi
  ${CMAKE_JS_LIB}
)

target_include_directories(napi-jsi
  PRIVATE
    ${NODE_ADDON_API_DIR}
  PUBLIC
    ${NAPI_JSI_DIR}
    ${NAPI_JSI_DIR}/public
)
