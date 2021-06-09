idf_component_register(SRCS "main.c"
                    INCLUDE_DIRS "")

set(CargoBuildType "release")

# Import the Rust library.
if((${IDF_TARGET} MATCHES "esp32") OR (${IDF_TARGET} MATCHES "esp32s2"))
    set(ToolchainTarget "xtensa-esp32-none-elf")
elseif(${IDF_TARGET} MATCHES "esp32c3")
    set(ToolchainTarget "riscv32i-unknown-none-elf")
endif()

add_prebuilt_library(rustlib "../rustlib/target/${ToolchainTarget}/${CargoBuildType}/librustlib.a")

# 'main' calls a function from the library, so link it to 'main'
target_link_libraries(${COMPONENT_LIB} PRIVATE rustlib)
