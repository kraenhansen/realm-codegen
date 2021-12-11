cmake_minimum_required(VERSION 3.15)

project (addon)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# The -fdeclspec is used by the v8-jsi headers
set(CMAKE_CXX_FLAGS -fdeclspec)

# Improve developer experience by exporting compile commands
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Essential include files to build a node addon,
# you should add this line in every CMake.js based project.
include_directories(${CMAKE_JS_INC})

# Include N-API wrappers
execute_process(COMMAND node -p "require('node-addon-api').include"
  WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
  OUTPUT_VARIABLE NODE_ADDON_API_DIR
)
string(REPLACE "\n" "" NODE_ADDON_API_DIR ${NODE_ADDON_API_DIR})
string(REPLACE "\"" "" NODE_ADDON_API_DIR ${NODE_ADDON_API_DIR})

add_subdirectory(../../vendor/v8-jsi/src/jsi jsi)
add_subdirectory(vendor)

add_library(${PROJECT_NAME} SHARED
  src/addon.cpp
  ${CMAKE_JS_SRC}
)

# This line will give our library file a .node extension without any "lib" prefix
set_target_properties(${PROJECT_NAME} PROPERTIES PREFIX "" SUFFIX ".node")

target_link_libraries(${PROJECT_NAME}
  napi-jsi
  ${CMAKE_JS_LIB}
)

target_include_directories(${PROJECT_NAME} PRIVATE
  ${NODE_ADDON_API_DIR}
  ${NAPI_JSI_DIR}
  ${NAPI_JSI_DIR}/public
)

add_definitions(-DNAPI_VERSION=8)
