# RISC-V Simulation

## RV32I
RV32I 是 32-bit 的基本整數指令集，該指令集會使用到32個暫存器(x0-x31)，且易共用有47道指令，RV32I的指令一共可分成六大類:

---

### R-Type 算數/邏輯 (暫存器運算) | opcode ```0110011```<br>
此類型的指令會有3個暫存器做完Input，分別是:rd,rs1,rs2
| 位元 | 31-25 | 24-20 | 19-15 | 14-12 | 11-7 | 6-0 |
| :-: | :-: | :-: | :-: | :-: | :-: | :-: |
| 欄位 | funct7 | rs2 | rs1 | funt3 | rd | opcode |

---

### I-Type 算數/邏輯 (立即數運算) | opcode ```0010011```<br>
包含了暫存器與立即數的運算
| 位元 | 31-20 | 19-15 | 14-12 | 11-7 | 6-0 |
| :-: | :-: | :-: | :-: | :-: | :-: |
| 欄位 | imm[11:0] | rs1 | funct3 | rd | opcode |

---

### I-Type 載入 (Load) | opcode ```0000011```<br>
包含了暫存器與立即數的載入
| 位元 | 31-20 | 19-15 | 14-12 | 11-7 | 6-0 |
| :-: | :-: | :-: | :-: | :-: | :-: |
| 欄位 | imm[11:0] | rs1 | funct3 | rd | opcode |

---

### S-Type 儲存 (Store) | opcode ```0100011``` <br>
包含存取記憶體的指令
| 位元 | 31-25 | 24-20 | 19-15 | 14-12 | 11-7 | 6-0 |
| :-: | :-: | :-: | :-: | :-: | :-: | :-: |
| 欄位 | imm[11:5] | rs2 | rs1 | funct3 | imm[4:0] | opcode |

---

### B-Type 條件分支 | opcode ```1100011```<br>
分支指令 (條件跳轉)
| 位元 | 31 | 30-25 | 24-20 | 19-15 | 14-12 | 11-8 | 7 | 6-0 |
| :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: |
| 欄位 | imm[12] | imm[10:5] | rs2 | rs1 | funct3 | imm[4:1] | imm[11] | opcode |
---

### U-Type 大立即數<br>
將立即數放到高位，這些指令被設計來實現完整的32bits運算
| 位元 | 31-12 | 11-7 | 6-0 |
| :-: | :-: | :-: | :-: |
| 欄位 | imm[31:12] | rd | opcode |

---

### J-Type/JALR 跳躍<br>
跳轉指令
| 位元 | 31 | 30-21 | 20 | 19-12 | 11-7 | 6-0 |
| :-: | :-: | :-: | :-: | :-: | :-: | :-: |
| 欄位 | imm[20] | imm[10:1] | imm[11] | imm[19:12] | rd | opcode |
---

為了方便， RISC-V 將一個 WORD 的大小設成 4 Bytes (32 bits) ，同時， RISC-V 的指令長度也是 32 bits，這些空間會被分割成好幾個 fields，不同類型的指令都會有不同的分配方式。

### 總表
> 狀態： :ballot_box_with_check: 已做 |  :white_square_button: 未做 | :black_square_button: 簡化版

> funct7 欄：`-` 表示該 type 不使用 funct7

| 狀態 | Type | 指令 | opcode | funct3 | funct7 | 作用 |
| :-: | :-: | :-: | :-: | :-: | :-: | :-- |
| :ballot_box_with_check: | R | ADD | 0110011 | 000 | 0000000 | rd = rs1 + rs2 |
| :ballot_box_with_check: | R | SUB | 0110011 | 000 | 0100000 | rd = rs1 − rs2 |
| :ballot_box_with_check: | R | SLL | 0110011 | 001 | 0000000 | rd = rs1 << rs2（邏輯左移）|
| :ballot_box_with_check: | R | SLT | 0110011 | 010 | 0000000 | rd = (rs1 < rs2) ? 1 : 0（有號）|
| :ballot_box_with_check: | R | SLTU | 0110011 | 011 | 0000000 | rd = (rs1 < rs2) ? 1 : 0（無號）|
| :ballot_box_with_check: | R | XOR | 0110011 | 100 | 0000000 | rd = rs1 ^ rs2 |
| :ballot_box_with_check: | R | SRL | 0110011 | 101 | 0000000 | rd = rs1 >> rs2（邏輯右移）|
| :ballot_box_with_check: | R | SRA | 0110011 | 101 | 0100000 | rd = rs1 >> rs2（算術右移，保符號）|
| :ballot_box_with_check: | R | OR | 0110011 | 110 | 0000000 | rd = rs1 \| rs2 |
| :ballot_box_with_check: | R | AND | 0110011 | 111 | 0000000 | rd = rs1 & rs2 |
| :ballot_box_with_check: | I | ADDI | 0010011 | 000 | - | rd = rs1 + imm |
| :ballot_box_with_check: | I | SLTI | 0010011 | 010 | - | rd = (rs1 < imm) ? 1 : 0（有號）|
| :ballot_box_with_check: | I | SLTIU | 0010011 | 011 | - | rd = (rs1 < imm) ? 1 : 0（無號）|
| :ballot_box_with_check: | I | XORI | 0010011 | 100 | - | rd = rs1 ^ imm |
| :ballot_box_with_check: | I | ORI | 0010011 | 110 | - | rd = rs1 \| imm |
| :ballot_box_with_check: | I | ANDI | 0010011 | 111 | - | rd = rs1 & imm |
| :ballot_box_with_check: | I | SLLI | 0010011 | 001 | 0000000 | rd = rs1 << shamt（shamt=imm低5位）|
| :ballot_box_with_check: | I | SRLI | 0010011 | 101 | 0000000 | rd = rs1 >> shamt（邏輯）|
| :ballot_box_with_check: | I | SRAI | 0010011 | 101 | 0100000 | rd = rs1 >> shamt（算術）|
| :ballot_box_with_check: | I | LB | 0000011 | 000 | - | rd = mem[rs1+imm]（1 byte，有號延伸）|
| :ballot_box_with_check: | I | LH | 0000011 | 001 | - | rd = mem[rs1+imm]（2 bytes，有號延伸）|
| :ballot_box_with_check: | I | LW | 0000011 | 010 | - | rd = mem[rs1+imm]（4 bytes）|
| :ballot_box_with_check: | I | LBU | 0000011 | 100 | - | rd = mem[rs1+imm]（1 byte，無號）|
| :ballot_box_with_check: | I | LHU | 0000011 | 101 | - | rd = mem[rs1+imm]（2 bytes，無號）|
| :ballot_box_with_check: | S | SB | 0100011 | 000 | - | mem[rs1+imm] = rs2（1 byte）|
| :ballot_box_with_check: | S | SH | 0100011 | 001 | - | mem[rs1+imm] = rs2（2 bytes）|
| :ballot_box_with_check: | S | SW | 0100011 | 010 | - | mem[rs1+imm] = rs2（4 bytes）|
| :ballot_box_with_check: | B | BEQ | 1100011 | 000 | - | if rs1 == rs2, pc += imm |
| :ballot_box_with_check: | B | BNE | 1100011 | 001 | - | if rs1 != rs2, pc += imm |
| :ballot_box_with_check: | B | BLT | 1100011 | 100 | - | if rs1 < rs2, pc += imm（有號）|
| :ballot_box_with_check: | B | BGE | 1100011 | 101 | - | if rs1 >= rs2, pc += imm（有號）|
| :ballot_box_with_check: | B | BLTU | 1100011 | 110 | - | if rs1 < rs2, pc += imm（無號）|
| :ballot_box_with_check: | B | BGEU | 1100011 | 111 | - | if rs1 >= rs2, pc += imm（無號）|
| :ballot_box_with_check: | U | LUI | 0110111 | - | - | rd = imm << 12 |
| :ballot_box_with_check: | U | AUIPC | 0010111 | - | - | rd = pc + (imm << 12) |
| :ballot_box_with_check: | J | JAL | 1101111 | - | - | rd = pc+4; pc += imm|
| :ballot_box_with_check: | I | JALR | 1100111 | 000 | - | rd = pc+4; pc = rs1+imm |
| :ballot_box_with_check: | - | FENCE | 0001111 | 000 | - | 記憶體屏障（模擬器可當 nop）|
| :ballot_box_with_check: | I | ECALL | 1110011 | 000 | - | 系統呼叫（imm=0）|
| :ballot_box_with_check: | I | EBREAK | 1110011 | 000 | - | 除錯中斷（imm=1）|
