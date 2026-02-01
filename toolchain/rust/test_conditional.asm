; Test conditional branching
.import end

    ldi r1, 5
    ldi r2, 5
    
    ; Compare and branch if equal
    cmp r1, r2
    beq is_equal
    
    ; This should be skipped
    ldi r3, 0
    br end
    
is_equal:
    ; This should execute
    ldi r3, 1
    
    br end
