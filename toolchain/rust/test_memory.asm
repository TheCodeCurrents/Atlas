; Test memory load/store operations
.import exit

    ldi r1, 42      ; value to store
    ldi r2, 0x100   ; memory address (base)
    ldi r3, 0       ; offset
    
    ; Store r1 at memory location [r2 + r3]
    st r1, [r2 + r3]
    
    ; Load from memory into r4
    ld r4, [r2 + r3]
    
    ; r4 should now equal 42
    
    br exit
