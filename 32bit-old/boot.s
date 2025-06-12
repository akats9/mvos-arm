.section .text._start
.global _start

_start:
    ldr r1, =stack_top
    mov sp, r1
    mov fp, #0
    mov lr, #0
    
    bl kernel_entry
    
halt:
    wfe
    b halt

.section .bss
.align 4
stack_bottom:
    .space 4096 * 16  @ 64KB stack
stack_top:
