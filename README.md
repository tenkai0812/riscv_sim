# riscv_sim

A **RISC-V (RV32I) CPU simulator** built from scratch in **Rust**.

It implements the complete fetch–decode–execute cycle of a CPU, starting from raw binary machine code, and can load and run real RISC-V programs.

```
$ cargo run
1+2+...+10 = 55
```

The line above is the actual result of the simulator executing 7 RISC-V machine-code instructions — including a loop, a conditional branch, and accumulation.

---

## ✨ Features

- **Full RV32I instruction set** — all 40 instructions implemented (arithmetic, logic, comparison, shifts, branches, memory access, jumps, and system instructions)
- **Real binary decoding** — parses raw 32-bit machine code directly: slices bit fields and reconstructs immediates for all six instruction formats (with sign extension)
- **Byte-addressed memory** — memory is addressed per byte, with little-endian access for word / half / byte
- **Realistic PC behavior** — the Program Counter is a byte address; instructions are fetched from memory and the PC advances by 4 bytes per instruction
- **Runs real machine code** — loads and executes RISC-V machine code directly; `ecall` / `ebreak` provide a program-halt mechanism
- **Modular architecture** — decoding, execution, memory, and instruction definitions are cleanly separated
- **Test coverage** — covers each instruction category and edge cases (signed/unsigned extension, loops and branches under byte-addressed PC)

---

## 🚀 Getting Started

```bash
# Run the built-in demo (computes 1+2+...+10)
cargo run

# Run the tests
cargo test
```

---

## 🏗️ Architecture

```
src/
├── main.rs          # CPU struct, fetch-decode-execute loop, entry point
├── instruction.rs   # Instruction enum (type definitions for all instructions)
├── decode.rs        # Binary decoding: slice bit fields, reconstruct immediates, machine code -> Instruction
├── execute.rs       # Instruction execution: effects on registers / memory / PC
└── memory.rs        # Byte-addressed memory (little-endian word/half/byte access)
```

**Execution flow:**

```
machine code (u32)  ->  decode  ->  Instruction  ->  execute  ->  update registers / memory / PC
   ^ fetched from memory                                                  |
   +--------------- PC += 4 (or branch / jump) <---------------------------+
```

---

## 💡 Implementation Highlights

The core challenge of this project is turning a stream of 0s and 1s into instructions that execute correctly. A few technical points worth noting:

**Decoding six immediate formats**
To simplify hardware decoding, RISC-V keeps rd/rs1/rs2 at fixed positions — at the cost of scattering and reordering the immediate bits across the instruction. The B-type and J-type immediates are especially fragmented across four segments, requiring each piece to be sliced out, shifted, ORed together, and finally sign-extended.

**Consistent signed / unsigned handling**
`SRL` vs `SRA` (logical vs arithmetic shift right), `SLT` vs `SLTU` (signed vs unsigned comparison), and `LB` vs `LBU` (sign extension vs zero extension on load) all rely on Rust's type casts (`as i8 as i32` vs `as i32`) to express the correct behavior.

**Byte addressing & little-endian**
Memory is implemented as a `Vec<u8>`. A 32-bit word is split into 4 bytes in little-endian order (low byte at the low address) on write, and reassembled on read. The PC is a real byte address, and branch/jump offsets are measured in bytes.

---

## 📋 RV32I Instruction Reference

RV32I is the base 32-bit integer instruction set. It uses 32 registers (x0–x31), has 40 instructions, a fixed 32-bit instruction length, and six instruction formats.

> Status: implemented (all instructions)

### Instruction Formats

| Format | Description | Immediate |
| :-: | :-- | :-- |
| **R** | Register-register ops (rd, rs1, rs2) | none |
| **I** | Immediate ops / loads | imm[11:0] (contiguous) |
| **S** | Stores | imm split into two segments |
| **B** | Conditional branches | imm scattered across four segments (imm[0] omitted) |
| **U** | Upper immediate | imm[31:12] (high bits) |
| **J** | Jumps | imm scattered across four segments (imm[0] omitted) |

### Full Instruction List

| Type | Instr | opcode | funct3 | funct7 | Effect |
| :-: | :-: | :-: | :-: | :-: | :-- |
| R | ADD | 0110011 | 000 | 0000000 | rd = rs1 + rs2 |
| R | SUB | 0110011 | 000 | 0100000 | rd = rs1 - rs2 |
| R | SLL | 0110011 | 001 | 0000000 | rd = rs1 << rs2 (logical left shift) |
| R | SLT | 0110011 | 010 | 0000000 | rd = (rs1 < rs2) ? 1 : 0 (signed) |
| R | SLTU | 0110011 | 011 | 0000000 | rd = (rs1 < rs2) ? 1 : 0 (unsigned) |
| R | XOR | 0110011 | 100 | 0000000 | rd = rs1 ^ rs2 |
| R | SRL | 0110011 | 101 | 0000000 | rd = rs1 >> rs2 (logical right shift) |
| R | SRA | 0110011 | 101 | 0100000 | rd = rs1 >> rs2 (arithmetic right shift) |
| R | OR | 0110011 | 110 | 0000000 | rd = rs1 \| rs2 |
| R | AND | 0110011 | 111 | 0000000 | rd = rs1 & rs2 |
| I | ADDI | 0010011 | 000 | - | rd = rs1 + imm |
| I | SLTI | 0010011 | 010 | - | rd = (rs1 < imm) ? 1 : 0 (signed) |
| I | SLTIU | 0010011 | 011 | - | rd = (rs1 < imm) ? 1 : 0 (unsigned) |
| I | XORI | 0010011 | 100 | - | rd = rs1 ^ imm |
| I | ORI | 0010011 | 110 | - | rd = rs1 \| imm |
| I | ANDI | 0010011 | 111 | - | rd = rs1 & imm |
| I | SLLI | 0010011 | 001 | 0000000 | rd = rs1 << shamt (shamt = imm[4:0]) |
| I | SRLI | 0010011 | 101 | 0000000 | rd = rs1 >> shamt (logical) |
| I | SRAI | 0010011 | 101 | 0100000 | rd = rs1 >> shamt (arithmetic) |
| I | LB | 0000011 | 000 | - | rd = mem[rs1+imm] (1 byte, sign-extended) |
| I | LH | 0000011 | 001 | - | rd = mem[rs1+imm] (2 bytes, sign-extended) |
| I | LW | 0000011 | 010 | - | rd = mem[rs1+imm] (4 bytes) |
| I | LBU | 0000011 | 100 | - | rd = mem[rs1+imm] (1 byte, unsigned) |
| I | LHU | 0000011 | 101 | - | rd = mem[rs1+imm] (2 bytes, unsigned) |
| S | SB | 0100011 | 000 | - | mem[rs1+imm] = rs2 (1 byte) |
| S | SH | 0100011 | 001 | - | mem[rs1+imm] = rs2 (2 bytes) |
| S | SW | 0100011 | 010 | - | mem[rs1+imm] = rs2 (4 bytes) |
| B | BEQ | 1100011 | 000 | - | if rs1 == rs2, pc += imm |
| B | BNE | 1100011 | 001 | - | if rs1 != rs2, pc += imm |
| B | BLT | 1100011 | 100 | - | if rs1 < rs2, pc += imm (signed) |
| B | BGE | 1100011 | 101 | - | if rs1 >= rs2, pc += imm (signed) |
| B | BLTU | 1100011 | 110 | - | if rs1 < rs2, pc += imm (unsigned) |
| B | BGEU | 1100011 | 111 | - | if rs1 >= rs2, pc += imm (unsigned) |
| U | LUI | 0110111 | - | - | rd = imm << 12 |
| U | AUIPC | 0010111 | - | - | rd = pc + (imm << 12) |
| J | JAL | 1101111 | - | - | rd = pc+4; pc += imm |
| I | JALR | 1100111 | 000 | - | rd = pc+4; pc = rs1+imm |
| - | FENCE | 0001111 | 000 | - | memory fence (no-op in this simulator) |
| I | ECALL | 1110011 | 000 | - | environment call (halts execution) |
| I | EBREAK | 1110011 | 000 | - | breakpoint (halts execution) |

---

## 📖 Demo Walkthrough

The example program in `main.rs` computes `1 + 2 + ... + 10`. The equivalent assembly:

```asm
        addi x1, x0, 0      # sum = 0
        addi x2, x0, 1      # i = 1
        addi x3, x0, 11     # limit = 11
loop:   add  x1, x1, x2     # sum += i
        addi x2, x2, 1      # i += 1
        blt  x2, x3, loop   # if i < 11, jump back to loop
        ecall               # halt (x1 = 55)
```

The simulator loads this as machine code into memory, runs it through the fetch-decode-execute cycle, loops 10 times, halts on `ecall`, and ends with `x1 = 55`.

---

## 🔧 Tech Stack

- **Language**: Rust
- **Reference**: RISC-V Unprivileged ISA — RV32I Base Integer Instruction Set
