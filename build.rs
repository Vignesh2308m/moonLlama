use std::env;
use std::path::PathBuf;

fn main() {
    // Get the manifest directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let llama_cpp_path = PathBuf::from(&manifest_dir).join("llama_cpp");

    // Configure CMake build for llama.cpp
    let mut cmake_config = cmake::Config::new(&llama_cpp_path);
    
    // Platform-specific settings
    #[cfg(target_os = "windows")]
    {
        // Try to use Ninja if available, otherwise use Visual Studio 17 2022
        if std::process::Command::new("ninja")
            .arg("--version")
            .output()
            .is_ok()
        {
            cmake_config.generator("Ninja");
        } else {
            cmake_config.generator("Visual Studio 17 2022");
            cmake_config.define("CMAKE_SYSTEM_NAME", "Windows");
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        cmake_config.define("CMAKE_OSX_DEPLOYMENT_TARGET", "10.13");
    }
    
    #[cfg(target_os = "linux")]
    {
        // Linux-specific configuration if needed
    }

    // Common CMake settings - disable all executables and only build the library
    cmake_config.define("BUILD_SHARED_LIBS", "OFF");
    cmake_config.define("LLAMA_BUILD_EXAMPLES", "OFF");
    cmake_config.define("LLAMA_BUILD_TESTS", "OFF");
    cmake_config.define("LLAMA_BUILD_SERVER", "OFF");
    cmake_config.define("LLAMA_BUILD_APP", "OFF");
    cmake_config.define("LLAMA_BUILD_TOOLS", "OFF");

    // Build llama.cpp
    let dst = cmake_config.build();

    // Link the llama library
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=llama");
    
    // Link common dependencies
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=shell32");
    }
    
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Accelerate");
        println!("cargo:rustc-link-lib=framework=CoreML");
    }

    // Output the build directory for potential use in other scripts
    println!("cargo:rustc-env=LLAMA_CPP_BUILD_DIR={}", dst.display());
}