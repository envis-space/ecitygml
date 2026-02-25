cmake_minimum_required(VERSION 3.15)

# Only declare a project when built standalone (not as a subdirectory)
if(CMAKE_CURRENT_SOURCE_DIR STREQUAL CMAKE_SOURCE_DIR)
    project(ecitygml LANGUAGES CXX)
    set(CMAKE_CXX_STANDARD 17)
    set(CMAKE_CXX_STANDARD_REQUIRED ON)
endif()

set(RUST_FFI_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../..")
set(RUST_LIB_NAME "ecitygml_ffi")

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(RUST_LIB_PATH "${RUST_FFI_ROOT}/target/debug")
else()
    set(RUST_LIB_PATH "${RUST_FFI_ROOT}/target/release")
endif()

set(RUST_LIB_FILE "${RUST_LIB_PATH}/${CMAKE_SHARED_LIBRARY_PREFIX}${RUST_LIB_NAME}${CMAKE_SHARED_LIBRARY_SUFFIX}")

# Create imported library target for Rust FFI
add_library(ecitygml_rust SHARED IMPORTED GLOBAL)
set_target_properties(ecitygml_rust PROPERTIES
    IMPORTED_LOCATION "${RUST_LIB_FILE}"
    IMPORTED_NO_SONAME TRUE
)

# Create C++ wrapper library (INTERFACE since it's header-only)
add_library(ecitygml INTERFACE)
add_library(ecitygml::ecitygml ALIAS ecitygml)

target_include_directories(ecitygml
    INTERFACE
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
)
target_link_libraries(ecitygml INTERFACE ecitygml_rust)
target_compile_features(ecitygml INTERFACE cxx_std_17)

# --- Install rules ---
# include(GNUDirs OPTIONAL)

# install(TARGETS ecitygml
#     EXPORT ecitygmlTargets
# )

# install(DIRECTORY include/
#     DESTINATION include
# )

# install(FILES "${RUST_LIB_FILE}"
#     DESTINATION lib
# )

# Only build tests when this is the top-level project
if(CMAKE_CURRENT_SOURCE_DIR STREQUAL CMAKE_SOURCE_DIR)
    add_subdirectory(tests)
endif()
