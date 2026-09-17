mod decode;
mod execute;
mod instruction;

use instruction::Instruction;
pub struct CPU {
    registers: [i32; 32],
    memory: Vec<i32>,
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
            memory: vec![0; 100],
            pc: 0,
        };
        cpu.addi(1, 0, 5);
        assert_eq!(cpu.registers[1], 5);
    }

    #[test]
    fn test_x0_stays_zero() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: vec![0; 100],
            pc: 0,
        };
        cpu.addi(0, 0, 5);
        assert_eq!(cpu.registers[0], 0);
    }

    #[test]
    fn test_add() {
        let mut cpu = CPU {
            registers: [0; 32],
            memory: vec![0; 100],
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
            memory: vec![0; 100],
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
            memory: vec![0; 100],
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
            memory: vec![0; 100],
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
            memory: vec![0; 100],
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
        let mut cpu = CPU { registers: [0; 32], memory: vec![0; 100], pc: 0 };
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
        let mut cpu = CPU { registers: [0; 32], memory: vec![0; 100], pc: 0 };
        cpu.addi(1, 0, -8);      // x1 = -8 (負數!)
        cpu.addi(2, 0, 1);       // x2 = 1 (移 1 位)

        cpu.sra(3, 1, 2);        // 算術右移:-8 >> 1 = -4 (補符號,保持負)
        assert_eq!(cpu.registers[3], -4);

        cpu.srl(4, 1, 2);        // 邏輯右移:-8 當 u32 >> 1,補0,變一個大正數
        assert_eq!(cpu.registers[4], 2147483644);   // (0xFFFFFFF8 >> 1) = 0x7FFFFFFC
    }

    #[test]
    fn test_sll() {
        let mut cpu = CPU { registers: [0; 32], memory: vec![0; 100], pc: 0 };
        cpu.addi(1, 0, 1);  //x1 = 1
        cpu.addi(2, 0, 4);  //x2 = 4 (位移4位)
        cpu.sll(3, 1, 2);   //x3 = 1 << 4 = 16
        assert_eq!(cpu.registers[3], 16);
    }

    #[test]
    fn test_slt_vs_sltu() {
        let mut cpu = CPU { registers: [0; 32], memory: vec![0; 100], pc: 0 };
        cpu.addi(1, 0, -1); //x1 = -1
        cpu.addi(2, 0, 5);  //x2 = 1 (位移1位)
        cpu.slt(3, 1, 2);   //有號:-1 < 5，成立
        assert_eq!(cpu.registers[3], 1);
        cpu.sltu(4, 1, 2);  //無號:-1 當大數，不 < 5 -> 0
        assert_eq!(cpu.registers[4], 0);
    }

    #[test]
    fn test_xori_andi_ori() {
        let mut cpu = CPU { registers: [0; 32], memory: vec![0; 100], pc: 0 };
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
        let mut cpu = CPU { registers: [0; 32], memory: vec![0; 100], pc: 0 };
        cpu.addi(1, 0, -1);       // x1 = -1
        cpu.slti(2, 1, 5);        // 有號:-1 < 5 → 1
        assert_eq!(cpu.registers[2], 1);
        cpu.sltiu(3, 1, 5);       // 無號:-1 當大數,不 < 5 → 0
        assert_eq!(cpu.registers[3], 0);
    }

    #[test]
    fn test_slli_srli_srai() {
        let mut cpu = CPU { registers: [0; 32], memory: vec![0; 100], pc: 0 };
        cpu.addi(1, 0, -8);       // x1 = -8
        cpu.slli(2, 1, 1);        // -8 << 1 = -16
        assert_eq!(cpu.registers[2], -16);
        cpu.srai(3, 1, 1);        // 算術右移 -8 >> 1 = -4 (補符號)
        assert_eq!(cpu.registers[3], -4);
        cpu.srli(4, 1, 1);        // 邏輯右移 補0 → 大正數
        assert_eq!(cpu.registers[4], 2147483644);
    }
}
