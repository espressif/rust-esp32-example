# The following lines of boilerplate have to be in your project's
# CMakeLists in this exact order for cmake to work correctly
cmake_minimum_required(VERSION 3.5)

include($ENV{IDF_PATH}/tools/cmake/project.cmake)
project(esp32-hello-rust)

# The Xtensa Rust build includes compiler intrinsics that conflict with names
# in GCC. To avoid multiple definition errors, allow the linker to pick one of
# the available options and ignore the others.
if((${IDF_TARGET} MATCHES "esp32") OR (${IDF_TARGET} MATCHES "esp32s2"))
    target_link_options(${CMAKE_PROJECT_NAME}.elf PRIVATE "-Wl,--allow-multiple-definition")
endif()
