; File 2: Uses labels and jumps to code in file1
; This demonstrates cross-file label usage

main:
    ; Some initialization
    addi r3, 5
    addi r4, 15
    add r3, r4
    
    ; Call start label from file1
    beq start
    
