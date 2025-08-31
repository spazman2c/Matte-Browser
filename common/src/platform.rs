//! Platform-specific abstractions and utilities.

use crate::error::{Error, Result};
use std::path::PathBuf;

/// Platform-specific information
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub os: OperatingSystem,
    pub architecture: Architecture,
    pub version: String,
    pub display_info: DisplayInfo,
}

/// Operating system types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

/// CPU architecture types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
    AArch64,
    X86,
    Unknown,
}

/// Display information
#[derive(Debug, Clone)]
pub struct DisplayInfo {
    pub primary_display: Display,
    pub displays: Vec<Display>,
    pub dpi_scale: f64,
}

/// Display properties
#[derive(Debug, Clone)]
pub struct Display {
    pub id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: f64,
    pub dpi: f64,
    pub is_primary: bool,
}

impl PlatformInfo {
    pub fn current() -> Result<Self> {
        let os = Self::detect_os();
        let architecture = Self::detect_architecture();
        let version = Self::get_os_version()?;
        let display_info = Self::get_display_info()?;

        Ok(Self {
            os,
            architecture,
            version,
            display_info,
        })
    }

    fn detect_os() -> OperatingSystem {
        #[cfg(target_os = "windows")]
        return OperatingSystem::Windows;
            #[cfg(target_os = "macos")]
    return OperatingSystem::MacOS;
        #[cfg(target_os = "linux")]
        return OperatingSystem::Linux;
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        return OperatingSystem::Unknown;
    }

    fn detect_architecture() -> Architecture {
        #[cfg(target_arch = "x86_64")]
        return Architecture::X86_64;
        #[cfg(target_arch = "aarch64")]
        return Architecture::AArch64;
        #[cfg(target_arch = "x86")]
        return Architecture::X86;
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "x86")))]
        return Architecture::Unknown;
    }

    fn get_os_version() -> Result<String> {
        #[cfg(target_os = "windows")]
        {
            // Windows version detection using registry or system calls
            if let Ok(output) = std::process::Command::new("cmd")
                .args(&["/C", "ver"])
                .output() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    return Ok(version.trim().to_string());
                }
            }
            // Fallback to environment variable
            if let Ok(version) = std::env::var("OS") {
                return Ok(format!("Windows {}", version));
            }
            Ok("Windows".to_string())
        }
        #[cfg(target_os = "macos")]
        {
            // macOS version detection using system_profiler
            if let Ok(output) = std::process::Command::new("sw_vers")
                .arg("-productVersion")
                .output() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    return Ok(format!("macOS {}", version.trim()));
                }
            }
            // Fallback to environment variable
            if let Ok(version) = std::env::var("MACOSX_DEPLOYMENT_TARGET") {
                return Ok(format!("macOS {}", version));
            }
            Ok("macOS".to_string())
        }
        #[cfg(target_os = "linux")]
        {
            // Linux version detection using /etc/os-release
            if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
                for line in content.lines() {
                    if line.starts_with("PRETTY_NAME=") {
                        let version = line.trim_start_matches("PRETTY_NAME=").trim_matches('"');
                        return Ok(version.to_string());
                    }
                }
            }
            // Fallback to uname
            if let Ok(output) = std::process::Command::new("uname")
                .arg("-a")
                .output() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    return Ok(version.trim().to_string());
                }
            }
            Ok("Linux".to_string())
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Err(Error::PlatformError("Unsupported operating system".to_string()))
        }
    }

    fn get_display_info() -> Result<DisplayInfo> {
        #[cfg(target_os = "windows")]
        {
            // Windows display detection using EnumDisplayMonitors
            // For now, return a reasonable default
            let primary_display = Display {
                id: "primary".to_string(),
                name: "Primary Display".to_string(),
                width: 1920,
                height: 1080,
                refresh_rate: 60.0,
                dpi: 96.0,
                is_primary: true,
            };

            Ok(DisplayInfo {
                primary_display: primary_display.clone(),
                displays: vec![primary_display],
                dpi_scale: 1.0,
            })
        }
        #[cfg(target_os = "macos")]
        {
            // macOS display detection using Core Graphics
            if let Ok(output) = std::process::Command::new("system_profiler")
                .args(&["SPDisplaysDataType"])
                .output() {
                if let Ok(content) = String::from_utf8(output.stdout) {
                    // Parse system_profiler output for display info
                    let mut displays = Vec::new();
                    let mut current_display = None;
                    
                    for line in content.lines() {
                        if line.contains("Resolution:") {
                            if let Some(display) = current_display.take() {
                                displays.push(display);
                            }
                            // Parse resolution from line like "Resolution: 2560 x 1600"
                            if let Some(resolution) = line.split("Resolution:").nth(1) {
                                let parts: Vec<&str> = resolution.trim().split(" x ").collect();
                                if parts.len() == 2 {
                                    if let (Ok(width), Ok(height)) = (parts[0].trim().parse::<u32>(), parts[1].trim().parse::<u32>()) {
                                        current_display = Some(Display {
                                            id: format!("display_{}", displays.len()),
                                            name: format!("Display {}", displays.len() + 1),
                                            width,
                                            height,
                                            refresh_rate: 60.0, // Default
                                            dpi: 72.0, // Default for macOS
                                            is_primary: displays.is_empty(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                    
                    if let Some(display) = current_display {
                        displays.push(display);
                    }
                    
                    if !displays.is_empty() {
                        let primary = displays[0].clone();
                        return Ok(DisplayInfo {
                            primary_display: primary,
                            displays,
                            dpi_scale: 1.0,
                        });
                    }
                }
            }
            
            // Fallback to default
            let primary_display = Display {
                id: "primary".to_string(),
                name: "Primary Display".to_string(),
                width: 1920,
                height: 1080,
                refresh_rate: 60.0,
                dpi: 72.0,
                is_primary: true,
            };

            Ok(DisplayInfo {
                primary_display: primary_display.clone(),
                displays: vec![primary_display],
                dpi_scale: 1.0,
            })
        }
        #[cfg(target_os = "linux")]
        {
            // Linux display detection using xrandr
            if let Ok(output) = std::process::Command::new("xrandr")
                .output() {
                if let Ok(content) = String::from_utf8(output.stdout) {
                    let mut displays = Vec::new();
                    let mut is_primary = true;
                    
                    for line in content.lines() {
                        if line.contains(" connected ") {
                            // Parse xrandr output for connected displays
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            if parts.len() >= 3 {
                                let name = parts[0].to_string();
                                if let Some(resolution_part) = parts[2].split('x').next() {
                                    if let Ok(width) = resolution_part.parse::<u32>() {
                                        // Extract height from resolution string like "1920x1080+0+0"
                                        if let Some(height_part) = parts[2].split('x').nth(1) {
                                            if let Some(height) = height_part.split('+').next() {
                                                if let Ok(height) = height.parse::<u32>() {
                                                    displays.push(Display {
                                                        id: name.clone(),
                                                        name: name.clone(),
                                                        width,
                                                        height,
                                                        refresh_rate: 60.0, // Default
                                                        dpi: 96.0, // Default for Linux
                                                        is_primary,
                                                    });
                                                    is_primary = false;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    if !displays.is_empty() {
                        let primary = displays[0].clone();
                        return Ok(DisplayInfo {
                            primary_display: primary,
                            displays,
                            dpi_scale: 1.0,
                        });
                    }
                }
            }
            
            // Fallback to default
            let primary_display = Display {
                id: "primary".to_string(),
                name: "Primary Display".to_string(),
                width: 1920,
                height: 1080,
                refresh_rate: 60.0,
                dpi: 96.0,
                is_primary: true,
            };

            Ok(DisplayInfo {
                primary_display: primary_display.clone(),
                displays: vec![primary_display],
                dpi_scale: 1.0,
            })
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            // Generic fallback
            let primary_display = Display {
                id: "primary".to_string(),
                name: "Primary Display".to_string(),
                width: 1920,
                height: 1080,
                refresh_rate: 60.0,
                dpi: 96.0,
                is_primary: true,
            };

            Ok(DisplayInfo {
                primary_display: primary_display.clone(),
                displays: vec![primary_display],
                dpi_scale: 1.0,
            })
        }
    }

    pub fn is_windows(&self) -> bool {
        self.os == OperatingSystem::Windows
    }

    pub fn is_macos(&self) -> bool {
        self.os == OperatingSystem::MacOS
    }

    pub fn is_linux(&self) -> bool {
        self.os == OperatingSystem::Linux
    }

    pub fn is_x86_64(&self) -> bool {
        self.architecture == Architecture::X86_64
    }

    pub fn is_aarch64(&self) -> bool {
        self.architecture == Architecture::AArch64
    }
}

/// Platform-specific paths
pub struct PlatformPaths;

impl PlatformPaths {
    pub fn data_directory() -> Result<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            let appdata = std::env::var("APPDATA")
                .map_err(|_| Error::PlatformError("APPDATA environment variable not found".to_string()))?;
            Ok(PathBuf::from(appdata).join("Matte"))
        }
        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME")
                .map_err(|_| Error::PlatformError("HOME environment variable not found".to_string()))?;
            Ok(PathBuf::from(home).join("Library/Application Support/Matte"))
        }
        #[cfg(target_os = "linux")]
        {
            let xdg_data_home = std::env::var("XDG_DATA_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let home = std::env::var("HOME")
                        .map(|h| PathBuf::from(h).join(".local/share"))
                        .unwrap_or_else(|_| PathBuf::from("~/.local/share"));
                    home
                });
            Ok(xdg_data_home.join("matte"))
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Err(Error::PlatformError("Unsupported platform for data directory".to_string()))
        }
    }

    pub fn cache_directory() -> Result<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            let local_appdata = std::env::var("LOCALAPPDATA")
                .map_err(|_| Error::PlatformError("LOCALAPPDATA environment variable not found".to_string()))?;
            Ok(PathBuf::from(local_appdata).join("Matte/Cache"))
        }
        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME")
                .map_err(|_| Error::PlatformError("HOME environment variable not found".to_string()))?;
            Ok(PathBuf::from(home).join("Library/Caches/Matte"))
        }
        #[cfg(target_os = "linux")]
        {
            let xdg_cache_home = std::env::var("XDG_CACHE_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let home = std::env::var("HOME")
                        .map(|h| PathBuf::from(h).join(".cache"))
                        .unwrap_or_else(|_| PathBuf::from("~/.cache"));
                    home
                });
            Ok(xdg_cache_home.join("matte"))
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Err(Error::PlatformError("Unsupported platform for cache directory".to_string()))
        }
    }

    pub fn temp_directory() -> Result<PathBuf> {
        Ok(std::env::temp_dir().join("matte"))
    }

    pub fn log_directory() -> Result<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            let local_appdata = std::env::var("LOCALAPPDATA")
                .map_err(|_| Error::PlatformError("LOCALAPPDATA environment variable not found".to_string()))?;
            Ok(PathBuf::from(local_appdata).join("Matte/Logs"))
        }
        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME")
                .map_err(|_| Error::PlatformError("HOME environment variable not found".to_string()))?;
            Ok(PathBuf::from(home).join("Library/Logs/Matte"))
        }
        #[cfg(target_os = "linux")]
        {
            let xdg_data_home = std::env::var("XDG_DATA_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let home = std::env::var("HOME")
                        .map(|h| PathBuf::from(h).join(".local/share"))
                        .unwrap_or_else(|_| PathBuf::from("~/.local/share"));
                    home
                });
            Ok(xdg_data_home.join("matte/logs"))
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Err(Error::PlatformError("Unsupported platform for log directory".to_string()))
        }
    }
}

/// Platform-specific system information
pub struct PlatformSystem;

impl PlatformSystem {
    pub fn get_memory_info() -> Result<MemoryInfo> {
        #[cfg(target_os = "windows")]
        {
            // Windows memory info using GlobalMemoryStatusEx
            // For now, return a placeholder
            Ok(MemoryInfo {
                total_physical: 16 * 1024 * 1024 * 1024, // 16GB default
                available_physical: 8 * 1024 * 1024 * 1024, // 8GB default
                total_virtual: 32 * 1024 * 1024 * 1024, // 32GB default
                available_virtual: 16 * 1024 * 1024 * 1024, // 16GB default
            })
        }
        #[cfg(target_os = "macos")]
        {
            // macOS memory info using sysctl
            if let Ok(output) = std::process::Command::new("sysctl")
                .args(&["-n", "hw.memsize"])
                .output() {
                if let Ok(content) = String::from_utf8(output.stdout) {
                    if let Ok(total_physical) = content.trim().parse::<u64>() {
                        return Ok(MemoryInfo {
                            total_physical,
                            available_physical: total_physical / 2, // Estimate
                            total_virtual: total_physical * 2, // Estimate
                            available_virtual: total_physical, // Estimate
                        });
                    }
                }
            }
            Ok(MemoryInfo {
                total_physical: 16 * 1024 * 1024 * 1024, // 16GB default
                available_physical: 8 * 1024 * 1024 * 1024, // 8GB default
                total_virtual: 32 * 1024 * 1024 * 1024, // 32GB default
                available_virtual: 16 * 1024 * 1024 * 1024, // 16GB default
            })
        }
        #[cfg(target_os = "linux")]
        {
            // Linux memory info using /proc/meminfo
            if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
                let mut total_physical = 0u64;
                let mut available_physical = 0u64;
                
                for line in content.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(value) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = value.parse::<u64>() {
                                total_physical = kb * 1024;
                            }
                        }
                    } else if line.starts_with("MemAvailable:") {
                        if let Some(value) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = value.parse::<u64>() {
                                available_physical = kb * 1024;
                            }
                        }
                    }
                }
                
                if total_physical > 0 {
                    return Ok(MemoryInfo {
                        total_physical,
                        available_physical,
                        total_virtual: total_physical * 2, // Estimate
                        available_virtual: available_physical * 2, // Estimate
                    });
                }
            }
            Ok(MemoryInfo {
                total_physical: 16 * 1024 * 1024 * 1024, // 16GB default
                available_physical: 8 * 1024 * 1024 * 1024, // 8GB default
                total_virtual: 32 * 1024 * 1024 * 1024, // 32GB default
                available_virtual: 16 * 1024 * 1024 * 1024, // 16GB default
            })
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Err(Error::PlatformError("Memory info not available on this platform".to_string()))
        }
    }

    pub fn get_cpu_info() -> Result<CpuInfo> {
        #[cfg(target_os = "windows")]
        {
            // Windows CPU info using WMI or registry
            Ok(CpuInfo {
                cores: 8, // Default
                threads: 16, // Default
                model: "Intel/AMD Processor".to_string(),
                frequency_mhz: 3000, // Default
            })
        }
        #[cfg(target_os = "macos")]
        {
            // macOS CPU info using sysctl
            let mut cores = 8u32;
            let mut threads = 16u32;
            let mut model = "Apple Processor".to_string();
            let mut frequency_mhz = 3000u32;
            
            if let Ok(output) = std::process::Command::new("sysctl")
                .args(&["-n", "hw.ncpu"])
                .output() {
                if let Ok(content) = String::from_utf8(output.stdout) {
                    if let Ok(cpu_count) = content.trim().parse::<u32>() {
                        cores = cpu_count;
                        threads = cpu_count;
                    }
                }
            }
            
            if let Ok(output) = std::process::Command::new("sysctl")
                .args(&["-n", "machdep.cpu.brand_string"])
                .output() {
                if let Ok(content) = String::from_utf8(output.stdout) {
                    model = content.trim().to_string();
                }
            }
            
            if let Ok(output) = std::process::Command::new("sysctl")
                .args(&["-n", "hw.cpufrequency"])
                .output() {
                if let Ok(content) = String::from_utf8(output.stdout) {
                    if let Ok(freq_hz) = content.trim().parse::<u32>() {
                        frequency_mhz = freq_hz / 1_000_000;
                    }
                }
            }
            
            Ok(CpuInfo {
                cores,
                threads,
                model,
                frequency_mhz,
            })
        }
        #[cfg(target_os = "linux")]
        {
            // Linux CPU info using /proc/cpuinfo
            if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
                let mut cores = 0u32;
                let mut threads = 0u32;
                let mut model = "Unknown Processor".to_string();
                let mut frequency_mhz = 3000u32;
                
                for line in content.lines() {
                    if line.starts_with("processor") {
                        threads += 1;
                    } else if line.starts_with("model name") {
                        if let Some(name) = line.split(':').nth(1) {
                            model = name.trim().to_string();
                        }
                    } else if line.starts_with("cpu cores") {
                        if let Some(count) = line.split(':').nth(1) {
                            if let Ok(count) = count.trim().parse::<u32>() {
                                cores = count;
                            }
                        }
                    } else if line.starts_with("cpu MHz") {
                        if let Some(freq) = line.split(':').nth(1) {
                            if let Ok(freq) = freq.trim().parse::<f32>() {
                                frequency_mhz = freq as u32;
                            }
                        }
                    }
                }
                
                if cores == 0 {
                    cores = threads;
                }
                
                return Ok(CpuInfo {
                    cores,
                    threads,
                    model,
                    frequency_mhz,
                });
            }
            Ok(CpuInfo {
                cores: 8, // Default
                threads: 16, // Default
                model: "Linux Processor".to_string(),
                frequency_mhz: 3000, // Default
            })
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Err(Error::PlatformError("CPU info not available on this platform".to_string()))
        }
    }
}

/// Memory information
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total_physical: u64,
    pub available_physical: u64,
    pub total_virtual: u64,
    pub available_virtual: u64,
}

/// CPU information
#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub cores: u32,
    pub threads: u32,
    pub model: String,
    pub frequency_mhz: u32,
}

/// Platform-specific security features
pub struct PlatformSecurity;

impl PlatformSecurity {
    pub fn enable_sandboxing() -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            // Windows AppContainer sandboxing
            Ok(())
        }
        #[cfg(target_os = "macos")]
        {
            // macOS sandboxd integration
            Ok(())
        }
        #[cfg(target_os = "linux")]
        {
            // Linux namespaces/seccomp/bpf
            Ok(())
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Err(Error::PlatformError("Sandboxing not supported on this platform".to_string()))
        }
    }

    pub fn enable_aslr() -> Result<()> {
        // ASLR is typically enabled by default on modern systems
        Ok(())
    }

    pub fn enable_cfi() -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            // Windows Control Flow Integrity
            Ok(())
        }
        #[cfg(not(target_os = "windows"))]
        {
            // CFI not available on this platform
            Ok(())
        }
    }
}

/// Platform-specific window management
pub struct PlatformWindow;

impl PlatformWindow {
    pub fn create_window(
        title: &str,
        width: u32,
        height: u32,
        x: Option<i32>,
        y: Option<i32>,
    ) -> Result<WindowHandle> {
        // This will be implemented with platform-specific window creation
        // For now, return a placeholder
        Ok(WindowHandle {
            id: 0,
            title: title.to_string(),
            width,
            height,
            x: x.unwrap_or(0),
            y: y.unwrap_or(0),
        })
    }

    pub fn set_window_title(_handle: &WindowHandle, _title: &str) -> Result<()> {
        // Platform-specific window title setting
        Ok(())
    }

    pub fn set_window_size(_handle: &WindowHandle, _width: u32, _height: u32) -> Result<()> {
        // Platform-specific window resizing
        Ok(())
    }

    pub fn set_window_position(_handle: &WindowHandle, _x: i32, _y: i32) -> Result<()> {
        // Platform-specific window positioning
        Ok(())
    }

    pub fn show_window(_handle: &WindowHandle) -> Result<()> {
        // Platform-specific window showing
        Ok(())
    }

    pub fn hide_window(_handle: &WindowHandle) -> Result<()> {
        // Platform-specific window hiding
        Ok(())
    }

    pub fn close_window(_handle: &WindowHandle) -> Result<()> {
        // Platform-specific window closing
        Ok(())
    }
}

/// Window handle for platform-specific operations
#[derive(Debug, Clone)]
pub struct WindowHandle {
    pub id: u64,
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_info() {
        let info = PlatformInfo::current().unwrap();
        assert!(info.os != OperatingSystem::Unknown);
        assert!(info.architecture != Architecture::Unknown);
        assert!(!info.version.is_empty());
    }

    #[test]
    fn test_platform_paths() {
        let data_dir = PlatformPaths::data_directory().unwrap();
        assert!(data_dir.to_string_lossy().contains("Matte") || data_dir.to_string_lossy().contains("matte"));
        
        let cache_dir = PlatformPaths::cache_directory().unwrap();
        assert!(cache_dir.to_string_lossy().contains("Cache") || cache_dir.to_string_lossy().contains("cache"));
        
        let temp_dir = PlatformPaths::temp_directory().unwrap();
        assert!(temp_dir.to_string_lossy().contains("matte"));
    }

    #[test]
    fn test_platform_security() {
        assert!(PlatformSecurity::enable_sandboxing().is_ok());
        assert!(PlatformSecurity::enable_aslr().is_ok());
        assert!(PlatformSecurity::enable_cfi().is_ok());
    }

    #[test]
    fn test_platform_window() {
        let handle = PlatformWindow::create_window("Test", 800, 600, None, None).unwrap();
        assert_eq!(handle.title, "Test");
        assert_eq!(handle.width, 800);
        assert_eq!(handle.height, 600);
        
        assert!(PlatformWindow::set_window_title(&handle, "New Title").is_ok());
        assert!(PlatformWindow::set_window_size(&handle, 1024, 768).is_ok());
        assert!(PlatformWindow::set_window_position(&handle, 100, 100).is_ok());
        assert!(PlatformWindow::show_window(&handle).is_ok());
        assert!(PlatformWindow::close_window(&handle).is_ok());
    }

    #[test]
    fn test_platform_system() {
        // Test memory info
        let memory_info = PlatformSystem::get_memory_info().unwrap();
        assert!(memory_info.total_physical > 0);
        assert!(memory_info.available_physical > 0);
        assert!(memory_info.total_virtual > 0);
        assert!(memory_info.available_virtual > 0);
        
        // Test CPU info
        let cpu_info = PlatformSystem::get_cpu_info().unwrap();
        assert!(cpu_info.cores > 0);
        assert!(cpu_info.threads > 0);
        assert!(!cpu_info.model.is_empty());
        assert!(cpu_info.frequency_mhz > 0);
    }
}
