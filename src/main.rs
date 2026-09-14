struct CPU {
    registers: [i32; 32],
    memory: Vec<i32>,
    pc: usize,
}

enum Instruction {
    Addi    { rd: usize, rs1: usize, imm: i32 },
    Add     { rd: usize, rs1: usize, rs2: usize},
}

impl CPU {
    fn addi(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd != 0 {
            // rd (distination register) = rs1 (source register) + imm (immediate)
            self.registers[rd] = self.registers[rs1] + imm;
        }
    }

    fn add(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] + self.registers[rs2];
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
}
