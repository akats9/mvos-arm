ENTRY(_start)

SECTIONS {
    . = 0x10000;

    .boot : {
        boot.o(.text)
    }

    .text : {
        *(.text*)    /* <---- FIXED this line */
    }

    .rodata : { *(.rodata*) }
    .data : { *(.data*) }

    .bss : {
        *(.bss*)
        *(COMMON)
    }

    . = ALIGN(8);
    . = . + 0x1000;
    stack_top = .;
}
