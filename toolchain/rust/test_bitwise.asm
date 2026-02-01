; Test bitwise operations
.import complete

    ldi r1, 0xFF    ; 11111111 in binary
    ldi r2, 0x0F    ; 00001111 in binary
    
    ; AND operation (0x0F)
    mov r3, r1
    and r3, r2
    
    ; OR operation (0xFF)
    mov r4, r1
    or r4, r2
    
    ; XOR operation (0xF0)
    mov r5, r1
    xor r5, r2
    
    ; Shift left
    ldi r6, 0x01
    shl r6, r6      ; r6 = 0x02
    
    ; Rotate right
    ldi r7, 0x80
    ror r7, r7      ; r7 = 0x40
    
    br complete
