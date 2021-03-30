ENTRY(_start)

SECTIONS
{
    . = 0x00100000;

    .multiboot ALIGN(4K): AT(0x00100000)
    {
        *(.multiboot)
    }
}
