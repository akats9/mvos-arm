use crate::{GPU_DEVICE, SCREENHEIGHT, SCREENWIDTH, thread};

pub fn trigonakalanta() {
    if let Some(geometry_gpu) = unsafe { (*GPU_DEVICE.unwrap()).as_geometry_mut() } {
        loop {
            geometry_gpu.draw_triangle(SCREENWIDTH / 2, SCREENHEIGHT / 2 + 100, SCREENWIDTH / 2 - 100, SCREENHEIGHT / 2 + 250, SCREENWIDTH / 2 + 100, SCREENHEIGHT / 2 + 250, 240, 232, 12, false);
            geometry_gpu.draw_line(SCREENWIDTH / 2, SCREENHEIGHT / 2 + 300, SCREENWIDTH /2 -100, SCREENHEIGHT / 2 + 200, 240, 232, 12);
            thread::sleep(300);
            geometry_gpu.draw_line(SCREENWIDTH / 2, SCREENHEIGHT / 2 + 300, SCREENWIDTH /2 -100, SCREENHEIGHT / 2 + 200, 0, 0, 0);
            geometry_gpu.draw_line(SCREENWIDTH / 2, SCREENHEIGHT / 2 + 300, SCREENWIDTH /2 +100, SCREENHEIGHT / 2 + 200, 240, 232, 12);
            thread::sleep(300);
            geometry_gpu.draw_line(SCREENWIDTH / 2, SCREENHEIGHT / 2 + 300, SCREENWIDTH /2 +100, SCREENHEIGHT / 2 + 200, 0, 0, 0);
        }
    }
}

pub mod templeos_color_palette {
    //! u32 hex RGB888 (#RRGGBB) color palette of TempleOS (CGA 16-color palette)
    //! L is light, D is dark

    pub const BLACK:     u32 = 0x00_00_00;
    pub const BLUE:      u32 = 0x00_00_AA;
    pub const GREEN:     u32 = 0x00_AA_00;
    pub const CYAN:      u32 = 0x00_AA_AA;
    pub const RED:       u32 = 0xAA_00_00;
    pub const MAGENTA:   u32 = 0xAA_00_AA;
    pub const BROWN:     u32 = 0xAA_55_00;
    pub const L_GRAY:    u32 = 0xAA_AA_AA;
    pub const D_GRAY:    u32 = 0x55_55_55;
    pub const L_BLUE:    u32 = 0x55_55_FF;
    pub const L_GREEN:   u32 = 0x55_FF_55;
    pub const L_CYAN:    u32 = 0x55_FF_FF;
    pub const L_RED:     u32 = 0xFF_55_55;
    pub const L_MAGENTA: u32 = 0xFF_55_FF;
    pub const YELLOW:    u32 = 0xFF_FF_55;
    pub const WHITE:     u32 = 0xFF_FF_FF;
}