use crate::cpu::decoder::Opcode;
use crate::cpu::decoder::{CC, R8, R16};

impl R8 {
    fn name(&self) -> &'static str {
        match self {
            R8::A => "A",
            R8::B => "B",
            R8::C => "C",
            R8::D => "D",
            R8::E => "E",
            R8::H => "H",
            R8::L => "L",
            R8::HLIndirect => "(HL)",
        }
    }
}

impl R16 {
    fn name(&self) -> &'static str {
        match self {
            R16::AF => "AF",
            R16::BC => "BC",
            R16::DE => "DE",
            R16::HL => "HL",
            R16::SP => "SP",
            R16::PC => "PC",
        }
    }
}

impl CC {
    fn name(&self) -> &'static str {
        match self {
            CC::NZ => "NZ",
            CC::Z => "Z",
            CC::NC => "NC",
            CC::C => "C",
        }
    }
}

pub fn disassemble(opcode: &Opcode) -> String {
    match opcode {
        Opcode::Nop => "NOP".into(),
        Opcode::Stop => "STOP".into(),
        Opcode::Halt => "HALT".into(),
        Opcode::Prefix => "PREFIX CB".into(),
        Opcode::LdR8R8(dst, src) => format!("LD {},{}", dst.name(), src.name()),
        Opcode::LdR8N8(r) => format!("LD {},n", r.name()),
        Opcode::LdR16N16(r) => format!("LD {},nn", r.name()),
        Opcode::LdPtrR16A(r) => format!("LD ({}),A", r.name()),
        Opcode::LdPtrN16A => "LD (nn),A".into(),
        Opcode::LdAPtrR16(r) => format!("LD A,({})", r.name()),
        Opcode::LdAPtrN16 => "LD A,(nn)".into(),
        Opcode::LdHAPtrC => "LD A,(0xFF00+C)".into(),
        Opcode::LDHAPtrN8 => "LD A,(0xFF00+n)".into(),
        Opcode::LDHPtrN8A => "LD (0xFF00+n),A".into(),
        Opcode::LdPtrHLIncA => "LD (HL+),A".into(),
        Opcode::LdPtrHLDecA => "LD (HL-),A".into(),
        Opcode::LdAPtrHLInc => "LD A,(HL+)".into(),
        Opcode::LdAPtrHLDec => "LD A,(HL-)".into(),
        Opcode::AddAR8(r) => format!("ADD A,{}", r.name()),
        Opcode::AddAN8 => "ADD A,n".into(),
        Opcode::AdcAR8(r) => format!("ADC A,{}", r.name()),
        Opcode::AdcAN8 => "ADC A,n".into(),
        Opcode::SubAR8(r) => format!("SUB {}", r.name()),
        Opcode::SubAN8 => "SUB n".into(),
        Opcode::SbcAR8(r) => format!("SBC A,{}", r.name()),
        Opcode::SbcAN8 => "SBC A,n".into(),
        Opcode::CpAR8(r) => format!("CP {}", r.name()),
        Opcode::CpAN8 => "CP n".into(),
        Opcode::IncR8(r) => format!("INC {}", r.name()),
        Opcode::DecR8(r) => format!("DEC {}", r.name()),
        Opcode::AddHLR16(r) => format!("ADD HL,{}", r.name()),
        Opcode::AddHLSP => "ADD HL,SP".into(),
        Opcode::AddSPe8 => "ADD SP,e".into(),
        Opcode::IncR16(r) => format!("INC {}", r.name()),
        Opcode::DecR16(r) => format!("DEC {}", r.name()),
        Opcode::AndAR8(r) => format!("AND {}", r.name()),
        Opcode::AndAN8 => "AND n".into(),
        Opcode::OrAR8(r) => format!("OR {}", r.name()),
        Opcode::OrAN8 => "OR n".into(),
        Opcode::XorAR8(r) => format!("XOR {}", r.name()),
        Opcode::XorAN8 => "XOR n".into(),
        Opcode::Cpl => "CPL".into(),
        Opcode::Daa => "DAA".into(),
        Opcode::Bit(b, r) => format!("BIT {},{}", b, r.name()),
        Opcode::Set(b, r) => format!("SET {},{}", b, r.name()),
        Opcode::Res(b, r) => format!("RES {},{}", b, r.name()),
        Opcode::RlR8(r) => format!("RL {}", r.name()),
        Opcode::RlA => "RL A".into(),
        Opcode::RlcR8(r) => format!("RLC {}", r.name()),
        Opcode::RlcA => "RLC A".into(),
        Opcode::RrR8(r) => format!("RR {}", r.name()),
        Opcode::RrA => "RR A".into(),
        Opcode::RrcR8(r) => format!("RRC {}", r.name()),
        Opcode::RrcA => "RRC A".into(),
        Opcode::SlaR8(r) => format!("SLA {}", r.name()),
        Opcode::SraR8(r) => format!("SRA {}", r.name()),
        Opcode::SrlR8(r) => format!("SRL {}", r.name()),
        Opcode::SwapR8(r) => format!("SWAP {}", r.name()),
        Opcode::JpN16 => "JP nn".into(),
        Opcode::JpHL => "JP (HL)".into(),
        Opcode::JpCCN16(cc) => format!("JP {},nn", cc.name()),
        Opcode::JrE8 => "JR e".into(),
        Opcode::JrCCE8(cc) => format!("JR {},e", cc.name()),
        Opcode::CallN16 => "CALL nn".into(),
        Opcode::CallCCN16(cc) => format!("CALL {},nn", cc.name()),
        Opcode::Ret => "RET".into(),
        Opcode::RetI => "RETI".into(),
        Opcode::RetCC(cc) => format!("RET {}", cc.name()),
        Opcode::Rst(v) => format!("RST ${:02X}", v),
        Opcode::PopAF => "POP AF".into(),
        Opcode::PopR16(r) => format!("POP {}", r.name()),
        Opcode::PushAF => "PUSH AF".into(),
        Opcode::PushR16(r) => format!("PUSH {}", r.name()),
        Opcode::LdSPHL => "LD SP,HL".into(),
        Opcode::LdSPN16 => "LD SP,nn".into(),
        Opcode::LdHLSPe8 => "LD HL,SP+e".into(),
        Opcode::LdPtrN16SP => "LD (nn),SP".into(),
        Opcode::Scf => "SCF".into(),
        Opcode::Ccf => "CCF".into(),
        Opcode::Di => "DI".into(),
        Opcode::Ei => "EI".into(),
        Opcode::Undefined => "DB ??".into(),
        Opcode::LdHPtrCA => "LD (0xFF00+C),A".into(),
        Opcode::DecSP => "DEC SP".into(),
        Opcode::IncSP => "INC SP".into(),
    }
}
