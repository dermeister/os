MULTIBOOT_MAGIC equ 0x1BADB002
MULTIBOOT_FLAGS equ 00000011b

section .multiboot
align 4
    dd MULTIBOOT_MAGIC
    dd MULTIBOOT_FLAGS
    dd -(MULTIBOOT_MAGIC + MULTIBOOT_FLAGS)

section .text
global _start
_start:
    mov esp, stack_top

    extern main
    push ebx
    call main
    pop ebx

    .loop:
        hlt
        jmp .loop

section .bss
align 16
    resb 4 * 4096
stack_top:
