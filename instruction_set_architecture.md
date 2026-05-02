# Instruction Set

## Op Codes

### JMP (0x10)

Sets the RIP register to the target address.

Format: `JMP imm64`

Example: `JMP 0x0100000000000000`

### HALT (0xFF)

Stops the simulator.

Format: `HALT`

Example: `HALT`