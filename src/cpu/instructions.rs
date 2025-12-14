use crate::cpu::decoder::{OpcodeEntry, Opcode, R8, R16};
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
                let addr = mmu.read_16(self.registers.hl());
                mmu.write_8(addr, self.registers.a);
                self.registers.set_hl(self.registers.hl() + 1);
            },
            Opcode::LdPtrHLDecA => {
                let addr = mmu.read_16(self.registers.hl());
                mmu.write_8(addr, self.registers.a);
                self.registers.set_hl(self.registers.hl() - 1);
            },
            Opcode::LdAPtrHLDec => {
                let addr = mmu.read_16(self.registers.hl());
                self.registers.a = mmu.read_8(addr);
                self.registers.set_hl(self.registers.hl() - 1);
            },
            Opcode::LdAPtrHLInc => {
                let addr = mmu.read_16(self.registers.hl());
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
            Opcode::AndAR8(reg) => {},
            Opcode::AndAN8 => {},
            Opcode::Cpl => {},
            Opcode::OrAR8(reg) => {},
            Opcode::OrAN8 => {},
            Opcode::XorAR8(reg) => {},
            Opcode::XorAN8 => {},
            Opcode::Bit(n, reg) => {},
            Opcode::Set(n, reg) => {},
            Opcode::Res(n, reg) => {},
            Opcode::RlR8(reg) => {},
            Opcode::RlA => {},
            Opcode::RlcR8(reg) => {},
            Opcode::RlcA => {},
            Opcode::RrR8(reg) => {},
            Opcode::RrA => {},
            Opcode::RrcR8(reg) => {},
            Opcode::RrcA => {},
            Opcode::SlaR8(reg) => {},
            Opcode::SraR8(r8reg) => {},
            Opcode::SrlR8(reg) => {},
            Opcode::SwapR8(reg) => {},
            Opcode::CallN16 => {},
            Opcode::CallCCN16(cc) => {},
            Opcode::JpHL => {},
            Opcode::JpN16 => {
                self.registers.pc = mmu.read_16(self.registers.pc.wrapping_add(1));
                increment = false;
            },
            Opcode::JpCCN16(cc) => {},
            Opcode::JrE8 => {},
            Opcode::JrCCE8(cc) => {},
            Opcode::Ret => {},
            Opcode::RetCC(cc) => {},
            Opcode::RetI => {},
            Opcode::Rst(n) => {
                let addr = mmu.read_16(self.registers.pc.wrapping_add(1));
                self.registers.sp = self.registers.sp.wrapping_sub(2);
                mmu.write_16(self.registers.sp, addr);
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
                mmu.interrupts.ie = 0;
            },
            Opcode::Ei => {},
            Opcode::Halt => {},
            Opcode::Daa => {},
            Opcode::Nop => {
                // Nothing
            },
            Opcode::Stop => {},
            Opcode::Undefined => {},
            Opcode::Prefix => {},
        }
        if increment {
            self.increment_pc(entry.length as u16);
        }
    }
}