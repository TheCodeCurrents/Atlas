; Test factorial calculation (5!)
; r1 = n (input)
; r2 = result (output)
.import factorial_done

    ldi r1, 5       ; calculate 5!
    ldi r2, 1       ; result = 1
    
factorial_loop:
    ; r1 is counter, r2 is accumulator
    
    ; multiply r2 by r1
    mov r3, r1
    
multiply_loop:
    ; r3 holds the multiplier countdown
    ; We add r1 to r2 a total of (original r1) times
    
    cmp r3, 0
    beq multiply_done
    
    add r2, r1
    subi r3, 1
    bne multiply_loop
    
multiply_done:
    ; Decrement counter
    subi r1, 1
    
    ; If r1 > 1, continue loop
    cmp r1, 1
    bne factorial_loop
    
    ; r2 now contains 5! = 120
    br factorial_done
