.global _Reset
_Reset:
	ldr x0, =stack_top
	mov sp, x0
	mov x29, xzr
	mov x30, xzr
	bl kernel_main
	b .
