.section .nro_header
.global __nro_header_start
.word 0
.word _mod_header
.word 0
.word 0

.section .rodata.mod0
.global _mod_header
_mod_header:
    .ascii "MOD0"
    .word __dynamic_start - _mod_header
    .word __bss_start - _mod_header
    .word __bss_end - _mod_header
    .word __eh_frame_hdr_start - _mod_header
    .word __eh_frame_hdr_end - _mod_header
    .word __nx_module_runtime - _mod_header // runtime-generated module object offset
.global IS_NRO
IS_NRO:
    .word 1

.section .bss.module_runtime
.space 0xD0

