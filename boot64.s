.global _Reset

_Reset:
    // Check processor ID is zero (executing on main core), else hang
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, primary_boot

secondary_hang:
	wfe
	b secondary_hang

primary_boot:
	ldr x0, =stack_top
	mov sp, x0
	mov x29, xzr
	mov x30, xzr
	bl kernel_main
	b .
