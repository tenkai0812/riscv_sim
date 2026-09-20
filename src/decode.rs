use core::panic;

use crate::instruction::Instruction;

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
    (inst as i32) >> 20
}

pub fn get_shamt(inst: u32) -> u32 {
    (inst >> 20) & 0b11111
}

pub fn get_imm_b(inst: u32) -> i32 {
    let imm12 = ((inst >> 31) & 0b1) << 12;
    let imm11 = ((inst >> 7) & 0b1) << 11;
    let imm10_5 = ((inst >> 25) & 0b111111) << 5;
    let imm4_1 = ((inst >> 8) & 0b1111) << 1;
    let imm = imm12 | imm11 | imm10_5 | imm4_1;
    (imm << 19) as i32 >> 19
}

pub fn get_imm_s(inst: u32) -> i32 {
    let imm11_5 = ((inst >> 25) & 0b1111111) << 5;
    let imm4_0 = (inst >> 7) & 0b11111;
    let imm = imm11_5 | imm4_0;
    (imm << 20) as i32 >> 20
}

pub fn get_imm_u(inst: u32) -> i32 {
    (inst & 0xfffff000) as i32
}

pub fn get_imm_j(inst: u32) -> i32 {
    let imm20   = ((inst >> 31) & 0b1) << 20;
    let imm10_1 = ((inst >> 21) & 0b1111111111) << 1;
    let imm11 = ((inst >> 20) & 0b1) << 11;
    let imm19_12 = ((inst >> 12) & 0b11111111) << 12;
    let imm = imm20 | imm10_1 | imm11 | imm19_12;
    (imm << 11) as i32 >> 11
}
pub fn decode(inst: u32) -> Instruction {
    let opcode = get_opcode(inst);

    match opcode {
        //R-type
        0b0110011 => {
            let rd = get_rd(inst) as usize;
            let rs1 = get_rs1(inst) as usize;
            let rs2 = get_rs2(inst) as usize;
            let funct3 = get_funct3(inst);
            let funct7 = get_funct7(inst);

            //R-type 裡，用funct3 + funct7 區分是哪條指令
            match (funct3, funct7) {
                (0b000, 0b0000000) => Instruction::Add  { rd, rs1, rs2 },
                (0b000, 0b0100000) => Instruction::Sub  { rd, rs1, rs2 },
                (0b001, 0b0000000) => Instruction::Sll  { rd, rs1, rs2 },
                (0b010, 0b0000000) => Instruction::Slt  { rd, rs1, rs2 },
                (0b011, 0b0000000) => Instruction::Sltu { rd, rs1, rs2 },
                (0b100, 0b0000000) => Instruction::Xor  { rd, rs1, rs2 },
                (0b101, 0b0000000) => Instruction::Srl  { rd, rs1, rs2 },
                (0b101, 0b0100000) => Instruction::Sra  { rd, rs1, rs2 },
                (0b110, 0b0000000) => Instruction::Or   { rd, rs1, rs2 },
                (0b111, 0b0000000) => Instruction::And  { rd, rs1, rs2 },
                _ => panic!("unknown R-type instruction"),
            }
        }
        //I-type
        0b0010011 => {
            let rd  = get_rd(inst) as usize;
            let rs1 = get_rs1(inst) as usize;
            let funct3= get_funct3(inst);
            let imm   = get_imm_i(inst);
            let shamt = get_shamt(inst);
            let funct7= get_funct7(inst);

            match funct3 {
                0b000 => Instruction::Addi  { rd, rs1, imm },
                0b001 => Instruction::Slli  { rd, rs1, shamt },
                0b010 => Instruction::Slti  { rd, rs1, imm },
                0b011 => Instruction::Sltiu { rd, rs1, imm },
                0b100 => Instruction::Xori  { rd, rs1, imm },
                0b101 => {
                    match funct7 {
                        0b0000000 => Instruction::Srli { rd, rs1, shamt },
                        0b0100000 => Instruction::Srai { rd, rs1, shamt },
                        _ => panic!("unknown shift"),
                    }
                }
                0b110 => Instruction::Ori   { rd, rs1, imm },
                0b111 => Instruction::Andi  { rd, rs1, imm },
                _ => panic!("unknown I-Type instruction"),
            }
        }
        //B-type
        0b1100011 => {
            let rs1 = get_rs1(inst) as usize;
            let rs2 = get_rs2(inst) as usize;
            let funct3 = get_funct3(inst);
            let imm = get_imm_b(inst);

            match funct3 {
                0b000 => Instruction::Beq   { rs1, rs2, imm },
                0b001 => Instruction::Bne   { rs1, rs2, imm },
                0b100 => Instruction::Blt   { rs1, rs2, imm },
                0b101 => Instruction::Bge   { rs1, rs2, imm },
                0b110 => Instruction::Bltu  { rs1, rs2, imm },
                0b111 => Instruction::Bgeu  { rs1, rs2, imm },
                _ => panic!("unknown B-Type instruction"),
            }
        }

        0b0000011 => {
            let rd = get_rd(inst) as usize;
            let rs1 = get_rs1(inst) as usize;
            let funct3 = get_funct3(inst);
            let imm = get_imm_i(inst);

            match funct3 {
                0b000 => Instruction::Lb { rd, rs1, imm },
                0b001 => Instruction::Lh { rd, rs1, imm },
                0b010 => Instruction::Lw { rd, rs1, imm },
                0b100 => Instruction::Lbu { rd, rs1, imm },
                0b101 => Instruction::Lhu { rd, rs1, imm },
                _ => panic!("unknown load"),
            }
        }

        0b0100011 => {
            let rs1 = get_rs1(inst) as usize;
            let rs2 = get_rs2(inst) as usize;
            let funct3 = get_funct3(inst);
            let imm = get_imm_s(inst);

            match funct3 {
                0b000 => Instruction::Sb { rs1, rs2, imm },
                0b001 => Instruction::Sh { rs1, rs2, imm },
                0b010 => Instruction::Sw { rs1, rs2, imm },
                _ => panic!("unknown store")
            }
        }

        0b0110111 => {
            let rd = get_rd(inst) as usize;
            let imm = get_imm_u(inst);
            Instruction::Lui { rd, imm }
        }

        0b0010111 => {
            let rd = get_rd(inst) as usize;
            let imm = get_imm_u(inst);
            Instruction::Auipc { rd, imm }
        }

        0b1101111 => {
            let rd = get_rd(inst) as usize;
            let imm = get_imm_j(inst);
            Instruction::Jal { rd, imm }
        }

        0b1100111 => {
            let rd = get_rd(inst) as usize;
            let rs1 = get_rs1(inst) as usize;
            let imm = get_imm_i(inst);
            Instruction::Jalr { rd, rs1, imm }
        }
        
        _ => panic!("unknown opcode"),
    }
}
