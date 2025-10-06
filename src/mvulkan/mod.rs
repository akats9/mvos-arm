//! GPU-agnostic graphics API for MVOS.
//! 
//! --- Global Conventions of the API ---
//! Supported color format is RGB888:
//! 8 bit Red, 8 bit Green, 8 bit Blue
//! unless specified otherwise
//! 
//! Origin (0,0) is at top left of the screen

/// This trait should be implemented by structs representing 
/// GPU drivers that are intended to be compatible with MVulkan.
pub trait MVulkanGPUDriver {
    /// Setup function to enable the GPU. 
    /// This function returns Ok(()) if the operation succeeds
    /// or Err(&'static str) containing the error message if the operation fails.
    fn setup(&mut self) -> Result<(), &'static str>;

    /// Clear the screen to a specific 8-bit color.
    /// 
    /// This function DOES NOT support RGB888 format!
    fn clear(&mut self, color: u8);

    /// Set pixel at (x,y) to a specific color..
    fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8);

    /// Draw a rectangle with sides (maxx - minx), (maxy - miny)
    /// starting at (minx, miny) with a specific color.
    fn draw_rect(&mut self, minx: u32, maxx: u32, miny: u32, maxy: u32, r: u8, g: u8, b: u8);
}