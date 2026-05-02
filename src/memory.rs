pub struct Memory {
    data: Vec<u8>,
}

// Simulates memory hardware on arcitecture running
// the simulator.
impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size]
        }
    }

    /// Reads one byte of memory
    pub fn read(&self, addr: usize) -> u8 {
        return self.data[addr];
    }

    /// Writes one byte of memory
    pub fn write(&mut self, addr: usize, data: u8) {
        self.data[addr] = data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes() {
        let size: usize = 1024 * 1024;
        let sub: Memory = Memory::new(size);

        let result: usize = sub.data.capacity();

        assert!(result == 1024 * 1024);
    }

    #[test]
    fn it_reads_and_writes() {
        let addr: usize = 2;
        let data: u8 = 174;

        let mut sub: Memory = Memory::new(1024 * 1024);
        sub.write(addr, data);
        
        let result: u8 = sub.read(addr);
        assert!(result == data);
    }
}