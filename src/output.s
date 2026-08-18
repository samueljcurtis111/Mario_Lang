	.file	"output.c"
	.intel_syntax noprefix
	.text
	.section	.text.startup,"x"
	.p2align 4
	.globl	"main"
	.def	"main";	.scl	2;	.type	32;	.endef
	.seh_proc	"main"
"main":
	sub	rsp, 40
	.seh_stackalloc	40
	.seh_endprologue
	call	"__main"
	mov	ecx, 77
	call	"putchar"
	mov	ecx, 97
	call	"putchar"
	mov	ecx, 114
	call	"putchar"
	mov	ecx, 105
	call	"putchar"
	mov	ecx, 111
	call	"putchar"
	xor	eax, eax
	add	rsp, 40
	ret
	.seh_endproc
	.def	"__main";	.scl	2;	.type	32;	.endef
	.ident	"GCC: (Rev5, Built by MSYS2 project) 16.1.0"
	.def	"putchar";	.scl	2;	.type	32;	.endef
