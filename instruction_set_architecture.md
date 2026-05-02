# Instruction Set

## Op Codes

[opcode: 1 byte][operands ...]

### MOV (0x01)

Moves immediate data into the specifed register.

#### Operands

[dest_register: 1 byte][imm64: 8 byte]

#### Examples

MOV RAX, 0x06

### JMP (0x10)

Sets the RIP register to the target address.

Format: [op: 1 byte][imm64: 8 byte]

Example: `JMP 0x0100000000000000`

### HALT (0xFF)

Stops the simulator.

Format: [op: 1 byte]

Example: `HALT`