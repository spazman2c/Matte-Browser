cmake_minimum_required(VERSION 3.28.1)
project(MatteBrowser VERSION 0.1.0 LANGUAGES CXX Rust)

# Set C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Enable testing
enable_testing()

# Set build type if not specified
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()

# Platform-specific settings
if(WIN32)
    # Windows-specific settings
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /W4 /WX")
    set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} /MTd")
    set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /MT")
elseif(APPLE)
    # macOS-specific settings
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wpedantic")
    set(CMAKE_OSX_DEPLOYMENT_TARGET "13.0")
else()
    # Linux-specific settings
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wpedantic -fPIC")
endif()

# Enable sanitizers for debug builds
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    if(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
        set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -fsanitize=address,undefined,thread")
        set(CMAKE_EXE_LINKER_FLAGS_DEBUG "${CMAKE_EXE_LINKER_FLAGS_DEBUG} -fsanitize=address,undefined,thread")
    elseif(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
        set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -fsanitize=address,undefined,thread")
        set(CMAKE_EXE_LINKER_FLAGS_DEBUG "${CMAKE_EXE_LINKER_FLAGS_DEBUG} -fsanitize=address,undefined,thread")
    endif()
endif()

# Enable clang-tidy if available
find_program(CLANG_TIDY_EXE NAMES "clang-tidy")
if(CLANG_TIDY_EXE)
    set(CMAKE_CXX_CLANG_TIDY 
        ${CLANG_TIDY_EXE};
        -checks=*,-fuchsia-*,-google-*,-zircon-*,-abseil-*,-modernize-use-trailing-return-type;
        -header-filter=.*;
        -warnings-as-errors=*;
    )
    message(STATUS "clang-tidy found: ${CLANG_TIDY_EXE}")
else()
    message(STATUS "clang-tidy not found, skipping static analysis")
endif()

# Find required packages
find_package(PkgConfig REQUIRED)

# Platform-specific dependencies
if(WIN32)
    # Windows dependencies
    # Add Windows-specific package requirements here
elseif(APPLE)
    # macOS dependencies
    pkg_check_modules(REQUIRED REQUIRED
        x11
        xrandr
        xfixes
        xcursor
    )
else()
    # Linux dependencies
    pkg_check_modules(REQUIRED REQUIRED
        x11
        xrandr
        xfixes
        xcursor
        xcomposite
        xdamage
        xext
        xrender
        xss
        xtst
        nss3
        atk-bridge-2.0
        gtk+-3.0
        gdk-pixbuf-2.0
        alsa
        pulse
        dbus-1
        udev
        evdev
        input
        xkbcommon
        wayland-client
        xcb
        xcb-render0
        xcb-shape0
        xcb-xfixes0
        xcb-keysyms1
        xcb-icccm4
        xcb-image0
        xcb-shm0
        xcb-util1
        xcb-xinerama0
        xcb-xkb
        xkbcommon-x11
        vulkan
        egl
        glesv2
        gl
        glu
        drm
        gbm
        xcb-dri3
        xcb-present
        xcb-sync
        xcb-xf86dri
        xcb-dri2
        xcb-glx
        xcb-xv
        xcb-randr0
        xcb-record0
        xcb-xvmc
        xcb-xv0
        xcb-aux0
        xcb-event
        xcb-render-util0
        xcb-errors
    )
endif()

# Include directories
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/common/include)

# Add subdirectories for C++ components
# These will be added as we implement C++ components
# add_subdirectory(platform)
# add_subdirectory(graphics)
# add_subdirectory(network)

# Create a dummy target for now (will be replaced by actual C++ components)
add_library(matte_platform INTERFACE)
target_include_directories(matte_platform INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/include)

# Set up Rust integration
set(RUST_CARGO_TARGET_DIR "${CMAKE_CURRENT_BINARY_DIR}/rust-target")
set(ENV{CARGO_TARGET_DIR} "${RUST_CARGO_TARGET_DIR}")

# Find Rust
find_program(RUSTC rustc REQUIRED)
find_program(CARGO cargo REQUIRED)

# Get Rust target triple
execute_process(
    COMMAND ${RUSTC} --version --verbose
    OUTPUT_VARIABLE RUST_VERSION_OUTPUT
    ERROR_VARIABLE RUST_VERSION_ERROR
    RESULT_VARIABLE RUST_VERSION_RESULT
)

if(NOT RUST_VERSION_RESULT EQUAL 0)
    message(FATAL_ERROR "Failed to get Rust version: ${RUST_VERSION_ERROR}")
endif()

# Extract host target from Rust version output
string(REGEX MATCH "host: ([^ ]+)" RUST_HOST_MATCH "${RUST_VERSION_OUTPUT}")
if(RUST_HOST_MATCH)
    set(RUST_HOST_TARGET "${CMAKE_MATCH_1}")
    message(STATUS "Rust host target: ${RUST_HOST_TARGET}")
else()
    message(FATAL_ERROR "Could not determine Rust host target")
endif()

# Set up Rust build
set(RUST_BUILD_TYPE ${CMAKE_BUILD_TYPE})
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(RUST_BUILD_TYPE debug)
else()
    set(RUST_BUILD_TYPE release)
endif()

# Custom target to build Rust components
add_custom_target(rust-build ALL
    COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${RUST_CARGO_TARGET_DIR} ${CARGO} build --${RUST_BUILD_TYPE}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Building Rust components"
    VERBATIM
)

# Custom target to test Rust components
add_custom_target(rust-test
    COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${RUST_CARGO_TARGET_DIR} ${CARGO} test --${RUST_BUILD_TYPE}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Testing Rust components"
    VERBATIM
)

# Custom target to clean Rust build artifacts
add_custom_target(rust-clean
    COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${RUST_CARGO_TARGET_DIR} ${CARGO} clean
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Cleaning Rust build artifacts"
    VERBATIM
)

# Add tests
add_test(NAME RustTests COMMAND ${CMAKE_COMMAND} --build . --target rust-test)

# Install targets
install(TARGETS matte_platform
    EXPORT MatteBrowserTargets
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
    INCLUDES DESTINATION include
)

# Install headers
install(DIRECTORY include/ DESTINATION include)

# Export targets
install(EXPORT MatteBrowserTargets
    FILE MatteBrowserTargets.cmake
    NAMESPACE MatteBrowser::
    DESTINATION lib/cmake/MatteBrowser
)

# Create config file
include(CMakePackageConfigHelpers)
write_basic_package_version_file(
    MatteBrowserConfigVersion.cmake
    VERSION ${PROJECT_VERSION}
    COMPATIBILITY SameMajorVersion
)

configure_package_config_file(
    ${CMAKE_CURRENT_SOURCE_DIR}/cmake/MatteBrowserConfig.cmake.in
    ${CMAKE_CURRENT_BINARY_DIR}/MatteBrowserConfig.cmake
    INSTALL_DESTINATION lib/cmake/MatteBrowser
)

install(FILES
    ${CMAKE_CURRENT_BINARY_DIR}/MatteBrowserConfig.cmake
    ${CMAKE_CURRENT_BINARY_DIR}/MatteBrowserConfigVersion.cmake
    DESTINATION lib/cmake/MatteBrowser
)

# Print configuration summary
message(STATUS "")
message(STATUS "Matte Browser Configuration Summary:")
message(STATUS "  Version: ${PROJECT_VERSION}")
message(STATUS "  Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "  C++ compiler: ${CMAKE_CXX_COMPILER_ID} ${CMAKE_CXX_COMPILER_VERSION}")
message(STATUS "  Rust compiler: ${RUSTC}")
message(STATUS "  Rust target: ${RUST_HOST_TARGET}")
message(STATUS "  Rust build type: ${RUST_BUILD_TYPE}")
message(STATUS "  Rust target directory: ${RUST_CARGO_TARGET_DIR}")
if(CLANG_TIDY_EXE)
    message(STATUS "  clang-tidy: ${CLANG_TIDY_EXE}")
else()
    message(STATUS "  clang-tidy: not found")
endif()
message(STATUS "")
