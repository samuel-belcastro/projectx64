use core::panic;

use crate::memory::Memory;

/// Deterministic starting point during startup
/// or reset.
const RESET_VECTOR: u64 = 0;

const MAX_MEMORY_FETCH_BYTES: usize = 8;

const RAX_REGISTER_NUMBER: u64 = 0x00;
const RBX_REGISTER_NUMBER: u64 = 0x03;

const MOV_OP_CODE: u8 = 0x01;
const JUMP_OP_CODE: u8 = 0x10;
const HALT_OP_CODE: u8 = 0xFF;

pub struct Cpu {
    /// Regiter A eXtended: 
    /// 64-bit accumulator register
    /// Also hold return value by
    /// convention.
    rax: u64,
    
    /// Register B eXtended:
    /// 64-bit general purpose register
    rbx: u64,

    /// Register Instruction Pointer
    rip: u64,

    /// Register Stack Pointer
    rsp: u64,

    /// FLAGS: Condition State Register
    flags: u64,

    /// Signals if the CPU should stop executing
    /// instructions.
    halted: bool,

    memory: Memory,
}

impl Cpu { 
    pub fn new(memory: Memory) -> Self {
        Self {
            rax: 0,
            rbx: 0,
            rip: RESET_VECTOR,
            rsp: 0,
            flags: 0,
            halted: false,
            memory: memory,
        }
    }

    pub fn start(&mut self) {
        while !self.halted {
            // load next instruction which
            // is always 1 byte
            let instruction: u8 = self.read_memory();

            // execute the instruction
            self.execute_instruction(instruction);
        }

        println!("All instructions executed.");
    }

    fn execute_instruction(&mut self, instruction: u8) {
        match instruction {
            MOV_OP_CODE => self.execute_mov(),
            JUMP_OP_CODE => self.execute_jmp(),
            HALT_OP_CODE => self.execute_halt(),
            _ => panic!("Unsupported operation code when executing instruction."),
        }
    }


    fn execute_mov(&mut self) {
        let dest: u64 = self.read_memory_bytes(1);
        let val: u64 = self.read_memory_bytes(8);

        println!("Moving {} to {}", val, dest);

        match dest {
            RAX_REGISTER_NUMBER => self.rax = val,
            RBX_REGISTER_NUMBER => self.rbx = val,
            _ => panic!("Invalid destination register for MOV command.")
        }
    }

    fn execute_jmp(&mut self) {
        const INSTRUCTION_SIZE: usize = 8;

        let next_address = self.read_memory_bytes(INSTRUCTION_SIZE);
    
        println!("Jumping to {}", next_address);

        self.rip = next_address;
    }

    fn execute_halt(&mut self) {
        println!("Halting! (return {})", self.rax);
        self.halted = true;
    }

    fn read_memory(&mut self) -> u8 {
        let data = self.memory.read(
            self.rip as usize);

        // We always want to move forward in
        // our program after reading memory.
        self.rip = self.rip + 1;
        return data;
    }

    fn read_memory_bytes(&mut self, bytes: usize) -> u64 {
        let mut counter: usize = 0;

        if bytes > MAX_MEMORY_FETCH_BYTES {
            panic!("Cannot read more than 8 bytes from memory at a time.")
        }

        // Pad data with 0 for data sizes less than MAX_MEMORY_FETCH_BYTES
        let mut data: [u8; MAX_MEMORY_FETCH_BYTES] = [0; MAX_MEMORY_FETCH_BYTES];

        while counter < bytes {
            data[counter] = self.read_memory();
            counter += 1;
        }

        return u64::from_le_bytes(data);
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let memory: Memory = Memory::new(1024);
        let sub: Cpu = Cpu::new(memory);

        assert!(sub.rax == 0);
        assert!(sub.rbx == 0);
        assert!(sub.rip == RESET_VECTOR);
        assert!(sub.rsp == 0);
        assert!(sub.flags == 0);
    }

    #[test]
    fn it_runs_program() {
        let mut memory: Memory = Memory::new(1024);

        let memory_contents: Vec<u8> = vec![
            // Bootstrap starting at Mem[0]
            0x10, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // jump to 0x09
            
            // Program starting at Mem[9]
            0x01, 0x03, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov rxb, 6
            0xFF, // halt
        ];

        let mut addr: usize = 0;
        for byte in memory_contents {
            memory.write(addr, byte);
            addr += 1;
        }

        let mut sub: Cpu = Cpu::new(memory);
        sub.start();
        assert!(sub.rax == 0);
        assert!(sub.rbx == 6);
        assert!(sub.halted);
    }
}