mod decode;

struct CPU {
    registers: [i32; 32],
    memory: Vec<i32>,
    pc: usize,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Instruction {
    Add     { rd: usize, rs1: usize, rs2: usize },
    Sub     { rd: usize, rs1: usize, rs2: usize },
    Sll     { rd: usize, rs1: usize, rs2: usize },
    Slt     { rd: usize, rs1: usize, rs2: usize },
    Sltu    { rd: usize, rs1: usize, rs2: usize },
    Xor     { rd: usize, rs1: usize, rs2: usize },
    Srl     { rd: usize, rs1: usize, rs2: usize },
    Sra     { rd: usize, rs1: usize, rs2: usize },
    Or      { rd: usize, rs1: usize, rs2: usize },
    And     { rd: usize, rs1: usize, rs2: usize },
    Addi    { rd: usize, rs1: usize, imm: i32 },
    Jump    { target: usize },
}

impl CPU {

    //暫存器互加
    fn add(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] + self.registers[rs2];
        }
        self.pc += 1;
    }

    //暫存器互減
    fn sub(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] - self.registers[rs2];
        }
        self.pc += 1;
    }
    //TODO:
    fn sll(&mut self, rd: usize, rs1: usize, rs2: usize) {

    }

    //TODO:
    fn slt(&mut self, rd: usize, rs1: usize, rs2: usize) {}

    //TODO:
    fn sltu(&mut self, rd: usize, rs1: usize, rs2: usize) {}

    //TODO:
    fn xor(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] ^ self.registers[rs2];
        }
        self.pc += 1;
    }

    //TODO
    fn srl(&mut self, rd: usize, rs1: usize, rs2: usize) {}

    //TODO
    fn sra(&mut self, rd: usize, rs1: usize, rs2: usize) {}

    fn or(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] | self.registers[rs2];
        }
        self.pc += 1;
    }

    fn and(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] & self.registers[rs2];
        }
        self.pc += 1;
    }

    //加立即數
    fn addi(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd != 0 {
            // rd (distination register) = rs1 (source register) + imm (immediate)
            self.registers[rd] = self.registers[rs1] + imm;
        }
        self.pc += 1;
    }

    fn jump(&mut self, target: usize) {
        self.pc = target;
    }

    fn execute(&mut self, inst: Instruction) {
        match inst {

            Instruction::Add { rd, rs1, rs2 } => {
                self.add(rd, rs1, rs2);
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                self.sub(rd, rs1, rs2);
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                self.sll(rd, rs1, rs2);
            }
            Instruction::Slt { rd, rs1, rs2 } => {
                self.slt(rd, rs1, rs2);
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                self.sltu(rd, rs1, rs2);
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                self.xor(rd, rs1, rs2);
            }
            Instruction::Srl{ rd, rs1, rs2 } => {
                self.srl(rd, rs1, rs2);
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                self.sra(rd, rs1, rs2);
            }
            Instruction::Or { rd, rs1, rs2 } => {
                self.or(rd, rs1, rs2);
            }
            Instruction::And { rd, rs1, rs2 } => {
                self.and(rd, rs1, rs2);
            }
            Instruction::Addi { rd, rs1, imm } => {
                self.addi(rd, rs1, imm);
            }
            Instruction::Jump { target } => {
                self.jump(target);
            }
        }
    }

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
}
