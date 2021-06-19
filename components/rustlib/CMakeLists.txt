idf_component_register(
    SRCS "include/RustApi.h"
    INCLUDE_DIRS "include"
    PRIV_REQUIRES clib
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
    DEPENDS idf::clib
)

add_prebuilt_library(rustlib_prebuilt "${RUST_STATIC_LIBRARY}" PRIV_REQUIRES clib)
add_dependencies(rustlib_prebuilt rustlib_project)
target_link_libraries(${COMPONENT_LIB} PRIVATE rustlib_prebuilt)
