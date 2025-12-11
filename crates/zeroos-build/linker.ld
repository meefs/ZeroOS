OUTPUT_ARCH(riscv)
ENTRY(_start)

MEMORY
{
    RAM (rwx) : ORIGIN = {MEMORY_ORIGIN}, LENGTH = {MEMORY_SIZE}
}

PHDRS
{
    text PT_LOAD FLAGS(5);        /* R-X */
    rodata PT_LOAD FLAGS(4);      /* R-- */
    data PT_LOAD FLAGS(6);        /* RW- */
    tls PT_TLS;                   /* TLS metadata */
}

SECTIONS
{
    . = {MEMORY_ORIGIN};
    PROVIDE_HIDDEN(__ehdr_start = .);
    
    .text : {
        *(.text.boot)
        *(.text .text.*)
        . = ALIGN(4);
    } > RAM : text
    
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(8);
    } > RAM : rodata
    
    .init_array : {
        PROVIDE_HIDDEN(__init_array_start = .);
        KEEP(*(SORT_BY_INIT_PRIORITY(.init_array.*)))
        KEEP(*(.init_array))
        PROVIDE_HIDDEN(__init_array_end = .);
    } > RAM : data
    
    .fini_array : {
        PROVIDE_HIDDEN(__fini_array_start = .);
        KEEP(*(SORT_BY_INIT_PRIORITY(.fini_array.*)))
        KEEP(*(.fini_array))
        PROVIDE_HIDDEN(__fini_array_end = .);
    } > RAM : data
    
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(8);
    } > RAM : data
    
    .tdata : ALIGN(16) {
        PROVIDE_HIDDEN(__tdata_start = .);
        *(.tdata .tdata.*)
        /* Force non-empty TLS section */
        . = . + 8;
        PROVIDE_HIDDEN(__tdata_end = .);
    } > RAM : data : tls
    
    .tbss : ALIGN(16) {
        PROVIDE_HIDDEN(__tbss_start = .);
        *(.tbss .tbss.*)
        *(.tcommon)
        /* Force non-empty TLS section */
        . = . + 8;
        PROVIDE_HIDDEN(__tbss_end = .);
    } > RAM : data : tls
    
    .bss : {
        PROVIDE(__bss_start = .);
        *(.bss .bss.*)
        *(COMMON)
        *(.sbss .sbss.*)
        . = ALIGN(8);
        PROVIDE(__bss_end = .);
        
        /* HTIF sections must be in writable memory */
        KEEP(*(.tohost));
        . = ALIGN(8);
        PROVIDE_HIDDEN(tohost = . - 8);
        KEEP(*(.fromhost));
        . = ALIGN(8);
        PROVIDE_HIDDEN(fromhost = . - 8);
    } > RAM : data
    
    /* Stack and Heap setup */
    /* Stack is at the top of RAM */
    __stack_size = {STACK_SIZE};
    __stack_top = ORIGIN(RAM) + LENGTH(RAM);
    __stack_bottom = __stack_top - __stack_size;
    
    /* Heap starts after BSS and goes up to Stack */
    .heap : {
        . = ALIGN(4096);
        PROVIDE(__heap_start = .);
        . = __stack_bottom;
        PROVIDE(__heap_end = .);
    } > RAM : data
    
    ASSERT(__heap_end >= __heap_start, "Error: memory overflow (heap size negative)")
    
    /DISCARD/ : {
        *(.eh_frame)
        *(.comment)
    }
}

