//! # UptechStar-RS - Rust Hardware Control Library
//!
//! UptechStar-RS is a comprehensive Rust library for interfacing with Uptech development boards
//! and their associated hardware components. This library provides safe, high-level abstractions
//! over the underlying C library (`libuptech.so`) while maintaining excellent performance and
//! memory safety guarantees.
//!
//! ## Overview
//!
//! This library enables Rust developers to interact with various hardware components commonly
//! found on Uptech development boards, including:
//!
//! - **ADC/IO Operations**: Analog-to-Digital Conversion and General Purpose Input/Output
//! - **Display Control**: LCD screen manipulation and graphics rendering
//! - **Motion Processing**: 6-axis motion sensing with MPU6500 integration
//!
//! ## Features
//!
//! - **Memory Safe**: Leverages Rust's ownership system to prevent common C interop issues
//! - **High Performance**: Direct FFI bindings with minimal overhead
//! - **Thread Safe**: All functions are designed for concurrent access
//! - **Well Documented**: Comprehensive documentation with examples
//! - **Error Handling**: Robust error reporting and logging integration
//!
//! ## Hardware Requirements
//!
//! - Uptech development board with compatible hardware modules
//! - Linux-based system (tested on embedded Linux distributions)
//! - `libuptech.so` shared library properly installed and accessible
//!
//! ## Quick Start
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! uptechstar-rs = "0.1.0"
//! log = "0.4"
//! env_logger = "0.10"
//! ```
//!
//! Basic usage example:
//!
//! ```rust,no_run
//! use uptechstar_rs::adc_io;
//! use uptechstar_rs::mpu;
//! use log::info;
//!
//! fn main() {
//!     env_logger::init();
//!     
//!     // Initialize ADC-IO system
//!     if adc_io::adc_open() >= 0 {
//!         info!("ADC-IO system initialized successfully");
//!         
//!         // Read ADC channels
//!         let mut adc_data = [0i32; 10];
//!         if adc_io::adc_get_all_channels(&mut adc_data).is_ok() {
//!             println!("ADC readings: {:?}", adc_data);
//!         }
//!         
//!         // Initialize MPU6500 motion sensor
//!         if mpu::mpu6500_open() == 0 {
//!             let mut accel_data = [0.0f32; 3];
//!             if mpu::mpu6500_get_accel(&mut accel_data) == 0 {
//!                 println!("Acceleration: X={:.2}g, Y={:.2}g, Z={:.2}g", 
//!                          accel_data[0], accel_data[1], accel_data[2]);
//!             }
//!         }
//!         
//!         // Clean up
//!         adc_io::adc_close();
//!     }
//! }
//! ```
//!
//! ## Module Overview
//!
//! ### [`adc_io`] - ADC and GPIO Operations
//!
//! Provides functions for:
//! - Analog-to-Digital Conversion (10 channels)
//! - General Purpose Input/Output control (8 channels)
//! - Digital I/O mode configuration (input/output)
//! - Real-time data acquisition
//!
//! Key functions:
//! - [`adc_io::adc_open()`] / [`adc_io::adc_close()`] - System initialization
//! - [`adc_io::adc_get_all_channels()`] - Read all ADC channels
//! - [`adc_io::set_all_io_levels()`] - Control GPIO output levels
//! - [`adc_io::set_io_mode()`] - Configure I/O pin modes
//!
//! ### [`display`] - LCD Display Control
//!
//! Comprehensive display management including:
//! - Screen initialization and configuration
//! - Text rendering with multiple font sizes
//! - Graphics primitives (lines, rectangles, circles)
//! - Color management and display orientation
//! - Chainable method calls for fluent API design
//!
//! Key features:
//! - [`display::Screen`] - Main display interface struct
//! - Multiple font sizes and color support
//! - Hardware-accelerated graphics operations
//! - Flexible screen orientation control
//!
//! ### [`mpu`] - Motion Processing Unit
//!
//! Advanced motion sensing capabilities:
//! - 6-axis motion detection (3-axis accelerometer + 3-axis gyroscope)
//! - Digital Motion Processor (DMP) integration
//! - Real-time attitude calculation (pitch, roll, yaw)
//! - Hardware sensor fusion algorithms
//!
//! Key functions:
//! - [`mpu::mpu6500_open()`] - Initialize MPU6500 with DMP
//! - [`mpu::mpu6500_get_accel()`] - Read acceleration data
//! - [`mpu::mpu6500_get_gyro()`] - Read angular velocity data
//! - [`mpu::mpu6500_get_attitude()`] - Get computed orientation angles
//!
//! ## Safety Considerations
//!
//! This library uses `unsafe` code internally to interface with the C library, but provides
//! safe abstractions to end users. Key safety measures include:
//!
//! - **Proper Initialization**: All hardware components must be initialized before use
//! - **Resource Management**: Automatic cleanup and proper resource lifecycle management
//! - **Error Handling**: Comprehensive error checking and reporting
//! - **Thread Safety**: All public functions are thread-safe
//!
//! ## Error Handling and Logging
//!
//! The library uses the `log` crate for comprehensive logging at different levels:
//!
//! - **Info**: System initialization and normal operations
//! - **Debug**: Detailed operational information
//! - **Error**: Hardware failures and communication issues
//!
//! Enable logging in your application:
//!
//! ```rust
//! use log::{info, debug, error};
//! use env_logger;
//!
//! fn main() {
//!     env_logger::init(); // Enable logging output
//!     // Your code here
//! }
//! ```
//!
//! ## Performance Characteristics
//!
//! - **ADC Sampling**: Up to 1kHz continuous sampling rate
//! - **Motion Sensing**: Up to 1kHz gyroscope/accelerometer, 200Hz attitude updates
//! - **Display Operations**: Hardware-accelerated graphics with minimal CPU overhead
//! - **Memory Usage**: Minimal heap allocations, stack-based data structures
//!
//! ## Platform Support
//!
//! Currently supported platforms:
//! - Linux (x86_64, ARM, RISC-V)
//! - Embedded Linux systems
//! - Uptech development boards
//!
//! ## Examples and Tutorials
//!
//! For comprehensive examples and tutorials, see the `examples/` directory in the
//! source repository. Common use cases include:
//!
//! - Sensor data logging and analysis
//! - Real-time motion tracking applications
//! - Interactive display applications
//! - IoT sensor nodes and data acquisition systems
//!
//! ## Contributing
//!
//! Contributions are welcome! Please see the project repository for contribution
//! guidelines and development setup instructions.
//!
//! ## License
//!
//! This project is licensed under the MIT License - see the LICENSE file for details.

mod extern_lib;
pub mod adc_io;
pub mod display;
pub mod mpu;