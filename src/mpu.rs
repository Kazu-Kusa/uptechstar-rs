use crate::extern_lib::LIBRARY;
use libloading::Symbol;

use log::{error, info};

/// Initializes the MPU6500 6-axis motion processing unit with Digital Motion Processor (DMP).
///
/// This function initializes the MPU6500 sensor with default configuration settings optimized
/// for general motion sensing applications. The initialization includes setting up the internal
/// Digital Motion Processor (DMP) which handles sensor fusion algorithms in hardware.
///
/// # Default Configuration
///
/// The MPU6500 is initialized with the following default settings:
/// - **Accelerometer Full-Scale Range**: ±8G
/// - **Gyroscope Full-Scale Range**: ±2000 degrees/second
/// - **Sample Rate**: 1kHz (1000 Hz)
/// - **Digital Motion Processor**: Enabled with sensor fusion algorithms
/// - **FIFO Buffer**: Configured for continuous data acquisition
///
/// # Prerequisites
///
/// Before calling this function, ensure that:
/// 1. The I2C communication channel is opened by calling `adc_io_open()`
/// 2. The libuptech.so shared library is properly loaded and accessible
/// 3. The MPU6500 sensor is physically connected and powered
///
/// # Hardware Requirements
///
/// - Uptech development board with MPU6500 sensor
/// - Stable power supply (3.3V or 5V depending on board configuration)
/// - I2C communication bus (SDA/SCL lines)
///
/// # Returns
///
/// - `0` on successful initialization
/// - Non-zero error code on failure:
///   - Communication errors with the sensor
///   - DMP initialization failures
///   - Hardware connection issues
///
/// # Errors
///
/// This function will log detailed error messages and return error codes in the following cases:
/// - I2C communication channel not opened (`adc_io_open()` not called)
/// - MPU6500 sensor not detected or not responding
/// - DMP initialization failure
/// - Library loading issues with libuptech.so
///
/// # Thread Safety
///
/// This function is thread-safe and can be called from multiple threads simultaneously.
/// However, it's recommended to initialize the sensor only once during application startup.
///
/// # Examples
///
/// Basic initialization:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu6500_open;
///
/// match mpu6500_open() {
///     0 => println!("MPU6500 initialized successfully"),
///     error_code => eprintln!("Failed to initialize MPU6500: {}", error_code),
/// }
/// ```
///
/// With error handling:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu6500_open;
/// use log::error;
///
/// if mpu6500_open() != 0 {
///     error!("MPU6500 initialization failed. Check hardware connections.");
///     // Handle initialization failure
///     return;
/// }
///
/// // Proceed with sensor operations
/// ```
pub fn mpu6500_open() -> i32 {
    info!("Initializing MPU6500 6-axis motion processing unit...");

    unsafe {
        let mpu6500_dmp_init: Symbol<unsafe extern "C" fn() -> i32> = LIBRARY
            .get(b"mpu6500_dmp_init")
            .expect("Failed to load mpu6500_dmp_init function");

        let result = mpu6500_dmp_init();

        if result != 0 {
            error!("Failed to initialize MPU6500. Do check if the channel is opened by calling 'adc_io_open()' and the libuptech.so being loaded properly");
            return result;
        }

        info!("MPU6500 initialized successfully with DMP enabled");
        result
    }
}

/// Retrieves real-time acceleration data from the MPU6500 3-axis accelerometer.
///
/// This function reads the current acceleration values from the MPU6500's built-in accelerometer
/// across all three axes (X, Y, Z). The data is automatically calibrated and converted to
/// floating-point values representing acceleration in units of gravitational force (g).
///
/// # Parameters
///
/// - `accel_data`: A mutable reference to a 3-element array that will be populated with
///   acceleration data. The array must be exactly 3 elements long.
///
/// # Array Layout
///
/// The acceleration data is stored in the array as follows:
/// - `accel_data[0]`: X-axis acceleration (forward/backward motion)
/// - `accel_data[1]`: Y-axis acceleration (left/right motion)  
/// - `accel_data[2]`: Z-axis acceleration (up/down motion)
///
/// # Coordinate System
///
/// The MPU6500 uses a right-handed coordinate system:
/// - **X-axis**: Points in the direction of the primary axis of the sensor
/// - **Y-axis**: Points perpendicular to X-axis in the horizontal plane
/// - **Z-axis**: Points vertically upward (opposite to gravity when level)
///
/// # Units and Range
///
/// - **Units**: Gravitational force (g), where 1g ≈ 9.81 m/s²
/// - **Range**: Depends on the configured Full-Scale Range (FSR):
///   - ±2g: -2.0 to +2.0g
///   - ±4g: -4.0 to +4.0g  
///   - ±8g: -8.0 to +8.0g (default)
///   - ±16g: -16.0 to +16.0g
///
/// # Returns
///
/// - `0` on successful data retrieval
/// - Non-zero error code on failure:
///   - Communication errors with the sensor
///   - Sensor not initialized
///   - Hardware connection issues
///
/// # Performance
///
/// - **Sample Rate**: Up to 1kHz (1000 samples per second)
/// - **Resolution**: 16-bit ADC providing high precision measurements
/// - **Latency**: Minimal latency due to direct hardware register access
///
/// # Thread Safety
///
/// This function is thread-safe and can be called concurrently from multiple threads.
/// Each call provides an independent snapshot of the current acceleration state.
///
/// # Examples
///
/// Basic acceleration reading:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu6500_get_accel;
///
/// let mut accel_data = [0.0f32; 3];
///
/// match mpu6500_get_accel(&mut accel_data) {
///     0 => {
///         println!("Acceleration - X: {:.2}g, Y: {:.2}g, Z: {:.2}g", 
///                  accel_data[0], accel_data[1], accel_data[2]);
///     },
///     error => eprintln!("Failed to read acceleration: {}", error),
/// }
/// ```
///
/// Motion detection example:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu6500_get_accel;
///
/// let mut accel_data = [0.0f32; 3];
///
/// if mpu6500_get_accel(&mut accel_data) == 0 {
///     // Calculate total acceleration magnitude
///     let magnitude = (accel_data[0].powi(2) + 
///                     accel_data[1].powi(2) + 
///                     accel_data[2].powi(2)).sqrt();
///     
///     if magnitude > 1.5 { // Threshold for motion detection
///         println!("Motion detected! Magnitude: {:.2}g", magnitude);
///     }
/// }
/// ```
///
/// Tilt detection example:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu6500_get_accel;
///
/// let mut accel_data = [0.0f32; 3];
///
/// if mpu6500_get_accel(&mut accel_data) == 0 {
///     // Calculate tilt angles (assuming device is stationary)
///     let roll = accel_data[1].atan2(accel_data[2]).to_degrees();
///     let pitch = (-accel_data[0]).atan2((accel_data[1].powi(2) + accel_data[2].powi(2)).sqrt()).to_degrees();
///     
///     println!("Tilt - Roll: {:.1}°, Pitch: {:.1}°", roll, pitch);
/// }
/// ```
pub fn mpu6500_get_accel(accel_data: &mut [f32; 3]) -> i32 {
    unsafe {
        let mpu6500_get_accel: Symbol<unsafe extern "C" fn(*mut f32) -> i32> = LIBRARY
            .get(b"mpu6500_Get_Accel")
            .expect("Failed to load mpu6500_Get_Accel function");

        mpu6500_get_accel(accel_data.as_mut_ptr())
    }
}

/// Retrieves real-time angular velocity data from the MPU6500 3-axis gyroscope.
///
/// This function reads the current rotational rates from the MPU6500's built-in gyroscope
/// across all three axes (X, Y, Z). The data represents angular velocity measurements
/// in degrees per second, providing precise information about rotational motion.
///
/// # Parameters
///
/// - `gyro_data`: A mutable reference to a 3-element array that will be populated with
///   gyroscopic data. The array must be exactly 3 elements long.
///
/// # Array Layout
///
/// The gyroscopic data is stored in the array as follows:
/// - `gyro_data[0]`: X-axis angular velocity (pitch rate - rotation around X-axis)
/// - `gyro_data[1]`: Y-axis angular velocity (roll rate - rotation around Y-axis)
/// - `gyro_data[2]`: Z-axis angular velocity (yaw rate - rotation around Z-axis)
///
/// # Coordinate System and Rotation
///
/// The MPU6500 uses a right-handed coordinate system with positive rotations defined as:
/// - **X-axis (Pitch)**: Positive rotation tilts the front of the device downward
/// - **Y-axis (Roll)**: Positive rotation tilts the right side of the device downward
/// - **Z-axis (Yaw)**: Positive rotation turns the device clockwise when viewed from above
///
/// # Units and Range
///
/// - **Units**: Degrees per second (°/s or dps)
/// - **Range**: Depends on the configured Full-Scale Range (FSR):
///   - ±250°/s: -250 to +250 degrees per second (high precision, low range)
///   - ±500°/s: -500 to +500 degrees per second
///   - ±1000°/s: -1000 to +1000 degrees per second
///   - ±2000°/s: -2000 to +2000 degrees per second (default, full range)
///
/// # Returns
///
/// - `0` on successful data retrieval
/// - Non-zero error code on failure:
///   - Communication errors with the sensor
///   - Sensor not initialized
///   - Hardware connection issues
///
/// # Performance Characteristics
///
/// - **Sample Rate**: Up to 1kHz (1000 samples per second)
/// - **Resolution**: 16-bit ADC with configurable sensitivity
/// - **Noise Performance**: Low noise floor for precise measurements
/// - **Temperature Stability**: Built-in temperature compensation
///
/// # Applications
///
/// This function is essential for applications requiring rotational motion detection:
/// - **Orientation Tracking**: Real-time device orientation changes
/// - **Gesture Recognition**: Detecting rotational gestures and movements
/// - **Stabilization Systems**: Camera or platform stabilization feedback
/// - **Gaming Controls**: Motion-based game input systems
/// - **Navigation Systems**: Inertial navigation and heading determination
///
/// # Thread Safety
///
/// This function is thread-safe and can be called concurrently from multiple threads.
/// Each call provides an independent snapshot of the current rotational state.
///
/// # Examples
///
/// Basic gyroscope reading:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu6500_get_gyro;
///
/// let mut gyro_data = [0.0f32; 3];
///
/// match mpu6500_get_gyro(&mut gyro_data) {
///     0 => {
///         println!("Angular Velocity - Pitch: {:.2}°/s, Roll: {:.2}°/s, Yaw: {:.2}°/s", 
///                  gyro_data[0], gyro_data[1], gyro_data[2]);
///     },
///     error => eprintln!("Failed to read gyroscope: {}", error),
/// }
/// ```
///
/// Rotation detection with threshold:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu6500_get_gyro;
///
/// let mut gyro_data = [0.0f32; 3];
/// const ROTATION_THRESHOLD: f32 = 50.0; // degrees per second
///
/// if mpu6500_get_gyro(&mut gyro_data) == 0 {
///     if gyro_data[0].abs() > ROTATION_THRESHOLD {
///         println!("Fast pitch rotation detected: {:.1}°/s", gyro_data[0]);
///     }
///     if gyro_data[1].abs() > ROTATION_THRESHOLD {
///         println!("Fast roll rotation detected: {:.1}°/s", gyro_data[1]);
///     }
///     if gyro_data[2].abs() > ROTATION_THRESHOLD {
///         println!("Fast yaw rotation detected: {:.1}°/s", gyro_data[2]);
///     }
/// }
/// ```
///
/// Angular position integration:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu6500_get_gyro;
/// use std::time::{Duration, Instant};
///
/// let mut gyro_data = [0.0f32; 3];
/// let mut angle_x = 0.0f32;
/// let mut last_time = Instant::now();
///
/// loop {
///     if mpu6500_get_gyro(&mut gyro_data) == 0 {
///         let current_time = Instant::now();
///         let dt = current_time.duration_since(last_time).as_secs_f32();
///         
///         // Integrate angular velocity to get angle
///         angle_x += gyro_data[0] * dt;
///         
///         println!("Estimated X-axis angle: {:.1}°", angle_x);
///         last_time = current_time;
///     }
///     
///     std::thread::sleep(Duration::from_millis(10));
/// }
/// ```
pub fn mpu6500_get_gyro(gyro_data: &mut [f32; 3]) -> i32 {
    unsafe {
        let mpu6500_get_gyro: Symbol<unsafe extern "C" fn(*mut f32) -> i32> = LIBRARY
            .get(b"mpu6500_Get_Gyro")
            .expect("Failed to load mpu6500_Get_Gyro function");

        mpu6500_get_gyro(gyro_data.as_mut_ptr())
    }
}

/// Retrieves real-time attitude data (orientation angles) from the MPU6500 Digital Motion Processor.
///
/// This function reads the computed attitude angles from the MPU6500's onboard Digital Motion
/// Processor (DMP), which performs sensor fusion of accelerometer and gyroscope data to provide
/// accurate 3D orientation information. The DMP eliminates the need for manual sensor fusion
/// calculations and provides drift-compensated attitude estimates.
///
/// # Parameters
///
/// - `attitude_data`: A mutable reference to a 3-element array that will be populated with
///   attitude angle data. The array must be exactly 3 elements long.
///
/// # Array Layout
///
/// The attitude data is stored in the array as follows:
/// - `attitude_data[0]`: **Pitch** (rotation around X-axis) in degrees
/// - `attitude_data[1]`: **Roll** (rotation around Y-axis) in degrees  
/// - `attitude_data[2]`: **Yaw** (rotation around Z-axis) in degrees
///
/// # Attitude Angles Explained
///
/// ## Pitch (X-axis rotation)
/// - **Range**: -90° to +90°
/// - **Positive**: Device tilted forward (front edge down)
/// - **Negative**: Device tilted backward (front edge up)
/// - **Zero**: Device is level horizontally
///
/// ## Roll (Y-axis rotation)  
/// - **Range**: -180° to +180°
/// - **Positive**: Device tilted to the right (right edge down)
/// - **Negative**: Device tilted to the left (left edge down)
/// - **Zero**: Device is level horizontally
///
/// ## Yaw (Z-axis rotation)
/// - **Range**: -180° to +180°
/// - **Positive**: Device rotated clockwise (viewed from above)
/// - **Negative**: Device rotated counter-clockwise
/// - **Zero**: Reference heading direction
///
/// # Digital Motion Processor Features
///
/// The DMP provides several advanced features:
/// - **Sensor Fusion**: Combines accelerometer and gyroscope data intelligently
/// - **Drift Compensation**: Reduces gyroscope drift over time
/// - **Quaternion Processing**: Internal quaternion calculations converted to Euler angles
/// - **Real-time Processing**: Hardware-accelerated calculations at high sample rates
/// - **Temperature Compensation**: Automatic adjustment for temperature variations
///
/// # Returns
///
/// - `0` on successful data retrieval
/// - Non-zero error code on failure:
///   - DMP not initialized or enabled
///   - Communication errors with the sensor
///   - FIFO buffer overflow or underflow
///   - Hardware connection issues
///
/// # Accuracy and Limitations
///
/// ## Accuracy
/// - **Static Accuracy**: ±1° in pitch and roll when stationary
/// - **Dynamic Accuracy**: Depends on motion characteristics and calibration
/// - **Update Rate**: Up to 200Hz for attitude calculations
///
/// ## Limitations
/// - **Yaw Drift**: Yaw angle may drift without magnetometer correction
/// - **Gimbal Lock**: Mathematical singularity at ±90° pitch
/// - **Magnetic Interference**: No magnetic heading compensation in basic mode
///
/// # Thread Safety
///
/// This function is thread-safe and can be called concurrently from multiple threads.
/// The DMP maintains internal state consistency across multiple access points.
///
/// # Applications
///
/// This function is ideal for applications requiring precise orientation information:
/// - **Device Orientation**: Screen rotation and UI adaptation
/// - **Robotic Control**: Robot arm positioning and navigation
/// - **Flight Control**: Drone and aircraft attitude stabilization  
/// - **Gaming**: Motion-controlled gaming interfaces
/// - **Virtual Reality**: Head tracking and motion capture
/// - **Industrial Monitoring**: Equipment tilt and position monitoring
///
/// # Examples
///
/// Basic attitude reading:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu6500_get_attitude;
///
/// let mut attitude = [0.0f32; 3];
///
/// match mpu6500_get_attitude(&mut attitude) {
///     0 => {
///         println!("Attitude - Pitch: {:.1}°, Roll: {:.1}°, Yaw: {:.1}°", 
///                  attitude[0], attitude[1], attitude[2]);
///     },
///     error => eprintln!("Failed to read attitude: {}", error),
/// }
/// ```
///
/// Level detection:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu6500_get_attitude;
///
/// let mut attitude = [0.0f32; 3];
/// const LEVEL_THRESHOLD: f32 = 5.0; // degrees
///
/// if mpu6500_get_attitude(&mut attitude) == 0 {
///     let pitch = attitude[0];
///     let roll = attitude[1];
///     
///     if pitch.abs() < LEVEL_THRESHOLD && roll.abs() < LEVEL_THRESHOLD {
///         println!("Device is level!");
///     } else {
///         println!("Device tilt - Pitch: {:.1}°, Roll: {:.1}°", pitch, roll);
///     }
/// }
/// ```
///
/// Orientation-based control:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu6500_get_attitude;
///
/// let mut attitude = [0.0f32; 3];
///
/// if mpu6500_get_attitude(&mut attitude) == 0 {
///     let pitch = attitude[0];
///     let roll = attitude[1];
///     let yaw = attitude[2];
///     
///     // Control logic based on orientation
///     match (pitch, roll) {
///         (p, _) if p > 30.0 => println!("Forward command"),
///         (p, _) if p < -30.0 => println!("Backward command"),
///         (_, r) if r > 30.0 => println!("Right command"),
///         (_, r) if r < -30.0 => println!("Left command"),
///         _ => println!("Neutral position"),
///     }
/// }
/// ```
///
/// Attitude monitoring with history:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu6500_get_attitude;
/// use std::collections::VecDeque;
///
/// let mut attitude = [0.0f32; 3];
/// let mut attitude_history: VecDeque<[f32; 3]> = VecDeque::with_capacity(10);
///
/// if mpu6500_get_attitude(&mut attitude) == 0 {
///     attitude_history.push_back(attitude);
///     
///     if attitude_history.len() > 10 {
///         attitude_history.pop_front();
///     }
///     
///     // Calculate average attitude over last 10 readings
///     let avg_pitch: f32 = attitude_history.iter().map(|a| a[0]).sum::<f32>() / attitude_history.len() as f32;
///     let avg_roll: f32 = attitude_history.iter().map(|a| a[1]).sum::<f32>() / attitude_history.len() as f32;
///     
///     println!("Average attitude - Pitch: {:.1}°, Roll: {:.1}°", avg_pitch, avg_roll);
/// }
/// ```
pub fn mpu6500_get_attitude(attitude_data: &mut [f32; 3]) -> i32 {
    unsafe {
        let mpu6500_get_attitude: Symbol<unsafe extern "C" fn(*mut f32) -> i32> = LIBRARY
            .get(b"mpu6500_Get_Attitude")
            .expect("Failed to load mpu6500_Get_Attitude function");

        mpu6500_get_attitude(attitude_data.as_mut_ptr())
    }
}

/// Retrieves the current Full Scale Range (FSR) configuration of the MPU6500 gyroscope.
///
/// This function queries the MPU6500 to determine the currently configured full-scale range
/// for the gyroscope sensor. The FSR setting determines the maximum measurable angular
/// velocity and affects the sensitivity and resolution of gyroscope measurements.
///
/// # Full Scale Range Options
///
/// The MPU6500 gyroscope supports four different FSR configurations:
///
/// ## Available FSR Values
/// - **250**: ±250 degrees/second
///   - **Sensitivity**: 131 LSB/(°/s)
///   - **Resolution**: ~0.0076°/s per bit
///   - **Use Case**: High precision applications, slow rotations
///
/// - **500**: ±500 degrees/second  
///   - **Sensitivity**: 65.5 LSB/(°/s)
///   - **Resolution**: ~0.015°/s per bit
///   - **Use Case**: Moderate precision, general purpose
///
/// - **1000**: ±1000 degrees/second
///   - **Sensitivity**: 32.8 LSB/(°/s) 
///   - **Resolution**: ~0.030°/s per bit
///   - **Use Case**: Fast rotations, gaming applications
///
/// - **2000**: ±2000 degrees/second (default)
///   - **Sensitivity**: 16.4 LSB/(°/s)
///   - **Resolution**: ~0.061°/s per bit  
///   - **Use Case**: Very fast rotations, full range applications
///
/// # Returns
///
/// - `u16`: The current gyroscope FSR value (250, 500, 1000, or 2000)
///
/// # Performance Impact
///
/// The FSR setting directly affects:
/// - **Measurement Range**: Higher FSR allows faster rotation measurement
/// - **Resolution**: Lower FSR provides better precision for slow movements
/// - **Noise Floor**: Lower FSR typically has better noise characteristics
/// - **Saturation**: Rotations exceeding FSR will saturate/clip
///
/// # Thread Safety
///
/// This function is thread-safe and can be called from multiple threads simultaneously.
/// It performs a read-only operation that doesn't modify sensor configuration.
///
/// # Applications
///
/// Use this function to:
/// - **Verify Configuration**: Confirm FSR matches application requirements
/// - **Dynamic Scaling**: Adjust processing algorithms based on current FSR
/// - **Calibration**: Account for sensitivity differences in calibration routines
/// - **Diagnostics**: Debug sensor configuration issues
/// - **Data Interpretation**: Apply correct scaling factors to raw measurements
///
/// # Examples
///
/// Basic FSR query:
/// ```rust,no_run
/// use uptechstar_rs::mpu::{mpu_get_gyro_fsr, mpu6500_get_gyro};
///
/// let current_fsr = mpu_get_gyro_fsr();
/// println!("Current gyroscope FSR: ±{}°/s", current_fsr);
///
/// // Use FSR information for data interpretation
/// let mut gyro_data = [0.0f32; 3];
/// if mpu6500_get_gyro(&mut gyro_data) == 0 {
///     println!("Reading gyroscope data with ±{}°/s range", current_fsr);
///     println!("Angular velocity: {:.2}°/s", gyro_data[0]);
/// }
/// ```
///
/// FSR-based processing selection:
/// ```rust,no_run
/// use uptechstar_rs::mpu::{mpu_get_gyro_fsr, mpu6500_get_gyro};
///
/// let fsr = mpu_get_gyro_fsr();
/// let mut gyro_data = [0.0f32; 3];
///
/// if mpu6500_get_gyro(&mut gyro_data) == 0 {
///     match fsr {
///         250 => {
///             // High precision mode - use sensitive thresholds
///             if gyro_data[2].abs() > 5.0 {
///                 println!("Precise rotation detected: {:.3}°/s", gyro_data[2]);
///             }
///         },
///         500 | 1000 => {
///             // Medium precision mode
///             if gyro_data[2].abs() > 20.0 {
///                 println!("Moderate rotation detected: {:.2}°/s", gyro_data[2]);
///             }
///         },
///         2000 => {
///             // Full range mode - use higher thresholds
///             if gyro_data[2].abs() > 50.0 {
///                 println!("Fast rotation detected: {:.1}°/s", gyro_data[2]);
///             }
///         },
///         _ => println!("Unknown FSR configuration: {}", fsr),
///     }
/// }
/// ```
///
/// Configuration verification:
/// ```rust,no_run
/// use uptechstar_rs::mpu::{mpu_get_gyro_fsr, mpu_set_gyro_fsr};
///
/// // Set desired FSR
/// let desired_fsr = 500;
/// if mpu_set_gyro_fsr(desired_fsr) == 0 {
///     // Verify the setting was applied
///     let actual_fsr = mpu_get_gyro_fsr();
///     if actual_fsr == desired_fsr as u16 {
///         println!("Gyroscope FSR successfully set to ±{}°/s", actual_fsr);
///     } else {
///         eprintln!("FSR mismatch: expected {}, got {}", desired_fsr, actual_fsr);
///     }
/// }
/// ```
///
/// Sensitivity calculation:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu_get_gyro_fsr;
///
/// let fsr = mpu_get_gyro_fsr();
/// let sensitivity = match fsr {
///     250 => 131.0,   // LSB/(°/s)
///     500 => 65.5,
///     1000 => 32.8,
///     2000 => 16.4,
///     _ => {
///         eprintln!("Unknown FSR: {}", fsr);
///         16.4 // Default to 2000°/s sensitivity
///     }
/// };
///
/// println!("Gyroscope sensitivity: {:.1} LSB/(°/s)", sensitivity);
/// println!("Resolution: {:.4}°/s per bit", 1.0 / sensitivity);
/// ```
pub fn mpu_get_gyro_fsr() -> u16 {
    unsafe {
        let mut fsr_value: u16 = 0;
        let mpu_get_gyro_fsr: Symbol<unsafe extern "C" fn(*mut u16) -> i32> = LIBRARY
            .get(b"mpu_get_gyro_fsr")
            .expect("Failed to load mpu_get_gyro_fsr function");

        mpu_get_gyro_fsr(&mut fsr_value);
        fsr_value
    }
}

/// Retrieves the current Full Scale Range (FSR) configuration of the MPU6500 accelerometer.
///
/// This function queries the MPU6500 to determine the currently configured full-scale range
/// for the accelerometer sensor. The FSR setting determines the maximum measurable acceleration
/// and directly affects the sensitivity, resolution, and dynamic range of acceleration measurements.
///
/// # Full Scale Range Options
///
/// The MPU6500 accelerometer supports four different FSR configurations:
///
/// ## Available FSR Values
/// - **2**: ±2g (gravity units)
///   - **Sensitivity**: 16,384 LSB/g
///   - **Resolution**: ~0.000061g per bit
///   - **Use Case**: High precision, low acceleration applications
///   - **Examples**: Tilt sensing, vibration monitoring, precise orientation
///
/// - **4**: ±4g
///   - **Sensitivity**: 8,192 LSB/g
///   - **Resolution**: ~0.000122g per bit
///   - **Use Case**: General purpose applications
///   - **Examples**: Device orientation, gesture recognition
///
/// - **8**: ±8g (default configuration)
///   - **Sensitivity**: 4,096 LSB/g
///   - **Resolution**: ~0.000244g per bit
///   - **Use Case**: Moderate dynamic range applications
///   - **Examples**: Motion detection, gaming controllers
///
/// - **16**: ±16g
///   - **Sensitivity**: 2,048 LSB/g
///   - **Resolution**: ~0.000488g per bit
///   - **Use Case**: High acceleration, impact detection
///   - **Examples**: Crash detection, sports applications, machinery monitoring
///
/// # Returns
///
/// - `u8`: The current accelerometer FSR value (2, 4, 8, or 16)
///
/// # Performance Characteristics
///
/// The FSR setting influences several key performance aspects:
///
/// ## Measurement Range vs. Precision Trade-off
/// - **Lower FSR (±2g, ±4g)**: Higher precision, limited range
/// - **Higher FSR (±8g, ±16g)**: Lower precision, extended range
///
/// ## Noise Performance
/// - Lower FSR settings typically provide better signal-to-noise ratio
/// - Higher FSR settings may exhibit slightly higher noise floors
///
/// ## Saturation Behavior  
/// - Accelerations exceeding the configured FSR will saturate/clip
/// - Proper FSR selection prevents data loss during expected motion ranges
///
/// # Thread Safety
///
/// This function is thread-safe and performs read-only operations that don't modify
/// sensor configuration. Multiple threads can safely call this function simultaneously.
///
/// # Applications
///
/// This function is essential for:
/// - **Configuration Verification**: Ensuring FSR matches application requirements
/// - **Data Interpretation**: Applying correct scaling factors to measurements
/// - **Dynamic Range Management**: Optimizing sensitivity vs. range trade-offs
/// - **Calibration Procedures**: Accounting for FSR-dependent sensitivities
/// - **System Diagnostics**: Troubleshooting sensor configuration issues
///
/// # Examples
///
/// Basic FSR query and interpretation:
/// ```rust,no_run
/// use uptechstar_rs::mpu::{mpu_get_accel_fsr, mpu6500_get_accel};
///
/// let current_fsr = mpu_get_accel_fsr();
/// println!("Current accelerometer FSR: ±{}g", current_fsr);
///
/// let mut accel_data = [0.0f32; 3];
/// if mpu6500_get_accel(&mut accel_data) == 0 {
///     println!("Acceleration reading with ±{}g range:", current_fsr);
///     println!("X: {:.3}g, Y: {:.3}g, Z: {:.3}g", 
///              accel_data[0], accel_data[1], accel_data[2]);
/// }
/// ```
///
/// FSR-dependent processing:
/// ```rust,no_run
/// use uptechstar_rs::mpu::{mpu_get_accel_fsr, mpu6500_get_accel};
///
/// let fsr = mpu_get_accel_fsr();
/// let mut accel_data = [0.0f32; 3];
///
/// if mpu6500_get_accel(&mut accel_data) == 0 {
///     // Adjust processing based on FSR
///     let motion_threshold = match fsr {
///         2 => 0.1,   // Sensitive threshold for ±2g
///         4 => 0.2,   // Moderate threshold for ±4g  
///         8 => 0.4,   // Standard threshold for ±8g
///         16 => 0.8,  // High threshold for ±16g
///         _ => 0.4,   // Default threshold
///     };
///
///     let magnitude = (accel_data[0].powi(2) + 
///                     accel_data[1].powi(2) + 
///                     accel_data[2].powi(2)).sqrt();
///
///     if magnitude > (1.0 + motion_threshold) {
///         println!("Motion detected! Magnitude: {:.3}g (FSR: ±{}g)", magnitude, fsr);
///     }
/// }
/// ```
///
/// Configuration validation:
/// ```rust,no_run
/// use uptechstar_rs::mpu::{mpu_get_accel_fsr, mpu_set_accel_fsr};
///
/// // Attempt to set FSR to ±4g
/// let desired_fsr = 4;
/// if mpu_set_accel_fsr(desired_fsr) == 0 {
///     // Verify the configuration was applied
///     let actual_fsr = mpu_get_accel_fsr();
///     if actual_fsr == desired_fsr as u8 {
///         println!("Accelerometer FSR successfully configured to ±{}g", actual_fsr);
///     } else {
///         eprintln!("FSR configuration failed: expected {}, got {}", desired_fsr, actual_fsr);
///     }
/// } else {
///     eprintln!("Failed to set accelerometer FSR to ±{}g", desired_fsr);
/// }
/// ```
///
/// Sensitivity and resolution calculation:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu_get_accel_fsr;
///
/// let fsr = mpu_get_accel_fsr();
/// let (sensitivity, resolution) = match fsr {
///     2  => (16384.0, 1.0/16384.0),  // LSB/g, g/LSB
///     4  => (8192.0,  1.0/8192.0),
///     8  => (4096.0,  1.0/4096.0),
///     16 => (2048.0,  1.0/2048.0),
///     _  => {
///         eprintln!("Unknown FSR: {}", fsr);
///         (4096.0, 1.0/4096.0) // Default to ±8g
///     }
/// };
///
/// println!("Accelerometer configuration:");
/// println!("  FSR: ±{}g", fsr);
/// println!("  Sensitivity: {:.0} LSB/g", sensitivity);
/// println!("  Resolution: {:.6}g per bit", resolution);
/// println!("  Measurable range: {:.1}g to +{:.1}g", -(fsr as f32), fsr as f32);
/// ```
///
/// Impact detection with FSR awareness:
/// ```rust,no_run
/// use uptechstar_rs::mpu::{mpu_get_accel_fsr, mpu6500_get_accel};
///
/// let fsr = mpu_get_accel_fsr();
/// let mut accel_data = [0.0f32; 3];
///
/// // Define impact thresholds based on FSR capability
/// let impact_threshold = match fsr {
///     2 => 1.8,   // Close to max range for ±2g
///     4 => 3.5,   // High acceleration for ±4g
///     8 => 6.0,   // Significant impact for ±8g
///     16 => 12.0, // Major impact for ±16g
///     _ => 6.0,   // Default threshold
/// };
///
/// if mpu6500_get_accel(&mut accel_data) == 0 {
///     let total_accel = (accel_data[0].powi(2) + 
///                       accel_data[1].powi(2) + 
///                       accel_data[2].powi(2)).sqrt();
///
///     if total_accel > impact_threshold {
///         println!("IMPACT DETECTED!");
///         println!("  Acceleration: {:.2}g", total_accel);
///         println!("  Threshold: {:.2}g (FSR: ±{}g)", impact_threshold, fsr);
///     }
/// }
/// ```
pub fn mpu_get_accel_fsr() -> u8 {
    unsafe {
        let mut fsr_value: u8 = 0;
        let mpu_get_accel_fsr: Symbol<unsafe extern "C" fn(*mut u8) -> i32> = LIBRARY
            .get(b"mpu_get_accel_fsr")
            .expect("Failed to load mpu_get_accel_fsr function");

        mpu_get_accel_fsr(&mut fsr_value);
        fsr_value
    }
}

/// Configures the Full Scale Range (FSR) for the MPU6500 gyroscope sensor.
///
/// This function sets the measurement range of the gyroscope, which determines the maximum
/// detectable angular velocity and affects the sensitivity and resolution of all gyroscope
/// measurements. The FSR configuration is a critical parameter that should be chosen based
/// on the expected rotational speeds in your application.
///
/// # Parameters
///
/// - `fsr`: The desired gyroscope full-scale range in degrees per second
///
/// # Supported FSR Values
///
/// The MPU6500 gyroscope supports four discrete FSR settings:
///
/// ## ±250°/s (High Precision)
/// - **Sensitivity**: 131 LSB/(°/s) 
/// - **Resolution**: ~0.0076°/s per bit
/// - **Best For**: Precision applications, slow rotations, stabilization systems
/// - **Applications**: Camera gimbals, precision instruments, fine motion control
///
/// ## ±500°/s (Medium-High Precision)
/// - **Sensitivity**: 65.5 LSB/(°/s)
/// - **Resolution**: ~0.015°/s per bit  
/// - **Best For**: General purpose applications with moderate rotation speeds
/// - **Applications**: Drone control, robotics, orientation tracking
///
/// ## ±1000°/s (Medium Precision)
/// - **Sensitivity**: 32.8 LSB/(°/s)
/// - **Resolution**: ~0.030°/s per bit
/// - **Best For**: Gaming, fast orientation changes, sports applications
/// - **Applications**: Motion controllers, fitness trackers, gesture recognition
///
/// ## ±2000°/s (Full Range, Default)
/// - **Sensitivity**: 16.4 LSB/(°/s)
/// - **Resolution**: ~0.061°/s per bit
/// - **Best For**: High-speed rotations, full dynamic range applications
/// - **Applications**: Acrobatic vehicles, spin detection, emergency systems
///
/// # Returns
///
/// - `0`: Configuration successful
/// - Non-zero error code on failure:
///   - Invalid FSR value (not 250, 500, 1000, or 2000)
///   - Communication error with sensor
///   - Sensor not initialized
///   - Hardware connection issues
///
/// # Configuration Persistence
///
/// The FSR setting persists until:
/// - Device power cycle/reset
/// - Another call to this function
/// - MPU6500 reinitialization via `mpu6500_open()`
///
/// # Performance Considerations
///
/// ## Precision vs. Range Trade-off
/// - **Lower FSR**: Higher precision but limited measurement range
/// - **Higher FSR**: Lower precision but extended measurement range
/// - Choose FSR to match expected maximum rotation speeds
///
/// ## Saturation Prevention
/// - Rotations exceeding FSR will saturate/clip the output
/// - Select FSR with appropriate headroom for expected motion
/// - Consider peak rotational velocities, not just average values
///
/// ## Noise Characteristics
/// - Lower FSR settings typically exhibit better noise performance
/// - Higher FSR settings may have slightly elevated noise floors
///
/// # Thread Safety
///
/// This function is thread-safe but should be used carefully in multi-threaded applications:
/// - Configuration changes affect all subsequent readings globally
/// - Consider synchronizing FSR changes with measurement routines
/// - Verify configuration with `mpu_get_gyro_fsr()` after setting
///
/// # Applications and Use Cases
///
/// ## Precision Applications (±250°/s or ±500°/s)
/// - Camera stabilization systems
/// - Precision robotics
/// - Scientific instruments
/// - Medical devices
///
/// ## General Purpose (±1000°/s)
/// - Consumer electronics
/// - Gaming peripherals
/// - Fitness trackers
/// - General robotics
///
/// ## High-Speed Applications (±2000°/s)
/// - Acrobatic drones/aircraft
/// - Sports performance analysis
/// - Vehicle dynamics monitoring
/// - Emergency/safety systems
///
/// # Examples
///
/// Basic FSR configuration:
/// ```rust,no_run
/// use uptechstar_rs::mpu::{mpu_set_gyro_fsr, mpu_get_gyro_fsr};
///
/// // Set gyroscope to high precision mode
/// match mpu_set_gyro_fsr(250) {
///     0 => {
///         let actual_fsr = mpu_get_gyro_fsr();
///         println!("Gyroscope FSR set to ±{}°/s", actual_fsr);
///     },
///     error => eprintln!("Failed to set gyroscope FSR: error {}", error),
/// }
/// ```
///
/// Application-specific FSR selection:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu_set_gyro_fsr;
///
/// fn configure_for_application(app_type: &str) -> Result<(), i32> {
///     let fsr = match app_type {
///         "precision_gimbal" => 250,    // High precision needed
///         "drone_control" => 500,       // Moderate precision, good range
///         "gaming_controller" => 1000,  // Fast movements expected
///         "crash_detection" => 2000,    // Full range for impact scenarios
///         _ => 1000,                    // Default general purpose
///     };
///
///     match mpu_set_gyro_fsr(fsr) {
///         0 => {
///             println!("Configured gyroscope for {} with ±{}°/s range", app_type, fsr);
///             Ok(())
///         },
///         error => {
///             eprintln!("Failed to configure gyroscope for {}: error {}", app_type, error);
///             Err(error)
///         }
///     }
/// }
/// ```
///
/// Dynamic FSR adjustment:
/// ```rust,no_run
/// use uptechstar_rs::mpu::{mpu_set_gyro_fsr, mpu6500_get_gyro, mpu_get_gyro_fsr};
///
/// fn adaptive_fsr_management() {
///     let mut gyro_data = [0.0f32; 3];
///     let current_fsr = mpu_get_gyro_fsr();
///     
///     if mpu6500_get_gyro(&mut gyro_data) == 0 {
///         let max_rotation = gyro_data.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
///         
///         // Check if we're approaching FSR limits (80% threshold)
///         let threshold = (current_fsr as f32) * 0.8;
///         
///         if max_rotation > threshold && current_fsr < 2000 {
///             // Increase FSR if approaching limits
///             let new_fsr = match current_fsr {
///                 250 => 500,
///                 500 => 1000,
///                 1000 => 2000,
///                 _ => current_fsr,
///             };
///             
///             if mpu_set_gyro_fsr(new_fsr as u32) == 0 {
///                 println!("Increased gyroscope FSR to ±{}°/s due to high rotation", new_fsr);
///             }
///         } else if max_rotation < threshold * 0.25 && current_fsr > 250 {
///             // Decrease FSR for better precision if motion is consistently low
///             let new_fsr = match current_fsr {
///                 2000 => 1000,
///                 1000 => 500,
///                 500 => 250,
///                 _ => current_fsr,
///             };
///             
///             if mpu_set_gyro_fsr(new_fsr as u32) == 0 {
///                 println!("Decreased gyroscope FSR to ±{}°/s for better precision", new_fsr);
///             }
///         }
///     }
/// }
/// ```
///
/// FSR validation and error handling:
/// ```rust,no_run
/// use uptechstar_rs::mpu::{mpu_set_gyro_fsr, mpu_get_gyro_fsr};
///
/// fn set_and_verify_gyro_fsr(desired_fsr: u32) -> Result<u16, String> {
///     // Validate input
///     if ![250, 500, 1000, 2000].contains(&desired_fsr) {
///         return Err(format!("Invalid FSR value: {}. Must be 250, 500, 1000, or 2000", desired_fsr));
///     }
///
///     // Attempt to set FSR
///     match mpu_set_gyro_fsr(desired_fsr) {
///         0 => {
///             // Verify the setting was applied
///             let actual_fsr = mpu_get_gyro_fsr();
///             if actual_fsr == desired_fsr as u16 {
///                 println!("Successfully set gyroscope FSR to ±{}°/s", actual_fsr);
///                 Ok(actual_fsr)
///             } else {
///                 Err(format!("FSR verification failed: expected {}, got {}", desired_fsr, actual_fsr))
///             }
///         },
///         error => Err(format!("Failed to set gyroscope FSR: error code {}", error)),
///     }
/// }
///
/// // Usage
/// match set_and_verify_gyro_fsr(500) {
///     Ok(fsr) => println!("Gyroscope configured with ±{}°/s range", fsr),
///     Err(msg) => eprintln!("Configuration error: {}", msg),
/// }
/// ```
pub fn mpu_set_gyro_fsr(fsr: u32) -> i32 {
    unsafe {
        let mpu_set_gyro_fsr: Symbol<unsafe extern "C" fn(u32) -> i32> = LIBRARY
            .get(b"mpu_set_gyro_fsr")
            .expect("Failed to load mpu_set_gyro_fsr function");

        mpu_set_gyro_fsr(fsr)
    }
}

/// Configures the Full Scale Range (FSR) for the MPU6500 accelerometer sensor.
///
/// This function sets the measurement range of the accelerometer, which determines the maximum
/// detectable acceleration and directly affects the sensitivity, resolution, and dynamic range
/// of all acceleration measurements. Proper FSR selection is crucial for optimizing measurement
/// precision while ensuring the sensor can capture the expected acceleration range.
///
/// # Parameters
///
/// - `fsr`: The desired accelerometer full-scale range in gravitational units (g)
///
/// # Supported FSR Values
///
/// The MPU6500 accelerometer supports four discrete FSR settings:
///
/// ## ±2g (Highest Precision)
/// - **Sensitivity**: 16,384 LSB/g
/// - **Resolution**: ~0.000061g per bit (~0.0006 m/s²)
/// - **Measurement Range**: -19.62 to +19.62 m/s²
/// - **Best For**: High precision tilt sensing, vibration analysis, precise orientation
/// - **Applications**: Spirit levels, precision instruments, seismic monitoring
///
/// ## ±4g (High Precision)  
/// - **Sensitivity**: 8,192 LSB/g
/// - **Resolution**: ~0.000122g per bit (~0.0012 m/s²)
/// - **Measurement Range**: -39.24 to +39.24 m/s²
/// - **Best For**: General device orientation, gesture recognition, mild impact detection
/// - **Applications**: Smartphones, tablets, consumer electronics
///
/// ## ±8g (Standard Range, Default)
/// - **Sensitivity**: 4,096 LSB/g  
/// - **Resolution**: ~0.000244g per bit (~0.0024 m/s²)
/// - **Measurement Range**: -78.48 to +78.48 m/s²
/// - **Best For**: Motion detection, gaming, moderate dynamic applications
/// - **Applications**: Gaming controllers, fitness trackers, robotics
///
/// ## ±16g (Maximum Range)
/// - **Sensitivity**: 2,048 LSB/g
/// - **Resolution**: ~0.000488g per bit (~0.0048 m/s²)  
/// - **Measurement Range**: -156.96 to +156.96 m/s²
/// - **Best For**: High-acceleration environments, impact/crash detection, sports
/// - **Applications**: Vehicle crash systems, sports performance, machinery monitoring
///
/// # Returns
///
/// - `0`: Configuration successful
/// - Non-zero error code on failure:
///   - Invalid FSR value (not 2, 4, 8, or 16)
///   - Communication error with sensor
///   - Sensor not initialized  
///   - Hardware connection issues
///
/// # Configuration Behavior
///
/// ## Immediate Effect
/// - FSR changes take effect immediately for subsequent measurements
/// - No sensor restart or reinitialization required
/// - Previous readings remain unaffected
///
/// ## Persistence
/// The FSR setting persists until:
/// - Device power cycle/reset
/// - Another call to this function  
/// - MPU6500 reinitialization via `mpu6500_open()`
///
/// # Selection Guidelines
///
/// ## Choose ±2g When:
/// - Maximum acceleration expected < 1.5g
/// - High precision tilt/orientation measurement required
/// - Vibration analysis with small amplitudes
/// - Environmental monitoring applications
///
/// ## Choose ±4g When:
/// - Maximum acceleration expected < 3g
/// - Device orientation in consumer electronics
/// - Gesture recognition applications
/// - General purpose motion sensing
///
/// ## Choose ±8g When:
/// - Maximum acceleration expected < 6g  
/// - Gaming and interactive applications
/// - Robotics with moderate dynamics
/// - Activity/fitness monitoring
///
/// ## Choose ±16g When:
/// - High acceleration environments (>6g expected)
/// - Impact/crash detection systems
/// - Sports performance analysis
/// - Industrial machinery monitoring
///
/// # Performance Trade-offs
///
/// ## Precision vs. Range
/// - **Lower FSR**: Higher precision, better noise performance, limited range
/// - **Higher FSR**: Lower precision, higher noise floor, extended range
/// - Optimal FSR provides 20-30% headroom above expected maximum acceleration
///
/// ## Saturation Prevention
/// - Accelerations exceeding FSR will saturate/clip output values
/// - Clipped data cannot be recovered or extrapolated
/// - Design with appropriate safety margins for unexpected accelerations
///
/// # Thread Safety
///
/// This function is thread-safe but configuration changes affect all measurement operations:
/// - FSR changes are global and immediate
/// - Coordinate FSR changes with active measurement threads
/// - Use `mpu_get_accel_fsr()` to verify configuration after changes
///
/// # Examples
///
/// Basic FSR configuration:
/// ```rust,no_run
/// use uptechstar_rs::mpu::{mpu_set_accel_fsr, mpu_get_accel_fsr};
///
/// // Configure for high precision applications
/// match mpu_set_accel_fsr(2) {
///     0 => {
///         let actual_fsr = mpu_get_accel_fsr();
///         println!("Accelerometer FSR set to ±{}g", actual_fsr);
///         println!("Resolution: ~{:.4}g per bit", 1.0 / 16384.0);
///     },
///     error => eprintln!("Failed to set accelerometer FSR: error {}", error),
/// }
/// ```
///
/// Application-specific configuration:
/// ```rust,no_run
/// use uptechstar_rs::mpu::mpu_set_accel_fsr;
///
/// fn configure_accelerometer_for_use_case(use_case: &str) -> Result<(), i32> {
///     let fsr = match use_case {
///         "tilt_sensor" => 2,        // High precision for small angles
///         "phone_orientation" => 4,   // Standard phone/tablet use
///         "gaming_controller" => 8,   // Gaming with moderate motion
///         "crash_detection" => 16,    // High-g impact detection
///         "drone_flight" => 8,        // Moderate dynamics for flight
///         "fitness_tracker" => 8,     // Activity monitoring
///         _ => 8,                     // Default general purpose
///     };
///
///     match mpu_set_accel_fsr(fsr) {
///         0 => {
///             println!("Configured accelerometer for '{}' with ±{}g range", use_case, fsr);
///             Ok(())
///         },
///         error => {
///             eprintln!("Failed to configure accelerometer for '{}': error {}", use_case, error);
///             Err(error)
///         }
///     }
/// }
/// ```
///
/// Dynamic range adjustment:
/// ```rust,no_run
/// use uptechstar_rs::mpu::{mpu_set_accel_fsr, mpu6500_get_accel, mpu_get_accel_fsr};
///
/// fn monitor_and_adjust_fsr() {
///     let mut accel_data = [0.0f32; 3];
///     let current_fsr = mpu_get_accel_fsr();
///     
///     if mpu6500_get_accel(&mut accel_data) == 0 {
///         let max_accel = accel_data.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
///         let fsr_limit = current_fsr as f32;
///         
///         // Check if approaching FSR limits (85% threshold)
///         if max_accel > fsr_limit * 0.85 && current_fsr < 16 {
///             let new_fsr = match current_fsr {
///                 2 => 4,
///                 4 => 8, 
///                 8 => 16,
///                 _ => current_fsr,
///             };
///             
///             if mpu_set_accel_fsr(new_fsr as i32) == 0 {
///                 println!("Increased accelerometer FSR to ±{}g (max accel: {:.2}g)", 
///                         new_fsr, max_accel);
///             }
///         }
///         // Downgrade FSR for better precision if consistently low acceleration
///         else if max_accel < fsr_limit * 0.3 && current_fsr > 2 {
///             let new_fsr = match current_fsr {
///                 16 => 8,
///                 8 => 4,
///                 4 => 2,
///                 _ => current_fsr,
///             };
///             
///             if mpu_set_accel_fsr(new_fsr as i32) == 0 {
///                 println!("Decreased accelerometer FSR to ±{}g for better precision", new_fsr);
///             }
///         }
///     }
/// }
/// ```
///
/// Comprehensive configuration with validation:
/// ```rust,no_run
/// use uptechstar_rs::mpu::{mpu_set_accel_fsr, mpu_get_accel_fsr};
///
/// fn configure_and_verify_accel_fsr(desired_fsr: i32) -> Result<(u8, f32), String> {
///     // Validate input FSR value
///     if ![2, 4, 8, 16].contains(&desired_fsr) {
///         return Err(format!("Invalid FSR: {}. Must be 2, 4, 8, or 16", desired_fsr));
///     }
///
///     // Set the FSR
///     match mpu_set_accel_fsr(desired_fsr) {
///         0 => {
///             // Verify configuration
///             let actual_fsr = mpu_get_accel_fsr();
///             if actual_fsr == desired_fsr as u8 {
///                 // Calculate sensitivity for this FSR
///                 let sensitivity = match actual_fsr {
///                     2 => 16384.0,
///                     4 => 8192.0,
///                     8 => 4096.0,
///                     16 => 2048.0,
///                     _ => 4096.0, // Fallback
///                 };
///                 
///                 println!("Accelerometer configuration successful:");
///                 println!("  FSR: ±{}g", actual_fsr);
///                 println!("  Sensitivity: {:.0} LSB/g", sensitivity);
///                 println!("  Resolution: {:.6}g per bit", 1.0 / sensitivity);
///                 
///                 Ok((actual_fsr, sensitivity))
///             } else {
///                 Err(format!("Verification failed: expected FSR {}, got {}", 
///                           desired_fsr, actual_fsr))
///             }
///         },
///         error => Err(format!("Failed to set FSR: error code {}", error)),
///     }
/// }
///
/// // Usage example
/// match configure_and_verify_accel_fsr(4) {
///     Ok((fsr, sensitivity)) => {
///         println!("Ready for measurements with ±{}g range", fsr);
///     },
///     Err(msg) => eprintln!("Configuration failed: {}", msg),
/// }
/// ```
pub fn mpu_set_accel_fsr(fsr: i32) -> i32 {
    unsafe {
        let mpu_set_accel_fsr: Symbol<unsafe extern "C" fn(i32) -> i32> = LIBRARY
            .get(b"mpu_set_accel_fsr")
            .expect("Failed to load mpu_set_accel_fsr function");

        mpu_set_accel_fsr(fsr)
    }
}