	.align 2
P256_order_local: // (arm clang assembler is broken for ldrd global labels defined in the same file)
	.type P256_order, %object
P256_order:
	.global P256_order
	.word 0xFC632551
	.word 0xF3B9CAC2
	.word 0xA7179E84
	.word 0xBCE6FAAD
	.word 0xFFFFFFFF
	.word 0xFFFFFFFF
	.word 0
	.word 0xFFFFFFFF
	.word 0
	.size P256_order, .-P256_order

// Checks whether the input number is within [1,n-1]
// in: *r0
// out: r0 = 1 if ok, else 0
	.type P256_check_range_n, %function
P256_check_range_n:
	.global P256_check_range_n
	push {{r4-r11,lr}}
	//frame push {{r4-r11,lr}}
	ldm r0,{{r1-r8}}
	orrs r0,r1,r2
	orrs r0,r3
	orrs r0,r4
	orrs r0,r5
	orrs r0,r6
	orrs r0,r7
	orrs r0,r8
	beq 0f
	
	adr r0,P256_order
	ldm r0!,{{r9-r12}}
	subs r1,r9
	sbcs r2,r2,r10
	sbcs r3,r3,r11
	sbcs r4,r4,r12
	ldm r0,{{r0-r3}}
	sbcs r5,r5,r0
	sbcs r6,r6,r1
	sbcs r7,r7,r2
	sbcs r8,r8,r3
	
	sbcs r0,r0,r0
	lsrs r0,#31
0:
	pop {{r4-r11,pc}}
	
	.size P256_check_range_n, .-P256_check_range_n
