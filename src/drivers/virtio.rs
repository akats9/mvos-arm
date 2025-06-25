use crate::{mmio::{alloc, mmio_read32, mmio_read64, mmio_write32}, pci::*, serial_println};
use core::ptr;
use core::arch::asm;
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
const VQ_DESC_F_NEXT: u8 = 1;

const GPU_RSC_ID: u8 = 1;
const FB_BPP: u64 = 8;

static mut VQ_BASE: u64 = 0;
static mut VQ_AVAIL: u64 = 0;
static mut VQ_USED: u64 = 0;

static mut VQ_CMD: u64 = 0;
static mut VQ_RESP: u64 = 0;
static mut VQ_DISP_INFO: u64 = 0;

static mut display_height: u64 = 600;
static mut display_width: u64 = 800;
static fb_mem: u64 = 0x0;
static mut scout_id: u32 = 0;
static mut scout_found: bool = false;

static mut FB_SIZE: u64 = 800 * 600 * (FB_BPP/8);

pub unsafe fn update_fb_size() {
    FB_SIZE = display_width * display_height * (FB_BPP/8);
}

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

#[repr(C, packed)]
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
    q_msix_vec: u16,
    q_enable: u16,
    q_ntf_off: u16,
    q_desc: u64,
    q_drv: u64,
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

#[repr(C, packed)]
pub struct VqDesc {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

#[repr(C packed)]
pub struct VqAvail {
    flags: u16,
    idx: u16,
    ring: [u16; 128],
}

#[repr(C, packed)]
pub struct VqUsedElem {
    id: u32,
    len: u32,
}

#[repr(C, packed)]
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
    serial_println!("[VirtIO] Setting up GPU BAR @ {:x} from BAR {:x}", bar_addr, bar);

    mmio_write32(bar_addr, 0xFFFFFFFF);
    let mut bar_val = mmio_read64(bar_addr);

    if bar_val == 0 || bar_val == 0xffffffff {
        serial_println!("[VirtIO] BAR size probing failed");
        return 0x0;
    }

    let size = (!(bar_val & !0xF)) as u64 + 1;
    serial_println!("[VirtIO] Calculated BAR size: {:x}", size);

    let mmio_base: u64 = 0x10010000;
    mmio_write32(bar_addr, (mmio_base & 0xFFFFFFFF) as u32);

    bar_val = mmio_read64(bar_addr);

    serial_println!("[VirtIO] Final BAR value: {:x}", bar_val);

    let mut cmd: u32 = mmio_read32((base + 0x4) as u32);
    cmd |= 0x2;
    mmio_write32(base + 0x4, cmd);

    bar_val & !0xf
}

pub unsafe fn vgp_start() {
    serial_println!("[VirtIO] Starting VirtIO GPU Initialization...");
    
    (*common_cfg).dev_sts = 0;
    while (*common_cfg).dev_sts != 0 {}

    serial_println!("[VirtIO] Device reset.");

    (*common_cfg).dev_sts |= VIO_STS_ACK;
    serial_println!("[VirtIO] ACK sent.");

    (*common_cfg).dev_sts |= VIO_STS_DRV;
    serial_println!("[VirtIO] DRV sent.");

    (*common_cfg).dev_ftr_sel = 0;
    let features: u32 = (*common_cfg).dev_ftr;

    serial_println!("[VirtIO] Features received: {:x}.", features);

    (*common_cfg).drv_ftr_sel = 0;
    (*common_cfg).drv_ftr = features;

    (*common_cfg).dev_sts |= VIO_STS_FTR_OK;

    if ((*common_cfg).dev_sts & VIO_STS_FTR_OK) != 0 {
        serial_println!("[VirtIO] \033[1;31m[error]:\033[0m FTR_OK not accepted, device unusable.");
        return;
    }

    (*common_cfg).q_sel = 0;
    let q_size: u32 = (*common_cfg).q_size as u32;

    VQ_BASE = alloc(4096);
    VQ_AVAIL = alloc(4096);
    VQ_USED = alloc(4096);
    VQ_CMD = alloc(4096);
    VQ_RESP = alloc(4096);
    VQ_DISP_INFO = alloc(core::mem::size_of::<VioGpuRespDspInfo>() as u64);

    (*common_cfg).q_desc = VQ_BASE;
    (*common_cfg).q_drv = VQ_AVAIL;
    (*common_cfg).q_dev = VQ_USED;
    (*common_cfg).q_enable = 1;

    (*common_cfg).dev_sts |= VIO_STS_DRV_OK;

    serial_println!("[VirtIO] \033[1;32mVirtIO GPU initialization complete!\033[0m");
}

pub unsafe fn vgp_get_capabilities(address: u64) -> *mut VioPciCap {
    let offset: u64 = mmio_read32((address + 0x34) as u32) as u64;
    while offset != 0 {
        let cap_address: u64 = address + offset;
        let cap: *mut VioPciCap = cap_address as *mut VioPciCap;

        serial_println!(
            "[VirtIO] Inspecting @ {:x} = {:x} ({:x} + {:x}) TYPE {:x} -> {:x}",
            cap_address,
            (*cap).cap_vndr,
            (*cap).bar,
            (*cap).offset,
            (*cap).cfg_type,
            (*cap).cap_next,
        );

        let target: u64 = pci_get_bar(address, (*cap).bar, 0x10);
        let mut val: u64 = mmio_read32(target as u32) as u64 & !0xf;

        if (*cap).cap_vndr == 0x9 {
            if (*cap).cfg_type < VIO_PCI_CAP_PCI_CFG && val == 0 {
                val = vgp_setup_bars(address, (*cap).bar);
            }

            if (*cap).cfg_type == VIO_PCI_CAP_CMN_CFG {
                serial_println!("[VirtIO] Found COMMON config @ {:x}", val + (*cap).offset as u64);
                common_cfg = (val + (*cap).offset as u64) as *mut VioPciCmnCfg;
            } else if (*cap).cfg_type == VIO_PCI_CAP_NTF_CFG {
                serial_println!("[VirtIO] Found NOTIFY config @ {:x}", val + (*cap).offset as u64);
                ntf_cfg = (val + (*cap).offset as u64) as *mut u8;
            } else if (*cap).cfg_type == VIO_PCI_CAP_DEV_CFG {
                serial_println!("[VirtIO] Found DEVICE config @ {:x}", val + (*cap).offset as u64);
                dev_cfg = (val + (*cap).offset as u64) as *mut u8;
            } else if (*cap).cfg_type == VIO_PCI_CAP_ISR_CFG {
                serial_println!("[VirtIO] Found ISR config @ {:x}", val + (*cap).offset as u64);
                isr_cfg = (val + (*cap).offset as u64) as *mut u8;
            }
        }
    }

    0x0 as *mut VioPciCap
}

pub unsafe fn vgp_send_command(cmd_addr: u64, cmd_size: u32, resp_addr: u64, resp_size: u32, notify_base: u64, notify_multiplier: u32, flags: u8) {
    let mut desc: *mut VqDesc = (*common_cfg).q_desc as *mut VqDesc;
    let mut avail: *mut VqAvail = (*common_cfg).q_drv as *mut VqAvail;
    let mut used: *mut VqUsed = (*common_cfg).q_dev as *mut VqUsed;

    ptr::write_volatile(&mut (*desc.offset(0)).addr, cmd_addr);
    ptr::write_volatile(&mut (*desc.offset(0)).len, cmd_size);
    ptr::write_volatile(&mut (*desc.offset(0)).flags, flags as u16);
    ptr::write_volatile(&mut (*desc.offset(0)).next, 1);

    ptr::write_volatile(&mut (*desc.offset(1)).addr, resp_addr);
    ptr::write_volatile(&mut (*desc.offset(1)).len, resp_size);
    ptr::write_volatile(&mut (*desc.offset(1)).flags, VQ_DESC_F_WRT as u16);
    ptr::write_volatile(&mut (*desc.offset(1)).next, 0);

    (*avail).ring[((*avail).idx % 128) as usize] = 0;
    (*avail).idx += 1;

    ((notify_base + notify_multiplier as u64 * 0) as *mut u16).write_volatile(0);

    let mut last_used_idx = (*used).idx;
    while last_used_idx == (*used).idx {}
    last_used_idx = (*used).idx;
}

pub unsafe fn vgp_get_display_info() -> bool {
    let mut cmd: *mut VioGpuCtrlHdr = VQ_CMD as *mut VioGpuCtrlHdr;

    (*cmd).tipe = VIO_GPU_CMD_GET_DISP_INFO as u32;
    (*cmd).flags = 0;
    (*cmd).fence_id = 0;
    (*cmd).ctx_id = 0;
    (*cmd).ring_idx = 0;
    (*cmd).padding[0] = 0;
    (*cmd).padding[1] = 0;
    (*cmd).padding[2] = 0;

    serial_println!("[VirtIO] Command prepared.");

    vgp_send_command(cmd as u64, core::mem::size_of::<VioGpuCtrlHdr>() as u32, VQ_DISP_INFO, core::mem::size_of::<VioGpuRespDspInfo>() as u32, ntf_cfg as u64, ntf_off_mult, 0);

    let mut resp: *mut VioGpuRespDspInfo = VQ_DISP_INFO as *mut VioGpuRespDspInfo;

    for i in 0..VIO_MAX_SCOUTS {
        serial_println!("[VirtIO] Scanout {:x}: enabled = {:x} size = {:x} x {:x}", i, (*resp).pmodes[i as usize].enabled, (*resp).pmodes[i as usize].width, (*resp).pmodes[i as usize].height);

        if (*resp).pmodes[i as usize].enabled != 0 {
            serial_println!("[VirtIO] \033[032;1mFOUND A VALID DISPLAY: {:x} x {:x}\033[0m", (*resp).pmodes[i as usize].width, (*resp).pmodes[i as usize].height);

            display_width = (*resp).pmodes[i as usize].width as u64;
            display_height = (*resp).pmodes[i as usize].height as u64;
            update_fb_size();
            scout_id = i as u32;
            scout_found = true;
            return true;
        }
    }

    serial_println!("[VirtIO] \033[1;33mWarning: Display not enabled yet. Using default but not allowing scanout.\033[0m");
    (*resp).pmodes[0].width = 1024;
    (*resp).pmodes[0].height = 768;
    scout_found = false;

    false
}

#[repr(C)]
pub struct GpuResourceCreate2dCmd {
    hdr: VioGpuCtrlHdr,
    resource_id: u32,
    format: u32,
    width: u32,
    height: u32,
}

pub unsafe fn vgp_create_2d_resource() {
    let mut cmd: *mut GpuResourceCreate2dCmd = VQ_CMD as *mut GpuResourceCreate2dCmd;

    (*cmd).hdr.tipe = VIO_GPU_CMD_RSC_CRT_2D as u32;
    (*cmd).hdr.flags = 0;
    (*cmd).hdr.fence_id = 0;
    (*cmd).hdr.ctx_id = 0;
    (*cmd).hdr.ring_idx = 0;
    (*cmd).hdr.padding[0] = 0;
    (*cmd).hdr.padding[1] = 0;
    (*cmd).hdr.padding[2] = 0;
    (*cmd).resource_id = GPU_RSC_ID as u32; 
    (*cmd).format = 1; //VIRTIO_GPU_FORMAT_B8G8R8A8_UNORM
    (*cmd).width = display_width as u32;
    (*cmd).height = display_height as u32;

    vgp_send_command(cmd as u64, core::mem::size_of_val(&*cmd) as u32, VQ_RESP, core::mem::size_of::<VioGpuCtrlHdr>() as u32, ntf_cfg as u64, ntf_off_mult as u32, VQ_DESC_F_NEXT);

    asm!("", options(nostack, preserves_flags));

    let mut resp: *mut VioGpuCtrlHdr = VQ_RESP as *mut VioGpuCtrlHdr;

    serial_println!("[VirtIO] Response type: {:x} flags: {:x}", (*resp).tipe, (*resp).flags);

    if (*resp).tipe == 0x1100 {
        serial_println!("[VirtIO] \033[0;32mRESOURCE_CREATE_2D OK\033[0m");
    } else {
        serial_println!("[VirtIO] \033[1;31mRESOURCE_CREATE_2D ERROR: {:x}\033[0m", (*resp).tipe);
    }
}

#[repr(C, packed)]
pub struct GpuAttachBackingCommand {
    hdr: VioGpuCtrlHdr,
    resource_id: u32,
    nr_entries: u32,
}

#[repr(C)]
pub struct GpuAttachBackingEntry {
    addr: u64,
    length: u32,
    padding: u32,
}

pub unsafe fn vgp_attach_backing() {
    let ptr: *mut u8 = VQ_CMD as *mut u8;

    let cmd: *mut GpuAttachBackingCommand = ptr as *mut GpuAttachBackingCommand;
    let entry: *mut GpuAttachBackingEntry = (ptr + (core::mem::size_of::<GpuAttachBackingCommand>() as *mut u8)) as *mut GpuAttachBackingEntry;
}