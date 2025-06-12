.global _start
_start:
	ldr sp, =stack_top
	bl kernel_main
1:	b .
