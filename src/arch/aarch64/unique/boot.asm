.section ".text.boot"

.globl _start

    .org 0x80000
_start:
    // Disable all cores except #0.
    mrs x1, mpidr_el1
    and x1, x1, #3
    cbnz x1, .hang
    // Set the stack to grow downwards from below the kernel.
    ldr x5, =_start
    mov sp, x5
    // Zero out BSS
    ldr x5, =__bss_start
    ldr w6, =__bss_size
.loop:
    cbz w6, .end
    str xzr, [x5], #8
    sub w6, w6, #1
    cbnz w6, .loop    
.end:
    bl rust_start
.hang:
    wfe
    b .hang
