.import test

loop:
    ldi r1, 10
    ldi r2, 1
    sub r1, r2
    bne loop

    ld r3, [pc + tr]

    br test