mod addressing_modes;
mod opcodes;

use addressing_modes::*;
use opcodes::*;

struct Registers {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    s: u8,
    p: u8,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0xc000,
            s: 0xfd,
            p: 0x24,
        }
    }
}

struct Cpu {
    mem: [u8; 0x2000],
    regs: Registers,
    clock: u64,
}

impl Cpu {
    fn load(&mut self, am: AddressingMode) -> u8 {
        match am {
            AddressingMode::Implicit => 0,
            AddressingMode::Accumulator => self.regs.a,
            AddressingMode::Immediate => {
                let val = self.mem[self.regs.pc as usize];
                self.regs.pc = self.regs.pc + 1;
                val
            }
            AddressingMode::ZeroPage => {
                let val = self.mem[self.mem[self.regs.pc as usize] as usize];
                self.regs.pc = self.regs.pc + 1;
                val
            }
            AddressingMode::ZeroPageX => {
                let addr = self.mem[self.regs.pc as usize].wrapping_add(self.regs.x);
                let val = self.mem[addr as usize];
                self.regs.pc = self.regs.pc + 1;
                val
            }
            AddressingMode::ZeroPageY => {
                let addr = self.mem[self.regs.pc as usize].wrapping_add(self.regs.y);
                let val = self.mem[addr as usize];
                self.regs.pc = self.regs.pc + 1;
                val
            }
            AddressingMode::Relative => 0,
            AddressingMode::Absolute => {
                let addr = ((self.mem[self.regs.pc as usize] as usize) << 8)
                    + (self.mem[(self.regs.pc + 1) as usize] as usize);
                let val = self.mem[addr];
                self.regs.pc = self.regs.pc + 2;
                val
            }
            AddressingMode::AbsoluteX => {
                let base_addr = ((self.mem[self.regs.pc as usize] as usize) << 8)
                    + (self.mem[(self.regs.pc + 1) as usize] as usize);
                let addr = (base_addr as u16).wrapping_add(self.regs.x as u16) as usize;
                let val = self.mem[addr];
                self.regs.pc = self.regs.pc + 2;
                val
            }
            AddressingMode::AbsoluteY => {
                let base_addr = ((self.mem[self.regs.pc as usize] as usize) << 8)
                    + (self.mem[(self.regs.pc + 1) as usize] as usize);
                let addr = (base_addr as u16).wrapping_add(self.regs.y as u16) as usize;
                let val = self.mem[addr];
                self.regs.pc = self.regs.pc + 2;
                val
            }
            AddressingMode::Indirect => 0,
            AddressingMode::IndirectX => {
                let addr_lo = self.mem[self.regs.pc as usize].wrapping_add(self.regs.x);
                let val_lo = self.mem[addr_lo as usize];

                let addr_hi = addr_lo.wrapping_add(1);
                let val_hi = (self.mem[addr_hi as usize] as u16) << 8;

                self.regs.pc = self.regs.pc + 2;

                self.mem[(addr_hi | addr_lo) as usize]
            }
            AddressingMode::IndirectY => {
                let addr_lo = self.mem[self.regs.pc as usize];
                let val_lo = self.mem[addr_lo as usize];

                let addr_hi = addr_lo.wrapping_add(1);
                let val_hi = (self.mem[addr_hi as usize] as u16) << 8;

                let addr = (addr_hi | addr_lo).wrapping_add(self.regs.y);

                self.regs.pc = self.regs.pc + 2;

                self.mem[addr as usize]
            }
        }
    }

    fn store(&mut self, am: AddressingMode, data: u8) -> () {
        match am {
            AddressingMode::Implicit => {}
            AddressingMode::Accumulator => {
                self.regs.a = data;
            }
            AddressingMode::Immediate => {}
            AddressingMode::ZeroPage => {
                self.mem[self.mem[self.regs.pc as usize] as usize] = data;
                self.regs.pc = self.regs.pc + 1;
            }
            AddressingMode::ZeroPageX => {
                let addr = self.mem[self.regs.pc as usize].wrapping_add(self.regs.x);
                self.mem[addr as usize] = data;
                self.regs.pc = self.regs.pc + 1;
            }
            AddressingMode::ZeroPageY => {
                let addr = self.mem[self.regs.pc as usize].wrapping_add(self.regs.y);
                self.mem[addr as usize] = data;
                self.regs.pc = self.regs.pc + 1;
            }
            AddressingMode::Relative => {}
            AddressingMode::Absolute => {
                let addr = ((self.mem[self.regs.pc as usize] as usize) << 8)
                    + (self.mem[(self.regs.pc + 1) as usize] as usize);
                self.mem[addr] = data;
                self.regs.pc = self.regs.pc + 2;
            }
            AddressingMode::AbsoluteX => {
                let base_addr = ((self.mem[self.regs.pc as usize] as usize) << 8)
                    + (self.mem[(self.regs.pc + 1) as usize] as usize);
                let addr = (base_addr as u16).wrapping_add(self.regs.x as u16) as usize;
                self.mem[addr] = data;
                self.regs.pc = self.regs.pc + 2;
            }
            AddressingMode::AbsoluteY => {
                let base_addr = ((self.mem[self.regs.pc as usize] as usize) << 8)
                    + (self.mem[(self.regs.pc + 1) as usize] as usize);
                let addr = (base_addr as u16).wrapping_add(self.regs.y as u16) as usize;
                self.mem[addr] = data;
                self.regs.pc = self.regs.pc + 2;
            }
            AddressingMode::Indirect => {}
            AddressingMode::IndirectX => {
                let addr_lo = self.mem[self.regs.pc as usize].wrapping_add(self.regs.x);
                let val_lo = self.mem[addr_lo as usize];

                let addr_hi = addr_lo.wrapping_add(1);
                let val_hi = (self.mem[addr_hi as usize] as u16) << 8;

                self.mem[(addr_hi | addr_lo) as usize] = data;

                self.regs.pc = self.regs.pc + 2;
            }
            AddressingMode::IndirectY => {
                let addr_lo = self.mem[self.regs.pc as usize];
                let val_lo = self.mem[addr_lo as usize];

                let addr_hi = addr_lo.wrapping_add(1);
                let val_hi = (self.mem[addr_hi as usize] as u16) << 8;

                let addr = (addr_hi | addr_lo).wrapping_add(self.regs.y);

                self.mem[addr as usize] = data;

                self.regs.pc = self.regs.pc + 2;
            }
        }
    }
}

fn decode(opcode: u8) -> (Instruction, AddressingMode) {
    match opcode {
        0x69 => (Instruction::Adc, AddressingMode::Immediate),
        0x65 => (Instruction::Adc, AddressingMode::ZeroPage),
        0x75 => (Instruction::Adc, AddressingMode::ZeroPageX),
        0x6D => (Instruction::Adc, AddressingMode::Absolute),
        0x7D => (Instruction::Adc, AddressingMode::AbsoluteX),
        0x79 => (Instruction::Adc, AddressingMode::AbsoluteY),
        0x61 => (Instruction::Adc, AddressingMode::IndirectX),
        0x71 => (Instruction::Adc, AddressingMode::IndirectY),
        0x0A => (Instruction::Adc, AddressingMode::Accumulator),
        0x06 => (Instruction::Adc, AddressingMode::ZeroPage),
        0x16 => (Instruction::Adc, AddressingMode::ZeroPageX),
        0x0E => (Instruction::Adc, AddressingMode::Absolute),
        0x1E => (Instruction::Adc, AddressingMode::AbsoluteX),
        0x90 => (Instruction::Bcc, AddressingMode::Relative),
        0xB0 => (Instruction::Bcs, AddressingMode::Relative),
        0xF0 => (Instruction::Beq, AddressingMode::Relative),
        0x24 => (Instruction::Bit, AddressingMode::ZeroPage),
        0x2C => (Instruction::Bit, AddressingMode::Absolute),
        0x30 => (Instruction::Bmi, AddressingMode::Relative),
        0xD0 => (Instruction::Bne, AddressingMode::Relative),
        0x10 => (Instruction::Bpl, AddressingMode::Relative),
        0x00 => (Instruction::Brk, AddressingMode::Implicit),
        0x50 => (Instruction::Bvc, AddressingMode::Relative),
        0x70 => (Instruction::Bvs, AddressingMode::Relative),
        0x18 => (Instruction::Clc, AddressingMode::Implicit),
        0xD8 => (Instruction::Cld, AddressingMode::Implicit),
        0x58 => (Instruction::Cli, AddressingMode::Implicit),
        0xB8 => (Instruction::Clv, AddressingMode::Implicit),
        0xC9 => (Instruction::Cmp, AddressingMode::Immediate),
        0xC5 => (Instruction::Cmp, AddressingMode::ZeroPage),
        0xD5 => (Instruction::Cmp, AddressingMode::ZeroPageX),
        0xCD => (Instruction::Cmp, AddressingMode::Absolute),
        0xDD => (Instruction::Cmp, AddressingMode::AbsoluteX),
        0xD9 => (Instruction::Cmp, AddressingMode::AbsoluteY),
        0xC1 => (Instruction::Cmp, AddressingMode::IndirectX),
        0xD1 => (Instruction::Cmp, AddressingMode::IndirectY),
        0xE0 => (Instruction::Cpx, AddressingMode::Immediate),
        0xE4 => (Instruction::Cpx, AddressingMode::ZeroPage),
        0xEC => (Instruction::Cpx, AddressingMode::Absolute),
        0xC0 => (Instruction::Cpy, AddressingMode::Immediate),
        0xC4 => (Instruction::Cpy, AddressingMode::ZeroPage),
        0xCC => (Instruction::Cpy, AddressingMode::Absolute),
        0xC6 => (Instruction::Dec, AddressingMode::ZeroPage),
        0xD6 => (Instruction::Dec, AddressingMode::ZeroPageX),
        0xCE => (Instruction::Dec, AddressingMode::Absolute),
        0xDE => (Instruction::Dec, AddressingMode::AbsoluteX),
        0xCA => (Instruction::Dex, AddressingMode::Implicit),
        0x88 => (Instruction::Dey, AddressingMode::Implicit),
        0x49 => (Instruction::Eor, AddressingMode::Immediate),
        0x45 => (Instruction::Eor, AddressingMode::ZeroPage),
        0x55 => (Instruction::Eor, AddressingMode::ZeroPageX),
        0x4D => (Instruction::Eor, AddressingMode::Absolute),
        0x5D => (Instruction::Eor, AddressingMode::AbsoluteX),
        0x59 => (Instruction::Eor, AddressingMode::AbsoluteY),
        0x41 => (Instruction::Eor, AddressingMode::IndirectX),
        0x51 => (Instruction::Eor, AddressingMode::IndirectY),
        0xE6 => (Instruction::Inc, AddressingMode::ZeroPage),
        0xF6 => (Instruction::Inc, AddressingMode::ZeroPageX),
        0xEE => (Instruction::Inc, AddressingMode::Absolute),
        0xFE => (Instruction::Inc, AddressingMode::AbsoluteX),
        0xE8 => (Instruction::Inx, AddressingMode::Implicit),
        0xC8 => (Instruction::Iny, AddressingMode::Implicit),
        0x4C => (Instruction::Jmp, AddressingMode::Absolute),
        0x6C => (Instruction::Jmp, AddressingMode::Indirect),
        0x20 => (Instruction::Jsr, AddressingMode::Absolute),
        0xA9 => (Instruction::Lda, AddressingMode::Immediate),
        0xA5 => (Instruction::Lda, AddressingMode::ZeroPage),
        0xB5 => (Instruction::Lda, AddressingMode::ZeroPageX),
        0xAD => (Instruction::Lda, AddressingMode::Absolute),
        0xBD => (Instruction::Lda, AddressingMode::AbsoluteX),
        0xB9 => (Instruction::Lda, AddressingMode::AbsoluteY),
        0xA1 => (Instruction::Lda, AddressingMode::IndirectX),
        0xB1 => (Instruction::Lda, AddressingMode::IndirectY),
        0xA2 => (Instruction::Ldx, AddressingMode::Immediate),
        0xA6 => (Instruction::Ldx, AddressingMode::ZeroPage),
        0xB6 => (Instruction::Ldx, AddressingMode::ZeroPageY),
        0xAE => (Instruction::Ldx, AddressingMode::Absolute),
        0xBE => (Instruction::Ldx, AddressingMode::AbsoluteY),
        0xA0 => (Instruction::Ldy, AddressingMode::Immediate),
        0xA4 => (Instruction::Ldy, AddressingMode::ZeroPage),
        0xB4 => (Instruction::Ldy, AddressingMode::ZeroPageX),
        0xAC => (Instruction::Ldy, AddressingMode::Absolute),
        0xBC => (Instruction::Ldy, AddressingMode::AbsoluteX),
        0x4A => (Instruction::Lsr, AddressingMode::Accumulator),
        0x46 => (Instruction::Lsr, AddressingMode::ZeroPage),
        0x56 => (Instruction::Lsr, AddressingMode::ZeroPageX),
        0x4E => (Instruction::Lsr, AddressingMode::Absolute),
        0x5E => (Instruction::Lsr, AddressingMode::AbsoluteX),
        0xEA => (Instruction::Nop, AddressingMode::Implicit),
        0x09 => (Instruction::Ora, AddressingMode::Immediate),
        0x05 => (Instruction::Ora, AddressingMode::ZeroPage),
        0x15 => (Instruction::Ora, AddressingMode::ZeroPageX),
        0x0D => (Instruction::Ora, AddressingMode::Absolute),
        0x1D => (Instruction::Ora, AddressingMode::AbsoluteX),
        0x19 => (Instruction::Ora, AddressingMode::AbsoluteY),
        0x01 => (Instruction::Ora, AddressingMode::IndirectX),
        0x11 => (Instruction::Ora, AddressingMode::IndirectY),
        0x48 => (Instruction::Pha, AddressingMode::Implicit),
        0x08 => (Instruction::Php, AddressingMode::Implicit),
        0x68 => (Instruction::Pla, AddressingMode::Implicit),
        0x28 => (Instruction::Plp, AddressingMode::Implicit),
        0x2A => (Instruction::Rol, AddressingMode::Accumulator),
        0x26 => (Instruction::Rol, AddressingMode::ZeroPage),
        0x36 => (Instruction::Rol, AddressingMode::ZeroPageX),
        0x2E => (Instruction::Rol, AddressingMode::Absolute),
        0x3E => (Instruction::Rol, AddressingMode::AbsoluteX),
        0x6A => (Instruction::Ror, AddressingMode::Accumulator),
        0x66 => (Instruction::Ror, AddressingMode::ZeroPage),
        0x76 => (Instruction::Ror, AddressingMode::ZeroPageX),
        0x6E => (Instruction::Ror, AddressingMode::Absolute),
        0x7E => (Instruction::Ror, AddressingMode::AbsoluteX),
        0x40 => (Instruction::Rti, AddressingMode::Implicit),
        0x60 => (Instruction::Rts, AddressingMode::Implicit),
        0xE9 => (Instruction::Sbc, AddressingMode::Immediate),
        0xE5 => (Instruction::Sbc, AddressingMode::ZeroPage),
        0xF5 => (Instruction::Sbc, AddressingMode::ZeroPageX),
        0xED => (Instruction::Sbc, AddressingMode::Absolute),
        0xFD => (Instruction::Sbc, AddressingMode::AbsoluteX),
        0xF9 => (Instruction::Sbc, AddressingMode::AbsoluteY),
        0xE1 => (Instruction::Sbc, AddressingMode::IndirectX),
        0xF1 => (Instruction::Sbc, AddressingMode::IndirectY),
        0x38 => (Instruction::Sec, AddressingMode::Implicit),
        0xF8 => (Instruction::Sed, AddressingMode::Implicit),
        0x78 => (Instruction::Sei, AddressingMode::Implicit),
        0x85 => (Instruction::Sta, AddressingMode::ZeroPage),
        0x95 => (Instruction::Sta, AddressingMode::ZeroPageX),
        0x8D => (Instruction::Sta, AddressingMode::Absolute),
        0x9D => (Instruction::Sta, AddressingMode::AbsoluteX),
        0x99 => (Instruction::Sta, AddressingMode::AbsoluteY),
        0x81 => (Instruction::Sta, AddressingMode::IndirectX),
        0x91 => (Instruction::Sta, AddressingMode::IndirectY),
        0x86 => (Instruction::Stx, AddressingMode::ZeroPage),
        0x96 => (Instruction::Stx, AddressingMode::ZeroPageY),
        0x8E => (Instruction::Stx, AddressingMode::Absolute),
        0x84 => (Instruction::Sty, AddressingMode::ZeroPage),
        0x94 => (Instruction::Sty, AddressingMode::ZeroPageX),
        0x8C => (Instruction::Sty, AddressingMode::Absolute),
        0xAA => (Instruction::Tax, AddressingMode::Implicit),
        0xA8 => (Instruction::Tay, AddressingMode::Implicit),
        0xBA => (Instruction::Tsx, AddressingMode::Implicit),
        0x8A => (Instruction::Txa, AddressingMode::Implicit),
        0x9A => (Instruction::Txs, AddressingMode::Implicit),
        0x98 => (Instruction::Tya, AddressingMode::Implicit),
        _ => (Instruction::Nop, AddressingMode::Implicit),
    }
}
