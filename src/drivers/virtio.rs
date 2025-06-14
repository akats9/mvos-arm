use crate::{mmio::{mmio_read32, mmio_read64, mmio_write32}, pci::*, serial_println};

use super::*;

const VIO_STS_RST: u8 = 0x0;
const VIO_STS_ACK: u8 = 0x1;
const VIO_STS_DRV: u8 = 0x2;
const VIO_STS_DRV_OK: u8 = 0x4;
const VIO_STS_FTR_OK: u8 = 0x8;
const VIO_STS_FAILED: u8 = 0x80;

const VIO_GPU_CMD_GET_DISP_INFO: u16 = 0x100;
const VIO_GPU_CMD_RSC_CRT_2D: u16 = 0x101;
const VIO_GPU_CMD_SET_SCOUT: u16 = 0x102;
const VIO_GPU_CMD_RSC_FLS: u16 = 0x103;
const VIO_GPU_CMD_TRS_TO_HOST_2D: u16 = 0x104;
const VIO_GPU_RSC_ATT_BKING: u16 = 0x106;

const VIO_PCI_CAP_CMN_CFG: u8 = 1;
const VIO_PCI_CAP_NTF_CFG: u8 = 2;
const VIO_PCI_CAP_ISR_CFG: u8 = 3;
const VIO_PCI_CAP_DEV_CFG: u8 = 4;
const VIO_PCI_CAP_PCI_CFG: u8 = 5;
const VIO_PCI_CAP_VDR_CFG: u8 = 9;

const VDR_ID: u16 = 0x1AF4;
const DEV_ID_BASE: u16 = 0x1040;
const GPU_DEV_ID: u16 = 0x10;

const VIO_MAX_SCOUTS: u8 = 16;

const VQ_DESC_F_WRT: u8 = 2;

const GPU_RSC_ID: u8 = 1;
const FB_BPP: u64 = 8;

static display_height: u64 = 600;
static display_width: u64 = 800;
static fb_mem: u64 = 0x0;
static scout_id: u32 = 0;
static scout_found: bool = false;

const FB_SIZE: u64 = display_width * display_height * (FB_BPP/8);

#[repr(C)]
pub struct VioPciCap {
    cap_vndr: u8,
    cap_next: u8,
    cap_len: u8,
    cfg_type: u8,
    bar: u8,
    id: u8,
    padding: [u8; 2],
    offset: u32,
    len: u32,
}

#[repr(C)]
pub struct VioPciCmnCfg {
    dev_ftr_sel: u32,
    dev_ftr: u32,
    drv_ftr_sel: u32,
    drv_ftr: u32,
    msix_cfg: u16,
    num_qs: u16,
    dev_sts: u8,
    cfg_gen: u8,
    q_sel: u16,
    q_size: u16,
    q_ntf_off: u16,
    q_desc: u64,
    q_dev: u64,
    q_ntf_data: u16,
    q_rst: u16,
}

#[repr(C)]
pub struct VioGpuCtrlHdr {
    tipe: u32,
    flags: u32,
    fence_id: u64,
    ctx_id: u32,
    ring_idx: u8,
    padding: [u8; 3],
}

#[repr(C)]
pub struct VioRect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[repr(C)]
pub struct VioGpuDspOne {
    enabled: u32,
    flags: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[repr(C)]
pub struct VioGpuRespDspInfo {
    hdr: VioGpuCtrlHdr,
    pmodes: [VioGpuDspOne; VIO_MAX_SCOUTS as usize],
}

#[repr(C)]
pub struct VqDesc {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

#[repr(C)]
pub struct VqAvail {
    flags: u16,
    idx: u16,
    ring: [u16; 128],
}

#[repr(C)]
pub struct VqUsedElem {
    id: u32,
    len: u32,
}

#[repr(C)]
pub struct VqUsed {
    flags: u16,
    idx: u16,
    ring: [VqUsedElem; 128]
}

static mut common_cfg: *mut VioPciCmnCfg = core::ptr::null_mut();
static mut ntf_cfg: *mut u8 = 0 as *mut u8;
static mut dev_cfg: *mut u8 = 0 as *mut u8;
static mut isr_cfg: *mut u8 = 0 as *mut u8;
static mut ntf_off_mult: u32 = 0;

pub fn vgp_setup_bars(base: u64, bar: u8) -> u64 {
    let bar_addr: u64 = pci_get_bar(base, bar, 0x10);
    serial_println!("Setting up GPU BAR @ {:x} from BAR {:x}", bar_addr, bar);

    mmio_write32(bar_addr, 0xFFFFFFFF);
    let mut bar_val = mmio_read64(bar_addr);

    if bar_val == 0 || bar_val == 0xffffffff {
        serial_println!("BAR size probing failed");
        return 0x0;
    }

    let size = (!(bar_val & !0xF)) as u64 + 1;
    serial_println!("Calculated BAR size: {:x}", size);

    let mmio_base: u64 = 0x10010000;
    mmio_write32(bar_addr, (mmio_base & 0xFFFFFFFF) as u32);

    bar_val = mmio_read64(bar_addr);

    serial_println!("Final BAR value: {:x}", bar_val);

    let mut cmd: u32 = mmio_read32((base + 0x4) as u32);
    cmd |= 0x2;
    mmio_write32(base + 0x4, cmd);

    bar_val & !0xf
}