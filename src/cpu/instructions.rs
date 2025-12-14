use crate::cpu::decoder::{CC, Opcode, OpcodeEntry, R8, R16};
use crate::cpu::Cpu;
use crate::cpu::registers::Flags;
use crate::mmu::{Mmu, HIGH_RAM};

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
            CC::NZ => self.registers.get_flag(Flags::Z) == 0,
            CC::Z  => self.registers.get_flag(Flags::Z) != 0,
            CC::NC => self.registers.get_flag(Flags::C) == 0,
            CC::C  => self.registers.get_flag(Flags::C) != 0,
        }
    }

    pub fn execute_instruction(&mut self, entry: &OpcodeEntry, mmu: &mut Mmu) {
        let mut increment = true;
        match &entry.opcode {
            Opcode::LdR8R8(reg, vreg) => {
                let value = self.read_r8(vreg, mmu);
                self.write_r8(reg, value, mmu);
            },
            Opcode::LdR8N8(reg) => {
                let value = mmu.read_8(self.registers.pc + 1);
                self.write_r8(reg, value, mmu);
            },
            Opcode::LdR16N16(reg) => {
                let value = mmu.read_16(self.registers.pc + 1);
                self.write_r16(reg, value);
            },
            Opcode::LdPtrR16A(reg) => {
                let addr = self.read_r16(reg);
                mmu.write_8(addr, self.registers.a);
            },
            Opcode::LdPtrN16A => {
                let addr = mmu.read_16(self.registers.pc + 1);
                mmu.write_8(addr, self.registers.a);
            },
            Opcode::LdHPtrCA => {
                let addr = HIGH_RAM | self.registers.c as u16;
                mmu.write_8(addr, self.registers.a);
            }
            Opcode::LdAPtrR16(reg) => {
                let addr = self.read_r16(reg);
                self.registers.a = mmu.read_8(addr)
            },
            Opcode::LdAPtrN16 => {
                self.registers.a = mmu.read_8(mmu.read_16(self.registers.pc + 1)); 
            },
            Opcode::LdHAPtrC => {
                let addr = HIGH_RAM | self.registers.c as u16;
                self.registers.a = mmu.read_8(addr);
            },
            Opcode::LDHAPtrN8 => {
                let addr = HIGH_RAM | mmu.read_8(self.registers.pc + 1) as u16;
                self.registers.a = mmu.read_8(addr);
            },
            Opcode::LDHPtrN8A => {
                let addr = HIGH_RAM | mmu.read_8(self.registers.pc + 1) as u16;
                mmu.write_8(addr, self.registers.a);
            },
            Opcode::LdPtrHLIncA => {
                let addr = self.registers.hl();
                mmu.write_8(addr, self.registers.a);
                self.registers.set_hl(self.registers.hl() + 1);
            },
            Opcode::LdPtrHLDecA => {
                let addr = self.registers.hl();
                mmu.write_8(addr, self.registers.a);
                self.registers.set_hl(self.registers.hl() - 1);
            },
            Opcode::LdAPtrHLDec => {
                let addr = self.registers.hl();
                self.registers.a = mmu.read_8(addr);
                self.registers.set_hl(self.registers.hl() - 1);
            },
            Opcode::LdAPtrHLInc => {
                let addr = self.registers.hl();
                self.registers.a = mmu.read_8(addr);
                self.registers.set_hl(self.registers.hl() + 1);
            },
            Opcode::AdcAR8(reg) => {
                let value = self.read_r8(reg, mmu);
                let result = self.registers.a.wrapping_add(value).wrapping_add(self.registers.get_flag(Flags::C));
                let z = result == 0;
                let h = ((self.registers.a & 0x0F) + (value & 0x0F) + self.registers.get_flag(Flags::C)) > 0x0F;
                let c = (self.registers.a as u16 + value as u16 + self.registers.get_flag(Flags::C) as u16) > 0xFF;
                self.registers.a = result;
                self.registers.set_flags(z, false, h, c);
            },
            Opcode::AdcAN8 => {
                let value = mmu.read_8(self.registers.pc + 1);
                let result = self.registers.a.wrapping_add(value).wrapping_add(self.registers.get_flag(Flags::C));
                let z = result == 0;
                let h = ((self.registers.a & 0x0F) + (value & 0x0F) + self.registers.get_flag(Flags::C)) > 0x0F;
                let c = (self.registers.a as u16 + value as u16 + self.registers.get_flag(Flags::C) as u16) > 0xFF;
                self.registers.a = result;
                self.registers.set_flags(z, false, h, c);
            },
            Opcode::AddAR8(reg) => {
                let value = self.read_r8(reg, mmu);
                let result = self.registers.a.wrapping_add(value);
                let z = result == 0;
                let h = ((self.registers.a & 0x0F) + (value & 0x0F)) > 0x0F;
                let c = (self.registers.a as u16 + value as u16) > 0xFF;
                self.registers.a = result;
                self.registers.set_flags(z, false, h, c);
            },
            Opcode::AddAN8 => {
                let value = mmu.read_8(self.registers.pc + 1);
                let result = self.registers.a.wrapping_add(value);
                let z = result == 0;
                let h = ((self.registers.a & 0x0F) + (value & 0x0F)) > 0x0F;
                let c = (self.registers.a as u16 + value as u16) > 0xFF;
                self.registers.a = result;
                self.registers.set_flags(z, false, h, c);
            
            },
            Opcode::CpAR8(reg) => {
                let value = self.read_r8(reg, mmu);
                let result = self.registers.a.wrapping_sub(value);
                let z = result == 0;
                let h = (value & 0x0F) > (self.registers.a & 0x0F);
                let c = value > self.registers.a;
                self.registers.set_flags(z, true, h, c);
            },
            Opcode::CpAN8 => {
                let value = mmu.read_8(self.registers.pc + 1);
                let result = self.registers.a.wrapping_sub(value);
                let z = result == 0;
                let h = (value & 0x0F) > (self.registers.a & 0x0F);
                let c = value > self.registers.a;
                self.registers.set_flags(z, true, h, c);
            },
            Opcode::DecR8(reg) => {
                let old = self.read_r8(reg, mmu);
                let result = old.wrapping_sub(1);
                self.write_r8(reg, result, mmu);

                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, true);
                self.registers.set_flag(Flags::H, (old & 0x0F) == 0);
            },
            Opcode::IncR8(reg) => {
                let old = self.read_r8(reg, mmu);
                let result = old.wrapping_add(1);
                self.write_r8(reg, result, mmu);

                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, (old & 0x0F) == 0x0F);
            },
            Opcode::SbcAR8(reg) => {
                let value = self.read_r8(reg, mmu);
                let result = self.registers.a.wrapping_sub(value).wrapping_sub(self.registers.get_flag(Flags::C));
                let z = result == 0;
                let h = self.registers.a & 0x0F < ((value & 0x0F) + self.registers.get_flag(Flags::C));
                let c = (self.registers.a as u16) < (value as u16 + self.registers.get_flag(Flags::C) as u16);
                self.registers.a = result;
                self.registers.set_flags(z, true, h, c);
            },
            Opcode::SbcAN8 => {
                let value = mmu.read_8(self.registers.pc + 1);
                let result = self.registers.a.wrapping_sub(value).wrapping_sub(self.registers.get_flag(Flags::C));
                let z = result == 0;
                let h = self.registers.a & 0x0F < ((value & 0x0F) + self.registers.get_flag(Flags::C));
                let c = (self.registers.a as u16) < (value as u16 + self.registers.get_flag(Flags::C) as u16);
                self.registers.a = result;
                self.registers.set_flags(z, true, h, c);
            },
            Opcode::SubAR8(reg) => {
                let value = self.read_r8(reg, mmu);
                let result = self.registers.a.wrapping_sub(value);
                let z = result == 0;
                let h = self.registers.a & 0x0F < (value & 0x0F);
                let c = (self.registers.a as u16) < (value as u16);
                self.registers.a = result;
                self.registers.set_flags(z, true, h, c);
            },
            Opcode::SubAN8 => {
                let value = mmu.read_8(self.registers.pc + 1);
                let result = self.registers.a.wrapping_sub(value);
                let z = result == 0;
                let h = self.registers.a & 0x0F < (value & 0x0F);
                let c = (self.registers.a as u16) < (value as u16);
                self.registers.a = result;
                self.registers.set_flags(z, true, h, c);
            },
            Opcode::AddHLR16(reg) => {
                let hl = self.registers.hl();
                let value = self.read_r16(reg);
                self.registers.set_flag(Flags::H, (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::C, hl as u32 + value as u32 > 0x0000FFFF);
                self.registers.set_hl(hl.wrapping_add(value));
            },
            Opcode::DecR16(reg) => {
                let value = self.read_r16(reg);
                self.write_r16(reg, value.wrapping_sub(1));
            },
            Opcode::IncR16(reg) => {
                let value = self.read_r16(reg);
                self.write_r16(reg, value.wrapping_add(1));
            },
            Opcode::AndAR8(reg) => {
                self.registers.a &= self.read_r8(reg, mmu);
                self.registers.set_flags(
                    self.registers.a == 0, 
                    false, 
                    true, 
                    false
                );
            },
            Opcode::AndAN8 => {
                self.registers.a &= mmu.read_8(self.registers.pc + 1);
                self.registers.set_flags(
                    self.registers.a == 0, 
                    false, 
                    true, 
                    false
                );
            },
            Opcode::Cpl => {
                self.registers.a = !self.registers.a;
                self.registers.set_flag(Flags::N, true);
                self.registers.set_flag(Flags::H, true);
            },
            Opcode::OrAR8(reg) => {
                self.registers.a |= self.read_r8(reg, mmu);
                self.registers.set_flags(
                    self.registers.a == 0, 
                    false, 
                    false, 
                    false
                );
            },
            Opcode::OrAN8 => {
                self.registers.a |= mmu.read_8(self.registers.pc + 1);
                self.registers.set_flags(
                    self.registers.a == 0, 
                    false, 
                    false, 
                    false
                );
            },
            Opcode::XorAR8(reg) => {
                self.registers.a ^= self.read_r8(reg, mmu);
                self.registers.set_flags(
                    self.registers.a == 0, 
                    false, 
                    false, 
                    false
                );
            },
            Opcode::XorAN8 => {
                self.registers.a ^= mmu.read_8(self.registers.pc + 1);
                self.registers.set_flags(
                    self.registers.a == 0, 
                    false, 
                    false, 
                    false
                );
            },
            Opcode::Bit(n, reg) => {
                self.registers.set_flag(Flags::Z, ((self.read_r8(reg, mmu) >> n) & 1) == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, true);
            },
            Opcode::Set(n, reg) => {
                let value = self.read_r8(reg, mmu);
                let result = value | (1 << n);
                self.write_r8(reg, result, mmu);
            },
            Opcode::Res(n, reg) => {
                let value = self.read_r8(reg, mmu);
                let result = value & !(1 << n);
                self.write_r8(reg, result, mmu);
            },
            Opcode::RlR8(reg) => {
                let val = self.read_r8(reg, mmu);
                let old_carry = self.registers.get_flag(Flags::C);

                self.registers.set_flag(Flags::C, (val & 0x80) != 0);

                let result = (val << 1) | old_carry;
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            },
            Opcode::RlA => {
                let a = self.registers.a;
                let old_carry = self.registers.get_flag(Flags::C);

                self.registers.set_flag(Flags::C, (a & 0x80) != 0);
                self.registers.a = (a << 1) | old_carry;
                self.registers.set_flag(Flags::Z, false);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            },
            Opcode::RlcR8(reg) => {
                let val = self.read_r8(reg, mmu);

                self.registers.set_flag(Flags::C, (val & 0x80) != 0);

                let result = (val << 1) | (val >> 7);
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            },
            Opcode::RlcA => {
                let a = self.registers.a;

                self.registers.set_flag(Flags::C, (a & 0x80) != 0);
                self.registers.a = (a << 1) | (a >> 7);
                self.registers.set_flag(Flags::Z, false);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            },
            Opcode::RrR8(reg) => {
                let val = self.read_r8(reg, mmu);
                let old_carry = self.registers.get_flag(Flags::C);

                self.registers.set_flag(Flags::C, (val & 0x01) != 0);

                let result = (val >> 1) | (old_carry << 7);
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            },
            Opcode::RrA => {
                let a = self.registers.a;
                let old_carry = self.registers.get_flag(Flags::C);

                self.registers.set_flag(Flags::C, (a & 0x01) != 0);
                self.registers.a = (a >> 1) | (old_carry << 7);
                self.registers.set_flag(Flags::Z, false);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            },
            Opcode::RrcR8(reg) => {
                let val = self.read_r8(reg, mmu);

                self.registers.set_flag(Flags::C, (val & 0x01) != 0);

                let result = (val >> 1) | (val << 7);
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            },
            Opcode::RrcA => {
                let a = self.registers.a;

                self.registers.set_flag(Flags::C, (a & 0x01) != 0);
                self.registers.a = (a >> 1) | (a << 7);
                self.registers.set_flag(Flags::Z, false);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            },
            Opcode::SlaR8(reg) => {
                let val = self.read_r8(reg, mmu);
                self.registers.set_flag(Flags::C, (val & 0x80) != 0);

                let result = val << 1;
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            },
            Opcode::SraR8(reg) => {
                let val = self.read_r8(reg, mmu);
                self.registers.set_flag(Flags::C, (val & 0x01) != 0);

                let result = (val >> 1) | (val & 0x80); // preserve bit 7
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            },
            Opcode::SrlR8(reg) => {
                let val = self.read_r8(reg, mmu);
                self.registers.set_flag(Flags::C, (val & 0x01) != 0);

                let result = val >> 1; // preserve bit 7
                self.write_r8(reg, result, mmu);
                self.registers.set_flag(Flags::Z, result == 0);
                self.registers.set_flag(Flags::N, false);
                self.registers.set_flag(Flags::H, false);
            },
            Opcode::SwapR8(reg) => {
                let val = self.read_r8(reg, mmu);
                let result = (val >> 4) | (val << 4);
                self.write_r8(reg, result, mmu);
                self.registers.set_flags(
                    result == 0, 
                    false, 
                    false, 
                    false
                );
            },
            Opcode::CallN16 => {
                let target = mmu.read_16(self.registers.pc + 1);
                let ret = self.registers.pc + 3;

                self.push_u16(mmu, ret);
                self.registers.pc = target;
                increment = false;
            }
            Opcode::CallCCN16(cc) => {
                if self.condition_met(cc) {
                    let target = mmu.read_16(self.registers.pc + 1);
                    let ret = self.registers.pc + 3;

                    self.push_u16(mmu, ret);
                    self.registers.pc = target;
                    increment = false;
                }
            },
            Opcode::JpHL => {},
            Opcode::JpN16 => {
                self.registers.pc = mmu.read_16(self.registers.pc.wrapping_add(1));
                increment = false;
            },
            Opcode::JpCCN16(cc) => {},
            Opcode::JrE8 => {},
            Opcode::JrCCE8(cc) => {},
            Opcode::Ret => {
                self.registers.pc = self.pop_u16(mmu);
                increment = false;
            },
            Opcode::RetCC(cc) => {
                if self.condition_met(cc) {
                    self.registers.pc = self.pop_u16(mmu);
                    increment = false;
                }
            },
            Opcode::RetI => {
                self.int.ime = true;
            },
            Opcode::Rst(n) => {
                self.push_u16(mmu, self.registers.pc + 1);
                self.registers.pc = *n as u16;
                increment = false;
            },
            Opcode::Scf => {},
            Opcode::Ccf => {},
            Opcode::AddHLSP => {},
            Opcode::AddSPe8 => {},
            Opcode::LdHLSPe8 => {},
            Opcode::DecSP => {},
            Opcode::IncSP => {},
            Opcode::LdSPN16 => {
                self.registers.sp = mmu.read_16(self.registers.pc + 1);
            },
            Opcode::LdPtrN16SP => {},
            Opcode::LdSPHL => {},
            Opcode::PopAF => {},
            Opcode::PopR16(reg) => {},
            Opcode::PushAF => {},
            Opcode::PushR16(reg) => {},
            Opcode::Di => {
                self.int.ime = false;
            },
            Opcode::Ei => {
                self.int.ime_scheduled = true;
            },
            Opcode::Halt => {},
            Opcode::Daa => {},
            Opcode::Nop => {
                // Nothing
            },
            Opcode::Stop => {},
            Opcode::Undefined => {
                // Undefined
            },
            Opcode::Prefix => {
                // Marks CB opcode
            },
        }
        if increment {
            self.increment_pc(entry.length as u16);
        }
    }
}