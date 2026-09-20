mod decode;
mod memory;
mod execute;
mod instruction;

use instruction::Instruction;
use memory::Memory;
pub struct CPU {
    registers: [i32; 32],
    memory: Memory,
    pc: usize,
}

impl CPU {

    fn run_binary(&mut self, program: Vec<u32>) {
        self.load_program(&program, 0);
        let end = program.len() * 4;
        while self.pc < end {
            let machine_code = self.memory.read_word(self.pc);
            let inst = decode::decode(machine_code);
            self.execute(inst);
        }
    }
}


fn main() {}

#[cfg(test)]
mod tests {
    //把外部(父模組)的東西引入近來
    use super::*; //引入外面的 CPU
    use crate::decode::*;
    #[test]
    fn test_branch_byte_pc() {
        let mut cpu = CPU { registers: [0; 32], memory: Memory::new(1000), pc: 0 };
        // beq x1,x2,+8 (跳過一條) — offset 現在是 byte!
        let program: Vec<u32> = vec![
            0x00500093,   // [0] addi x1,x0,5
            0x00500113,   // [4] addi x2,x0,5
            0x00208463,   // [8] beq x1,x2,8 → 相等,跳+8到[16],跳過[12]
            0x06300193,   // [12] addi x3,x0,99 (被跳過)
            0x00700213,   // [16] addi x4,x0,7
        ];
        cpu.run_binary(program);
        assert_eq!(cpu.registers[3], 0);   // 被跳過
        assert_eq!(cpu.registers[4], 7);
    }
    #[test]
    fn test_run_add() {
        let mut cpu = CPU { registers: [0; 32], memory: Memory::new(1000), pc: 0 };
        let program: Vec<u32> = vec![
            0x00500093,   // addi x1, x0, 5
            0x00300113,   // addi x2, x0, 3
            0x002081b3,   // add x3, x1, x2 → 8
        ];
        cpu.run_binary(program);
        assert_eq!(cpu.registers[3], 8);
    }
    #[test]
    fn test_loop_byte() {
        let mut cpu = CPU { registers: [0; 32], memory: Memory::new(1000), pc: 0 };
        // x1=0; while x1!=5 { x1+=1 }
        let program: Vec<u32> = vec![
            0x00000093,   // [0] addi x1,x0,0
            0x00500113,   // [4] addi x2,x0,5
            0x00108093,   // [8] addi x1,x1,1  ← 迴圈體
            0xfe209ee3,   // [12] bne x1,x2,-4 → 往回跳到[8]
        ];
        cpu.run_binary(program);
        assert_eq!(cpu.registers[1], 5);
    }
    #[test]
    fn test_get_imm_i() {
        assert_eq!(get_imm_i(0x00500093), 5);
        assert_eq!(get_imm_i(0xffb00093), -5);
    }
}
