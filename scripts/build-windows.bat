@echo off
setlocal enabledelayedexpansion

:: Matte Browser Windows Build Script
:: This script builds the Matte browser on Windows using MSVC

echo ========================================
echo Matte Browser - Windows Build Script
echo ========================================

:: Check if we're in the right directory
if not exist "Cargo.toml" (
    echo Error: Cargo.toml not found. Please run this script from the project root.
    exit /b 1
)

:: Set build configuration
set BUILD_TYPE=%1
if "%BUILD_TYPE%"=="" set BUILD_TYPE=release

:: Set target architecture
set TARGET_ARCH=%2
if "%TARGET_ARCH%"=="" set TARGET_ARCH=x86_64

:: Set MSVC environment
echo Setting up MSVC environment...
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat" 2>nul
if errorlevel 1 (
    call "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat" 2>nul
    if errorlevel 1 (
        echo Error: Could not find Visual Studio installation.
        echo Please install Visual Studio 2019 or 2022 with C++ development tools.
        exit /b 1
    )
)

:: Create build directory
set BUILD_DIR=build-windows
if not exist "%BUILD_DIR%" mkdir "%BUILD_DIR%"
cd "%BUILD_DIR%"

:: Configure CMake
echo Configuring CMake...
cmake .. -G "Visual Studio 17 2022" -A x64 -DCMAKE_BUILD_TYPE=%BUILD_TYPE% -DCMAKE_CXX_FLAGS="/W4 /WX"
if errorlevel 1 (
    echo Error: CMake configuration failed.
    exit /b 1
)

:: Build the project
echo Building Matte browser...
cmake --build . --config %BUILD_TYPE% --parallel
if errorlevel 1 (
    echo Error: Build failed.
    exit /b 1
)

:: Build Rust components
echo Building Rust components...
cd ..
cargo build --%BUILD_TYPE%
if errorlevel 1 (
    echo Error: Rust build failed.
    exit /b 1
)

:: Run tests
echo Running tests...
cargo test --%BUILD_TYPE%
if errorlevel 1 (
    echo Warning: Some tests failed.
)

echo ========================================
echo Build completed successfully!
echo ========================================
echo.
echo Build artifacts:
echo - C++ components: %BUILD_DIR%\bin\%BUILD_TYPE%\
echo - Rust components: target\%BUILD_TYPE%\
echo.
echo To run the browser:
echo cargo run --%BUILD_TYPE%
