use crate::memory::Memory;

/// Deterministic starting point during startup
/// or reset.
const RESET_VECTOR: u64 = 0;

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
            0x10 => self.execute_jmp(),
            0xFF => self.execute_halt(),
            _ => self.execute_halt(),
        }
    }

    fn execute_jmp(&mut self) {
        const INSTRUCTION_SIZE: usize = 8;

        let mut memory_bytes: [u8; INSTRUCTION_SIZE] = [0; INSTRUCTION_SIZE];
        
        let mut counter: usize = 0;
        while counter < INSTRUCTION_SIZE {
            memory_bytes[counter] = self.read_memory();
            counter += 1;
        }

        let next_address: u64 = u64::from_le_bytes(memory_bytes);
    
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

        let program: Vec<u8> = vec![
            0x10, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // jump to 0x0009
            0xFF, // halt
        ];

        let mut addr: usize = 0;
        for byte in program {
            memory.write(addr, byte);
            addr += 1;
        }

        let mut sub: Cpu = Cpu::new(memory);
        sub.start();
    }
}