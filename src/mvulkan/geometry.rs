//! High-level API for drawing and animating shapes.

use crate::{GPU_DEVICE, SCREENHEIGHT, SCREENWIDTH, THEME, mvulkan::{MVulkanGPUDriver, color::IntoRGB}, thread};

/// Defines functions for drawing and animating shape objects.
pub trait Shape {
    fn draw(&self);
    fn erase(&mut self);
    fn slide_h(&mut self, right: bool, dist: u32, spd: usize);
    fn slide_v(&mut self, down: bool, dist: u32, spd: usize);
}

pub struct Rectangle {
    minx: u32,
    maxx: u32, 
    miny: u32,
    maxy: u32,
    r: u8,
    g: u8,
    b: u8,
    fill: bool,
}

impl Rectangle {
    pub fn new<C: IntoRGB>(minx: u32, maxx: u32, miny: u32, maxy: u32, color: C, fill: bool) -> Self {
        minx.clamp(0, SCREENWIDTH);
        maxx.clamp(0, SCREENWIDTH);
        miny.clamp(0, SCREENHEIGHT);
        maxy.clamp(0, SCREENHEIGHT);
        let (r,g,b) = color.into_rgb();
        Self {
            minx, maxx, miny, maxy, r, g, b, fill
        }
    }
}

impl Shape for Rectangle {
    fn draw(&self) {
        if let Some(geometry_gpu) = unsafe { (*GPU_DEVICE.unwrap()).as_geometry_mut() } {
            if self.fill {
                geometry_gpu.draw_rect(self.minx, self.maxx, self.miny, self.maxy, self.r, self.g, self.b);
            } else {
                for x in self.minx..self.maxx {
                    geometry_gpu.set_pixel(x, self.miny, self.r, self.g, self.b);
                    geometry_gpu.set_pixel(x, self.maxy, self.r, self.g, self.b);
                }
                for y in self.miny..self.maxy {
                    geometry_gpu.set_pixel(self.minx, y, self.r, self.g, self.b); 
                    geometry_gpu.set_pixel(self.maxx, y, self.r, self.g, self.b);
                }
            }
        }
    }

    fn erase(&mut self) {
        unsafe {
            let theme = THEME;
            let r = ((theme.background() >> 16) & 0xff) as u8;
            let g = ((theme.background() >> 8) & 0xff) as u8;
            let b = ((theme.background()) & 0xff) as u8;
            if let Some(geometry_gpu) = unsafe { (*GPU_DEVICE.unwrap()).as_geometry_mut() } {
                if self.fill {
                    geometry_gpu.draw_rect(self.minx, self.maxx, self.miny, self.maxy, r, g, b);
                } else {
                    for x in self.minx..self.maxx {
                        geometry_gpu.set_pixel(x, self.miny, r, g, b);
                        geometry_gpu.set_pixel(x, self.maxy, r, g, b);
                    }
                    for y in self.miny..self.maxy {
                        geometry_gpu.set_pixel(self.minx, y, r, g, b); 
                        geometry_gpu.set_pixel(self.maxx, y, r, g, b);
                    }
                }
            }
        }
    }

    fn slide_h(&mut self, right: bool, dist: u32, spd: usize) {
        spd.clamp(1, 50);
        if right {
            dist.clamp(0, SCREENWIDTH-self.maxx);
            for i in 0..dist {
                self.erase();
                self.maxx += 1;
                self.minx += 1;
                self.draw();
                thread::sleep(50/spd);
            }
        } else {
            dist.clamp(0, self.minx);
            for i in 0..dist {
                self.erase();
                self.maxx -= 1;
                self.minx -= 1;
                self.draw();
                thread::sleep(50/spd);
            }
        }
    }

    fn slide_v(&mut self, down: bool, dist: u32, spd: usize) {
        spd.clamp(1, 50);
        if down {
            dist.clamp(0, SCREENHEIGHT-self.maxy);
            for i in 0..dist {
                self.erase();
                self.maxy += 1;
                self.miny += 1;
                self.draw();
                thread::sleep(50/spd);
            }
        } else {
            dist.clamp(0, self.miny);
            for i in 0..dist {
                self.erase();
                self.maxy -= 1;
                self.miny -= 1;
                self.draw();
                thread::sleep(50/spd);
            }
        }
    }
}

pub struct Circle {
    Ox: u32,
    Oy: u32,
    radius: u32, 
    r: u8,
    g: u8,
    b: u8,
    fill: bool,
}

impl Circle {
    pub fn new<C: IntoRGB>(Ox: u32, Oy: u32, radius: u32, color: C, fill: bool) -> Self {
        Ox.clamp(radius, SCREENWIDTH-radius);
        Oy.clamp(radius, SCREENHEIGHT-radius);
        let (r,g,b) = color.into_rgb();
        Self {
            Ox, Oy, radius, r, g, b, fill
        }
    }
}

impl Shape for Circle {
    fn draw(&self) {
        if let Some(geometry_gpu) = unsafe { (*GPU_DEVICE.unwrap()).as_geometry_mut() } {
            geometry_gpu.draw_circle(self.Ox, self.Oy, self.radius, self.r, self.g, self.b, self.fill);
        }
    }

    fn erase(&mut self) {
        unsafe {
            let theme = THEME;
            let r = ((theme.background() >> 16) & 0xff) as u8;
            let g = ((theme.background() >> 8) & 0xff) as u8;
            let b = ((theme.background()) & 0xff) as u8;
            if let Some(geometry_gpu) = unsafe { (*GPU_DEVICE.unwrap()).as_geometry_mut() } {
                geometry_gpu.draw_circle(self.Ox, self.Oy, self.radius, r, g, b, self.fill);
            }
        }
    }

    fn slide_h(&mut self, right: bool, dist: u32, spd: usize) {
        spd.clamp(1, 50);
        if right {
            dist.clamp(0, SCREENWIDTH-self.radius);
            for i in 0..dist {
                self.erase();
                self.Ox += 1;
                self.draw();
                thread::sleep(50/spd);
            }
        } else {
            dist.clamp(0, self.Ox-self.radius);
            for i in 0..dist {
                self.erase();
                self.Ox -= 1;
                self.draw();
                thread::sleep(50/spd);
            }
        }
    }

    fn slide_v(&mut self, down: bool, dist: u32, spd: usize) {
        spd.clamp(1, 50);
        if down {
            dist.clamp(0, SCREENHEIGHT-self.radius);
            for i in 0..dist {
                self.erase();
                self.Oy += 1;
                self.draw();
                thread::sleep(50/spd);
            }
        } else {
            dist.clamp(0, self.Oy-self.radius);
            for i in 0..dist {
                self.erase();
                self.Oy -= 1;
                self.draw();
                thread::sleep(50/spd);
            }
        }
    }
}