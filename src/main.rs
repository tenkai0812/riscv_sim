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
    fn run(&mut self, program: Vec<Instruction>) {
        while self.pc < program.len() {
            let inst = program[self.pc];
            self.execute(inst);
        }
    }

    fn run_binary(&mut self, program: Vec<u32>) {
        while self.pc < program.len() {
            let machine_code = program[self.pc];
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
    fn test_addi_basic() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0,
        };
        cpu.addi(1, 0, 5);
        assert_eq!(cpu.registers[1], 5);
    }

    #[test]
    fn test_x0_stays_zero() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0,
        };
        cpu.addi(0, 0, 5);
        assert_eq!(cpu.registers[0], 0);
    }

    #[test]
    fn test_add() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0,
        };
        cpu.addi(1, 0, 3);
        cpu.addi(2, 0, 4);
        cpu.add(3, 1, 2);
        assert_eq!(cpu.registers[3], 7);
    }

    #[test]
    fn test_sub() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0,
        };
        cpu.addi(1, 0, 10);
        cpu.addi(2, 0, 3);
        cpu.sub(3, 1, 2);
        assert_eq!(cpu.registers[3], 7);
    }

    #[test]
    fn test_execute() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0,
        };
        cpu.execute(Instruction::Addi {
            rd: 1,
            rs1: 0,
            imm: 10,
        });
        assert_eq!(cpu.registers[1], 10);
    }

    #[test]
    fn test_run_program() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0,
        };
        let program = vec![
            Instruction::Addi {
                rd: 1,
                rs1: 0,
                imm: 3,
            }, // x1 = 3
            Instruction::Addi {
                rd: 2,
                rs1: 0,
                imm: 4,
            }, // x2 = 4
            Instruction::Add {
                rd: 3,
                rs1: 1,
                rs2: 2,
            }, // x3 = x1 + x2 = 7
        ];
        cpu.run(program);
        assert_eq!(cpu.registers[3], 7);
    }

    #[test]
    fn test_jump() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0,
        };
        let program = vec![
            Instruction::Addi {
                rd: 1,
                rs1: 0,
                imm: 1,
            },
            Instruction::Jump { target: 3 },
            Instruction::Addi {
                rd: 1,
                rs1: 0,
                imm: 99,
            },
            Instruction::Addi {
                rd: 2,
                rs1: 0,
                imm: 5,
            },
        ];
        cpu.run(program);
        assert_eq!(cpu.registers[1], 1);
        assert_eq!(cpu.registers[2], 5);
    }

    #[test]
    fn test_decode_fields() {
        let inst: u32 = 0x002081b3;
        assert_eq!(get_opcode(inst), 0b0110011); // R-type add 的 opcode
        assert_eq!(get_rd(inst), 3); // x3
        assert_eq!(get_rs1(inst), 1); // x1
        assert_eq!(get_rs2(inst), 2); // x2
        assert_eq!(get_funct3(inst), 0);
        assert_eq!(get_funct7(inst), 0); // add 的 funct3 = 000
    }

    #[test]
    fn test_decode_add() {
        let inst: u32 = 0x002081b3;
        let decoded = decode(inst);
        assert_eq!(
            decoded,
            Instruction::Add {
                rd: 3,
                rs1: 1,
                rs2: 2
            }
        );
    }

    #[test]
    fn test_get_imm_i() {
        assert_eq!(get_imm_i(0x00500093), 5);
        assert_eq!(get_imm_i(0xffb00093), -5);
    }

    #[test]
    fn test_decode_addi() {
        let decoded = decode(0x00500093);
        assert_eq!(decoded, Instruction::Addi { rd: 1, rs1: 0, imm: 5 });
    }

    #[test]
    fn test_run_binary() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0
        };
        let program: Vec<u32> = vec![
            0x00500093,   // addi x1, x0, 5
            0x00300113,   // addi x2, x0, 3
            0x002081b3,   // add  x3, x1, x2  →  x3 = 8
        ];
        cpu.run_binary(program);
        assert_eq!(cpu.registers[3], 8);
    }

    #[test]
    fn test_sra_vs_srl() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0
        };
        cpu.addi(1, 0, -8);      // x1 = -8 (負數!)
        cpu.addi(2, 0, 1);       // x2 = 1 (移 1 位)

        cpu.sra(3, 1, 2);        // 算術右移:-8 >> 1 = -4 (補符號,保持負)
        assert_eq!(cpu.registers[3], -4);

        cpu.srl(4, 1, 2);        // 邏輯右移:-8 當 u32 >> 1,補0,變一個大正數
        assert_eq!(cpu.registers[4], 2147483644);   // (0xFFFFFFF8 >> 1) = 0x7FFFFFFC
    }

    #[test]
    fn test_sll() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0
        };
        cpu.addi(1, 0, 1);  //x1 = 1
        cpu.addi(2, 0, 4);  //x2 = 4 (位移4位)
        cpu.sll(3, 1, 2);   //x3 = 1 << 4 = 16
        assert_eq!(cpu.registers[3], 16);
    }

    #[test]
    fn test_slt_vs_sltu() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0
        };
        cpu.addi(1, 0, -1); //x1 = -1
        cpu.addi(2, 0, 5);  //x2 = 1 (位移1位)
        cpu.slt(3, 1, 2);   //有號:-1 < 5，成立
        assert_eq!(cpu.registers[3], 1);
        cpu.sltu(4, 1, 2);  //無號:-1 當大數，不 < 5 -> 0
        assert_eq!(cpu.registers[4], 0);
    }

    #[test]
    fn test_xori_andi_ori() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0,
        };
        cpu.addi(1, 0, 0b1100);
        cpu.xori(2, 1, 0b1010);
        assert_eq!(cpu.registers[2], 6);
        cpu.andi(3, 1, 0b1010);
        assert_eq!(cpu.registers[3], 8);
        cpu.ori(4,1,0b1010);
        assert_eq!(cpu.registers[4], 14);
    }

    #[test]
    fn test_slti_sltiu() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0,
        };
        cpu.addi(1, 0, -1);       // x1 = -1
        cpu.slti(2, 1, 5);        // 有號:-1 < 5 → 1
        assert_eq!(cpu.registers[2], 1);
        cpu.sltiu(3, 1, 5);       // 無號:-1 當大數,不 < 5 → 0
        assert_eq!(cpu.registers[3], 0);
    }

    #[test]
    fn test_slli_srli_srai() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0,
        };
        cpu.addi(1, 0, -8);       // x1 = -8
        cpu.slli(2, 1, 1);        // -8 << 1 = -16
        assert_eq!(cpu.registers[2], -16);
        cpu.srai(3, 1, 1);        // 算術右移 -8 >> 1 = -4 (補符號)
        assert_eq!(cpu.registers[3], -4);
        cpu.srli(4, 1, 1);        // 邏輯右移 補0 → 大正數
        assert_eq!(cpu.registers[4], 2147483644);
    }
    #[test]
    fn test_branch_loop() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0,
        };
        // 用迴圈把 x1 累加到 5:
        // x1 = 0; while x1 != 5 { x1 += 1 }
        let program = vec![
            Instruction::Addi { rd: 1, rs1: 0, imm: 0 },   // [0] x1 = 0
            Instruction::Addi { rd: 2, rs1: 0, imm: 5 },   // [1] x2 = 5 (目標)
            Instruction::Addi { rd: 1, rs1: 1, imm: 1 },   // [2] x1 += 1  ← 迴圈體
            Instruction::Bne  { rs1: 1, rs2: 2, imm: -1 }, // [3] if x1 != 5, 往回跳 1 格(回到 [2])
        ];
        cpu.run(program);
        assert_eq!(cpu.registers[1], 5);   // 迴圈跑完,x1 = 5
    }
    #[test]
    fn test_beq() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: Memory::new(100),
            pc: 0,
        };
        let program = vec![
            Instruction::Addi { rd: 1, rs1: 0, imm: 5 },   // [0] x1 = 5
            Instruction::Addi { rd: 2, rs1: 0, imm: 5 },   // [1] x2 = 5
            Instruction::Beq  { rs1: 1, rs2: 2, imm: 2 },  // [2] x1==x2 成立,跳 +2 → 到 [4],跳過 [3]
            Instruction::Addi { rd: 3, rs1: 0, imm: 99 },  // [3] 被跳過
            Instruction::Addi { rd: 4, rs1: 0, imm: 7 },   // [4] x4 = 7
        ];
        cpu.run(program);
        assert_eq!(cpu.registers[3], 0);   // [3] 被跳過,x3 還是 0
        assert_eq!(cpu.registers[4], 7);   // [4] 有執行
    }
    #[test]
    fn test_get_imm_b() {
        // beq x1, x2, 8  (正偏移)
        assert_eq!(get_imm_b(0x00208463), 8);
        // beq x1, x2, -4 (負偏移,驗證符號延伸!)
        assert_eq!(get_imm_b(0xfe208ee3), -4);
    }

    #[test]
    fn test_decode_beq() {
        let decoded = decode(0x00208463);   // beq x1, x2, 8
        assert_eq!(decoded, Instruction::Beq { rs1: 1, rs2: 2, imm: 8 });
    }

    #[test]
    fn test_sw_lw() {
        let mut cpu = CPU { registers: [0; 32], memory: Memory::new(100), pc: 0 };
        cpu.addi(1, 0, 40);       // x1 = 40 (基底位址)
        cpu.addi(2, 0, 12345);    // x2 = 12345 (要存的值)
        cpu.sw(1, 2, 0);          // mem[x1 + 0] = x2  → mem[40] = 12345
        cpu.lw(3, 1, 0);          // x3 = mem[x1 + 0]  → x3 = 12345
        assert_eq!(cpu.registers[3], 12345);   // 存進去再讀出來,一致!
    }
}
