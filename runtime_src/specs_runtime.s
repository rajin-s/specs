	.text
	.intel_syntax noprefix
	.def	 main;
	.scl	2;
	.type	32;
	.endef
	.globl	__xmm@000001900000012c000000c800000064 # -- Begin function main
	.section	.rdata,"dr",discard,__xmm@000001900000012c000000c800000064
	.p2align	4
__xmm@000001900000012c000000c800000064:
	.long	100                     # 0x64
	.long	200                     # 0xc8
	.long	300                     # 0x12c
	.long	400                     # 0x190
	.text
	.globl	main
	.p2align	4, 0x90
main:                                   # @main
.seh_proc main
# %bb.0:                                # %while.body14.lr.ph
	push	r14
	.seh_pushreg 14
	push	rsi
	.seh_pushreg 6
	push	rdi
	.seh_pushreg 7
	push	rbp
	.seh_pushreg 5
	push	rbx
	.seh_pushreg 3
	sub	rsp, 48
	.seh_stackalloc 48
	.seh_endprologue
	lea	rsi, [rip + "??_C@_03PMGGPEJJ@?$CFd?6?$AA@"]
	xor	edx, edx
	mov	rcx, rsi
	call	printf
	mov	edx, 1
	mov	rcx, rsi
	call	printf
	mov	edx, 2
	mov	rcx, rsi
	call	printf
	mov	edx, 3
	mov	rcx, rsi
	call	printf
	mov	edx, 4
	mov	rcx, rsi
	call	printf
	mov	edx, 5
	mov	rcx, rsi
	call	printf
	mov	edx, 6
	mov	rcx, rsi
	call	printf
	mov	edx, 7
	mov	rcx, rsi
	call	printf
	mov	edx, 8
	mov	rcx, rsi
	call	printf
	mov	edx, 9
	mov	rcx, rsi
	call	printf
	mov	ecx, 20
	call	malloc
	mov	dword ptr [rsp + 32], 5
	mov	qword ptr [rsp + 40], rax
	movaps	xmm0, xmmword ptr [rip + __xmm@000001900000012c000000c800000064] # xmm0 = [100,200,300,400]
	movups	xmmword ptr [rax], xmm0
	mov	dword ptr [rax + 16], 500
	movsxd	rbx, dword ptr [rsp + 32]
	mov	rdi, qword ptr [rsp + 40]
	mov	edx, 100
	mov	rcx, rsi
	call	printf
	cmp	rbx, 2
	jl	.LBB0_3
# %bb.1:                                # %while.body14.while.body14_crit_edge.preheader
	mov	ebp, 1
	lea	rsi, [rip + "??_C@_03PMGGPEJJ@?$CFd?6?$AA@"]
	.p2align	4, 0x90
.LBB0_2:                                # %while.body14.while.body14_crit_edge
                                        # =>This Inner Loop Header: Depth=1
	mov	edx, dword ptr [rdi + 4*rbp]
	add	rbp, 1
	mov	rcx, rsi
	call	printf
	cmp	rbp, rbx
	jl	.LBB0_2
.LBB0_3:                                # %while.end17
	test	ebx, ebx
	jle	.LBB0_7
# %bb.4:                                # %while.body22.lr.ph
	mov	rdi, qword ptr [rsp + 40]
	mov	esi, dword ptr [rsp + 32]
	add	ebx, -1
	movsxd	rax, ebx
	mov	edx, dword ptr [rdi + 4*rax]
	lea	rcx, [rip + "??_C@_03PMGGPEJJ@?$CFd?6?$AA@"]
	call	printf
	cmp	esi, 2
	jl	.LBB0_7
# %bb.5:                                # %while.body22.preheader
	lea	ebx, [rsi - 2]
	mov	ebp, 1
	lea	r14, [rip + "??_C@_03PMGGPEJJ@?$CFd?6?$AA@"]
	.p2align	4, 0x90
.LBB0_6:                                # %while.body22
                                        # =>This Inner Loop Header: Depth=1
	movsxd	rbx, ebx
	add	ebp, 1
	mov	edx, dword ptr [rdi + 4*rbx]
	mov	rcx, r14
	call	printf
	add	ebx, -1
	cmp	ebp, esi
	jl	.LBB0_6
.LBB0_7:                                # %while.end26
	mov	rcx, qword ptr [rsp + 40]
	call	free
	xor	eax, eax
	add	rsp, 48
	pop	rbx
	pop	rbp
	pop	rdi
	pop	rsi
	pop	r14
	ret
	.seh_handlerdata
	.text
	.seh_endproc
                                        # -- End function
	.def	 printf;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",discard,printf
	.globl	printf                  # -- Begin function printf
	.p2align	4, 0x90
printf:                                 # @printf
.seh_proc printf
# %bb.0:                                # %entry
	push	rsi
	.seh_pushreg 6
	push	rdi
	.seh_pushreg 7
	push	rbx
	.seh_pushreg 3
	sub	rsp, 48
	.seh_stackalloc 48
	.seh_endprologue
	mov	rsi, rcx
	mov	qword ptr [rsp + 104], r9
	mov	qword ptr [rsp + 96], r8
	mov	qword ptr [rsp + 88], rdx
	lea	rbx, [rsp + 88]
	mov	qword ptr [rsp + 40], rbx
	mov	ecx, 1
	call	__acrt_iob_func
	mov	rdi, rax
	call	__local_stdio_printf_options
	mov	rcx, qword ptr [rax]
	mov	qword ptr [rsp + 32], rbx
	xor	r9d, r9d
	mov	rdx, rdi
	mov	r8, rsi
	call	__stdio_common_vfprintf
	nop
	add	rsp, 48
	pop	rbx
	pop	rdi
	pop	rsi
	ret
	.seh_handlerdata
	.section	.text,"xr",discard,printf
	.seh_endproc
                                        # -- End function
	.def	 __local_stdio_printf_options;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",discard,__local_stdio_printf_options
	.globl	__local_stdio_printf_options # -- Begin function __local_stdio_printf_options
	.p2align	4, 0x90
__local_stdio_printf_options:           # @__local_stdio_printf_options
# %bb.0:                                # %entry
	lea	rax, [rip + __local_stdio_printf_options._OptionsStorage]
	ret
                                        # -- End function
	.section	.rdata,"dr",discard,"??_C@_03PMGGPEJJ@?$CFd?6?$AA@"
	.globl	"??_C@_03PMGGPEJJ@?$CFd?6?$AA@" # @"??_C@_03PMGGPEJJ@?$CFd?6?$AA@"
"??_C@_03PMGGPEJJ@?$CFd?6?$AA@":
	.asciz	"%d\n"

	.lcomm	__local_stdio_printf_options._OptionsStorage,8,8 # @__local_stdio_printf_options._OptionsStorage

