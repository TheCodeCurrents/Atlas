; Test loop with counter
.import done

    ldi r1, 0      ; counter = 0
    ldi r2, 10     ; limit = 10
    
loop:
    ; increment counter
    addi r1, 1
    
    ; compare counter with limit
    cmp r1, r2
    
    ; branch if not equal (counter != limit)
    bne loop
    
    ; exit loop when counter == limit
    br done
