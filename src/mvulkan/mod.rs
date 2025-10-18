//! GPU-agnostic graphics API for MVOS.  
//! 
//! --- Global Conventions of the API ---  
//! * Supported color format is RGB888:  
//!     8 bit Red, 8 bit Green, 8 bit Blue  
//!     unless specified otherwise  
//! 
//! * Origin (0,0) is at top left of the screen

use core::ffi::c_char;

/// This trait should be implemented by structs representing 
/// GPU drivers that are intended to be compatible with MVulkan.
/// 
/// Note: This trait does not include the full MVulkan spec, just 
/// the bare minimum functionality that a GPU driver should support
/// to be considered MVulkan-compatible (i.e, the bare minimum that MVOS
/// needs to present a visual console.)
pub trait MVulkanGPUDriver {

    /// Return a reference to self if the device driver supports advanced geometry features 
    /// (must implement this method in driver, default is `None`)
    fn as_geometry(&self) -> Option<&dyn MVulkanGeometry> {
        None
    }

    /// Return a mutable reference to self if the device driver supports advanced geometry features 
    /// (must implement this method in driver, default is `None`)
    fn as_geometry_mut(&mut self) -> Option<&mut dyn MVulkanGeometry> {
        None
    }

    /// Setup function to enable the GPU. 
    /// 
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

    /// Print a UTF-8 character to the screen with given coordinates, scaling and color
    fn draw_char(&mut self, utf8: usize, r: u8, g: u8, b: u8, x: u32, y: u32, scale: u8);
}

/// This trait includes methods to draw advanced geometric shapes
/// such as triangles and circles. Since advanced geometry is not 
/// required for the visual console, this trait is optional.  
/// 
/// To opt in, the driver must implement the `as_geometry` and `as_geometry_mut`
/// methods of the `MVulkanGPUDriver` trait and return `Some(self)`. 
pub trait MVulkanGeometry : MVulkanGPUDriver {
    /// Draw a circle with given center (Ox, Oy) and radius R.
    fn draw_circle(&mut self, Ox: u32, Oy: u32, R: u32, r: u8, g: u8, b: u8);
}

pub mod console;