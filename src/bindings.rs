use crate::{GPU_DEVICE, PI};

unsafe extern "C" {}

#[unsafe(no_mangle)]
unsafe extern "C" fn pi() -> f64 {
    PI
}

#[unsafe(no_mangle)]
unsafe extern "C" fn sin(x: f64) -> f64 {
    libm::sin(x)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn cos(x: f64) -> f64 {
    libm::cos(x)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tan(x: f64) -> f64 {
    libm::tan(x)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn sqrt(x: f64) -> f64 {
    libm::sqrt(x)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn c_draw_pixel(x: u32, y: u32, r: u8, g: u8, b: u8) {
    (*GPU_DEVICE.unwrap()).set_pixel(x, y, r, g, b);
}