use crate::Instruction;

pub fn get_opcode(inst: u32) -> u32 {
    inst & 0b1111111
}

pub fn get_rd(inst: u32) -> u32 {
    (inst >> 7) & 0b11111
}

pub fn get_rs1(inst: u32) -> u32 {
    (inst >> 15) & 0b11111
}

pub fn get_rs2(inst: u32) -> u32 {
    (inst >> 20) & 0b11111
}

pub fn get_funct3(inst: u32) -> u32 {
    (inst >> 12) & 0b111
}

pub fn get_funct7(inst: u32) -> u32 {
    (inst >> 25) & 0b1111111
}

pub fn get_imm_i(inst: u32) -> i32 {
    ((inst as i32) >> 20)
}

pub fn decode(inst: u32) -> Instruction {
    let opcode = get_opcode(inst);

    match opcode {
        0b0110011 => {
            //R-type
            let rd = get_rd(inst) as usize;
            let rs1 = get_rs1(inst) as usize;
            let rs2 = get_rs2(inst) as usize;
            let funct3 = get_funct3(inst);
            let funct7 = get_funct7(inst);

            //R-type 裡，用funct3 + funct7 區分是哪條指令
            match (funct3, funct7) {
                (0b000, 0b0000000) => Instruction::Add { rd, rs1, rs2 },
                (0b000, 0b0100000) => Instruction::Sub { rd, rs1, rs2 },
                (0b001, 0b0000000) => Instruction::Sll { rd, rs1, rs2 },
                (0b010, 0b0000000) => Instruction::Slt { rd, rs1, rs2 },
                (0b011, 0b0000000) => Instruction::Sltu { rd, rs1, rs2 },
                (0b100, 0b0000000) => Instruction::Xor { rd, rs1, rs2 },
                (0b101, 0b0000000) => Instruction::Srl { rd, rs1, rs2 },
                (0b101, 0b0100000) => Instruction::Sra { rd, rs1, rs2 },
                (0b110, 0b0000000) => Instruction::Or  { rd, rs1, rs2 },
                (0b111, 0b0000000) => Instruction::And { rd, rs1, rs2 },
                _ => panic!("unknown R-type instruction"),
            }
        }
        0b0010011 => {
            let rd = get_rd(inst) as usize;
            let rs1 = get_rs1(inst) as usize;
            let funct3 = get_funct3(inst);
            let imm = get_imm_i(inst);

            match funct3 {
                0b000 => Instruction::Addi { rd, rs1, imm },
                _ => panic!("unknown I-Type instruction"),
            }
        }
        _ => panic!("unknown opcode"),
    }
}
