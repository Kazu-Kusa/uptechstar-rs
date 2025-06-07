use crate::extern_lib::LIBRARY;
use libloading::Symbol;

use log::{debug, error, info};

/// Opens the ADC-IO plug.
///
/// This function initializes the ADC-IO interface by loading and invoking the `adc_io_open` function
/// from the external shared library. It logs initialization status and handles potential failures.
///
/// # Returns
///
/// * `i32` - The number of times the ADC-IO has been opened. Returns `-1` if the operation fails.
///
/// # Safety
///
/// This function uses unsafe code to interact with a C library. Ensure that the shared library ([libuptech.so](file://L:\RustProjects\uptechstar-rs\lib\libuptech.so))
/// is properly loaded and the `adc_io_open` function is available.
pub fn adc_open() -> i32 {
    info!("Initializing ADC-IO");

    unsafe {
        let adc_io_open: Symbol<unsafe extern "C" fn() -> i32> = LIBRARY
            .get(b"adc_io_open")
            .expect("Failed to load adc_io_open function");

        let open_times = adc_io_open();

        if open_times == -1 {
            error!(
                "Failed to open ADC-IO. Do check if the channel is opened by calling 'adc_io_open()' \
                 and the libuptech.so being loaded properly"
            );
        } else {
            debug!("ADC-IO open {} times", open_times);
        }

        open_times
    }
}

/// Closes the ADC-IO plug.
///
/// This function terminates the ADC-IO interface by loading and invoking the `adc_io_close` function
/// from the external shared library. It logs closure status and handles potential failures.
///
/// # Returns
///
/// * `i32` - Returns `0` on success, `-1` on failure.
///
/// # Safety
///
/// This function uses unsafe code to interact with a C library. Ensure that the shared library ([libuptech.so](file://L:\RustProjects\uptechstar-rs\lib\libuptech.so))
/// is properly loaded and the `adc_io_close` function is available.
pub fn adc_close() -> i32 {
    info!("Closing ADC-IO");

    unsafe {
        let adc_io_close: Symbol<unsafe extern "C" fn() -> i32> = LIBRARY
            .get(b"adc_io_close")
            .expect("Failed to load adc_io_close function");

        let result = adc_io_close();

        if result == -1 {
            error!(
                "Failed to close ADC-IO. Do check if the channel is opened by calling 'adc_io_open()' \
                 and the libuptech.so being loaded properly"
            );
        } else {
            debug!("ADC-IO closed");
        }

        result
    }
}

/// Retrieves all ADC channels' data.
///
/// This function loads and invokes the `ADC_GetAll` function from the external shared library to fetch
/// data for all 10 ADC channels. It logs errors if the operation fails.
///
/// # Arguments
///
/// * `adc_data` - A mutable array of length 10 to store the retrieved ADC channel data.
///
/// # Returns
///
/// * `Result<(), &'static str>` - Returns `Ok(())` on success, or an error message on failure.
///
/// # Safety
///
/// This function uses unsafe code to interact with a C library. Ensure that the shared library ([libuptech.so](file://L:\RustProjects\uptechstar-rs\lib\libuptech.so))
/// is properly loaded and the `ADC_GetAll` function is available.
pub fn adc_get_all_channels(adc_data: &mut [i32; 10]) -> Result<(), &'static str> {
    unsafe {
        let adc_get_all: Symbol<unsafe extern "C" fn(*mut i32) -> i32> = LIBRARY
            .get(b"ADC_GetAll")
            .expect("Failed to load ADC_GetAll function");

        let result = adc_get_all(adc_data.as_mut_ptr());

        if result != 0 {
            error!(
                "Failed to get all ADC channels. Do check if the channel is opened by calling 'adc_io_open()' \
                 and the libuptech.so being loaded properly"
            );
            return Err("Failed to get all ADC channels");
        }

        Ok(())
    }
}

/// Retrieves the input levels of all IO channels.
///
/// This function loads and invokes the `adc_io_InputGetAll` function from the external shared library
/// to fetch the input levels of all IO channels. Each bit in the returned `u8` represents the state
/// of a channel (1 for high, 0 for low).
///
/// # Returns
///
/// * `u8` - A bitmask representing the input levels of all IO channels.
///
/// # Examples
///
/// ```
/// use uptechstar_rs::adc_io::io_get_all_channels;
/// let levels = io_get_all_channels();
/// if levels & 0b0000_0001 != 0 {
///     println!("IO0 is high");
/// }
/// ```
///
/// # Safety
///
/// This function uses unsafe code to interact with a C library. Ensure that the shared library ([libuptech.so](file://L:\RustProjects\uptechstar-rs\lib\libuptech.so))
/// is properly loaded and the `adc_io_InputGetAll` function is available.
pub fn io_get_all_channels() -> u8 {
    unsafe {
        let adc_io_input_get_all: Symbol<unsafe extern "C" fn() -> u8> = LIBRARY
            .get(b"adc_io_InputGetAll")
            .expect("Failed to load adc_io_InputGetAll function");

        adc_io_input_get_all()
    }
}

/// Retrieves the level of a specific IO index.
///
/// This function calculates the level of the specified IO index based on the result of `io_get_all_channels`.
/// It shifts and masks the bitmask to isolate the desired bit.
///
/// # Arguments
///
/// * `index` - The index of the IO channel (0-7).
///
/// # Returns
///
/// * `u8` - The level of the specified IO index (`1` for high, `0` for low).
///
/// # Notes
///
/// This function only works in OUTPUT MODE.
pub fn get_io_level(index: usize) -> u8 {
    (io_get_all_channels() >> index) & 1
}

/// Sets the levels of all IO channels.
///
/// This function loads and invokes the `adc_io_SetAll` function from the external shared library to set
/// the levels of all IO channels. Each bit in the `levels` parameter corresponds to the state of a channel
/// (1 for high, 0 for low).
///
/// # Arguments
///
/// * `levels` - A bitmask specifying the desired levels of all IO channels.
///
/// # Returns
///
/// * `i32` - Returns `0` on success, non-zero on failure.
///
/// # Examples
///
/// ```
/// use uptechstar_rs::adc_io::set_all_io_levels;
/// set_all_io_levels(0b0000_0001); // Set IO0 to high, others to low
/// ```
///
/// # Safety
///
/// This function uses unsafe code to interact with a C library. Ensure that the shared library ([libuptech.so](file://L:\RustProjects\uptechstar-rs\lib\libuptech.so))
/// is properly loaded and the `adc_io_SetAll` function is available.
pub fn set_all_io_levels(levels: u32) -> i32 {
    unsafe {
        let adc_io_set_all: Symbol<unsafe extern "C" fn(u32) -> i32> = LIBRARY
            .get(b"adc_io_SetAll")
            .expect("Failed to load adc_io_SetAll function");

        let result = adc_io_set_all(levels);

        if result != 0 {
            error!(
                "Failed to set all IO level. Do check if the channel is opened by calling 'adc_io_open()' \
                 and the libuptech.so being loaded properly"
            );
        }

        result
    }
}

/// Flips the level of a specific IO index.
///
/// This function loads and invokes the `adc_io_Set` function from the external shared library to flip
/// the level of the specified IO index.
///
/// # Arguments
///
/// * `index` - The index of the IO channel (0-7).
///
/// # Returns
///
/// * `i32` - Returns `0` on success, `-1` on failure.
///
/// # Notes
///
/// This function only works in OUTPUT MODE.
///
/// # Safety
///
/// This function uses unsafe code to interact with a C library. Ensure that the shared library ([libuptech.so](file://L:\RustProjects\uptechstar-rs\lib\libuptech.so))
/// is properly loaded and the `adc_io_Set` function is available.
pub fn flip_io_level(index: u32) -> i32 {
    unsafe {
        let adc_io_set: Symbol<unsafe extern "C" fn(u32) -> i32> = LIBRARY
            .get(b"adc_io_Set")
            .expect("Failed to load adc_io_Set function");

        let result = adc_io_set(index);

        if result == -1 {
            error!(
                "Failed to flip IO level, index: {}. Do check if the channel is opened by calling 'adc_io_open()' \
                 and the libuptech.so being loaded properly",
                index
            );
        }

        result
    }
}

/// Retrieves the modes of all IO channels.
///
/// This function loads and invokes the `adc_io_ModeGetAll` function from the external shared library
/// to fetch the modes of all IO channels. Each bit in the returned `u8` represents the mode of a channel
/// (`1` for output, `0` for input).
///
/// # Returns
///
/// * `u8` - A bitmask representing the modes of all IO channels.
///
/// # Examples
///
/// ```
/// use uptechstar_rs::adc_io::get_all_io_mode;
/// let modes = get_all_io_mode();
/// if modes & 0b0000_0001 != 0 {
///     println!("IO0 is in output mode");
/// }
/// ```
///
/// # Safety
///
/// This function uses unsafe code to interact with a C library. Ensure that the shared library ([libuptech.so](file://L:\RustProjects\uptechstar-rs\lib\libuptech.so))
/// is properly loaded and the `adc_io_ModeGetAll` function is available.
pub fn get_all_io_mode() -> u8 {
    unsafe {
        let mut buffer: u8 = 0;
        let adc_io_mode_get_all: Symbol<unsafe extern "C" fn(*mut u8) -> i32> = LIBRARY
            .get(b"adc_io_ModeGetAll")
            .expect("Failed to load adc_io_ModeGetAll function");

        if adc_io_mode_get_all(&mut buffer) != 0 {
            error!(
                "Failed to get all IO mode. Do check if the channel is opened by calling 'adc_io_open()' \
                 and the libuptech.so being loaded properly"
            );
        }

        buffer
    }
}

/// Sets the modes of all IO channels.
///
/// This function iteratively sets the mode of each IO channel using the `adc_io_ModeSet` function
/// from the external shared library.
///
/// # Arguments
///
/// * `mode` - The mode to set for all IO channels (`0` for input, `1` for output).
///
/// # Returns
///
/// * `i32` - Returns `0` on success, `-1` on failure.
///
/// # Safety
///
/// This function uses unsafe code to interact with a C library. Ensure that the shared library ([libuptech.so](file://L:\RustProjects\uptechstar-rs\lib\libuptech.so))
/// is properly loaded and the `adc_io_ModeSet` function is available.
pub fn set_all_io_mode(mode: u8) -> i32 {
    unsafe {
        let adc_io_mode_set: Symbol<unsafe extern "C" fn(u32, i32) -> i32> = LIBRARY
            .get(b"adc_io_ModeSet")
            .expect("Failed to load adc_io_ModeSet function");

        let mut failed = false;
        for index in 0..8 {
            if adc_io_mode_set(index, mode as i32) != 0 {
                failed = true;
            }
        }

        if failed {
            error!(
                "Failed to set all IO mode to {}. Do check if the channel is opened by calling 'adc_io_open()' \
                 and the libuptech.so being loaded properly",
                mode
            );
            return -1;
        }

        0
    }
}

/// Sets the mode of a specific IO index.
///
/// This function loads and invokes the `adc_io_ModeSet` function from the external shared library to set
/// the mode of the specified IO index.
///
/// # Arguments
///
/// * `index` - The index of the IO channel (0-7).
/// * `mode` - The mode to set for the IO channel (`0` for input, `1` for output).
///
/// # Returns
///
/// * `i32` - Returns `0` on success, non-zero on failure.
///
/// # Safety
///
/// This function uses unsafe code to interact with a C library. Ensure that the shared library ([libuptech.so](file://L:\RustProjects\uptechstar-rs\lib\libuptech.so))
/// is properly loaded and the `adc_io_ModeSet` function is available.
pub fn set_io_mode(index: u32, mode: u8) -> i32 {
    unsafe {
        let adc_io_mode_set: Symbol<unsafe extern "C" fn(u32, i32) -> i32> = LIBRARY
            .get(b"adc_io_ModeSet")
            .expect("Failed to load adc_io_ModeSet function");

        let result = adc_io_mode_set(index, mode as i32);

        if result != 0 {
            error!(
                "Failed to set IO mode, index: {}, mode: {}. Do check if the channel is opened by calling 'adc_io_open()' \
                 and the libuptech.so being loaded properly",
                index, mode
            );
        }

        result
    }
}
