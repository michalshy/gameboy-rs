use crate::cpu::Cpu;
use crate::cpu::decoder::decode_cb;
use crate::cpu::decoder::{CC, Opcode, OpcodeEntry, R8, R16};
use crate::cpu::registers::Flags;
use crate::mmu::{HIGH_RAM, Mmu};

impl Cpu {
    fn increment_pc(&mut self, v: u16) {
        self.registers.pc += v;
    }

    fn read_r8(&self, r: &R8, mmu: &mut Mmu) -> u8 {
        match r {
            R8::A => self.registers.a,
            R8::B => self.registers.b,
            R8::C => self.registers.c,
            R8::D => self.registers.d,
            R8::E => self.registers.e,
            R8::H => self.registers.h,
            R8::L => self.registers.l,
            R8::HLIndirect => {
                let addr = self.registers.hl();
                mmu.read_8(addr)
            }
        }
    }

    fn write_r8(&mut self, r: &R8, value: u8, mmu: &mut Mmu) {
        match r {
            R8::A => self.registers.a = value,
            R8::B => self.registers.b = value,
            R8::C => self.registers.c = value,
            R8::D => self.registers.d = value,
            R8::E => self.registers.e = value,
            R8::H => self.registers.h = value,
            R8::L => self.registers.l = value,
            R8::HLIndirect => {
                let addr = self.registers.hl();
                mmu.write_8(addr, value);
            }
        }
    }

    fn read_r16(&mut self, r: &R16) -> u16 {
        match r {
            R16::AF => self.registers.af(),
            R16::BC => self.registers.bc(),
            R16::DE => self.registers.de(),
            R16::HL => self.registers.hl(),
            R16::SP => self.registers.sp,
            R16::PC => self.registers.pc,
        }
    }

    fn write_r16(&mut self, r: &R16, value: u16) {
        match r {
            R16::AF => self.registers.set_af(value),
            R16::BC => self.registers.set_bc(value),
            R16::DE => self.registers.set_de(value),
            R16::HL => self.registers.set_hl(value),
            R16::SP => self.registers.sp = value,
            R16::PC => self.registers.pc = value,
        }
    }

    fn push_u8(&mut self, mmu: &mut Mmu, value: u8) {
        self.registers.sp -= 1;
        mmu.write_8(self.registers.sp, value);
    }

    fn pop_u8(&mut self, mmu: &mut Mmu) -> u8 {
        let value = mmu.read_8(self.registers.sp);
        self.registers.sp += 1;
        value
    }

    fn push_u16(&mut self, mmu: &mut Mmu, value: u16) {
        self.push_u8(mmu, (value >> 8) as u8);
        self.push_u8(mmu, value as u8);
    }

    fn pop_u16(&mut self, mmu: &mut Mmu) -> u16 {
        let low = self.pop_u8(mmu) as u16;
        let high = self.pop_u8(mmu) as u16;
        (high << 8) | low
    }

    fn condition_met(&self, cc: &CC) -> bool {
        match cc {
            CC::NZ => !self.registers.get_flag(Flags::Z),
            CC::Z => self.registers.get_flag(Flags::Z),
            CC::NC => !self.registers.get_flag(Flags::C),
            CC::C => self.registers.get_flag(Flags::C),
        }
    }

    pub fn execute_instruction(&mut self, entry: &OpcodeEntry, mmu: &mut Mmu, increment: bool) {
        let mut increment = increment;
        let set_ime = self.interrupts.ime_scheduled;
        self.instruction_number += 1;
        match &entry.opcode {
            Opcode::LdR8R8(reg, vreg) => {
                let value = self.read_r8(vreg, mmu);
                self.write_r8(reg, value, mmu);
            }
            Opcode::LdR8N8(reg) => {
                let value = mmu.read_8(self.registers.pc + 1);
                self.write_r8(reg, value, mmu);
            }
            Opcode::LdR16N16(reg) => {
                let value = mmu.read_16(self.registers.pc + 1);
                self.write_r16(reg, value);
            }
            Opcode::LdPtrR16A(reg) => {
                let addr = self.read_r16(reg);
                mmu.write_8(addr, self.registers.a);
            }
            Opcode::LdPtrN16A => {
                let addr = mmu.read_16(self.registers.pc + 1);
                mmu.write_8(addr, self.registers.a);
            }
            Opcode::LdHPtrCA => {
                let addr = HIGH_RAM | self.registers.c as u16;
                mmu.write_8(addr, self.registers.a);
            }
            Opcode::LdAPtrR16(reg) => {
                let addr = self.read_r16(reg);
                self.registers.a = mmu.read_8(addr)
            }
            Opcode::LdAPtrN16 => {
                self.registers.a = mmu.read_8(mmu.read_16(self.registers.pc + 1));
            }
            Opcode::LdHAPtrC => {
                let addr = HIGH_RAM | self.registers.c as u16;
                self.registers.a = mmu.read_8(addr);
            }
            Opcode::LDHAPtrN8 => {
                let addr = HIGH_RAM | mmu.read_8(self.registers.pc + 1) as u16;
                self.registers.a = mmu.read_8(addr);
            }
            Opcode::LDHPtrN8A => {
                let addr = HIGH_RAM | mmu.read_8(self.registers.pc + 1) as u16;
                mmu.write_8(addr, self.registers.a);
            }
            Opcode::LdPtrHLIncA => {
                let addr = self.registers.hl();
                mmu.write_8(addr, self.registers.a);
                self.registers.set_hl(self.registers.hl() + 1);
            }
            Opcode::LdPtrHLDecA => {
                let addr = self.registers.hl();
                mmu.write_8(addr, self.registers.a);
                self.registers.set_hl(self.registers.hl() - 1);
            }
            Opcode::LdAPtrHLDec => {
                let addr = self.registers.hl();
                self.registers.a = mmu.read_8(addr);
                self.registers.set_hl(self.registers.hl() - 1);
            }
            Opcode::LdAPtrHLInc => {
                let addr = self.registers.hl();
                self.registers.a = mmu.read_8(addr);
                self.registers.set_hl(self.registers.hl() + 1);
            }
            Opcode::AdcAR8(reg) => {
                let value = self.read_r8(reg, mmu);
                let result = self
                    .registers
                    .a
                    .wrapping_add(value)
                    .wrapping_add(self.registers.get_flag(Flags::C) as u8);
                let z = result == 0;
                let h = ((self.registers.a & 0x0F)
                    + (value & 0x0F)
                    + self.registers.get_flag(Flags::C) as u8)
                    > 0x0F;
                let c = (self.registers.a as u16
                    + value as u16
                    + self.registers.get_flag(Flags::C) as u16)
                    > 0xFF;
                self.registers.a = result;
                self.registers.set_flags(z, false, h, c);
            }
            Opcode::AdcAN8 => {
                let value = mmu.read_8(self.registers.pc + 1);
                let result = self
                    .registers
                    .a
                    .wrapping_add(value)
                    .wrapping_add(self.registers.get_flag(Flags::C) as u8);
                let z = result == 0;
                let h = ((self.registers.a & 0x0F)
                    + (value & 0x0F)
                    + self.registers.get_flag(Flags::C) as u8)
                    > 0x0F;
                let c = (self.registers.a as u16
                    + value as u16
                    + self.registers.get_flag(Flags::C) as u16)
                    > 0xFF;
                self.registers.a = result;
                self.registers.set_flags(z, false, h, c);
            }
            Opcode::AddAR8(reg) => {
                let value = self.read_r8(reg, mmu);
                let result = self.registers.a.wrapping_add(value);
                let z = result == 0;
                let h = ((self.registers.a & 0x0F) + (value & 0x0F)) > 0x0F;
                let c = (self.registers.a as u16 + value as u16) > 0xFF;
                self.registers.a = result;
                self.registers.set_flags(z, false, h, c);
            }
            Opcode::AddAN8 => {
                let value = mmu.read_8(self.registers.pc + 1);
                let result = self.registers.a.wrapping_add(value);
                let z = result == 0;
                let h = ((self.registers.a & 0x0F) + (value & 0x0F)) > 0x0F;
                let c = (self.registers.a as u16 + value as u16) > 0xFF;
                self.registers.a = result;
                self.registers.set_flags(z, false, h, c);
            }
            Opcode::CpAR8(reg) => {
                let value = self.read_r8(reg, mmu);
                let result = self.registers.a.wrapping_sub(value);
                let z = result == 0;
                let h = (value & 0x0F) > (self.registers.a & 0x0F);
                let c = value > self.registers.a;
                self.registers.set_flags(z, true, h, c);
            }
            Opcode::CpAN8 => {
                let value = mmu.read_8(self.registers.pc + 1);
                let result = self.registers.a.wrapping_sub(value);
                let z = result == 0;
                let h = (value & 0x0F) > (self.registers.a & 0x0F);
                let c = value > self.registers.a;
                self.registers.set_flags(z, true, h, c);
            }
            Opcode::DecR8(reg) => {
                let old = self.read_r8(reg, mmu);
                let result = old.wrapping_sub(1);
                self.write_r8(reg, result, mmu);

                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, true);
                self.registers.set_flag(Flags::H, (old & 0x0F) == 0);
            }
            Opcode::IncR8(reg) => {
                let old = self.read_r8(reg, mmu);
                let result = old.wrapping_add(1);
                self.write_r8(reg, result, mmu);

                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, (old & 0x0F) == 0x0F);
            }
            Opcode::SbcAR8(reg) => {
                let value = self.read_r8(reg, mmu);
                let result = self
                    .registers
                    .a
                    .wrapping_sub(value)
                    .wrapping_sub(self.registers.get_flag(Flags::C) as u8);
                let z = result == 0;
                let h = self.registers.a & 0x0F
                    < ((value & 0x0F) + self.registers.get_flag(Flags::C) as u8);
                let c = (self.registers.a as u16)
                    < (value as u16 + self.registers.get_flag(Flags::C) as u16);
                self.registers.a = result;
                self.registers.set_flags(z, true, h, c);
            }
            Opcode::SbcAN8 => {
                let value = mmu.read_8(self.registers.pc + 1);
                let result = self
                    .registers
                    .a
                    .wrapping_sub(value)
                    .wrapping_sub(self.registers.get_flag(Flags::C) as u8);
                let z = result == 0;
                let h = self.registers.a & 0x0F
                    < ((value & 0x0F) + self.registers.get_flag(Flags::C) as u8);
                let c = (self.registers.a as u16)
                    < (value as u16 + self.registers.get_flag(Flags::C) as u16);
                self.registers.a = result;
                self.registers.set_flags(z, true, h, c);
            }
            Opcode::SubAR8(reg) => {
                let value = self.read_r8(reg, mmu);
                let result = self.registers.a.wrapping_sub(value);
                let z = result == 0;
                let h = self.registers.a & 0x0F < (value & 0x0F);
                let c = (self.registers.a as u16) < (value as u16);
                self.registers.a = result;
                self.registers.set_flags(z, true, h, c);
            }
            Opcode::SubAN8 => {
                let value = mmu.read_8(self.registers.pc + 1);
                let result = self.registers.a.wrapping_sub(value);
                let z = result == 0;
                let h = self.registers.a & 0x0F < (value & 0x0F);
                let c = (self.registers.a as u16) < (value as u16);
                self.registers.a = result;
                self.registers.set_flags(z, true, h, c);
            }
            Opcode::AddHLR16(reg) => {
                let hl = self.registers.hl();
                let value = self.read_r16(reg);
                self.registers
                    .set_flag(Flags::H, (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
                self.registers.set_flag(Flags::N, false);
                self.registers
                    .set_flag(Flags::C, hl as u32 + value as u32 > 0x0000FFFF);
                self.registers.set_hl(hl.wrapping_add(value));
            }
            Opcode::DecR16(reg) => {
                let value = self.read_r16(reg);
                self.write_r16(reg, value.wrapping_sub(1));
            }
            Opcode::IncR16(reg) => {
                let value = self.read_r16(reg);
                self.write_r16(reg, value.wrapping_add(1));
            }
            Opcode::AndAR8(reg) => {
                self.registers.a &= self.read_r8(reg, mmu);
                self.registers
                    .set_flags(self.registers.a == 0, false, true, false);
            }
            Opcode::AndAN8 => {
                self.registers.a &= mmu.read_8(self.registers.pc + 1);
                self.registers
                    .set_flags(self.registers.a == 0, false, true, false);
            }
            Opcode::Cpl => {
                self.registers.a = !self.registers.a;
                self.registers.set_flag(Flags::N, true);
                self.registers.set_flag(Flags::H, true);
            }
            Opcode::OrAR8(reg) => {
                self.registers.a |= self.read_r8(reg, mmu);
                self.registers
                    .set_flags(self.registers.a == 0, false, false, false);
            }
            Opcode::OrAN8 => {
                self.registers.a |= mmu.read_8(self.registers.pc + 1);
                self.registers
                    .set_flags(self.registers.a == 0, false, false, false);
            }
            Opcode::XorAR8(reg) => {
                self.registers.a ^= self.read_r8(reg, mmu);
                self.registers
                    .set_flags(self.registers.a == 0, false, false, false);
            }
            Opcode::XorAN8 => {
                self.registers.a ^= mmu.read_8(self.registers.pc + 1);
                self.registers
                    .set_flags(self.registers.a == 0, false, false, false);
            }
            Opcode::Bit(n, reg) => {
                self.registers
                    .set_flag(Flags::Z, ((self.read_r8(reg, mmu) >> n) & 1) == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, true);
            }
            Opcode::Set(n, reg) => {
                let value = self.read_r8(reg, mmu);
                let result = value | (1 << n);
                self.write_r8(reg, result, mmu);
            }
            Opcode::Res(n, reg) => {
                let value = self.read_r8(reg, mmu);
                let result = value & !(1 << n);
                self.write_r8(reg, result, mmu);
            }
            Opcode::RlR8(reg) => {
                let val = self.read_r8(reg, mmu);
                let old_carry = self.registers.get_flag(Flags::C) as u8;

                self.registers.set_flag(Flags::C, (val & 0x80) != 0);

                let result = (val << 1) | old_carry;
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            }
            Opcode::RlA => {
                let a = self.registers.a;
                let old_carry = self.registers.get_flag(Flags::C) as u8;

                self.registers.set_flag(Flags::C, (a & 0x80) != 0);
                self.registers.a = (a << 1) | old_carry;
                self.registers.set_flag(Flags::Z, false);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            }
            Opcode::RlcR8(reg) => {
                let val = self.read_r8(reg, mmu);

                self.registers.set_flag(Flags::C, (val & 0x80) != 0);

                let result = val.rotate_left(1);
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            }
            Opcode::RlcA => {
                let a = self.registers.a;

                self.registers.set_flag(Flags::C, (a & 0x80) != 0);
                self.registers.a = a.rotate_left(1);
                self.registers.set_flag(Flags::Z, false);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            }
            Opcode::RrR8(reg) => {
                let val = self.read_r8(reg, mmu);
                let old_carry = self.registers.get_flag(Flags::C) as u8;

                self.registers.set_flag(Flags::C, (val & 0x01) != 0);

                let result = (val >> 1) | (old_carry << 7);
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            }
            Opcode::RrA => {
                let a = self.registers.a;
                let old_carry = self.registers.get_flag(Flags::C) as u8;

                self.registers.set_flag(Flags::C, (a & 0x01) != 0);
                self.registers.a = (a >> 1) | (old_carry << 7);
                self.registers.set_flag(Flags::Z, false);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            }
            Opcode::RrcR8(reg) => {
                let val = self.read_r8(reg, mmu);

                self.registers.set_flag(Flags::C, (val & 0x01) != 0);

                let result = val.rotate_right(1);
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            }
            Opcode::RrcA => {
                let a = self.registers.a;

                self.registers.set_flag(Flags::C, (a & 0x01) != 0);
                self.registers.a = a.rotate_right(1);
                self.registers.set_flag(Flags::Z, false);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            }
            Opcode::SlaR8(reg) => {
                let val = self.read_r8(reg, mmu);
                self.registers.set_flag(Flags::C, (val & 0x80) != 0);

                let result = val << 1;
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            }
            Opcode::SraR8(reg) => {
                let val = self.read_r8(reg, mmu);
                self.registers.set_flag(Flags::C, (val & 0x01) != 0);

                let result = (val >> 1) | (val & 0x80); // preserve bit 7
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            }
            Opcode::SrlR8(reg) => {
                let val = self.read_r8(reg, mmu);
                self.registers.set_flag(Flags::C, (val & 0x01) != 0);

                let result = val >> 1; // preserve bit 7
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            }
            Opcode::SwapR8(reg) => {
                let val = self.read_r8(reg, mmu);
                let result = val.rotate_left(4);
                self.write_r8(reg, result, mmu);
                self.registers.set_flags(result == 0, false, false, false);
            }
            Opcode::CallN16 => {
                let target = mmu.read_16(self.registers.pc + 1);
                let ret = self.registers.pc + 3;
                // pushes instruction after to the stack
                self.push_u16(mmu, ret);
                // jp n16
                self.registers.pc = target;
                increment = false;
            }
            Opcode::CallCCN16(cc) => {
                if self.condition_met(cc) {
                    //TODO: cycles
                    let target = mmu.read_16(self.registers.pc + 1);
                    let ret = self.registers.pc + 3;

                    self.push_u16(mmu, ret);
                    self.registers.pc = target;
                    increment = false;
                }
            }
            Opcode::JpHL => {
                increment = false;
                self.registers.pc = self.registers.hl();
            }
            Opcode::JpN16 => {
                self.registers.pc = mmu.read_16(self.registers.pc.wrapping_add(1));
                increment = false;
            }
            Opcode::JpCCN16(cc) => {
                if self.condition_met(cc) {
                    //TODO: CYCLES
                    self.registers.pc = mmu.read_16(self.registers.pc.wrapping_add(1));
                    increment = false;
                }
            }
            Opcode::JrE8 => {
                let offset = mmu.read_8(self.registers.pc + 1) as i8;
                let pc = self.registers.pc.wrapping_add(2);
                self.registers.pc = ((pc as i32) + (offset as i32)) as u16;
                increment = false;
            }
            Opcode::JrCCE8(cc) => {
                if self.condition_met(cc) {
                    let offset = mmu.read_8(self.registers.pc + 1) as i8;
                    let pc = self.registers.pc.wrapping_add(2);
                    self.registers.pc = ((pc as i32) + (offset as i32)) as u16;
                    increment = false;
                }
            }
            Opcode::Ret => {
                self.registers.pc = self.pop_u16(mmu);
                increment = false;
            }
            Opcode::RetCC(cc) => {
                if self.condition_met(cc) {
                    self.registers.pc = self.pop_u16(mmu);
                    increment = false;
                }
            }
            Opcode::RetI => {
                self.interrupts.set_ime();
                self.registers.pc = self.pop_u16(mmu);
                increment = false;
            }
            Opcode::Rst(vec) => {
                let ret = self.registers.pc + entry.length as u16;
                self.push_u16(mmu, ret);
                self.registers.pc = *vec as u16;
                increment = false;
            }
            Opcode::Scf => {
                self.registers.set_flag(Flags::H, false);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::C, true);
            }
            Opcode::Ccf => {
                self.registers.set_flag(Flags::H, false);
                self.registers.set_flag(Flags::N, false);
                self.registers
                    .set_flag(Flags::C, !self.registers.get_flag(Flags::C));
            }
            Opcode::AddHLSP => {
                let hl = self.registers.hl();
                let value = self.registers.sp;
                self.registers
                    .set_flag(Flags::H, (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
                self.registers.set_flag(Flags::N, false);
                self.registers
                    .set_flag(Flags::C, hl as u32 + value as u32 > 0x0000FFFF);
                self.registers.set_hl(hl.wrapping_add(value));
            }
            Opcode::AddSPe8 => {
                let e = mmu.read_8(self.registers.pc + 1) as i8;
                let sp = self.registers.sp;

                let result = (sp as i32 + e as i32) as u16;

                self.registers.set_flag(Flags::Z, false);
                self.registers.set_flag(Flags::N, false);
                self.registers
                    .set_flag(Flags::H, ((sp & 0xF) + ((e as u16) & 0xF)) > 0xF);
                self.registers
                    .set_flag(Flags::C, ((sp & 0xFF) + ((e as u16) & 0xFF)) > 0xFF);

                self.registers.sp = result;
            }
            Opcode::LdHLSPe8 => {
                let offset = mmu.read_8(self.registers.pc + 1) as i8;
                let sp = self.registers.sp;

                let unsigned = offset as u16;

                self.registers.set_flag(Flags::Z, false);
                self.registers.set_flag(Flags::N, false);
                self.registers
                    .set_flag(Flags::H, ((sp & 0x000F) + (unsigned & 0x000F)) > 0x000F);
                self.registers
                    .set_flag(Flags::C, ((sp & 0x00FF) + (unsigned & 0x00FF)) > 0x00FF);

                let result = (sp as i32) + (offset as i32);
                self.registers.set_hl(result as u16);
            }
            Opcode::DecSP => {
                self.registers.sp = self.registers.sp.wrapping_sub(1);
            }
            Opcode::IncSP => {
                self.registers.sp = self.registers.sp.wrapping_add(1);
            }
            Opcode::LdSPN16 => {
                self.registers.sp = mmu.read_16(self.registers.pc + 1);
            }
            Opcode::LdPtrN16SP => {
                let addr = mmu.read_16(self.registers.pc + 1);
                mmu.write_8(addr, (self.registers.sp & 0x00FF) as u8);
                mmu.write_8(addr + 1, (self.registers.sp >> 8) as u8);
            }
            Opcode::LdSPHL => {
                self.registers.sp = self.registers.hl();
            }
            Opcode::PopAF => {
                self.registers.f = self.pop_u8(mmu) & 0xF0; // mask low 4 bits
                self.registers.a = self.pop_u8(mmu);
            }
            Opcode::PopR16(reg) => {
                let value = self.pop_u16(mmu);
                self.write_r16(reg, value);
            }
            Opcode::PushAF => {
                self.push_u16(mmu, self.registers.af());
            }
            Opcode::PushR16(reg) => {
                let value = self.read_r16(reg);
                self.push_u16(mmu, value);
            }
            Opcode::Di => {
                self.interrupts.reset_ime();
            }
            Opcode::Ei => {
                self.interrupts.ime_scheduled = true;
            }
            Opcode::Halt => {
                self.interrupts.halted = true;
            }
            Opcode::Daa => {
                let mut a = self.registers.a;
                let mut adjust = 0;
                let mut carry = self.registers.get_flag(Flags::C);

                if !self.registers.get_flag(Flags::N) {
                    if self.registers.get_flag(Flags::H) || (a & 0x0F) > 9 {
                        adjust |= 0x06;
                    }
                    if carry || a > 0x99 {
                        adjust |= 0x60;
                        carry = true;
                    }
                    a = a.wrapping_add(adjust);
                } else {
                    if self.registers.get_flag(Flags::H) {
                        adjust |= 0x06;
                    }
                    if carry {
                        adjust |= 0x60;
                    }
                    a = a.wrapping_sub(adjust);
                }

                self.registers.a = a;
                self.registers.set_flag(Flags::Z, a == 0);
                self.registers.set_flag(Flags::H, false);
                self.registers.set_flag(Flags::C, carry);
            }
            Opcode::Nop => {
                // Nothing
            }
            Opcode::Stop => {}
            Opcode::Undefined => {
                // Undefined
            }
            Opcode::Prefix => {
                increment = false;
                let opcode_byte = mmu.read_8(self.registers.pc.wrapping_add(1));
                let entry = decode_cb(opcode_byte);
                self.registers.pc += 2;
                self.execute_instruction(entry, mmu, false);
            }
        }
        if increment {
            self.increment_pc(entry.length as u16);
        }
        if set_ime {
            self.interrupts.set_ime();
        }
    }
}
