idf_component_register(
    SRCS "include/RustApi.h"
    INCLUDE_DIRS "include"
    )

set(CARGO_BUILD_TYPE "release")
set(CARGO_BUILD_ARG "--release")
if(CONFIG_IDF_TARGET_ARCH_RISCV)
    set(CARGO_TARGET "riscv32i-unknown-none-elf")
    set(CARGO_FEATURES_ARG "")
elseif(CONFIG_IDF_TARGET_ARCH_XTENSA)
    set(CARGO_TARGET "xtensa-esp32-none-elf")
    set(CARGO_FEATURES_ARG "--features=std")
endif()

set(RUST_PROJECT_DIR "${CMAKE_CURRENT_LIST_DIR}")
set(RUST_BUILD_DIR "${CMAKE_CURRENT_BINARY_DIR}")
set(RUST_STATIC_LIBRARY "${RUST_BUILD_DIR}/${CARGO_TARGET}/${CARGO_BUILD_TYPE}/librustlib.a")

ExternalProject_Add(
    rustlib_project
    PREFIX "${RUST_PROJECT_DIR}"
    BUILD_COMMAND ${CMAKE_COMMAND} -E env
        CARGO_BUILD_TARGET=${CARGO_TARGET}
        CARGO_BUILD_TARGET_DIR=${RUST_BUILD_DIR}
        cargo build ${CARGO_BUILD_ARG} ${CARGO_FEATURES_ARG}
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    INSTALL_COMMAND ""
    BUILD_ALWAYS TRUE
    INSTALL_DIR "${RUST_BUILD_DIR}"
    STAMP_DIR "${RUST_BUILD_DIR}/stamp"
    TMP_DIR "${RUST_BUILD_DIR}/tmp"
    DOWNLOAD_DIR "${RUST_BUILD_DIR}"
    SOURCE_DIR "${RUST_PROJECT_DIR}"
    BUILD_BYPRODUCTS "${RUST_STATIC_LIBRARY}"
)

add_library(rustlib_prebuilt STATIC IMPORTED GLOBAL REQUIRES idf::clib)
add_dependencies(rustlib_prebuilt idf::clib rustlib_project)

set_target_properties(rustlib_prebuilt PROPERTIES IMPORTED_LOCATION "${RUST_STATIC_LIBRARY}")
