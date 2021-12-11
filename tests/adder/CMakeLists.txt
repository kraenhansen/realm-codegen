cmake_minimum_required(VERSION 3.15)

# TODO: Update the name
project (adder-addon)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Improve developer experience by exporting compile commands
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

include_directories(${CMAKE_JS_INC})

# Essential include files to build a node addon,
# you should add this line in every CMake.js based project.
include_directories(
  ${CMAKE_JS_INC}
  src
)

add_subdirectory(src/vendor)
add_subdirectory(src/generated/cpp)

target_link_libraries(${PROJECT_NAME}
  vendor
)
