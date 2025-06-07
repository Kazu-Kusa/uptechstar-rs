use crate::extern_lib::LIBRARY;
use libloading::Symbol;

use log::{error, info};


// MPU section
pub fn mpu6500_open() -> i32 {
    // Initialize the 6-axis enhancer MPU6500
    // Default settings:
    //     acceleration: ±8G
    //     gyro: ±2000 degree/s
    //     sampling rate: 1kHz
    info!("Initializing MPU6500...");

    unsafe {
        let mpu6500_dmp_init: Symbol<unsafe extern "C" fn() -> i32> = LIBRARY
            .get(b"mpu6500_dmp_init")
            .expect("Failed to load mpu6500_dmp_init function");

        let result = mpu6500_dmp_init();

        if result != 0 {
            error!("Failed to initialize MPU6500. Do check if the channel is opened by calling 'adc_io_open()' and the libuptech.so being loaded properly");
            return result;
        }

        info!("MPU6500 initialized");
        result
    }
}

pub fn mpu6500_get_accel(accel_data: &mut [f32; 3]) -> i32 {
    // Retrieves the acceleration data from the MPU6500 sensor.
    //
    // Args:
    //     accel_data: Array to store acceleration data
    // Notes:
    //     length = 3
    //     [0] ==> axis X
    //     [1] ==> axis Y
    //     [2] ==> axis Z
    unsafe {
        let mpu6500_get_accel: Symbol<unsafe extern "C" fn(*mut f32) -> i32> = LIBRARY
            .get(b"mpu6500_Get_Accel")
            .expect("Failed to load mpu6500_Get_Accel function");

        mpu6500_get_accel(accel_data.as_mut_ptr())
    }
}

pub fn mpu6500_get_gyro(gyro_data: &mut [f32; 3]) -> i32 {
    // Retrieves the gyroscope data from the MPU6500 sensor.
    //
    // Args:
    //     gyro_data: Array to store gyroscope data
    // Notes:
    //     length = 3
    //     [0] ==> axis X
    //     [1] ==> axis Y
    //     [2] ==> axis Z
    unsafe {
        let mpu6500_get_gyro: Symbol<unsafe extern "C" fn(*mut f32) -> i32> = LIBRARY
            .get(b"mpu6500_Get_Gyro")
            .expect("Failed to load mpu6500_Get_Gyro function");

        mpu6500_get_gyro(gyro_data.as_mut_ptr())
    }
}

pub fn mpu6500_get_attitude(attitude_data: &mut [f32; 3]) -> i32 {
    // Retrieves the attitude data from the MPU6500 sensor.
    //
    // Args:
    //     attitude_data: Array to store attitude data
    // Notes:
    //     length = 3
    //     [0] ==> Pitch|axis X
    //     [1] ==> Roll |axis Y
    //     [2] ==> Yaw  |axis Z
    unsafe {
        let mpu6500_get_attitude: Symbol<unsafe extern "C" fn(*mut f32) -> i32> = LIBRARY
            .get(b"mpu6500_Get_Attitude")
            .expect("Failed to load mpu6500_Get_Attitude function");

        mpu6500_get_attitude(attitude_data.as_mut_ptr())
    }
}

pub fn mpu_get_gyro_fsr() -> u16 {
    // Retrieves the Full Scale Range (FSR) of the gyroscope.
    //
    // Returns:
    //     u16: The Full Scale Range value of the gyroscope.
    unsafe {
        let mut fsr_value: u16 = 0;
        let mpu_get_gyro_fsr: Symbol<unsafe extern "C" fn(*mut u16) -> i32> = LIBRARY
            .get(b"mpu_get_gyro_fsr")
            .expect("Failed to load mpu_get_gyro_fsr function");

        mpu_get_gyro_fsr(&mut fsr_value);
        fsr_value
    }
}

pub fn mpu_get_accel_fsr() -> u8 {
    // Retrieves the accelerometer full-scale range.
    //
    // Returns:
    //     u8: The value representing the accelerometer full-scale range.
    unsafe {
        let mut fsr_value: u8 = 0;
        let mpu_get_accel_fsr: Symbol<unsafe extern "C" fn(*mut u8) -> i32> = LIBRARY
            .get(b"mpu_get_accel_fsr")
            .expect("Failed to load mpu_get_accel_fsr function");

        mpu_get_accel_fsr(&mut fsr_value);
        fsr_value
    }
}

pub fn mpu_set_gyro_fsr(fsr: u32) -> i32 {
    // Sets the full-scale range for the gyroscope in the MPU.
    //
    // Parameters:
    //     fsr: Gyroscope full-scale range, can be one of 250, 500, 1000, or 2000 degrees per second.
    //
    // Returns:
    //     i32: 0 on success, non-zero on failure.
    unsafe {
        let mpu_set_gyro_fsr: Symbol<unsafe extern "C" fn(u32) -> i32> = LIBRARY
            .get(b"mpu_set_gyro_fsr")
            .expect("Failed to load mpu_set_gyro_fsr function");

        mpu_set_gyro_fsr(fsr)
    }
}

pub fn mpu_set_accel_fsr(fsr: i32) -> i32 {
    // Sets the accelerometer full-scale range (FSR) for the MPU.
    //
    // Parameters:
    //     fsr: The full-scale range of the accelerometer, with options being 2g, 4g, 8g, and 16g.
    //
    // Returns:
    //     i32: 0 on success, non-zero on failure.
    unsafe {
        let mpu_set_accel_fsr: Symbol<unsafe extern "C" fn(i32) -> i32> = LIBRARY
            .get(b"mpu_set_accel_fsr")
            .expect("Failed to load mpu_set_accel_fsr function");

        mpu_set_accel_fsr(fsr)
    }
}
