#ifndef EXCEPTION_HANDLER_H_
#define EXCEPTION_HANDLER_H_

#include "../include/types.h"

void serror_lower_el_aarch32_handler();
void irq_lower_el_aarch32_handler();
void fiq_lower_el_aarch32_handler();
void sync_lower_el_aarch32_handler();
void serror_lower_el_aarch64_handler();
void irq_lower_el_aarch64_handler();
void fiq_lower_el_aarch64_handler();
void sync_lower_el_aarch64_handler();
void sync_current_el_sp0_handler();
void irq_current_el_sp0_handler();
void fiq_current_el_sp0_handler();
void serror_current_el_sp0_handler();

#endif