; Test basic arithmetic operations
.import result

    ldi r1, 5
    ldi r2, 3
    
    ; r1 = r1 + r2 (5 + 3 = 8)
    add r1, r2
    
    ; r3 = r1 - r2 (8 - 3 = 5)
    mov r3, r1
    sub r3, r2
    
    ; r4 = r1 AND r2 (bitwise AND)
    mov r4, r1
    and r4, r2
    
    br result
