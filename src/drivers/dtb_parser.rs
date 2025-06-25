use core::mem;

use crate::serial_println;

pub const FDT_BEGIN_NODE: u32 = 0x1;
pub const FDT_END_NODE: u32 = 0x2;
pub const FDT_PROP: u32 = 0x3;
pub const FDT_NOP: u32 = 0x4;
pub const FDT_END: u32 = 0x9;

#[repr(C)]
struct FdtHeader {
    magic: u32, //0xd00dfeed (BE)
    totalsize: u32, //Total DTB size
    off_dt_struct: u32, //Offset to structure block
    off_dt_strings: u32, //Offset to strings block
    off_mem_rsvmap: u32, //Offset to memory reservation map
    version: u32, //DTB version
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32, //Size of strings block
    size_dt_struct: u32, //Size of structure block
}

pub struct DeviceTreeParser {
    dtb_base: *const u8,
    header: &'static FdtHeader,
    struct_block: *const u32,
    strings_block: *const u8,
}

impl DeviceTreeParser {
    pub fn new(dtb_ptr: *const u8) -> Result<Self, &'static str> {
        unsafe {
            let header = &*(dtb_ptr as *const FdtHeader);

            //Verify magic number (convert from BE)
            if u32::from_be(header.magic) != 0xd00dfeed {
                return Err("[DEVICE TREE] \x1B[1;31mERROR: Invalid DTB magic\x1B[0m");
            }

            let struct_block = dtb_ptr.add(u32::from_be(header.off_dt_struct) as usize) as *const u32;
            let strings_block = dtb_ptr.add(u32::from_be(header.off_dt_strings) as usize);

            Ok(DeviceTreeParser { dtb_base: dtb_ptr, header, struct_block, strings_block, })
        }
    }

    pub fn get_string(&self, offset: u32) -> &'static str {
        unsafe {
            let str_ptr = self.strings_block.add(offset as usize);
            let mut len = 0;
            while *str_ptr.add(len) != 0 { len += 1; }
            let slice = core::slice::from_raw_parts(str_ptr, len);
            core::str::from_utf8_unchecked(slice)
        }
    }

    pub fn debug_dtb(&self) {
        unsafe {
            serial_println!("[DEVICE TREE] \x1B[0;33mHeader validation:\x1B[0m");
            serial_println!("[DEVICE TREE] \x1B[0;33m  Magic: 0x{:x} (expected: 0xd00dfeed)\x1B[0m", u32::from_be(self.header.magic));
            serial_println!("[DEVICE TREE] \x1B[0;33m  Total size: {}\x1B[0m", u32::from_be(self.header.totalsize));
            serial_println!("[DEVICE TREE] \x1B[0;33m  Structure offset: 0x{:x}\x1B[0m", u32::from_be(self.header.off_dt_struct));
            serial_println!("[DEVICE TREE] \x1B[0;33m  Strings offset: 0x{:x}\x1B[0m", u32::from_be(self.header.off_dt_strings));
            serial_println!("[DEVICE TREE] \x1B[0;33m  Version: {}\x1B[0m", u32::from_be(self.header.version));
        }
    }
}

impl DeviceTreeParser {
    pub fn find_ramfb(&self) -> Option<(u64, u64)> {
        unsafe {
            let mut ptr = self.struct_block;
            let mut in_ramfb_node = false;
            
            loop {
                let token = u32::from_be(*ptr);
                ptr = ptr.add(1);
                
                match token {
                    FDT_BEGIN_NODE => {
                        // Read node name
                        let name_ptr = ptr as *const u8;
                        let name = self.read_string_at(name_ptr);
                        
                        // Skip to next 4-byte boundary
                        ptr = self.align_ptr(ptr as *const u8) as *const u32;
                        
                        // Check if this is a RAMFB node (name often starts with address)
                        in_ramfb_node = false; // Will be set by compatible property
                    }
                    
                    FDT_PROP => {
                        let len = u32::from_be(*ptr);
                        ptr = ptr.add(1);
                        let nameoff = u32::from_be(*ptr);
                        ptr = ptr.add(1);
                        
                        let prop_name = self.get_string(nameoff);
                        let prop_data = ptr as *const u8;
                        
                        if prop_name == "compatible" {
                            let compat_str = self.read_string_at(prop_data);
                            if compat_str == "qemu,fw-cfg-mmio" {
                                in_ramfb_node = true;
                            }
                        }
                        
                        if prop_name == "reg" && in_ramfb_node {
                            // Parse reg property (address, size)
                            let reg_data = prop_data as *const u32;
                            let addr_high = u32::from_be(*reg_data);
                            let addr_low = u32::from_be(*reg_data.add(1));
                            let size_high = u32::from_be(*reg_data.add(2));
                            let size_low = u32::from_be(*reg_data.add(3));
                            
                            let addr = ((addr_high as u64) << 32) | (addr_low as u64);
                            let size = ((size_high as u64) << 32) | (size_low as u64);

                            serial_println!("[DEVICE TREE] \x1B[0;33mReturning Some({:x}, {:x})\x1B[0m", addr, size);
                            
                            return Some((addr, size));
                        }
                        
                        // Skip property data (align to 4 bytes)
                        ptr = self.align_ptr(prop_data.add(len as usize)) as *const u32;
                    }
                    
                    FDT_END_NODE => {
                        in_ramfb_node = false;
                    }
                    
                    FDT_END => break,
                    
                    FDT_NOP => {} // Skip
                    
                    _ =>  { 
                        serial_println!("[DEVICE TREE] \x1B[0;33mInvalid token; returning None\x1B[0m");
                        return None 
                    }, // Invalid token
                }
            }
        }
        serial_println!("[DEVICE TREE -> FIND_RAMFB] \x1B[0;33mLoop break; returning None\x1B[0m");
        None
    }
    
    pub fn align_ptr(&self, ptr: *const u8) -> *const u8 {
        let addr = ptr as usize;
        let aligned = (addr + 3) & !3; // Align to 4 bytes
        aligned as *const u8
    }
    
    pub fn read_string_at(&self, ptr: *const u8) -> &'static str {
        unsafe {
            let mut len = 0;
            while *ptr.add(len) != 0 { len += 1; }
            let slice = core::slice::from_raw_parts(ptr, len);
            core::str::from_utf8_unchecked(slice)
        }
    }
}