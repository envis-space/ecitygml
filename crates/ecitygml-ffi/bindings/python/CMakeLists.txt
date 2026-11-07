# Builds the nanobind extension module `ecitygml` from the sources Diplomat generates into src/.
# Meant to be run by scikit-build-core (`pip install .` / `uv pip install .`).
cmake_minimum_required(VERSION 3.15...3.31)
project(ecitygml_python LANGUAGES CXX)

find_package(Python 3.12 REQUIRED COMPONENTS Interpreter Development.Module OPTIONAL_COMPONENTS Development.SABIModule)
find_package(nanobind CONFIG REQUIRED)

# Static Rust library: the debug build (`cargo build -p ecitygml-ffi`), unless overridden with
# -DECITYGML_FFI_LIBRARY_DIR=... (`mise run build:bindings:python --release` passes target/release).
# A relative directory is relative to the workspace root, so that the wheel builds (build-wheels.yml) pass the same
# value on every platform. STRING, not PATH: CMake would resolve a relative PATH against the build directory.
get_filename_component(WORKSPACE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../../.." ABSOLUTE)
set(ECITYGML_FFI_LIBRARY_DIR "target/debug"
    CACHE STRING "Directory with the static library built from crates/ecitygml-ffi")
get_filename_component(ECITYGML_FFI_LIBRARY_DIR "${ECITYGML_FFI_LIBRARY_DIR}" ABSOLUTE BASE_DIR "${WORKSPACE_DIR}")
set(ECITYGML_FFI_LIBRARY
    "${ECITYGML_FFI_LIBRARY_DIR}/${CMAKE_STATIC_LIBRARY_PREFIX}ecitygml_ffi${CMAKE_STATIC_LIBRARY_SUFFIX}")
if(NOT EXISTS "${ECITYGML_FFI_LIBRARY}")
    message(FATAL_ERROR "${ECITYGML_FFI_LIBRARY} not found; run `cargo build -p ecitygml-ffi` (or with --release) first")
endif()

file(GLOB_RECURSE BINDING_SOURCES CONFIGURE_DEPENDS src/sub_modules/*.cpp)
nanobind_add_module(ecitygml STABLE_ABI NB_STATIC src/ecitygml_ext.cpp ${BINDING_SOURCES})
target_include_directories(ecitygml PRIVATE src/include "${NB_DIR}/ext/robin_map/include")
target_compile_features(ecitygml PRIVATE cxx_std_20)  # generated code uses std::bind_front

# Native libraries the Rust standard library needs (`--print native-static-libs`).
if(APPLE)
    set(RUST_NATIVE_LIBRARIES "-framework CoreFoundation" iconv)
elseif(WIN32)
    set(RUST_NATIVE_LIBRARIES kernel32 advapi32 ntdll userenv ws2_32 bcrypt dbghelp)
elseif(UNIX)
    set(RUST_NATIVE_LIBRARIES gcc_s util rt pthread m dl c)
endif()
target_link_libraries(ecitygml PRIVATE "${ECITYGML_FFI_LIBRARY}" ${RUST_NATIVE_LIBRARIES})

nanobind_add_stub(ecitygml_stub
    MODULE ecitygml
    OUTPUT ecitygml.pyi
    PYTHON_PATH $<TARGET_FILE_DIR:ecitygml>
    DEPENDS ecitygml)

install(TARGETS ecitygml LIBRARY DESTINATION .)
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/ecitygml.pyi" DESTINATION .)
