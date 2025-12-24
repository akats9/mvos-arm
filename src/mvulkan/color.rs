//! Color palette for MVOS
//! Includes hex rgb888 colors for most text.

pub const SUCCESS_GREEN: u32 = 0x00ff00;
pub const INFO_GREEN: u32 = 0x41bd08;
pub const FAIL_RED: u32 = 0xaa0000;
pub const ERROR_RED: u32 = 0xdd0000;
pub const PANIC_RED: u32 = 0xff0000;
pub const DBG_YELLOW: u32 = 0xdbc323;
pub const WARNING_ORANGE: u32 = 0xf29f0f;
pub const GENERIC_WHITE: u32 = 0xffffff;

/// Trait to abstract over different colorschemes
/// so that they can be arbitrarily set and changed.
/// 
/// Implementors can define any amount of the (included) 
/// colors as they want, and the rest (if any) follow
/// the default definition, which follows the constants
/// defined above.
pub trait MVulkanColorScheme {
    /// Color for operation success.
    fn success(&self) -> u32 { SUCCESS_GREEN }

    /// Color for printing general information.
    fn info(&self) -> u32 { INFO_GREEN }

    /// Color for operation failure (not critical).
    fn fail(&self) -> u32 { FAIL_RED }

    /// Color for operation error (critical but recoverable).
    fn error(&self) -> u32 { ERROR_RED }

    /// Color for fatal errors (e.g. KERNEL PANIC)
    /// (use red)
    fn panic_red(&self) -> u32 { PANIC_RED }

    /// Color for printing debug information
    fn debug(&self) -> u32 { DBG_YELLOW }

    /// Color for printing warnings
    fn warning(&self) -> u32 { WARNING_ORANGE }

    // -- GENERIC COLORS -- 

    /// White
    fn white(&self) -> u32 { GENERIC_WHITE }


}

pub struct DefaultColorScheme;

impl MVulkanColorScheme for DefaultColorScheme {}

impl DefaultColorScheme {
    pub fn new() -> Self { Self {} }
}