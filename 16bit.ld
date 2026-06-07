OUTPUT_FORMAT(binary)
ENTRY(main)

SECTIONS {
    . = 0x7c00;
    .text : { *(.text .text.*) }
    .rodata : { *(.rodata .rodata.*) }
    .data : { *(.data .data.*) }
    .bss : { *(.bss .bss.*) }
    /DISCARD/ : {
        *(.eh_frame*)
        *(.eh_frame_hdr*)
        *(.comment*)
        *(.note*)
        *(.debug*)
    }
    . = 0x7c00 + 510;
    .magic_number : { SHORT(0xaa55) }
}
