@ The text section is a section of an ELF (Executable Linkable Format) file
@ that can be executed by the CPU.
.section .text

@ The CPU needs to have an exported star symbol
.section _start

_start:
    mov r0, pc
    sub r0, #8
    bx r0

@ Infinite loop. Don't run
