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
            let shift = (self.registers[rs2] & 0b11111) as u32; // 只取低 5 位
            self.registers[rd] = self.registers[rs1] << shift;
        }
        self.pc += 1;
    }

    //TODO:
    pub fn slt(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = if self.registers[rs1] < self.registers[rs2] {
                1
            } else {
                0
            };
        }
        self.pc += 1;
    }

    //TODO:
    pub fn sltu(&mut self, rd: usize, rs1: usize, rs2: usize) {
        if rd != 0 {
            self.registers[rd] = if (self.registers[rs1] as u32) < (self.registers[rs2] as u32) {
                1
            } else {
                0
            };
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

    pub fn slti(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd != 0 {
            self.registers[rd] = if (self.registers[rs1] as i32) < (imm as i32) {
                1
            } else {
                0
            };
        }
        self.pc += 1;
    }

    pub fn sltiu(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd != 0 {
            self.registers[rd] = if (self.registers[rs1] as u32) < (imm as u32) {
                1
            } else {
                0
            };
        }
        self.pc += 1;
    }

    pub fn xori(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] ^ imm;
        }
        self.pc += 1;
    }

    pub fn ori(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd != 0 {
            self.registers[rd] = self.registers[rs1] | imm;
        }
        self.pc += 1;
    }

    pub fn andi(&mut self, rd: usize, rs1: usize, imm: i32) {
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

    pub fn beq(&mut self, rs1: usize, rs2: usize, imm: i32) {
        if self.registers[rs1] == self.registers[rs2] {
            self.pc = (self.pc as i32 + imm) as usize;
        } else {
            self.pc += 1;
        }
    }

    pub fn bne(&mut self, rs1: usize, rs2: usize, imm: i32) {
        if self.registers[rs1] != self.registers[rs2] {
            self.pc = (self.pc as i32 + imm) as usize;
        } else {
            self.pc += 1;
        }
    }

    pub fn blt(&mut self, rs1: usize, rs2: usize, imm: i32) {
        if (self.registers[rs1] as i32) < (self.registers[rs2] as i32) {
            self.pc = (self.pc as i32 + imm) as usize;
        } else {
            self.pc += 1;
        }
    }

    pub fn bge(&mut self, rs1: usize, rs2: usize, imm: i32) {
        if (self.registers[rs1] as i32) >= (self.registers[rs2] as i32) {
            self.pc = (self.pc as i32 + imm) as usize;
        } else {
            self.pc += 1;
        }
    }

    pub fn bltu(&mut self, rs1: usize, rs2: usize, imm: i32) {
        if (self.registers[rs1] as u32) < (self.registers[rs2] as u32) {
            self.pc = (self.pc as i32 + imm) as usize;
        } else {
            self.pc += 1;
        }
    }

    pub fn bgeu(&mut self, rs1: usize, rs2: usize, imm: i32) {
        if (self.registers[rs1] as u32) >= (self.registers[rs2] as u32) {
            self.pc = (self.pc as i32 + imm) as usize;
        } else {
            self.pc += 1;
        }
    }

    pub fn jump(&mut self, target: usize) {
        self.pc = target;
    }

    // LW: rd = mem[rs1 + imm]
    pub fn lw(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd != 0 {
            let addr = (self.registers[rs1] + imm) as usize;
            self.registers[rd] = self.memory.read_word(addr) as i32; // 從記憶體讀
        }
        self.pc += 1;
    }

    // LH:讀 half,有號延伸(把 byte 當有號數)
    pub fn lh(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd != 0 {
            let addr = (self.registers[rs1] + imm) as usize;
            let byte = self.memory.read_half(addr); //get raw u8
            self.registers[rd] = byte as i16 as i32; //as i8 -> as i32
        }
        self.pc += 1;
    }

    // LHU:讀 half,無號延伸(補0)
    pub fn lhu(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd != 0 {
            let addr = (self.registers[rs1] + imm) as usize;
            let byte = self.memory.read_half(addr);
            self.registers[rd] = byte as i32;
        }
        self.pc += 1;
    }

    // LB:讀 byte,有號延伸(把 byte 當有號數)
    pub fn lb(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd != 0 {
            let addr = (self.registers[rs1] + imm) as usize;
            let byte = self.memory.read_byte(addr); //get raw u8
            self.registers[rd] = byte as i8 as i32; //as i8 -> as i32
        }
        self.pc += 1;
    }

    // LBU:讀 byte,無號延伸(補0)
    pub fn lbu(&mut self, rd: usize, rs1: usize, imm: i32) {
        if rd != 0 {
            let addr = (self.registers[rs1] + imm) as usize;
            let byte = self.memory.read_byte(addr);
            self.registers[rd] = byte as i32;
        }
        self.pc += 1;
    }

    // SW: mem[rs1 + imm] = rs2
    pub fn sw(&mut self, rs1: usize, rs2: usize, imm: i32) {
        let addr = (self.registers[rs1] + imm) as usize; // 算位址
        self.memory.write_word(addr, self.registers[rs2] as u32); // 寫進記憶體
        self.pc += 1;
    }

    //SB:寫 half(取暫存器低 8 位)
    pub fn sh(&mut self, rs1: usize, rs2: usize, imm: i32) {
        let addr = (self.registers[rs1] + imm) as usize;
        self.memory.write_half(addr, self.registers[rs2] as u16);
        self.pc += 1;
    }

    //SB:寫 byte(取暫存器低 8 位)
    pub fn sb(&mut self, rs1: usize, rs2: usize, imm: i32) {
        let addr = (self.registers[rs1] + imm) as usize;
        self.memory.write_byte(addr, self.registers[rs2] as u8);
        self.pc += 1;
    }

    //LUI: rd = imm(已經是高位形式)
    pub fn lui(&mut self, rd: usize, imm: i32) {
        if rd != 0 {
            self.registers[rd] = imm;
        }
        self.pc += 1;
    }

    //AUIPC: rd = pc + imm
    pub fn auipc(&mut self, rd: usize, imm: i32) {
        if rd != 0 {
            self.registers[rd] = (self.pc as i32) + imm;
        }
        self.pc += 1;
    }

    pub fn jal(&mut self, rd: usize, imm: i32) {
        if rd != 0 {
            self.registers[rd] = (self.pc + 1) as i32;
        }
        self.pc = (self.pc as i32 + imm) as usize;
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
            Instruction::Srl { rd, rs1, rs2 } => {
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
            Instruction::Lw { rd, rs1, imm } => {
                self.lw(rd, rs1, imm);
            }
            Instruction::Lh { rd, rs1, imm } => {
                self.lh(rd, rs1, imm);
            }
            Instruction::Lhu { rd, rs1, imm } => {
                self.lhu(rd, rs1, imm);
            }
            Instruction::Lb { rd, rs1, imm } => {
                self.lb(rd, rs1, imm);
            }
            Instruction::Lbu { rd, rs1, imm } => {
                self.lbu(rd, rs1, imm);
            }
            Instruction::Sw { rs1, rs2, imm } => {
                self.sw(rs1, rs2, imm);
            }
            Instruction::Sh { rs1, rs2, imm } => {
                self.sh(rs1, rs2, imm);
            }
            Instruction::Sb { rs1, rs2, imm } => {
                self.sb(rs1, rs2, imm);
            }
            Instruction::Lui { rd, imm } => {
                self.lui(rd, imm);
            }
            Instruction::Auipc { rd, imm } => {
                self.auipc(rd, imm);
            }
            Instruction::Jal { rd, imm} => {
                self.jal(rd, imm);
            }
            Instruction::Jump { target } => {
                self.jump(target);
            }
        }
    }
}
