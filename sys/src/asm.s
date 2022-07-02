/*
Pre-processed output of p256-cortex-m4-asm-gcc.S

arm-none-eabi-gcc \
  -O0 -ffunction-sections -fdata-sections -g -fno-omit-frame-pointer -mthumb \
  -march=armv7e-m -Wall -Wextra -std=c11 -march=armv7e-m \
  -c P256-Cortex-M4/p256-cortex-m4-asm-gcc.S -E > asm.s


*/
# 1 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
# 1 "/home/alex/git/rmme//"
# 1 "<built-in>"
# 1 "<command-line>"
# 1 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
 .syntax unified
 .thumb
# 26 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
# 1 "P256-Cortex-M4/p256-cortex-m4-config.h" 1
# 27 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S" 2






 .text
 .align 2
# 113 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
 .type mul288x288, %function
mul288x288:
 push {{r4-r11,lr}}


 mov r4,r0
 mov r5,r2
 mov r6,r1

 movs r1,#72
 bl setzero

 ldm r5,{{r0-r2,r8-r12,lr}}

 movs r7,#9
0:
 ldm r6!,{{r5}}
 push {{r6,r7}}

 movs r3,#0
 ldm r4,{{r6,r7}}
 umaal r6,r3,r5,r0
 umaal r7,r3,r5,r1
 stm r4!,{{r6,r7}}
 ldm r4,{{r6,r7}}
 umaal r6,r3,r5,r2
 umaal r7,r3,r5,r8
 stm r4!,{{r6,r7}}
 ldm r4,{{r6,r7}}
 umaal r6,r3,r5,r9
 umaal r7,r3,r5,r10
 stm r4!,{{r6,r7}}
 ldm r4,{{r6,r7}}
 umaal r6,r3,r5,r11
 umaal r7,r3,r5,r12
 stm r4!,{{r6,r7}}
 ldm r4,{{r6}}
 umaal r3,r6,r5,lr
 stm r4!,{{r3,r6}}

 subs r4,r4,#36
 pop {{r6,r7}}

 subs r7,r7,#1
 bne 0b

 pop {{r4-r11,pc}}
 .size mul288x288, .-mul288x288

 .type setzero, %function
setzero:
 movs r2,#0
 movs r3,#0
0:
 stm r0!,{{r2,r3}}
 subs r1,r1,#8
 bne 0b
 bx lr
 .size setzero, .-setzero
# 733 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
 .type P256_mulmod, %function
P256_mulmod:
 push {{r2,lr}}



 sub sp,#28

 ldm r2,{{r2,r3,r4,r5}}

 ldm r1!,{{r0,r10,lr}}
 umull r6,r11,r2,r0

 umull r7,r12,r3,r0
 umaal r7,r11,r2,r10

 push {{r6,r7}}


 umull r8,r6,r4,r0
 umaal r8,r11,r3,r10

 umull r9,r7,r5,r0
 umaal r9,r11,r4,r10

 umaal r11,r7,r5,r10

 umaal r8,r12,r2,lr
 umaal r9,r12,r3,lr
 umaal r11,r12,r4,lr
 umaal r12,r7,r5,lr

 ldm r1!,{{r0,r10,lr}}

 umaal r9,r6,r2,r0
 umaal r11,r6,r3,r0
 umaal r12,r6,r4,r0
 umaal r6,r7,r5,r0

 strd r8,r9,[sp,#8]

 mov r9,#0
 umaal r11,r9,r2,r10
 umaal r12,r9,r3,r10
 umaal r6,r9,r4,r10
 umaal r7,r9,r5,r10

 mov r10,#0
 umaal r12,r10,r2,lr
 umaal r6,r10,r3,lr
 umaal r7,r10,r4,lr
 umaal r9,r10,r5,lr

 ldr r8,[r1],#4
 mov lr,#0
 umaal lr,r6,r2,r8
 umaal r7,r6,r3,r8
 umaal r9,r6,r4,r8
 umaal r10,r6,r5,r8



 ldr r8,[r1],#-28
 mov r0,#0
 umaal r7,r0,r2,r8
 umaal r9,r0,r3,r8
 umaal r10,r0,r4,r8
 umaal r6,r0,r5,r8

 push {{r0}}




 ldr r2,[sp,#40]
 adds r2,r2,#16
 ldm r2,{{r2,r3,r4,r5}}

 ldr r8,[r1],#4
 mov r0,#0
 umaal r11,r0,r2,r8
 str r11,[sp,#16+4]
 umaal r12,r0,r3,r8
 umaal lr,r0,r4,r8
 umaal r0,r7,r5,r8



 ldr r8,[r1],#4
 mov r11,#0
 umaal r12,r11,r2,r8
 str r12,[sp,#20+4]
 umaal lr,r11,r3,r8
 umaal r0,r11,r4,r8
 umaal r11,r7,r5,r8



 ldr r8,[r1],#4
 mov r12,#0
 umaal lr,r12,r2,r8
 str lr,[sp,#24+4]
 umaal r0,r12,r3,r8
 umaal r11,r12,r4,r8
 umaal r10,r12,r5,r8



 ldr r8,[r1],#4
 mov lr,#0
 umaal r0,lr,r2,r8
 str r0,[sp,#28+4]
 umaal r11,lr,r3,r8
 umaal r10,lr,r4,r8
 umaal r6,lr,r5,r8



 ldm r1!,{{r0,r8}}
 umaal r11,r9,r2,r0
 str r11,[sp,#32+4]
 umaal r9,r10,r3,r0
 umaal r10,r6,r4,r0
 pop {{r11}}

 umaal r11,r6,r5,r0



 umaal r9,r7,r2,r8
 umaal r10,r7,r3,r8
 umaal r11,r7,r4,r8
 umaal r6,r7,r5,r8

 ldm r1!,{{r0,r8}}
 umaal r10,r12,r2,r0
 umaal r11,r12,r3,r0
 umaal r6,r12,r4,r0
 umaal r7,r12,r5,r0

 umaal r11,lr,r2,r8
 umaal lr,r6,r3,r8
 umaal r6,r7,r4,r8
 umaal r7,r12,r5,r8


 push {{r6,r7,r12}}

 add r7,sp,#12
 ldm r7,{{r0-r8}}

 mov r12,#0

 adds r3,r0
 adcs r4,r4,r1
 adcs r5,r5,r2
 adcs r6,r6,r0
 adcs r7,r7,r1
 adcs r8,r8,r0
 adcs r9,r9,r1
 adcs r10,r10,#0
 adcs r11,r11,#0
 adcs r12,r12,#0

 adds r6,r3
 adcs r7,r7,r4
 adcs r8,r8,r2
 adcs r9,r9,r3
 adcs r10,r10,r2
 adcs r11,r11,r3
 adcs r12,r12,#0

 subs r7,r0
 sbcs r8,r8,r1
 sbcs r9,r9,r2
 sbcs r10,r10,r3
 sbcs r11,r11,#0
 sbcs r12,r12,#0

 pop {{r1-r3}}


 adds r0,lr,r12
 adcs r1,r1,#0
 mov r12,#0
 adcs r12,r12,#0


 adcs r8,r8,r5
 adcs r9,r9,r6
 adcs r10,r10,r4
 adcs r11,r11,r5
 adcs r0,r0,r4
 adcs r1,r1,r5
 adcs r2,r2,r12
 adcs r3,r3,#0
 mov r12,#0
 adcs r12,r12,#0

 adcs r10,r10,r7
 adcs r11,r11,#0
 adcs r0,r0,r6
 adcs r1,r1,r7
 adcs r2,r2,r6
 adcs r3,r3,r7
 adcs r12,r12,#0

 subs r11,r4
 sbcs r0,r0,r5
 sbcs r1,r1,r6
 sbcs r2,r2,r7
 sbcs r3,r3,#0
 sbcs r12,r12,#0




 subs r8,r8,#0xffffffff
 sbcs r9,r9,#0xffffffff
 sbcs r10,r10,#0xffffffff
 sbcs r11,r11,#0
 sbcs r4,r0,#0
 sbcs r5,r1,#0
 sbcs r6,r2,#1
 sbcs r7,r3,#0xffffffff
 sbc r12,r12,#0

 adds r0,r8,r12
 adcs r1,r9,r12
 adcs r2,r10,r12
 adcs r3,r11,#0
 adcs r4,r4,#0
 adcs r5,r5,#0
 adcs r6,r6,r12, lsr #31
 adcs r7,r7,r12

 add sp,sp,#40


 pop {{pc}}

 .size P256_mulmod, .-P256_mulmod






 .type P256_sqrmod, %function
P256_sqrmod:
 push {{lr}}



 umull r9,r10,r0,r0
 umull r11,r12,r0,r1
 adds r11,r11,r11
 mov lr,#0
 umaal r10,r11,lr,lr





 push {{r9,r10}}



 mov r9,#0
 umaal r9,r12,r0,r2
 adcs r9,r9,r9
 umaal r9,r11,r1,r1





 push {{r9}}



 umull r9,r10,r0,r3
 umaal r9,r12,r1,r2
 adcs r9,r9,r9
 umaal r9,r11,lr,lr





 push {{r9}}



 mov r9,#0
 umaal r9,r10,r0,r4
 umaal r9,r12,r1,r3
 adcs r9,r9,r9
 umaal r9,r11,r2,r2





 push {{r9}}



 umull r9,r8,r0,r5
 umaal r9,r10,r1,r4
 umaal r9,r12,r2,r3
 adcs r9,r9,r9
 umaal r9,r11,lr,lr





 push {{r9}}



 mov r9,#0
 umaal r9,r8,r1,r5
 umaal r9,r12,r2,r4
 umaal r9,r10,r0,r6
 adcs r9,r9,r9
 umaal r9,r11,r3,r3





 push {{r9}}



 umull r9,r0,r0,r7
 umaal r9,r10,r1,r6
 umaal r9,r12,r2,r5
 umaal r9,r8,r3,r4
 adcs r9,r9,r9


 umaal r9,r11,lr,lr






 umaal r0,r8,r1,r7
 umaal r0,r10,r2,r6


 umaal r0,r12,r3,r5
 adcs r0,r0,r0
 umaal r11,r0,r4,r4






 umaal r12,r8,r2,r7
 umaal r12,r10,r3,r6
 movs r2,#0
 umaal r12,r2,r4,r5
 adcs r1,r12,r12
 umaal r0,r1,lr,lr






 umaal r2,r8,r3,r7
 umaal r2,r10,r4,r6
 adcs r2,r2,r2
 umaal r1,r2,r5,r5






 movs r3,#0
 umaal r3,r8,r4,r7
 umaal r3,r10,r5,r6
 adcs r3,r3,r3
 umaal r2,r3,lr,lr






 umaal r8,r10,r5,r7
 adcs r8,r8,r8
 umaal r3,r8,r6,r6






 umull r4,r5,lr,lr
 umaal r4,r10,r6,r7
 adcs r4,r4,r4
 umaal r4,r8,lr,lr






 adcs r10,r10,r10
 umaal r8,r10,r7,r7
 adcs r10,r10,lr







 push {{r4,r8,r10}}

 add r4,sp,#12
 ldm r4,{{r4-r8,r10,r12}}

 X0 .req r10
 X1 .req r12
 X2 .req r8
 X3 .req r7
 X4 .req r6
 X5 .req r5
 X6 .req r4
 X7 .req r9
 X8 .req r11
 X9 .req r0
 X10 .req r1
 X11 .req r2
 X12 .req r3

 X13 .req r7
 X14 .req r8
 X15 .req r12

 adcs X3,X3,X0
 adcs X4,X4,X1
 adcs X5,X5,X2
 adcs X6,X6,X0
 adcs X7,X7,X1
 adcs X8,X8,X0
 adcs X9,X9,X1
 adcs X10,X10,#0
 adcs X11,X11,#0
 adcs lr,lr,#0

 adds X6,X3
 adcs X7,X7,X4
 adcs X8,X8,X2
 adcs X9,X9,X3
 adcs X10,X10,X2
 adcs X11,X11,X3
 adcs lr,lr,#0

 subs X7,X0
 sbcs X8,X8,X1
 sbcs X9,X9,X2
 sbcs X10,X10,X3
 sbcs X11,X11,#0
 sbcs lr,lr,#0

 pop {{X13,X14,X15}}


 adds X0,X12,lr
 adcs X13,X13,#0
 mov lr,#0
 adcs lr,lr,#0


 adcs X8,X8,X5
 adcs X9,X9,X6
 adcs X10,X10,X4
 adcs X11,X11,X5
 adcs X0,X0,X4
 adcs X13,X13,X5
 adcs X14,X14,lr
 adcs X15,X15,#0
 mov lr,#0
 adcs lr,lr,#0

 adcs X10,X10,X7
 adcs X11,X11,#0
 adcs X0,X0,X6
 adcs X13,X13,X7
 adcs X14,X14,X6
 adcs X15,X15,X7
 adcs lr,lr,#0

 subs X11,X4
 sbcs X0,X0,X5
 sbcs X13,X13,X6
 sbcs X14,X14,X7
 sbcs X15,X15,#0
 sbcs lr,lr,#0





 subs r11,r11,#0xffffffff
 sbcs r9,r0,#0xffffffff
 sbcs r4,r1,#0xffffffff
 sbcs r3,r2,#0
 sbcs r6,r10,#0
 sbcs r5,r7,#0
 sbcs r10,r8,#1
 sbcs r8,r12,#0xffffffff
 sbcs r7,lr,#0

 adds r0,r11,r7
 adcs r1,r9,r7
 adcs r2,r4,r7
 adcs r3,r3,#0
 adcs r4,r6,#0
 adcs r5,r5,#0
 adcs r6,r10,r7, lsr #31
 adcs r7,r8,r7

 add sp,#28

 pop {{pc}}

 .size P256_sqrmod, .-P256_sqrmod
# 1279 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
 .type P256_submod, %function
P256_submod:
 ldm r1,{{r3-r10}}
 ldm r2!,{{r0,r1,r11,r12}}
 subs r3,r0
 sbcs r4,r4,r1
 sbcs r5,r5,r11
 sbcs r6,r6,r12
 ldm r2,{{r0,r1,r11,r12}}
 sbcs r7,r7,r0
 sbcs r8,r8,r1
 sbcs r9,r9,r11
 sbcs r10,r10,r12

 sbcs r11,r11,r11

 adds r0,r3,r11
 adcs r1,r4,r11
 adcs r2,r5,r11
 adcs r3,r6,#0
 adcs r4,r7,#0
 adcs r5,r8,#0
 adcs r6,r9,r11, lsr #31
 adcs r7,r10,r11

 bx lr

 .size P256_submod, .-P256_submod
# 1315 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
 .type P256_addmod, %function
P256_addmod:
 ldm r2,{{r2-r9}}
 ldm r1!,{{r0,r10,r11,r12}}
 adds r2,r0
 adcs r3,r3,r10
 adcs r4,r4,r11
 adcs r5,r5,r12
 ldm r1,{{r0,r1,r11,r12}}
 adcs r6,r6,r0
 adcs r7,r7,r1
 adcs r8,r8,r11
 adcs r9,r9,r12
 movs r10,#0
 adcs r10,r10,r10

 subs r2,#0xffffffff
 sbcs r3,r3,#0xffffffff
 sbcs r4,r4,#0xffffffff
 sbcs r5,r5,#0
 sbcs r6,r6,#0
 sbcs r7,r7,#0
 sbcs r8,r8,#1
 sbcs r9,r9,#0xffffffff
 sbcs r10,r10,#0

 adds r0,r2,r10
 adcs r1,r3,r10
 adcs r2,r4,r10
 adcs r3,r5,#0
 adcs r4,r6,#0
 adcs r5,r7,#0
 adcs r6,r8,r10, lsr #31
 adcs r7,r9,r10

 bx lr

 .size P256_addmod, .-P256_addmod




 .type P256_sqrmod_many, %function
P256_sqrmod_many:


 push {{r8,lr}}

0:
 bl P256_sqrmod

 ldr r8,[sp,#0]
 subs r8,r8,#1
 str r8,[sp,#0]
 bne 0b

 pop {{r8,pc}}
 .size P256_sqrmod_many, .-P256_sqrmod_many


 .type P256_sqrmod_many_and_mulmod, %function
P256_sqrmod_many_and_mulmod:
 push {{r9,lr}}

 bl P256_sqrmod_many
 push {{r0-r7}}

 mov r1,sp
 ldr r2,[sp,#32]
 bl P256_mulmod
 add sp,#36

 pop {{pc}}
 .size P256_sqrmod_many_and_mulmod, .-P256_sqrmod_many_and_mulmod






 .type P256_modinv_sqrt, %function
P256_modinv_sqrt:
 push {{r0-r8,lr}}


 mov r8,#1
 mov r9,sp
 bl P256_sqrmod_many_and_mulmod
 push {{r0-r7}}


 bl P256_sqrmod
 bl P256_sqrmod
 push {{r0-r7}}


 mov r1,sp
 add r2,sp,#32
 bl P256_mulmod
 add r8,sp,#32
 stm r8,{{r0-r7}}


 mov r8,#8-4
 add r9,sp,#32
 bl P256_sqrmod_many_and_mulmod
 push {{r0-r7}}


 mov r8,#16-8
 mov r9,sp
 bl P256_sqrmod_many_and_mulmod
 push {{r0-r7}}


 mov r8,#16
 mov r9,sp
 bl P256_sqrmod_many_and_mulmod
 push {{r0-r7}}


 mov r8,#32
 add r9,sp,#5*32
 bl P256_sqrmod_many_and_mulmod

 ldr r8,[sp,#6*32]
 cmp r8,#0
 bne 0f


 mov r8,#192-64
 mov r9,sp
 bl P256_sqrmod_many_and_mulmod


 mov r8,#224-192
 mov r9,sp
 bl P256_sqrmod_many_and_mulmod


 mov r8,#240-224
 add r9,sp,#32
 bl P256_sqrmod_many_and_mulmod


 mov r8,#248-240
 add r9,sp,#64
 bl P256_sqrmod_many_and_mulmod


 mov r8,#252-248
 add r9,sp,#128
 bl P256_sqrmod_many_and_mulmod


 mov r8,#256-252
 add r9,sp,#96
 bl P256_sqrmod_many_and_mulmod
 stm sp,{{r0-r7}}


 mov r1,sp
 add r2,sp,#5*32
 bl P256_mulmod
 b 1f

0:

 mov r8,#160-64
 add r9,sp,#5*32
 bl P256_sqrmod_many_and_mulmod


 mov r8,#254-160
 bl P256_sqrmod_many
1:

 add sp,#6*32+4

 pop {{pc}}

 .size P256_modinv_sqrt, .-P256_modinv_sqrt





 .type P256_times2, %function
P256_times2:
 adds r0,r0
 adcs r1,r1,r1
 adcs r2,r2,r2
 adcs r3,r3,r3
 adcs r4,r4,r4
 adcs r5,r5,r5
 adcs r6,r6,r6
 adcs r7,r7,r7
 mov r8,#0
 adcs r8,r8,r8

 subs r0,#0xffffffff
 sbcs r1,r1,#0xffffffff
 sbcs r2,r2,#0xffffffff
 sbcs r3,r3,#0
 sbcs r4,r4,#0
 sbcs r5,r5,#0
 sbcs r6,r6,#1
 sbcs r7,r7,#0xffffffff
 sbcs r8,r8,#0

 adds r0,r8
 adcs r1,r1,r8
 adcs r2,r2,r8
 adcs r3,r3,#0
 adcs r4,r4,#0
 adcs r5,r5,#0
 adcs r6,r6,r8, lsr #31
 adcs r7,r7,r8

 bx lr
 .size P256_times2, .-P256_times2



 .align 2

R2_mod_p:
 .word 3
 .word 0
 .word 0xffffffff
 .word 0xfffffffb
 .word 0xfffffffe
 .word 0xffffffff
 .word 0xfffffffd
 .word 4



 .type P256_to_montgomery, %function
P256_to_montgomery:
 .global P256_to_montgomery
 push {{r0,r4-r11,lr}}


 adr r2,R2_mod_p
 bl P256_mulmod
 pop {{r8}}

 stm r8,{{r0-r7}}
 pop {{r4-r11,pc}}
 .size P256_to_montgomery, .-P256_to_montgomery





 .type P256_from_montgomery, %function
P256_from_montgomery:
 .global P256_from_montgomery
 push {{r0,r4-r11,lr}}


 movs r2,#0
 movs r3,#0
 push {{r2-r3}}

 push {{r2-r3}}

 push {{r2-r3}}

 movs r2,#1
 push {{r2-r3}}

 mov r2,sp
 bl P256_mulmod
 add sp,#32

 pop {{r8}}

 stm r8,{{r0-r7}}
 pop {{r4-r11,pc}}
 .size P256_from_montgomery, .-P256_from_montgomery






 .type P256_check_range_p, %function
P256_check_range_p:
 .global P256_check_range_p
 push {{r4-r8,lr}}


 ldm r0,{{r1-r8}}

 movs r0,#0xffffffff

 subs r1,r0
 sbcs r2,r2,r0
 sbcs r3,r3,r0
 sbcs r4,r4,#0
 sbcs r5,r5,#0
 sbcs r6,r6,#0
 sbcs r7,r7,#1
 sbcs r8,r8,r0

 sbcs r0,r0,r0
 lsrs r0,#31

 pop {{r4-r8,pc}}

 .size P256_check_range_p, .-P256_check_range_p







 .align 2
P256_order_mu:
 .word 0xeedf9bfe
 .word 0x012ffd85
 .word 0xdf1a6c21
 .word 0x43190552
 .word 0xffffffff
 .word 0xfffffffe
 .word 0xffffffff
 .word 0x0
 .word 0x1





 .type P256_reduce_mod_n_once, %function
P256_reduce_mod_n_once:
 push {{lr}}


 adr r10,P256_order
 ldm r10,{{r10,r11,r12,lr}}
 subs r0,r10
 sbcs r1,r1,r11
 sbcs r2,r2,r12
 sbcs r3,r3,lr
 sbcs r4,r4,#0xffffffff
 sbcs r5,r5,#0xffffffff
 sbcs r6,r6,#0
 sbcs r7,r7,#0xffffffff
 sbcs r8,r8,#0

 sbc r9,r9,r9
 and r10,r9
 and r11,r9
 and r12,r9
 and lr,r9

 adds r0,r10
 adcs r1,r1,r11
 adcs r2,r2,r12
 adcs r3,r3,lr
 adcs r4,r4,r9
 adcs r5,r5,r9
 adcs r6,r6,#0
 adcs r7,r7,r9
 adcs r8,r8,#0

 pop {{pc}}
 .size P256_reduce_mod_n_once, .-P256_reduce_mod_n_once



 .type P256_reduce_mod_n_64bytes, %function
P256_reduce_mod_n_64bytes:
 push {{r0,r4-r11,lr}}

 sub sp,sp,#108


 mov r10,r1

 add r0,sp,#36
 adds r1,r1,#28
 adr r2,P256_order_mu
 bl mul288x288

 mov r0,sp
 add r1,sp,#72
 adr r2,P256_order
 bl mul288x288

 ldm r10,{{r0-r8}}
 pop {{r9-r12}}

 subs r0,r0,r9
 sbcs r1,r1,r10
 sbcs r2,r2,r11
 sbcs r3,r3,r12
 pop {{r9-r12,lr}}

 sbcs r4,r4,r9
 sbcs r5,r5,r10
 sbcs r6,r6,r11
 sbcs r7,r7,r12
 sbcs r8,r8,lr

 bl P256_reduce_mod_n_once
 bl P256_reduce_mod_n_once
 add sp,sp,#72

 pop {{r9}}


 stm r9,{{r0-r7}}

 pop {{r4-r11,pc}}
 .size P256_reduce_mod_n_64bytes, .-P256_reduce_mod_n_64bytes




 .type P256_reduce_mod_n_32bytes, %function
P256_reduce_mod_n_32bytes:
 .global P256_reduce_mod_n_32bytes
 push {{r0,r4-r11,lr}}


 ldm r1,{{r0-r7}}
 mov r8,#0
 bl P256_reduce_mod_n_once
 pop {{r8}}

 stm r8,{{r0-r7}}
 pop {{r4-r11,pc}}
 .size P256_reduce_mod_n_32bytes, .-P256_reduce_mod_n_32bytes






 .type P256_add_mod_n, %function
P256_add_mod_n:
 .global P256_add_mod_n
 push {{r0,r4-r11,lr}}



 mov r12,r1

 ldm r2,{{r4-r11}}
 ldm r12!,{{r0-r3}}
 adds r0,r4
 adcs r1,r1,r5
 adcs r2,r2,r6
 adcs r3,r3,r7
 ldm r12,{{r4-r7}}
 adcs r4,r4,r8
 adcs r5,r5,r9
 adcs r6,r6,r10
 adcs r7,r7,r11
 movs r8,#0
 adcs r8,r8,r8

 bl P256_reduce_mod_n_once
 bl P256_reduce_mod_n_once
 pop {{r8}}

 stm r8,{{r0-r7}}

 pop {{r4-r11,pc}}

 .size P256_add_mod_n, .-P256_add_mod_n







 .type P256_mul_mod_n, %function
P256_mul_mod_n:
 .global P256_mul_mod_n
 movs r3,#0
 push {{r3-r10,lr}}



 mov r4,r0

 ldm r1,{{r1,r3,r5-r10}}
 push {{r1,r3,r5-r10}}


 movs r1,#0
 push {{r1}}

 ldm r2,{{r1,r3,r5-r10}}
 push {{r1,r3,r5-r10}}


 sub sp,#72


 mov r0,sp
 add r1,sp,#72
 add r2,sp,#108
 bl mul288x288

 mov r0,r4
 mov r1,sp
 bl P256_reduce_mod_n_64bytes

 add sp,#144

 pop {{r4-r10,pc}}

 .size P256_mul_mod_n, .-P256_mul_mod_n






 .type P256_divsteps2_31, %function
P256_divsteps2_31:
 .global P256_divsteps2_31
 push {{r3,r4-r8,lr}}




 movs r4,#1
 movs r5,#0
 movs r6,#0
 movs r7,#1


 mov lr,#31

0:
 subs r3,r0,#1
 lsl r12,r2,#31
 bic r3,r12,r3
 asrs r3,r3,#31
 lsr r8,r3,#31


 eors r0,r0,r3
 subs r0,r0,r3

 mul r12,r1,r3
 bics r1,r1,r3
 umlal r1,r12,r2,r8
 umaal r2,r12,r2,r3

 mul r12,r4,r3
 bics r4,r4,r3
 umlal r4,r12,r6,r8
 umaal r6,r12,r6,r3

 mul r12,r5,r3
 bics r5,r5,r3
 umlal r5,r12,r7,r8
 umaal r7,r12,r7,r3

 ands r12,r2,#1
 adds r0,r0,#1


 mul r3,r12,r1
 adds r2,r2,r3
 lsrs r2,r2,#1

 umlal r6,r8,r12,r4
 umlal r7,r8,r12,r5

 adds r4,r4,r4
 adds r5,r5,r5

 subs lr,lr,#1
 bne 0b

 pop {{r3}}
 stm r3!,{{r4-r7}}

 pop {{r4-r8,pc}}
 .size P256_divsteps2_31, .-P256_divsteps2_31





 .type P256_matrix_mul_fg_9, %function
P256_matrix_mul_fg_9:
 .global P256_matrix_mul_fg_9
 push {{r4-r11,lr}}






 and r4,r0,r0,lsl #1
 asrs r4,r4,#31
 eors r0,r0,r4
 subs r0,r0,r4

 and r5,r1,r1,lsl #1
 asrs r5,r5,#31
 eors r1,r1,r5
 subs r1,r1,r5

 ldm r2!,{{r6}}
 ldr r7,[r2,#36]


 eors r4,r4,r6
 eors r5,r5,r7
 eors r4,r4,r5
 stm r3!,{{r5}}
 push {{r1,r2,r3}}




 ldm r2!,{{r1,r3,r5-r11}}
 eors r1,r1,r4
 eors r3,r3,r4
 eors r5,r5,r4
 eors r6,r6,r4
 eors r7,r7,r4
 eor r8,r8,r4
 eor r9,r9,r4
 eor r10,r10,r4

 subs r1,r1,r4
 sbcs r3,r3,r4
 sbcs r5,r5,r4
 sbcs r6,r6,r4
 sbcs r7,r7,r4
 sbcs r8,r8,r4
 sbcs r9,r9,r4
 sbcs r10,r10,r4

 eor r4,r4,r11


 umull r1,lr,r0,r1
 movs r2,#0
 umull r11,r12,r2,r2
 umaal r2,lr,r0,r3
 umaal r11,lr,r0,r5
 umull r3,r5,r12,r12
 umaal r3,lr,r0,r6
 umaal r5,lr,r0,r7
 umull r6,r7,r12,r12
 umaal r6,lr,r0,r8
 umaal r7,lr,r0,r9
 umaal r12,lr,r0,r10
 mla lr,r0,r4,lr



 pop {{r0,r4}}

 adds r4,r4,#40
 ldm r4!,{{r8,r9}}
 mov r10,#0
 umaal r1,r10,r0,r8
 umaal r2,r10,r0,r9
 adds r1,r1,r1
 adcs r2,r2,r2
 ldm r4!,{{r1,r8,r9}}
 umaal r10,r11,r0,r1
 umaal r11,r3,r0,r8
 umaal r3,r5,r0,r9
 adcs r10,r10,r10
 adcs r11,r11,r11
 adcs r3,r3,r3
 ldm r4,{{r1,r4,r8,r9}}
 umaal r5,r6,r0,r1
 umaal r6,r7,r0,r4
 umaal r7,r12,r0,r8
 umaal r12,lr,r0,r9
 adcs r5,r5,r5
 adcs r6,r6,r6
 adcs r7,r7,r7
 adcs r12,r12,r12
 sbcs lr,lr,lr
 mvn lr,lr
 pop {{r1}}

 stm r1!,{{r2,r10,r11}}
 stm r1!,{{r3,r5,r6,r7,r12,lr}}

 pop {{r4-r11,pc}}
 .size P256_matrix_mul_fg_9, .-P256_matrix_mul_fg_9





 .align 2
 .type P256_matrix_mul_mod_n, %function
P256_matrix_mul_mod_n:
 .global P256_matrix_mul_mod_n
 push {{r4-r11,lr}}






 and r4,r0,r0,lsl #1
 asrs r4,r4,#31
 eors r0,r0,r4
 subs r0,r0,r4

 and r5,r1,r1,lsl #1
 asrs r5,r5,#31
 eors r1,r1,r5
 subs r1,r1,r5

 ldm r2!,{{r6}}
 ldr r7,[r2,#32]


 eors r4,r4,r6
 eors r5,r5,r7
 eors r4,r4,r5
 stm r3!,{{r5}}
 push {{r1,r2,r3}}




 ldm r2,{{r1-r3,r5-r9}}
 eors r1,r1,r4
 eors r2,r2,r4
 eors r3,r3,r4
 eors r5,r5,r4
 eors r6,r6,r4
 eors r7,r7,r4
 eor r8,r8,r4
 eor r9,r9,r4

 subs r1,r1,r4
 sbcs r2,r2,r4
 sbcs r3,r3,r4
 sbcs r5,r5,r4
 sbcs r6,r6,r4
 sbcs r7,r7,r4
 sbcs r8,r8,r4
 sbcs r9,r9,r4

 sbcs r4,r4,r4

 lsrs lr,r4,#31
 mov r12,#0
 ldrd r10,r11,P256_order_local
 umaal r1,r12,lr,r10
 umaal r2,r12,lr,r11
 ldrd r10,r11,P256_order_local+8
 umaal r3,r12,lr,r10
 umaal r5,r12,lr,r11
 umaal r6,r12,lr,r4
 umaal r7,r12,lr,r4
 mov r10,#0
 umaal r8,r12,lr,r10
 umaal r9,r12,lr,r4


 umull r11,lr,r10,r10
 umull r10,lr,r0,r1
 umull r1,r12,r11,r11
 umaal r11,lr,r0,r2
 umaal r1,lr,r0,r3
 umull r2,r3,r12,r12
 umaal r2,lr,r0,r5
 umaal r3,lr,r0,r6
 umull r4,r5,r12,r12
 umaal r4,lr,r0,r7
 umaal r5,lr,r0,r8
 umaal r12,lr,r0,r9


 pop {{r0,r6}}

 adds r6,r6,#36
 ldm r6!,{{r8,r9}}
 movs r7,#0
 umaal r10,r7,r0,r8
 umaal r11,r7,r0,r9
 ldm r6!,{{r8,r9}}
 umaal r1,r7,r0,r8
 umaal r2,r7,r0,r9
 ldm r6!,{{r8,r9}}
 umaal r3,r7,r0,r8
 umaal r4,r7,r0,r9
 ldm r6!,{{r8,r9}}
 umaal r5,r7,r0,r8
 umaal r12,r7,r0,r9
 add lr,lr,r7


 ldr r0,=0xee00bc4f
 mul r0,r10,r0
 movs r6,#0
 ldrd r8,r9,P256_order_local
 umaal r10,r6,r0,r8
 umaal r11,r6,r0,r9
 subs r11,r11,r8
 ldrd r8,r10,P256_order_local+8
 umaal r1,r6,r0,r8
 sbcs r1,r1,r9
 umaal r2,r6,r0,r10
 mov r9,#-1
 umaal r3,r6,r0,r9
 umaal r4,r6,r0,r9
 movs r7,#0
 umaal r5,r6,r0,r7
 umaal r12,r6,r0,r9
 umaal lr,r6,r7,r7
 sbcs r2,r2,r8
 sbcs r3,r3,r10
 sbcs r4,r4,r9
 sbcs r5,r5,r9
 sbcs r12,r12,r7
 sbcs lr,lr,r9
 sbcs r6,r6,r7


 ldrd r0,r9,P256_order_local
 lsrs r6,r6,#31
 umaal r7,r11,r6,r0
 umaal r1,r11,r6,r9
 umaal r2,r11,r6,r8
 umaal r3,r11,r6,r10
 rsbs r0,r6,#0
 umaal r4,r11,r6,r0
 umaal r5,r11,r6,r0
 mov r8,#0
 umaal r11,r12,r6,r8
 umaal r12,lr,r6,r0

 pop {{r6}}

 stm r6!,{{r7}}
 stm r6!,{{r1,r2,r3,r4,r5,r11,r12}}

 pop {{r4-r11,pc}}

 .ltorg
 .size P256_matrix_mul_mod_n, .-P256_matrix_mul_mod_n
# 2371 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
 .align 2
P256_order_local:
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






 .type P256_check_range_n, %function
P256_check_range_n:
 .global P256_check_range_n
 push {{r4-r11,lr}}

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






 .align 2
b_mont:
 .word 0x29c4bddf
 .word 0xd89cdf62
 .word 0x78843090
 .word 0xacf005cd
 .word 0xf7212ed6
 .word 0xe5a220ab
 .word 0x04874834
 .word 0xdc30061d
three_mont:
 .word 0x3
 .word 0x0
 .word 0x0
 .word 0xfffffffd
 .word 0xffffffff
 .word 0xffffffff
 .word 0xfffffffc
 .word 0x2






 .type P256_point_is_on_curve, %function
P256_point_is_on_curve:
 .global P256_point_is_on_curve
 push {{r0,r4-r11,lr}}






 ldm r1,{{r0-r7}}
 bl P256_sqrmod
 push {{r0-r7}}



 ldr r0,[sp,#32]
 ldm r0,{{r0-r7}}
 bl P256_sqrmod
 push {{r0-r7}}



 mov r1,sp
 adr r2,three_mont
 bl P256_submod
 stm sp,{{r0-r7}}


 ldr r1,[sp,#64]
 mov r2,sp
 bl P256_mulmod
 stm sp,{{r0-r7}}


 add r1,sp,#32
 mov r2,sp
 bl P256_submod


 adr r8,b_mont
 ldm r8!,{{r9-r12}}
 eors r0,r9
 ittt eq
 eorseq r1,r10
 eorseq r2,r11
 eorseq r3,r12
 ldm r8,{{r9-r12}}
 itttt eq
 eorseq r4,r9
 eorseq r5,r10
 eorseq r6,r11
 eorseq r7,r12
 mov r0,#0
 it eq
 moveq r0,#1

 add sp,#68


 pop {{r4-r11,pc}}

 .size P256_point_is_on_curve, .-P256_point_is_on_curve



 .align 2
P256_p:
 .word 0xffffffff
 .word 0xffffffff
 .word 0xffffffff
 .word 0
 .word 0
 .word 0
 .word 1
 .word 0xffffffff





 .type P256_decompress_point, %function
P256_decompress_point:
 .global P256_decompress_point
 push {{r0,r2,r4-r11,lr}}


 sub sp,#32


 mov r0,sp
 bl P256_to_montgomery
 ldm sp,{{r0-r7}}

 bl P256_sqrmod
 push {{r0-r7}}

 mov r1,sp
 adr r2,three_mont
 bl P256_submod
 stm sp,{{r0-r7}}


 add r1,sp,#32
 mov r2,sp
 bl P256_mulmod
 stm sp,{{r0-r7}}

 mov r1,sp
 adr r2,b_mont
 bl P256_addmod
 stm sp,{{r0-r7}}

 mov r8,#1
 bl P256_modinv_sqrt
 add r8,sp,#32
 stm r8,{{r0-r7}}

 bl P256_sqrmod

 pop {{r8-r11}}

 eors r8,r0
 ittt eq
 eorseq r9,r1
 eorseq r10,r2
 eorseq r11,r3
 pop {{r8-r11}}

 itttt eq
 eorseq r8,r4
 eorseq r9,r5
 eorseq r10,r6
 eorseq r11,r7
 it ne
 movne r0,#0
 bne 1f

 mov r0,sp
 mov r1,sp
 bl P256_from_montgomery

 ldr r3,[sp]
 ldrd r0,r1,[sp,#32]
 and r2,r3,#1
 eors r2,r1
 mov r1,sp
 adr r3,P256_p
 bl P256_negate_mod_m_if
 movs r0,#1
1:
 add sp,#32+8

 pop {{r4-r11,pc}}

 .size P256_decompress_point, .-P256_decompress_point






 .type P256_jacobian_to_affine, %function
P256_jacobian_to_affine:
 .global P256_jacobian_to_affine
 push {{r0,r1,r2,r4-r11,lr}}



 adds r2,#64
 ldm r2,{{r0-r7}}
 mov r8,#0
 bl P256_modinv_sqrt
 push {{r0-r7}}


 bl P256_sqrmod
 push {{r0-r7}}


 add r1,sp,#32
 mov r2,sp
 bl P256_mulmod
 add r8,sp,#32
 stm r8,{{r0-r7}}

 mov r1,sp
 ldr r2,[sp,#72]
 bl P256_mulmod
 ldr r8,[sp,#64]
 stm r8,{{r0-r7}}

 ldr r2,[sp,#72]
 add r1,sp,#32
 adds r2,r2,#32
 bl P256_mulmod
 ldr r8,[sp,#68]
 stm r8,{{r0-r7}}

 add sp,#76


 pop {{r4-r11,pc}}
 .size P256_jacobian_to_affine, .-P256_jacobian_to_affine





 .type P256_double_j, %function
P256_double_j:
 .global P256_double_j
 push {{r0,r1,r4-r11,lr}}






 adds r1,#64
 ldm r1,{{r0-r7}}
 bl P256_sqrmod
 push {{r0-r7}}



 ldr r1,[sp,#36]
 adds r1,#32
 add r2,r1,#32
 bl P256_mulmod
 ldr r8,[sp,#32]
 add r8,#64
 stm r8,{{r0-r7}}


 ldr r1,[sp,#36]
 mov r2,sp
 bl P256_addmod
 push {{r0-r7}}



 ldr r1,[sp,#68]
 add r2,sp,#32
 bl P256_submod
 add r8,sp,#32
 stm r8,{{r0-r7}}


 add r1,sp,#32
 mov r2,sp
 bl P256_mulmod
 add r8,sp,#32
 stm r8,{{r0-r7}}


 lsl r8,r0,#31
 adds r0,r0,r8, asr #31
 adcs r1,r1,r8, asr #31
 adcs r2,r2,r8, asr #31
 adcs r3,r3,#0
 adcs r4,r4,#0
 adcs r5,r5,#0
 adcs r6,r6,r8, lsr #31
 adcs r7,r7,r8, asr #31
 rrxs r7,r7
 rrxs r6,r6
 rrxs r5,r5
 rrxs r4,r4
 rrxs r3,r3
 rrxs r2,r2
 rrxs r1,r1
 rrx r0,r0
 stm sp,{{r0-r7}}


 add r1,sp,#32
 mov r2,sp
 bl P256_addmod
 add r8,sp,#32
 stm r8,{{r0-r7}}


 bl P256_sqrmod
 stm sp,{{r0-r7}}


 ldr r0,[sp,#68]
 adds r0,#32
 ldm r0,{{r0-r7}}
 bl P256_sqrmod
 ldr r8,[sp,#64]
 add r8,#32
 stm r8,{{r0-r7}}


 bl P256_sqrmod
 push {{r0-r7}}



 ldrd r0,r1,[sp,#96]
 add r2,r0,#32
 bl P256_mulmod
 ldr r8,[sp,#96]
 add r8,#32
 stm r8,{{r0-r7}}


 bl P256_times2
 ldr r8,[sp,#96]
 stm r8,{{r0-r7}}


 add r1,sp,#32
 mov r2,r8
 bl P256_submod
 ldr r8,[sp,#96]
 stm r8,{{r0-r7}}


 mov r2,r8
 add r1,r2,#32
 bl P256_submod
 add r8,sp,#32
 stm r8,{{r0-r7}}


 add r1,sp,#64
 add r2,sp,#32
 bl P256_mulmod
 add r8,sp,#64
 stm r8,{{r0-r7}}


 add r1,sp,#64
 mov r2,sp
 bl P256_submod
 ldr r8,[sp,#96]
 add r8,#32
 stm r8,{{r0-r7}}

 add sp,#104


 pop {{r4-r11,pc}}
 .size P256_double_j, .-P256_double_j





 .type add_sub_helper, %function
add_sub_helper:
 push {{lr}}

 ldm r1!,{{r5-r12}}
 stm r0!,{{r5-r12}}
 ldm r1!,{{r5-r12}}
 cbz r2,0f

 mov lr,#0
 rsbs r4,r2,#0
 subs r5,r4,r5
 sbcs r6,r4,r6
 sbcs r7,r4,r7
 sbcs r8,lr,r8
 sbcs r9,lr,r9
 sbcs r10,lr,r10
 sbcs r11,r2,r11
 sbcs r12,r4,r12
0:
 stm r0!,{{r5-r12}}
 cbnz r3,1f
 ldm r1,{{r5-r12}}
 stm r0,{{r5-r12}}
 b 2f
1:

 movs r4,#0
 umull r5,r10,r4,r4
 mvns r6,r4
 mvns r7,r4
 mov r8,#0xffffffff
 mov r9,#0xfffffffe

 stm r0,{{r3-r10}}
2:
 pop {{pc}}

 .size add_sub_helper, .-add_sub_helper
# 2857 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
 .type P256_add_sub_j, %function
P256_add_sub_j:
 .global P256_add_sub_j
 push {{r0-r11,lr}}





 add r4,r0,#64
 ldm r4,{{r4-r11}}
 orrs r4,r5
 orrs r4,r6
 orrs r4,r7
 orrs r4,r8
 orrs r4,r9
 orrs r4,r10
 orrs r4,r11
 bne 2f


 bl add_sub_helper
 add sp,#16

 pop {{r4-r11,pc}}
2:







 cbnz r3,100f


 adds r1,#64
 ldm r1,{{r0-r7}}
 bl P256_sqrmod
 push {{r0-r7}}



 ldr r1,[sp,#32]
 mov r2,sp
 bl P256_mulmod
 ldr r8,[sp,#32]
 stm r8,{{r0-r7}}


 ldr r1,[sp,#36]
 adds r1,#64
 mov r2,sp
 bl P256_mulmod
 stm sp,{{r0-r7}}


 ldr r1,[sp,#32]
 adds r1,#32
 mov r2,sp
 bl P256_mulmod
 ldr r8,[sp,#32]
 add r8,#32
 stm r8,{{r0-r7}}
 b 101f
100:
 sub sp,#32

101:


 ldr r1,[sp,#32]
 adds r1,#64
 ldm r1,{{r0-r7}}
 bl P256_sqrmod
 push {{r0-r7}}



 ldr r1,[sp,#68]
 mov r2,sp
 bl P256_mulmod
 add r8,sp,#32
 stm r8,{{r0-r7}}


 ldr r1,[sp,#64]
 adds r1,#64
 mov r2,sp
 bl P256_mulmod
 stm sp,{{r0-r7}}


 ldr r1,[sp,#68]
 adds r1,#32
 mov r2,sp
 bl P256_mulmod
 stm sp,{{r0-r7}}



 ldr r1,[sp,#64]
 add r2,sp,#32
 bl P256_submod
 ldr r8,[sp,#64]
 stm r8,{{r0-r7}}


 bl P256_sqrmod
 push {{r0-r7}}



 ldr r2,[sp,#96]
 add r1,r2,#64
 bl P256_mulmod
 ldr r8,[sp,#96]
 add r8,#64
 stm r8,{{r0-r7}}


 ldr r1,[sp,#108]
 cbnz r1,102f
 ldr r1,[sp,#100]
 adds r1,#64
 mov r2,r8
 bl P256_mulmod
 ldr r8,[sp,#96]
 add r8,#64
 stm r8,{{r0-r7}}
102:


 ldr r1,[sp,#96]
 mov r2,sp
 bl P256_mulmod
 ldr r8,[sp,#96]
 stm r8,{{r0-r7}}


 orrs r1,r0
 orrs r1,r2
 orrs r1,r3
 orrs r1,r4
 orrs r1,r5
 orrs r1,r6
 orrs r0,r1,r7
3:
 push {{r0}}



 ldr r1,[sp,#100]
 adds r1,#32
 add r2,sp,#36
 ldr r3,[sp,#108]
 cbz r3,4f
 bl P256_addmod
 b 5f
4:
 bl P256_submod
5:
 ldr r8,[sp,#100]
 add r8,#32
 stm r8,{{r0-r7}}


 pop {{r8}}


 orrs r1,r0
 orrs r1,r2
 orrs r1,r3
 orrs r1,r4
 orrs r1,r5
 orrs r1,r6
 orrs r1,r7
 orrs r1,r8
 bne 6f



 add sp,#96

 ldm sp,{{r0-r3}}
 bl add_sub_helper

 ldr r0,[sp,#0]
 mov r1,r0
 add sp,#16

 bl P256_double_j
 pop {{r4-r11,pc}}
6:



 add r1,sp,#64
 mov r2,sp
 bl P256_mulmod
 add r8,sp,#64
 stm r8,{{r0-r7}}


 ldr r0,[sp,#96]
 adds r0,#32
 ldm r0,{{r0-r7}}
 bl P256_sqrmod
 stm sp,{{r0-r7}}


 add r1,sp,#32
 ldr r2,[sp,#96]
 bl P256_mulmod
 add r8,sp,#32
 stm r8,{{r0-r7}}


 mov r1,sp
 ldr r2,[sp,#96]
 bl P256_submod
 ldr r8,[sp,#96]
 stm r8,{{r0-r7}}


 add r0,sp,#64
 ldm r0,{{r0-r7}}
 bl P256_times2
 stm sp,{{r0-r7}}


 ldr r1,[sp,#96]
 mov r2,sp
 bl P256_submod
 ldr r8,[sp,#96]
 stm r8,{{r0-r7}}


 add r1,sp,#64
 ldr r2,[sp,#96]
 bl P256_submod
 stm sp,{{r0-r7}}


 ldr r1,[sp,#96]
 adds r1,#32
 mov r2,sp
 bl P256_mulmod
 stm sp,{{r0-r7}}


 ldr r0,[sp,#104]
 mov r1,sp
 add r2,sp,#32
 cbz r0,7f
 bl P256_addmod
 b 8f
7:
 bl P256_submod
8:
 ldr r8,[sp,#96]
 add r8,#32
 stm r8,{{r0-r7}}

 add sp,#112


 pop {{r4-r11,pc}}
 .size P256_add_sub_j, .-P256_add_sub_j






 .type P256_verify_last_step, %function
P256_verify_last_step:
 .global P256_verify_last_step
 push {{r0,r1,r4-r11,lr}}


 sub sp,#32
# 3158 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
 add r1,#64
 ldm r1,{{r0-r7}}
 bl P256_sqrmod
 push {{r0-r7}}



 orrs r0,r1
 orrs r0,r2
 orrs r0,r3
 orrs r0,r4
 orrs r0,r5
 orrs r0,r6
 orrs r0,r7
 beq 0f


 ldr r1,[sp,#64]
2:
 add r0,sp,#32
 bl P256_to_montgomery


 add r1,sp,#32
 mov r2,sp
 bl P256_mulmod


 ldr r8,[sp,#68]
 ldm r8!,{{r9-r12}}
 eors r0,r9
 ittt eq
 eorseq r1,r10
 eorseq r2,r11
 eorseq r3,r12
 ldm r8!,{{r9-r12}}
 itttt eq
 eorseq r4,r9
 eorseq r5,r10
 eorseq r6,r11
 eorseq r7,r12
 mov r0,#1
 beq 1f


 adr r0,P256_order
 ldm r0,{{r8-r11}}
 ldr r0,[sp,#64]
 cbz r0,0f
 ldm r0,{{r0-r7}}
 adds r0,r8
 adcs r1,r1,r9
 adcs r2,r2,r10
 adcs r3,r3,r11
 adcs r4,r4,#0xffffffff
 adcs r5,r5,#0xffffffff
 adcs r6,r6,#0
 adcs r7,r7,#0xffffffff
 bcs 0f

 subs r8,r0,#0xffffffff
 sbcs r8,r1,#0xffffffff
 sbcs r8,r2,#0xffffffff
 sbcs r8,r3,#0
 sbcs r8,r4,#0
 sbcs r8,r5,#0
 sbcs r8,r6,#1
 sbcs r8,r7,#0xffffffff
 bcs 0f

 add r8,sp,#32
 stm r8,{{r0-r7}}
 movs r2,#0
 str r2,[sp,#64]

 mov r1,r8
 b 2b

0:
 movs r0,#0
1:
 add sp,#72

 pop {{r4-r11,pc}}

 .size P256_verify_last_step, .-P256_verify_last_step
# 3252 "P256-Cortex-M4/p256-cortex-m4-asm-gcc.S"
 .type P256_negate_mod_m_if, %function
P256_negate_mod_m_if:
 push {{r4-r8,lr}}

 rsb r8,r2,#1
 movs r6,#8
 subs r7,r7
0:
 ldm r1!,{{r4,r12}}
 ldm r3!,{{r5,lr}}
 sbcs r5,r5,r4
 umull r4,r7,r8,r4
 umaal r4,r7,r2,r5
 sbcs lr,lr,r12
 umull r12,r7,r8,r12
 umaal r12,r7,r2,lr
 stm r0!,{{r4,r12}}
 sub r6,#2
 cbz r6,1f
 b 0b
1:
 pop {{r4-r8,pc}}
 .size P256_negate_mod_m_if, .-P256_negate_mod_m_if



 .type P256_negate_mod_n_if, %function
P256_negate_mod_n_if:
 .global P256_negate_mod_n_if
 ldr r3,=P256_order
 b P256_negate_mod_m_if
 .size P256_negate_mod_n_if, .-P256_negate_mod_n_if

 .type P256_negate_mod_p_if, %function
P256_negate_mod_p_if:
 .global P256_negate_mod_p_if
 adr r3,P256_p
 b P256_negate_mod_m_if
 .size P256_negate_mod_p_if, .-P256_negate_mod_p_if


 .align 2
 .end
