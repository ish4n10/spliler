section .text
global _start
_start:
	push rbp
	mov rbp, rsp
	mov	r8, 2
	mov	r9, 3
	add	r8, r9
	mov	r9, 5
	add	r8, r9
	mov	r9, 6
	add	r8, r9
	pop rbp
	ret
