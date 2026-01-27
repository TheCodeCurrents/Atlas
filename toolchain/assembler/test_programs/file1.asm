; File 1: Defines a label and some code
; This file defines routines with labels

start:
    addi r0, 10
    addi r1, 20
    add r0, r1

loop:
    addi r2, 1
    sub r0, r2
    bne loop
    
