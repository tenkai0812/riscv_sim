struct CPU {
    registers: [i32; 32],
    memory: Vec<i32>,
    pc: usize,
}

#[derive(Clone, Copy)]
enum Instruction {
    Addi { rd: usize, rs1: usize, imm: i32 },
    Add { rd: usize, rs1: usize, rs2: usize },
    Sub { rd: usize, rs1: usize, rs2: usize },
}

impl CPU {
    //加立即數
    fn addi(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd != 0 {
            // rd (distination register) = rs1 (source register) + imm (immediate)
            self.registers[rd] = self.registers[rs1] + imm;
        }
    }
    //暫存器互加
    fn add(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] + self.registers[rs2];
        }
    }
    //暫存器互減
    fn sub(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] - self.registers[rs2];
        }
    }

    fn execute(&mut self, inst: Instruction) {
        match inst {
            Instruction::Addi { rd, rs1, imm } => {
                self.addi(rd, rs1, imm);
            }
            Instruction::Add { rd, rs1, rs2 } => {
                self.add(rd, rs1, rs2);
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                self.sub(rd, rs1, rs2);
            }
        }
    }

    fn run(&mut self, program: Vec<Instruction>) {
        while self.pc < program.len() {
            let inst = program[self.pc];
            self.execute(inst);
            self.pc += 1;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    //把外部(父模組)的東西引入近來
    use super::*; //引入外面的 CPU

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
}
