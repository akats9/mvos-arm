use core::{
    ffi::CStr, mem, num, ptr::addr_of
};

use crate::serial_println;

#[repr(C, align(8))]
#[derive(Debug)]
pub struct FWCfgFile {
    size: u32,
    select: u16,
    reserved: u16,
    name: [u8; 56],
}

impl FWCfgFile {
    fn zero() -> Self {
        serial_println!("[   RAMFB   ] \x1b[0;33mDebug: Entered FWCfgFile::zero() constuctor.\x1b[0m");
        let mut zero = Self {
            size: 1_u32,
            select: 1_u16,
            reserved: 1_u16,
            name: [1_u8; 56],
        };
        serial_println!("[   RAMFB   ] \x1b[0;33mDebug: Declared 1 FWCfgFile struct in constructor.\x1b[0m");
        zero
    }
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct FWCfgDmaAccess {
    control: u32,
    len: u32,
    addr: u64,
}

#[repr(C, packed)]
pub struct RamFBCfg {
    addr: u64,
    fmt: u32,
    flags: u32,
    width: u32,
    height: u32,
    st: u32,
}

const QEMU_CFG_DMA_CTL_ERROR:  u32 = 0x01;
const QEMU_CFG_DMA_CTL_READ:   u32 = 0x02;
const QEMU_CFG_DMA_CTL_SELECT: u32 = 0x08;
const QEMU_CFG_DMA_CTL_WRITE:  u32 = 0x10;

unsafe fn qemu_dma_transfer(control: u32, len: u32, addr: u64) {

    serial_println!("[   RAMFB   ] \x1b[0;33mDebug: Entered qemu_dma_transfer()\x1b[0m");
    
    //Address of the DMA register on the aarch64 virt board
    let fw_cfg_dma: *mut u64 = 0x9020010 as *mut u64;

    serial_println!("[   RAMFB   ]\x1b[0;33m Debug: declared fw_cfg_dma pointer with address 0x{:x}\x1b[0m", fw_cfg_dma as u64);

    let dma = FWCfgDmaAccess {
        control: control.to_be(),
        len: len.to_be(),
        addr: addr.to_be(),
    };

    serial_println!("[   RAMFB   ] \x1b[0;33mDebug: declared dma struct variable: {:?}\x1b[0m", dma);

    unsafe {
        fw_cfg_dma.write_volatile((addr_of!(dma) as u64).to_be());
    }

    serial_println!("[   RAMFB   ] \x1b[0;33mDebug: DMA struct written; waiting.\x1b[0m");

    while (dma.control & !QEMU_CFG_DMA_CTL_ERROR) != 0 {}

    serial_println!("[   RAMFB   ] \x1b[0;33mDebug: loop exited; checking error bit.\x1b[0m");

    if (dma.control & QEMU_CFG_DMA_CTL_ERROR) == 1 {
        serial_println!("[   RAMFB   ] \x1B[1;31mERROR: An error occured in qemu_dma_transfer\x1B[0m");
    }

    serial_println!("[   RAMFB   ] \x1b[0;33mDebug: Error bit ok; returning.\x1b[0m");
}

pub fn setup_ramfb(fb_addr: *mut u64, width: u32, height: u32) {
    let mut num_entries: u32 = 0xFFFFFFFF;
    let fw_cfg_file_directory = 0x19;

    serial_println!("[   RAMFB   ] \x1b[0;33mDebug: Setup started.\x1b[0m");

    unsafe {
        qemu_dma_transfer((fw_cfg_file_directory << 16 | QEMU_CFG_DMA_CTL_SELECT | QEMU_CFG_DMA_CTL_READ) as u32, mem::size_of::<u32>() as u32, addr_of!(num_entries) as u64);
    }

    serial_println!("[   RAMFB   ] \x1B[0;33mDebug: first dma transfer done.\x1B[0m");

    //QEMU DMA is BE so need to byte swap arguments and results on LE
    num_entries = num_entries.to_be();

    serial_println!("[   RAMFB   ] \x1b[0;33mDebug: Found QEMU entries: {}\x1b[0m", num_entries);

    let ramfb = FWCfgFile::zero(); 

    serial_println!("[   RAMFB   ] \x1b[0;33mDebug: Declared ramfb struct variable: {:?}\x1b[0m", ramfb);

    serial_println!("[   RAMFB   ] \x1b[0;33mDebug: Entering 0..num_entries for loop.\x1b[0m");

    for _ in 0..num_entries {
        unsafe {
            qemu_dma_transfer(QEMU_CFG_DMA_CTL_READ, mem::size_of::<FWCfgFile>() as u32, addr_of!(ramfb) as u64);
        }

        serial_println!("[   RAMFB   ] \x1b[0;33mDebug: Transfer done.\x1b[0m");

        serial_println!("[   RAMFB   ] \x1b[0;33mDebug: attempting to create a C String entry variable from &ramfb.name: {:?}\x1b[0m", &ramfb.name);
        
        let entry = match CStr::from_bytes_until_nul(&ramfb.name) {
            Ok(cstr) => cstr,
            Err(e) => {
                serial_println!("[   RAMFB   ] \x1b[1;31mERROR: CStr error: {:?}", e);
                CStr::from_bytes_until_nul(&[0;12]).unwrap()
            }
        };

        serial_println!("[   RAMFB   ] \x1b[0;33mDebug: Declared entry variable.\x1b[0m");

        let entry = entry.to_str().unwrap();
        if entry == "etc/ramfb" {
            serial_println!("[   RAMFB   ] \x1b[0;33mDebug: Found entry \"etc/ramfb\", breaking from loop.\x1b[0m");
            break;
        }
    }

    serial_println!("[   RAMFB   ] \x1B[0;33mDebug: dma transfer loop done.\x1B[0m");

    //See fourcc : https://github.com/qemu/qemu/blob/54294b23e16dfaeb72e0ffa8b9f13ca8129edfce/include/standard-headers/drm/drm_fourcc.h#L188

    //serial_println!("[   RAMFB   ] {:x}", ramfb.select.to_be());

    let pixel_format = ('R' as u32) | (('G' as u32) << 8) | (('2' as u32) << 16) | (('4' as u32) << 24);

    //Stride 0 means QEMU calculates from bpp_of_format*width: https://github.com/qemu/qemu/blob/54294b23e16dfaeb72e0ffa8b9f13ca8129edfce/hw/display/ramfb.c#L60

    //serial_println!("[   RAMFB   ] Placing FB at 0x{:x}", fb_addr as u64); 

    let bpp = 4;

    let ramfb_cfg = RamFBCfg {
        addr: (fb_addr as u64).to_be(),
        fmt: (pixel_format).to_be(),
        flags: (0_u32).to_be(),
        width: (width as u32).to_be(),
        height: (height as u32).to_be(),
        st: (bpp*width as u32).to_be(),
    };

    unsafe {
        qemu_dma_transfer((ramfb.select.to_be() as u32) << 16 | QEMU_CFG_DMA_CTL_SELECT | QEMU_CFG_DMA_CTL_WRITE, mem::size_of::<RamFBCfg>() as u32, addr_of!(ramfb_cfg) as u64);
    }

    //serial_println!("[   RAMFB   ] \x1B[0;33mDebug: final dma transfer done.\x1B[0m");
}