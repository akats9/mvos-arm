ENTRY(_Start)
SECTIONS
{
  	. = 0x40000000; /* kernel load address */
  	kernel_start = . ;
	.startup . : { boot.o(.text) }
	.text : { *(.text) }
	.data : { *(.data) }
	.bss : { *(.bss COMMON) } 
	
	. = ALIGN(8);
	. = . + 0x2800000; /* 40MiB of stack memory */
	stack_top = .;

	.bss : {
        sbss = .;
        *(COMMON)
        *(.bss)
        ebss = .;
	
    }

	. = 0x41000000;
	heap_bottom = .;
}