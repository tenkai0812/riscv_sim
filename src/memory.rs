// memory.rs
pub struct Memory {
    data: Vec<u8>,     // byte 定址!一格一個 byte
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory { data: vec![0; size] }   // 建一個 size bytes 的記憶體,全 0
    }

    // 讀一個 byte (回傳原始 u8,延伸交給CPU 的 lb/lbu 決定)
    pub fn read_byte(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    // 寫 1 byte
    pub fn write_byte(&mut self, addr: usize, val: u8) {
        self.data[addr] = val;
    }

    // 讀兩個 byte (half,小端序組回 u16)
    pub fn read_half(&self, addr: usize) -> u16 {
        let b0 = self.data[addr] as u16;
        let b1 = self.data[addr + 1] as u16;
        b0 | (b1 << 8)
    }

    // 寫兩個 byte
    pub fn write_half(&mut self, addr: usize, val: u16) {
        self.data[addr] = (val & 0xff) as u8;
        self.data[addr + 1] = ((val >> 8) & 0xff) as u8;
    }

    // 讀一個 word (4 bytes,小端序組回 u32)
    pub fn read_word(&self, addr: usize) -> u32 {
        // 從 addr 讀 4 個 byte,小端序拼成 i32
        let b0 = self.data[addr]     as u32;
        let b1 = self.data[addr + 1] as u32;
        let b2 = self.data[addr + 2] as u32;
        let b3 = self.data[addr + 3] as u32;
        (b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)) as u32
    }

    // 寫一個 word (i32 拆成 4 bytes,小端序存)
    pub fn write_word(&mut self, addr: usize, val: u32) {
        self.data[addr]     = (val & 0xff) as u8;          // 最低 byte
        self.data[addr + 1] = ((val >> 8) & 0xff) as u8;
        self.data[addr + 2] = ((val >> 16) & 0xff) as u8;
        self.data[addr + 3] = ((val >> 24) & 0xff) as u8;  // 最高 byte
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_memory_word() {
        let mut mem = Memory::new(100);
        mem.write_word(4, 0x12345678);       // 寫進位址 4
        assert_eq!(mem.read_word(4), 0x12345678);   // 讀回來應該一樣
    }
}
