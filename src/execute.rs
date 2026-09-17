use crate::CPU;
use crate::instruction::Instruction;

impl CPU {

    //暫存器互加
    pub fn add(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] + self.registers[rs2];
        }
        self.pc += 1;
    }

    //暫存器互減
    pub fn sub(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] - self.registers[rs2];
        }
        self.pc += 1;
    }

    //TODO:
    pub fn sll(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            let shift = (self.registers[rs2] & 0b11111) as u32;   // 只取低 5 位
            self.registers[rd] = self.registers[rs1] << shift;
        }
        self.pc += 1;
    }

    //TODO:
    pub fn slt(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
             self.registers[rd] = if self.registers[rs1] < self.registers[rs2] { 1 } else { 0 };
        }
        self.pc += 1;
    }

    //TODO:
    pub fn sltu(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = if (self.registers[rs1] as u32) < (self.registers[rs2] as u32) { 1 } else { 0 };
        }
        self.pc += 1;
    }

    //TODO:
    pub fn xor(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] ^ self.registers[rs2];
        }
        self.pc += 1;
    }

    // SRL:邏輯右移(補0)
    pub fn srl(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let shift = (self.registers[rs2] & 0b11111) as u32;
        if rd != 0 {
            self.registers[rd] = ((self.registers[rs1] as u32) >> shift) as i32;
        }
        self.pc += 1;
    }

    //TODO
    pub fn sra(&mut self, rd: usize, rs1: usize, rs2: usize) {
        let shift = (self.registers[rs2] & 0b11111) as u32;
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] >> shift;
        }
        self.pc += 1;
    }

    pub fn or(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] | self.registers[rs2];
        }
        self.pc += 1;
    }

    pub fn and(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] & self.registers[rs2];
        }
        self.pc += 1;
    }

    //加立即數
    pub fn addi(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd != 0 {
            // rd (distination register) = rs1 (source register) + imm (immediate)
            self.registers[rd] = self.registers[rs1] + imm;
        }
        self.pc += 1;
    }

    pub fn slti(&mut self, rd: usize, rs1: usize,imm: i32) {
        if rd != 0 {
            self.registers[rd] = if (self.registers[rs1] as i32) < (imm as i32) { 1 } else { 0 };
        }
        self.pc += 1;
    }

    pub fn sltiu(&mut self, rd: usize, rs1: usize,imm: i32) {
        if rd != 0 {
            self.registers[rd] = if (self.registers[rs1] as u32) < (imm as u32) { 1 } else { 0 };
        }
        self.pc += 1;
    }

    pub fn xori(&mut self, rd: usize, rs1: usize,imm: i32) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] ^ imm;
        }
        self.pc += 1;
    }

    pub fn ori(&mut self, rd: usize, rs1: usize,imm: i32) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] | imm;
        }
        self.pc += 1;
    }

    pub fn andi(&mut self, rd: usize, rs1: usize,imm: i32) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] & imm;
        }
        self.pc += 1;
    }

    pub fn slli(&mut self, rd: usize, rs1: usize, shamt: u32) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] << shamt;
        }
        self.pc += 1;
    }

    pub fn srli(&mut self, rd: usize, rs1: usize, shamt: u32) {
        if rd != 0 {
            self.registers[rd] = ((self.registers[rs1] as u32) >> shamt) as i32;
        }
        self.pc += 1;
    }

    pub fn srai(&mut self, rd: usize, rs1: usize, shamt: u32) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] >> shamt;
        }
        self.pc += 1;
    }

    pub fn beq(&mut self, rs1: usize, rs2: usize, imm:i32) {
        if self.registers[rs1] == self.registers[rs2] {
            self.pc = (self.pc as i32 + imm) as usize;
        } else {
            self.pc += 1;
        }
    }

    pub fn bne(&mut self, rs1: usize, rs2: usize, imm:i32) {
        if self.registers[rs1] != self.registers[rs2] {
            self.pc = (self.pc as i32 + imm) as usize;
        } else {
            self.pc += 1;
        }
    }

    pub fn blt(&mut self, rs1: usize, rs2: usize, imm:i32) {
        if (self.registers[rs1] as i32) < (self.registers[rs2] as i32) {
            self.pc = (self.pc as i32 + imm) as usize;
        } else {
            self.pc += 1;
        }
    }

    pub fn bge(&mut self, rs1: usize, rs2: usize, imm:i32) {
        if (self.registers[rs1] as i32) >= (self.registers[rs2] as i32) {
            self.pc = (self.pc as i32 + imm) as usize;
        } else {
            self.pc += 1;
        }
    }

    pub fn bltu(&mut self, rs1: usize, rs2: usize, imm:i32) {
        if (self.registers[rs1] as u32) < (self.registers[rs2] as u32) {
            self.pc = (self.pc as i32 + imm) as usize;
        } else {
            self.pc += 1;
        }
    }

    pub fn bgeu(&mut self, rs1: usize, rs2: usize, imm:i32) {
        if (self.registers[rs1] as u32) >= (self.registers[rs2] as u32) {
            self.pc = (self.pc as i32 + imm) as usize;
        } else {
            self.pc += 1;
        }
    }

    pub fn jump(&mut self, target: usize) {
        self.pc = target;
    }

    pub fn execute(&mut self, inst: Instruction) {
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
            Instruction::Slti { rd, rs1, imm } => {
                self.slti(rd, rs1, imm);
            }
            Instruction::Sltiu { rd, rs1, imm } => {
                self.sltiu(rd, rs1, imm);
            }
            Instruction::Xori { rd, rs1, imm } => {
                self.xori(rd, rs1, imm);
            }
            Instruction::Ori { rd, rs1, imm } => {
                self.ori(rd, rs1, imm);
            }
            Instruction::Andi { rd, rs1, imm } => {
                self.andi(rd, rs1, imm);
            }
            Instruction::Slli { rd, rs1, shamt } => {
                self.slli(rd, rs1, shamt);
            }
            Instruction::Srli { rd, rs1, shamt } => {
                self.srli(rd, rs1, shamt);
            }
            Instruction::Srai { rd, rs1, shamt } => {
                self.srai(rd, rs1, shamt);
            }
            Instruction::Beq { rs1, rs2, imm } => {
                self.beq(rs1, rs2, imm);
            }
            Instruction::Bne { rs1, rs2, imm } => {
                self.bne(rs1, rs2, imm);
            }
            Instruction::Blt { rs1, rs2, imm } => {
                self.blt(rs1, rs2, imm);
            }
            Instruction::Bge { rs1, rs2, imm } => {
                self.bge(rs1, rs2, imm);
            }
            Instruction::Bltu { rs1, rs2, imm } => {
                self.bltu(rs1, rs2, imm);
            }
            Instruction::Bgeu { rs1, rs2, imm } => {
                self.bgeu(rs1, rs2, imm);
            }
            Instruction::Jump { target } => {
                self.jump(target);
            }
        }
    }
}
