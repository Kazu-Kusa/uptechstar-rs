use libloading::Library;
use std::io::Write;
use tempfile::NamedTempFile;

use once_cell::sync::Lazy;

/// Global library instance for the Uptech hardware library.
///
/// This static variable provides a thread-safe, lazily-initialized instance of the
/// dynamically loaded Uptech hardware library (`libuptech.so`). The library is embedded
/// as a binary resource and extracted to a temporary file at runtime for loading.
///
/// # Library Loading Process
///
/// The library loading follows a secure 4-step process:
///
/// 1. **Resource Extraction**: The compiled `.so` library is embedded as a byte array
///    using `include_bytes!` macro, ensuring the library is bundled with the executable.
///
/// 2. **Temporary File Creation**: A secure temporary file is created using `NamedTempFile`
///    to store the extracted library bytes. This ensures proper cleanup and security.
///
/// 3. **Library Writing**: The embedded library bytes are written to the temporary file,
///    creating a valid shared object file that can be loaded by the system.
///
/// 4. **Dynamic Loading**: The library is loaded using `libloading::Library`, providing
///    access to all exported functions and symbols.
///
/// # Thread Safety
///
/// This library instance is thread-safe through the use of `once_cell::sync::Lazy`,
/// ensuring that the library is loaded exactly once regardless of concurrent access
/// from multiple threads.
///
/// # Panic Conditions
///
/// The library initialization will panic in the following scenarios:
/// - Failed to create a temporary file (system resource exhaustion, permissions)
/// - Failed to write library bytes to temporary file (disk space, I/O errors)
/// - Failed to load the shared library (missing dependencies, architecture mismatch)
///
/// # Usage
///
/// This library provides access to various hardware functions including:
/// - LCD/LED display operations (`lcd_open`, `lcd_close`, `LCD_Refresh`)
/// - Graphics rendering functions (`UG_*` family of functions)
/// - LED control functions (`adc_led_set`)
/// - Font and color management
///
/// # Example
///
/// ```rust,no_run
/// use crate::extern_lib::LIBRARY;
/// use libloading::Symbol;
///
/// unsafe {
///     let lcd_open: Symbol<unsafe extern "C" fn(i32) -> i32> = LIBRARY
///         .get(b"lcd_open")
///         .expect("Failed to load lcd_open function");
///     
///     lcd_open(1); // Open LCD in vertical mode
/// }
/// ```
///
/// # Safety
///
/// This static variable is marked as `unsafe` due to the inherent risks of dynamic
/// library loading and FFI operations. All function calls through this library
/// must be wrapped in `unsafe` blocks and proper error handling should be implemented.
///
/// # Platform Support
///
/// Currently supports Linux-based systems with the Uptech hardware platform.
/// The embedded library is architecture-specific and compiled for the target platform.
pub(crate) static LIBRARY: Lazy<Library> = Lazy::new(|| unsafe {
    // Step 1: Read the .so bytes from resources
    let so_bytes = include_bytes!("../lib/libuptech.so");

    // Step 2: Create a temporary file and write the .so content
    let mut tmp_file: NamedTempFile = NamedTempFile::new().expect("Failed to create temp file");
    tmp_file.write_all(so_bytes).expect("Failed to write .so to temp file");

    // Step 3: Get the temporary file path
    let so_path = tmp_file.into_temp_path();

    // Step 4: Load the .so library
    Library::new(so_path.as_os_str()).expect("Failed to load library")
});


