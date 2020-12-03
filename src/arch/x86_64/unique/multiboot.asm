MAGIC        equ 0xe85250d6
ARCHITECTURE equ 0 ; x86 (32-bit)
; Video mode
VIDEO_TYPE   equ 5
VIDEO_FLAGS  equ 1 ; Framebuffer is optional
VIDEO_SIZE   equ 20
VIDEO_WIDTH  equ 1024
VIDEO_HEIGHT equ 768
VIDEO_DEPTH  equ 32

section .multiboot_header
header_start:
    dd MAGIC
    dd ARCHITECTURE
    dd header_end - header_start
    dd 0x100000000 - (MAGIC + 0 + (header_end - header_start))
    ; Video mode header
    dw VIDEO_TYPE
    dw VIDEO_FLAGS
    dd VIDEO_SIZE   
    dd VIDEO_WIDTH
    dd VIDEO_HEIGHT
    dd VIDEO_DEPTH
align 8
    ; End tag
    dw 0
    dw 0
    dd 8
header_end:
