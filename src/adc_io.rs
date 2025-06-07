use crate::extern_lib::LIBRARY;
use libloading::Symbol;

use log::{debug, error, info};


pub fn adc_open() -> i32 {
    // Open the adc-io plug
    info!("Initializing ADC-IO");

    unsafe {
        let adc_io_open: Symbol<unsafe extern "C" fn() -> i32> = LIBRARY
            .get(b"adc_io_open")
            .expect("Failed to load adc_io_open function");

        let open_times = adc_io_open();

        if open_times == -1 {
            error!("Failed to open ADC-IO. Do check if the channel is opened by calling 'adc_io_open()' and the libuptech.so being loaded properly");
        } else {
            debug!("ADC-IO open {} times", open_times);
        }

        open_times
    }
}
pub fn adc_close() -> i32 {
    // Close the adc-io plug
    info!("Closing ADC-IO");

    unsafe {
        let adc_io_close: Symbol<unsafe extern "C" fn() -> i32> = LIBRARY
            .get(b"adc_io_close")
            .expect("Failed to load adc_io_close function");

        let result = adc_io_close();

        if result == -1 {
            error!("Failed to close ADC-IO. Do check if the channel is opened by calling 'adc_io_open()' and the libuptech.so being loaded properly");
        } else {
            debug!("ADC-IO closed");
        }

        result
    }
}

pub fn adc_get_all_channels(adc_data: &mut [i32; 10]) -> Result<(), &'static str> {
    // Get all the ADC channels. Length = 10
    unsafe {
        let adc_get_all: Symbol<unsafe extern "C" fn(*mut i32) -> i32> = LIBRARY
            .get(b"ADC_GetAll")
            .expect("Failed to load ADC_GetAll function");

        let result = adc_get_all(adc_data.as_mut_ptr());

        if result != 0 {
            error!("Failed to get all ADC channels. Do check if the channel is opened by calling 'adc_io_open()' and the libuptech.so being loaded properly");
            return Err("Failed to get all ADC channels");
        }

        Ok(())
    }
}

pub fn io_get_all_channels() -> u8 {
    // Get all io plug input levels
    // uint8, each bit represents a channel, 1 for high, 0 for low
    // Examples:
    //   0b10000000 => io7 is high level
    //   0b00000001 => io0 is high level
    unsafe {
        let adc_io_input_get_all: Symbol<unsafe extern "C" fn() -> u8> = LIBRARY
            .get(b"adc_io_InputGetAll")
            .expect("Failed to load adc_io_InputGetAll function");

        adc_io_input_get_all()
    }
}

pub fn get_io_level(index: usize) -> u8 {
    // Get the level of the specified IO index.
    // Returns: The level of the specified IO index, calculated based on adc_io_InputGetAll()
    // Note: ONLY works in OUTPUT MODE
    (io_get_all_channels() >> index) & 1
}

pub fn set_all_io_levels(levels: u32) -> i32 {
    // Sets the level of all IOs to the specified level.
    // Note: ONLY works in OUTPUT MODE
    // Examples:
    //   levels = 0b0000_0001 => io0 is high level, others are low
    //   levels = 0b1000_0000 => io7 is high level, others are low
    unsafe {
        let adc_io_set_all: Symbol<unsafe extern "C" fn(u32) -> i32> = LIBRARY
            .get(b"adc_io_SetAll")
            .expect("Failed to load adc_io_SetAll function");

        let result = adc_io_set_all(levels);

        if result != 0 {
            error!("Failed to set all IO level. Do check if the channel is opened by calling 'adc_io_open()' and the libuptech.so being loaded properly");
        }

        result
    }
}


pub fn flip_io_level(index: u32) -> i32 {
    // Flips the level of the specified IO index.
    //
    // Args:
    //     index: The index of the IO.
    //
    // Returns:
    //     i32: 0 on success, -1 on failure.
    //
    // Notes:
    //     ONLY work in OUTPUT MODE
    unsafe {
        let adc_io_set: Symbol<unsafe extern "C" fn(u32) -> i32> = LIBRARY
            .get(b"adc_io_Set")
            .expect("Failed to load adc_io_Set function");

        let result = adc_io_set(index);

        if result == -1 {
            error!("Failed to flip IO level, index: {}. Do check if the channel is opened by calling 'adc_io_open()' and the libuptech.so being loaded properly", index);
        }

        result
    }
}

pub fn get_all_io_mode() -> u8 {
    // Get all IO modes. length = 8,store as bit0,bit1,bit2,bit3,bit4,bit5,bit6,bit7
    //
    // Returns:
    //     u8: A buffer containing all IO modes.
    // Examples:
    //     0b10000000 => io7 is output mode, can be used to drive servos
    //     0b00000001 => io0 is input mode, can be used for external sensors
    unsafe {
        let mut buffer: u8 = 0;
        let adc_io_mode_get_all: Symbol<unsafe extern "C" fn(*mut u8) -> i32> = LIBRARY
            .get(b"adc_io_ModeGetAll")
            .expect("Failed to load adc_io_ModeGetAll function");

        if adc_io_mode_get_all(&mut buffer) != 0 {
            error!("Failed to get all IO mode. Do check if the channel is opened by calling 'adc_io_open()' and the libuptech.so being loaded properly");
        }

        buffer
    }
}

pub fn set_all_io_mode(mode: u8) -> i32 {
    // Sets the mode of all IOs to the specified mode.
    //
    // Args:
    //     mode: The mode to set for all IOs (0 or 1).
    //
    // Returns:
    //     i32: 0 on success, non-zero on failure.
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
            error!("Failed to set all IO mode to {}. Do check if the channel is opened by calling 'adc_io_open()' and the libuptech.so being loaded properly", mode);
            return -1;
        }

        0
    }
}

pub fn set_io_mode(index: u32, mode: u8) -> i32 {
    // Sets the mode of the specified IO index to the specified mode.
    //
    // Args:
    //     index: The index of the IO (0-7).
    //     mode: The mode to set for the IO (0 or 1).
    //
    // Returns:
    //     i32: 0 on success, non-zero on failure.
    unsafe {
        let adc_io_mode_set: Symbol<unsafe extern "C" fn(u32, i32) -> i32> = LIBRARY
            .get(b"adc_io_ModeSet")
            .expect("Failed to load adc_io_ModeSet function");

        let result = adc_io_mode_set(index, mode as i32);

        if result != 0 {
            error!("Failed to set IO mode, index: {}, mode: {}. Do check if the channel is opened by calling 'adc_io_open()' and the libuptech.so being loaded properly", index, mode);
        }

        result
    }
}
