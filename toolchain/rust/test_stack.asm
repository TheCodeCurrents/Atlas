; Test stack operations
.import finish

    ldi r1, 42
    ldi r2, 100
    ldi r3, 255
    
    ; Push values onto stack
    push r1
    push r2
    push r3
    
    ; Pop values back
    pop r4
    pop r5
    pop r6
    
    ; Now r4=255, r5=100, r6=42
    ; Stack is empty again
    
    br finish
