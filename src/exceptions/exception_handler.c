#include "exception_handler.h"
#include "../include/mvos_bindings.h"


// Handlers for current el exceptions with sp0
// These shouldnt happen in normal kernel code, panic

void sync_current_el_sp0_handler() {
    c_panic("SYNC_CURRENT_EL_SP0_HANDLER INVOKED");
}

void fiq_current_el_sp0_handler() {
    c_panic("FIQ_CURRENT_EL_SP0_HANDLER INVOKED");
}

void irq_current_el_sp0_handler() {
    c_panic("IRQ_CURRENT_EL_SP0_HANDLER INVOKED");
}

void serror_current_el_sp0_handler() {
    c_panic("SERROR_CURRENT_EL_SP0_HANDLER INVOKED");
}


// Handlers for 64bit userspace exceptions
// No userspace yet, panic

void sync_lower_el_aarch64_handler() {
    c_panic("SYNC_LOWER_EL_AARCH64_HANDLER INVOKED");
}

void fiq_lower_el_aarch64_handler() {
    c_panic("FIQ_LOWER_EL_AARCH64_HANDLER INVOKED");
}

void irq_lower_el_aarch64_handler() {
    c_panic("IRQ_LOWER_EL_AARCH64_HANDLER INVOKED");
}

void serror_lower_el_aarch64_handler() {
    c_panic("SERROR_LOWER_EL_AARCH64_HANDLER INVOKED");
}

// Handlers for 32bit userspace exceptions
// No userspace yet, panic

void sync_lower_el_aarch32_handler() {
    c_panic("SYNC_LOWER_EL_AARCH32_HANDLER INVOKED");
}

void fiq_lower_el_aarch32_handler() {
    c_panic("FIQ_LOWER_EL_AARCH32_HANDLER INVOKED");
}

void irq_lower_el_aarch32_handler() {
    c_panic("IRQ_LOWER_EL_AARCH32_HANDLER INVOKED");
}

void serror_lower_el_aarch32_handler() {
    c_panic("SERROR_LOWER_EL_AARCH32_HANDLER INVOKED");
}