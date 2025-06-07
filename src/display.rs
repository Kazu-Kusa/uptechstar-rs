use crate::extern_lib::LIBRARY;
use libloading::Symbol;

use log::info;


/// All supported screen direction enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenDirection {
    Vertical = 1,
    Horizontal = 2,
}

impl ScreenDirection {
    /// Returns the width of the screen based on the screen direction.
    pub fn width(&self) -> i32 {
        match self {
            ScreenDirection::Vertical => 64,
            ScreenDirection::Horizontal => 128,
        }
    }

    /// Returns the height of the screen based on the screen direction.
    pub fn height(&self) -> i32 {
        match self {
            ScreenDirection::Vertical => 128,
            ScreenDirection::Horizontal => 64,
        }
    }
}

/// All supported font size enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontSize {
    Font4x6 = 0,
    Font5x8 = 1,
    Font5x12 = 2,
    Font6x8 = 3,
    Font6x10 = 4,
    Font7x12 = 5,
    Font8x8 = 6,
    Font8x12 = 7,
    Font8x14 = 8,
    Font10x16 = 9,
    Font12x16 = 10,
    Font12x20 = 11,
    Font16x26 = 12,
    Font22x36 = 13,
    Font24x40 = 14,
}

impl FontSize {
    /// Returns the row height of the current font size.
    pub fn row_height(&self) -> i32 {
        match self {
            FontSize::Font4x6 => 6,
            FontSize::Font5x8 => 8,
            FontSize::Font5x12 => 12,
            FontSize::Font6x8 => 8,
            FontSize::Font6x10 => 10,
            FontSize::Font7x12 => 12,
            FontSize::Font8x8 => 8,
            FontSize::Font8x12 => 12,
            FontSize::Font8x14 => 14,
            FontSize::Font10x16 => 16,
            FontSize::Font12x16 => 16,
            FontSize::Font12x20 => 20,
            FontSize::Font16x26 => 26,
            FontSize::Font22x36 => 36,
            FontSize::Font24x40 => 40,
        }
    }

    /// Returns the column width of the current font size.
    pub fn column_width(&self) -> i32 {
        match self {
            FontSize::Font4x6 => 4,
            FontSize::Font5x8 => 5,
            FontSize::Font5x12 => 5,
            FontSize::Font6x8 => 6,
            FontSize::Font6x10 => 6,
            FontSize::Font7x12 => 7,
            FontSize::Font8x8 => 8,
            FontSize::Font8x12 => 8,
            FontSize::Font8x14 => 8,
            FontSize::Font10x16 => 10,
            FontSize::Font12x16 => 12,
            FontSize::Font12x20 => 12,
            FontSize::Font16x26 => 16,
            FontSize::Font22x36 => 22,
            FontSize::Font24x40 => 24,
        }
    }
}

/// All supported color display on the led/lcd
pub struct Color;

impl Color {
    /// Generates a new color value based on the specified red, green, and blue components.
    ///
    /// Parameters:
    /// - r: The red component of the color, between 0 and 255.
    /// - g: The green component of the color, between 0 and 255.
    /// - b: The blue component of the color, between 0 and 255.
    ///
    /// Returns:
    /// A 24-bit color value, with 8 bits for each red, green, and blue component.
    ///
    /// Panics:
    /// If any color component is outside the range of 0 to 255.
    pub const fn new_color(r: u8, g: u8, b: u8) -> u32 {
        ((r as u32) << 16) + ((g as u32) << 8) + (b as u32)
    }

    pub const WHITE: u32 = Self::new_color(255, 255, 255);
    pub const GRAY: u32 = Self::new_color(128, 128, 128);
    pub const BLACK: u32 = Self::new_color(0, 0, 0);

    pub const RED: u32 = Self::new_color(255, 0, 0);
    pub const GREEN: u32 = Self::new_color(0, 255, 0);
    pub const BLUE: u32 = Self::new_color(0, 0, 255);

    pub const B_RED: u32 = Self::new_color(255, 0, 128);
    pub const G_RED: u32 = Self::new_color(255, 128, 0);

    pub const G_BLUE: u32 = Self::new_color(0, 128, 255);
    pub const R_BLUE: u32 = Self::new_color(128, 0, 255);

    pub const R_GREEN: u32 = Self::new_color(128, 255, 0);
    pub const B_GREEN: u32 = Self::new_color(0, 255, 128);

    pub const YELLOW: u32 = Self::new_color(255, 255, 0);
    pub const MAGENTA: u32 = Self::new_color(255, 0, 255);
    pub const CYAN: u32 = Self::new_color(0, 255, 255);

    pub const ORANGE: u32 = Self::new_color(128, 128, 0);
    pub const PURPLE: u32 = Self::new_color(128, 0, 128);
    pub const BLUEGREEN: u32 = Self::new_color(0, 128, 128);

    pub const DARKBLUE: u32 = Self::new_color(0, 0, 139);
    pub const DARKGREEN: u32 = Self::new_color(0, 139, 0);
    pub const DARKRED: u32 = Self::new_color(139, 0, 0);
}
/// Screen module
///
/// This struct represents an LCD screen and provides methods to manipulate it.
/// Each method returns self to enable chainable calls.
pub struct Screen {
    screen_size: (i32, i32),
    font_size: FontSize,
    screen_dir: Option<ScreenDirection>,
}

impl Screen {
    /// Initializes the Screen struct.
    ///
    /// Parameters:
    ///     screen_dir: The direction to open the screen in. None for no initialization.
    ///
    /// Returns:
    ///     A new Screen instance
    pub fn new(screen_dir: Option<ScreenDirection>) -> Self {
        let mut screen = Screen {
            screen_size: (0, 0),
            font_size: FontSize::Font12x20,
            screen_dir,
        };

        if let Some(dir) = screen_dir {
            screen.open(dir).fill_screen(Color::BLACK).refresh();
        }

        screen
    }

    /// Open the LCD and set the displaying direction.
    ///
    /// Args:
    ///   direction: Display direction; Vertical or Horizontal.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn open(&mut self, direction: ScreenDirection) -> &mut Self {
        info!("Open LCD with direction: {:?}", direction);

        unsafe {
            let lcd_open: Symbol<unsafe extern "C" fn(i32) -> i32> = LIBRARY
                .get(b"lcd_open")
                .expect("Failed to load lcd_open function");

            lcd_open(direction as i32);
        }

        self.screen_dir = Some(direction);
        self
    }

    /// Close the LCD.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn close(&mut self) -> &mut Self {
        info!("Closing LCD");

        unsafe {
            let lcd_close: Symbol<unsafe extern "C" fn() -> i32> = LIBRARY
                .get(b"lcd_close")
                .expect("Failed to load lcd_close function");

            lcd_close();
        }

        self
    }

    /// Refresh the screen, printing the display data from the cache onto the screen.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn refresh(&mut self) -> &mut Self {
        unsafe {
            let lcd_refresh: Symbol<unsafe extern "C" fn() -> i32> = LIBRARY
                .get(b"LCD_Refresh")
                .expect("Failed to load LCD_Refresh function");

            lcd_refresh();
        }

        self
    }

    /// Set the font size.
    ///
    /// Args:
    ///   font_size: The desired font size.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn set_font_size(&mut self, font_size: FontSize) -> &mut Self {
        self.font_size = font_size;

        unsafe {
            let lcd_set_font: Symbol<unsafe extern "C" fn(i32) -> i32> = LIBRARY
                .get(b"LCD_SetFont")
                .expect("Failed to load LCD_SetFont function");

            lcd_set_font(font_size as i32);
        }

        self
    }

    /// Set the foreground color.
    ///
    /// Args:
    ///   color: The desired foreground color.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn set_fore_color(&mut self, color: u32) -> &mut Self {
        unsafe {
            let ug_set_forecolor: Symbol<unsafe extern "C" fn(u32) -> i32> = LIBRARY
                .get(b"UG_SetForecolor")
                .expect("Failed to load UG_SetForecolor function");

            ug_set_forecolor(color);
        }

        self
    }

    /// Set the background color of the LCD.
    ///
    /// Args:
    ///   color: The desired background color.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn set_back_color(&mut self, color: u32) -> &mut Self {
        unsafe {
            let ug_set_backcolor: Symbol<unsafe extern "C" fn(u32) -> i32> = LIBRARY
                .get(b"UG_SetBackcolor")
                .expect("Failed to load UG_SetBackcolor function");

            ug_set_backcolor(color);
        }

        self
    }

    /// Set the LED color at a specific index.
    ///
    /// Parameters:
    ///     index: The index of the LED to set the color for (0 or 1).
    ///     color: The color to set for the LED.
    ///
    /// Returns:
    ///     Self for method chaining.
    pub fn set_led_color(&mut self, index: i32, color: u32) -> &mut Self {
        unsafe {
            let adc_led_set: Symbol<unsafe extern "C" fn(i32, u32) -> i32> = LIBRARY
                .get(b"adc_led_set")
                .expect("Failed to load adc_led_set function");

            adc_led_set(index, color);
        }

        self
    }

    /// Set the color of LED 0.
    ///
    /// Args:
    ///     color: The color to set for LED 0.
    ///
    /// Returns:
    ///     Self for method chaining.
    pub fn set_led_0(&mut self, color: u32) -> &mut Self {
        self.set_led_color(0, color)
    }

    /// Set the color of LED 1.
    ///
    /// Args:
    ///     color: The color to set for LED 1.
    ///
    /// Returns:
    ///     Self for method chaining.
    pub fn set_led_1(&mut self, color: u32) -> &mut Self {
        self.set_led_color(1, color)
    }

    /// Sets the color of both LEDs to the same value.
    ///
    /// Parameters:
    ///     color: The color to set for both LEDs.
    ///
    /// Returns:
    ///     Self for method chaining.
    pub fn set_all_leds_same(&mut self, color: u32) -> &mut Self {
        self.set_led_color(0, color);
        self.set_led_color(1, color);
        self
    }

    /// Sets the color of both LEDs to different values.
    ///
    /// Parameters:
    ///     first: The color to set for the first LED.
    ///     second: The color to set for the second LED.
    ///
    /// Returns:
    ///     Self for method chaining.
    pub fn set_all_leds_single(&mut self, first: u32, second: u32) -> &mut Self {
        self.set_led_color(0, first);
        self.set_led_color(1, second);
        self
    }

    /// Set all LEDs to off state.
    ///
    /// This function sets the color of both LEDs to 0, effectively turning them off.
    ///
    /// Returns:
    ///     Self for method chaining.
    pub fn set_all_leds_off(&mut self) -> &mut Self {
        self.set_led_color(0, 0);
        self.set_led_color(1, 0);
        self
    }

    /// Fill the entire screen with the specified color.
    ///
    /// Args:
    ///   color: The color to fill the screen with.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn fill_screen(&mut self, color: u32) -> &mut Self {
        unsafe {
            let ug_fill_screen: Symbol<unsafe extern "C" fn(u32) -> i32> = LIBRARY
                .get(b"UG_FillScreen")
                .expect("Failed to load UG_FillScreen function");

            ug_fill_screen(color);
        }

        self
    }

    /// Place a string at specific coordinates on the LCD.
    ///
    /// Args:
    ///   x: X coordinate (in pixels).
    ///   y: Y coordinate (in pixels).
    ///   display_string: The string to display on the LCD.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn put_string(&mut self, x: i32, y: i32, display_string: &str) -> &mut Self {
        let c_string = std::ffi::CString::new(display_string).expect("CString::new failed");

        unsafe {
            let ug_put_string: Symbol<unsafe extern "C" fn(i32, i32, *const i8) -> i32> = LIBRARY
                .get(b"UG_PutString")
                .expect("Failed to load UG_PutString function");

            ug_put_string(x, y, c_string.as_ptr());
        }

        self
    }

    /// Print a string to the LCD, automatically handling line breaks based on screen width.
    ///
    /// Args:
    ///   display_string: The string to display on the LCD.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn print(&mut self, display_string: &str) -> &mut Self {
        self.put_string(0, 0, display_string)
    }

    /// Fill a rectangular frame with the specified color.
    ///
    /// Args:
    ///   x1: The X coordinate of the top-left corner.
    ///   y1: The Y coordinate of the top-left corner.
    ///   x2: The X coordinate of the bottom-right corner.
    ///   y2: The Y coordinate of the bottom-right corner.
    ///   color: The color to fill the frame with.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn fill_frame(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) -> &mut Self {
        unsafe {
            let ug_fill_frame: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, u32) -> i32> = LIBRARY
                .get(b"UG_FillFrame")
                .expect("Failed to load UG_FillFrame function");

            ug_fill_frame(x1, y1, x2, y2, color);
        }

        self
    }

    /// Fill a rounded rectangular frame with the specified color.
    ///
    /// Args:
    ///   x1: The X coordinate of the top-left corner.
    ///   y1: The Y coordinate of the top-left corner.
    ///   x2: The X coordinate of the bottom-right corner.
    ///   y2: The Y coordinate of the bottom-right corner.
    ///   r: The radius of the corners.
    ///   color: The color to fill the round frame with.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn fill_round_frame(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, r: i32, color: u32) -> &mut Self {
        unsafe {
            let ug_fill_round_frame: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, i32, u32) -> i32> = LIBRARY
                .get(b"UG_FillRoundFrame")
                .expect("Failed to load UG_FillRoundFrame function");

            ug_fill_round_frame(x1, y1, x2, y2, r, color);
        }

        self
    }

    /// Fill a circle with the specified color.
    ///
    /// Args:
    ///   x0: The X coordinate of the circle center.
    ///   y0: The Y coordinate of the circle center.
    ///   r: The radius of the circle.
    ///   color: The color to fill the circle with.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn fill_circle(&mut self, x0: i32, y0: i32, r: i32, color: u32) -> &mut Self {
        unsafe {
            let ug_fill_circle: Symbol<unsafe extern "C" fn(i32, i32, i32, u32) -> i32> = LIBRARY
                .get(b"UG_FillCircle")
                .expect("Failed to load UG_FillCircle function");

            ug_fill_circle(x0, y0, r, color);
        }

        self
    }

    /// Draw a mesh pattern within a rectangle with the specified color.
    ///
    /// Args:
    ///   x1: The X coordinate of the top-left corner.
    ///   y1: The Y coordinate of the top-left corner.
    ///   x2: The X coordinate of the bottom-right corner.
    ///   y2: The Y coordinate of the bottom-right corner.
    ///   color: The color of the mesh lines.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn draw_mesh(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) -> &mut Self {
        unsafe {
            let ug_draw_mesh: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, u32) -> i32> = LIBRARY
                .get(b"UG_DrawMesh")
                .expect("Failed to load UG_DrawMesh function");

            ug_draw_mesh(x1, y1, x2, y2, color);
        }

        self
    }

    /// Draw an empty rectangular frame with the specified color.
    ///
    /// Args:
    ///   x1: The X coordinate of the top-left corner.
    ///   y1: The Y coordinate of the top-left corner.
    ///   x2: The X coordinate of the bottom-right corner.
    ///   y2: The Y coordinate of the bottom-right corner.
    ///   color: The color of the frame lines.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn draw_frame(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) -> &mut Self {
        unsafe {
            let ug_draw_frame: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, u32) -> i32> = LIBRARY
                .get(b"UG_DrawFrame")
                .expect("Failed to load UG_DrawFrame function");

            ug_draw_frame(x1, y1, x2, y2, color);
        }

        self
    }

    /// Draw an empty rounded rectangular frame with the specified color.
    ///
    /// Args:
    ///   x1: The X coordinate of the top-left corner.
    ///   y1: The Y coordinate of the top-left corner.
    ///   x2: The X coordinate of the bottom-right corner.
    ///   y2: The Y coordinate of the bottom-right corner.
    ///   r: The radius of the corners.
    ///   color: The color of the frame lines.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn draw_round_frame(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, r: i32, color: u32) -> &mut Self {
        unsafe {
            let ug_draw_round_frame: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, i32, u32) -> i32> = LIBRARY
                .get(b"UG_DrawRoundFrame")
                .expect("Failed to load UG_DrawRoundFrame function");

            ug_draw_round_frame(x1, y1, x2, y2, r, color);
        }

        self
    }

    /// Draw a single pixel at the specified coordinates with the specified color.
    ///
    /// Args:
    ///   x0: The X coordinate of the pixel.
    ///   y0: The Y coordinate of the pixel.
    ///   color: The color of the pixel.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn draw_pixel(&mut self, x0: i32, y0: i32, color: u32) -> &mut Self {
        unsafe {
            let ug_draw_pixel: Symbol<unsafe extern "C" fn(i32, i32, u32) -> i32> = LIBRARY
                .get(b"UG_DrawPixel")
                .expect("Failed to load UG_DrawPixel function");

            ug_draw_pixel(x0, y0, color);
        }

        self
    }

    /// Draw an empty circle with the specified color.
    ///
    /// Args:
    ///   x0: The X coordinate of the circle center.
    ///   y0: The Y coordinate of the circle center.
    ///   r: The radius of the circle.
    ///   color: The color of the circle lines.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn draw_circle(&mut self, x0: i32, y0: i32, r: i32, color: u32) -> &mut Self {
        unsafe {
            let ug_draw_circle: Symbol<unsafe extern "C" fn(i32, i32, i32, u32) -> i32> = LIBRARY
                .get(b"UG_DrawCircle")
                .expect("Failed to load UG_DrawCircle function");

            ug_draw_circle(x0, y0, r, color);
        }

        self
    }

    /// Draw an arc with the specified color.
    ///
    /// Args:
    ///   x0: The X coordinate of the circle center.
    ///   y0: The Y coordinate of the circle center.
    ///   r: The radius of the arc circle.
    ///   s: The starting angle of the arc.
    ///   color: The color of the arc lines.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn draw_arc(&mut self, x0: i32, y0: i32, r: i32, s: i32, color: u32) -> &mut Self {
        unsafe {
            let ug_draw_arc: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, u32) -> i32> = LIBRARY
                .get(b"UG_DrawArc")
                .expect("Failed to load UG_DrawArc function");

            ug_draw_arc(x0, y0, r, s, color);
        }

        self
    }

    /// Draw a line between two points with the specified color.
    ///
    /// Args:
    ///   x1: The X coordinate of the first point.
    ///   y1: The Y coordinate of the first point.
    ///   x2: The X coordinate of the second point.
    ///   y2: The Y coordinate of the second point.
    ///   color: The color of the line.
    ///
    /// Returns:
    ///   Self for chainable calls.
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) -> &mut Self {
        unsafe {
            let ug_draw_line: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, u32) -> i32> = LIBRARY
                .get(b"UG_DrawLine")
                .expect("Failed to load UG_DrawLine function");

            ug_draw_line(x1, y1, x2, y2, color);
        }

        self
    }
}