# Find or fetch Catch2
find_package(Catch2 3 QUIET)
if(NOT Catch2_FOUND)
    include(FetchContent)
    FetchContent_Declare(
            Catch2
            GIT_REPOSITORY https://github.com/catchorg/Catch2.git
            GIT_TAG v3.5.2
    )
    FetchContent_MakeAvailable(Catch2)
endif()

# Create test executable
add_executable(ecitygml_tests
        test_io.cpp
)

target_link_libraries(ecitygml_tests
        PRIVATE ecitygml
        PRIVATE Catch2::Catch2WithMain
)

# Discover tests
include(CTest)
include(Catch)
catch_discover_tests(ecitygml_tests)
