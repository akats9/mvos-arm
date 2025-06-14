use crate::serial_println;

#[derive(Debug)]
pub enum SupportedGPU {
    NONE,
    VIRTIO_GPU_PCI,
    RAMFB,
}

pub static mut chosen_gpu: SupportedGPU = SupportedGPU::NONE;

pub fn gpu_init() {
    if vgp_init() { unsafe { chosen_gpu = SupportedGPU::VIRTIO_GPU_PCI; } }
    unsafe { serial_println!("Selected and initialized GPU {:#?}", chosen_gpu); }
}

pub fn gpu_flush() {
    unsafe { 
        match chosen_gpu {
            SupportedGPU::VIRTIO_GPU_PCI => {
                vgp_transfer_to_host();
                vgp_flush();
            }
            _ => {}
        } 
    }
}

pub fn gpu_clear(color: u32) {
    unsafe {
        match chosen_gpu {
            SupportedGPU::VIRTIO_GPU_PCI => vgp_clear(color),
            _ => {}
        }
    }
}