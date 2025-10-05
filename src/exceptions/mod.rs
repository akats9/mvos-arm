use core::{arch::asm, panic};

use crate::{dbg, serial_println};

pub unsafe fn set_exception_vectors() {
    unsafe extern "C" { static exception_vectors: [u8; 0]; }

    let vector_addr = exception_vectors.as_ptr() as u64;
    if (vector_addr & 0x7ff) != 0 {
        panic!("PANIC: EXCEPTION VECTORS MUST BE 2KB ALIGNED"); 
    }

    // Set VBAR_EL1
    asm!(
        "msr vbar_el1, {}",
        "isb",
        in(reg) vector_addr,
        options(nostack, preserves_flags)
    );
}

#[derive(Debug,Copy,Clone)]
#[repr(C)]
pub struct InterruptFrame {
    x0: u64,
    x1: u64,
    x2: u64,
    x3: u64,
    x4: u64,
    x5: u64,
    x6: u64,
    x7: u64,
    x8: u64,
    x9: u64,
    x10: u64,
    x11: u64,
    x12: u64,
    x13: u64,
    x14: u64,
    x15: u64,
    x16: u64,
    x17: u64,
    x18: u64,
    x29: u64,
    x30: u64,
    elr: u64,
    esr: u64, 
    far: u64,
}

#[derive(Debug, Clone, Copy)]
enum ExceptionClass {
    Unknown = 0x00,
    WfiWfe = 0x01,
    SimdFp = 0x07,
    IllegalExecution = 0x0e,
    SvcAarch32 = 0x11,
    SvcAarch64 = 0x15,
    MsrMrsTrap = 0x18,
    InstructionAbortLowerEL = 0x20,
    InstructionAbortSameEL = 0x21,
    PcAlignment = 0x22,
    DataAbortLowerEL = 0x24,
    DataAbortSameEL = 0x25,
    SpAlignment = 0x26,
    FpAarch32 = 0x28, 
    FpAarch64 = 0x2c,
    Serror = 0x2f,
    BreakpointLowerEL = 0x30,
    BreakpointSameEL = 0x31,
    SoftwareStepLowerEL = 0x32,
    SoftwareStepSameEL = 0x33,
    WatchpointLowerEL = 0x34,
    WatchpointSameEL = 0x35,
    BkptAarch32 = 0x38,
    BrkAarch64 = 0x3c,
}

impl From<u8> for ExceptionClass {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Unknown,
            0x01 => Self::WfiWfe,
            0x07 => Self::SimdFp,
            0x0E => Self::IllegalExecution,
            0x11 => Self::SvcAarch32,
            0x15 => Self::SvcAarch64,
            0x18 => Self::MsrMrsTrap,
            0x20 => Self::InstructionAbortLowerEL,
            0x21 => Self::InstructionAbortSameEL,
            0x22 => Self::PcAlignment,
            0x24 => Self::DataAbortLowerEL,
            0x25 => Self::DataAbortSameEL,
            0x26 => Self::SpAlignment,
            0x28 => Self::FpAarch32,
            0x2C => Self::FpAarch64,
            0x2F => Self::Serror,
            0x30 => Self::BreakpointLowerEL,
            0x31 => Self::BreakpointSameEL,
            0x32 => Self::SoftwareStepLowerEL,
            0x33 => Self::SoftwareStepSameEL,
            0x34 => Self::WatchpointLowerEL,
            0x35 => Self::WatchpointSameEL,
            0x38 => Self::BkptAarch32,
            0x3C => Self::BrkAarch64,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
struct EsrInfo {
    exception_class: ExceptionClass,
    instruction_length: bool, // 0 = 16bit, 1 = 32bit
    instruction_specific_syndrome: u32,
}

impl EsrInfo {
    fn parse(esr: u64) -> Self {
        let exception_class = ExceptionClass::from(((esr >> 26) & 0x3f) as u8);
        let instruction_length = (esr & (1 << 25)) != 0;
        let instruction_specific_syndrome = (esr & 0x1ffffff) as u32;

        Self {
            exception_class,
            instruction_length,
            instruction_specific_syndrome,
        }
    }
}

#[derive(Debug)]
struct DataAbortInfo {
    valid: bool,
    write_not_read: bool,
    s1ptw: bool, 
    cache_maintenance: bool, 
    external_abort_type: u8,
    fault_status_code: u8,
    access_size: u8,
}

impl DataAbortInfo {
    fn parse_data_abort_iss(iss: u32) -> Self {
        Self {
            valid: (iss & (1 << 24)) != 0,
            write_not_read: (iss & (1 << 6)) != 0,
            s1ptw: (iss & (1 << 7)) != 0,
            cache_maintenance: (iss & (1 << 8)) != 0,
            external_abort_type: ((iss >> 9) & 0x3) as u8,
            fault_status_code: (iss & 0x3F) as u8,
            access_size: ((iss >> 22) & 0x3) as u8,
        }
    }

    fn get_fault_type(&self) -> &'static str {
        let fsc = self.fault_status_code;
        match fsc & 0x3C {
            0x00 => match fsc & 0x03 {
                0 => "Address size fault, level 0",
                1 => "Address size fault, level 1", 
                2 => "Address size fault, level 2",
                3 => "Address size fault, level 3",
                _ => unreachable!(),
            },
            0x04 => match fsc & 0x03 {
                0 => "Translation fault, level 0",
                1 => "Translation fault, level 1",
                2 => "Translation fault, level 2", 
                3 => "Translation fault, level 3",
                _ => unreachable!(),
            },
            0x08 => match fsc & 0x03 {
                0 => "Access flag fault, level 0",
                1 => "Access flag fault, level 1",
                2 => "Access flag fault, level 2",
                3 => "Access flag fault, level 3", 
                _ => unreachable!(),
            },
            0x0C => match fsc & 0x03 {
                0 => "Permission fault, level 0",
                1 => "Permission fault, level 1",
                2 => "Permission fault, level 2",
                3 => "Permission fault, level 3",
                _ => unreachable!(),
            },
            0x10 => "Synchronous external abort",
            0x18 => "Synchronous parity or ECC error",
            0x1C => "Synchronous parity or ECC error on translation table walk",
            0x20 => "Alignment fault",
            0x30 => "TLB conflict abort",
            0x34 => "Implementation defined fault",
            _ => "Unknown fault",
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sync_current_el_spx_handler(frame: *mut InterruptFrame) {
    let frame = *frame;
    serial_println!("[ EXCEPTION ] Synchronous exception occured, ELR: 0x{:x}, ESR: 0x{:x}, FAR: 0x{:x}", frame.elr, frame.esr, frame.far);
    serial_println!("[ EXCEPTION ] Attempting to parse exception...");
    
    let esr_info = EsrInfo::parse(frame.esr);
    match esr_info.exception_class {
        ExceptionClass::DataAbortLowerEL | ExceptionClass::DataAbortSameEL => {
            let abort_info = DataAbortInfo::parse_data_abort_iss(esr_info.instruction_specific_syndrome);
            let fault_addr = frame.far;
            let fault_pc = frame.elr;

            serial_println!("[ EXCEPTION ] Data abort at PC: {:#018x}, Address: {:#018x}", fault_pc, fault_addr);
            serial_println!("[ EXCEPTION ] Fault type: {}", abort_info.get_fault_type());
            serial_println!("[ EXCEPTION ] Access: {}", if abort_info.write_not_read {"Write"} else {"Read"});

            handle_page_fault(fault_addr, fault_pc, abort_info.write_not_read);
        },

        ExceptionClass::InstructionAbortLowerEL | ExceptionClass::InstructionAbortSameEL => {
            let fault_addr = frame.elr;
            serial_println!("[ EXCEPTION ] Instruction aborted at PC: 0x{:#018x}", fault_addr);
            handle_instruction_abort(fault_addr);
        },

        ExceptionClass::PcAlignment => {
            serial_println!("[ EXCEPTION ] PC alignment exception at 0x{:#018x}", frame.elr);
            panic!("UNALIGNED PC");
        },

        ExceptionClass::SpAlignment => {
            serial_println!("[ EXCEPTION ] SP alignment exception at 0x{:018x}", frame.elr);
            panic!("UNALIGNED SP");
        }, 

        _ => {
            serial_println!("[ EXCEPTION ] Unhandled exception: {:?}", esr_info.exception_class);
            panic!("UNHANDLED EXCEPTION");
        }
    }
}

fn handle_page_fault(fault_addr: u64, fault_pc: u64, access: bool) {
    unimplemented!()
}

fn handle_instruction_abort(fault_addr: u64) {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn interrupt_handler() {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn serror_current_el_spx_handler() {
    unimplemented!()
}