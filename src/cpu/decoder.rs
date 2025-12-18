static LOOKUP: [OpcodeEntry; 256] = [
    // 0x00–0x1F
    OpcodeEntry {
        opcode: Opcode::Nop,
        length: 1,
        cycles: 4,
    }, // 0x00
    OpcodeEntry {
        opcode: Opcode::LdR16N16(R16::BC),
        length: 3,
        cycles: 12,
    }, // 0x01
    OpcodeEntry {
        opcode: Opcode::LdPtrR16A(R16::BC),
        length: 1,
        cycles: 8,
    }, // 0x02
    OpcodeEntry {
        opcode: Opcode::IncR16(R16::BC),
        length: 1,
        cycles: 8,
    }, // 0x03
    OpcodeEntry {
        opcode: Opcode::IncR8(R8::B),
        length: 1,
        cycles: 4,
    }, // 0x04
    OpcodeEntry {
        opcode: Opcode::DecR8(R8::B),
        length: 1,
        cycles: 4,
    }, // 0x05
    OpcodeEntry {
        opcode: Opcode::LdR8N8(R8::B),
        length: 2,
        cycles: 8,
    }, // 0x06
    OpcodeEntry {
        opcode: Opcode::RlcA,
        length: 1,
        cycles: 4,
    }, // 0x07
    OpcodeEntry {
        opcode: Opcode::LdPtrN16SP,
        length: 3,
        cycles: 20,
    }, // 0x08 (LD (nn), SP)
    OpcodeEntry {
        opcode: Opcode::AddHLR16(R16::BC),
        length: 1,
        cycles: 8,
    }, // 0x09
    OpcodeEntry {
        opcode: Opcode::LdAPtrR16(R16::BC),
        length: 1,
        cycles: 8,
    }, // 0x0A
    OpcodeEntry {
        opcode: Opcode::DecR16(R16::BC),
        length: 1,
        cycles: 8,
    }, // 0x0B
    OpcodeEntry {
        opcode: Opcode::IncR8(R8::C),
        length: 1,
        cycles: 4,
    }, // 0x0C
    OpcodeEntry {
        opcode: Opcode::DecR8(R8::C),
        length: 1,
        cycles: 4,
    }, // 0x0D
    OpcodeEntry {
        opcode: Opcode::LdR8N8(R8::C),
        length: 2,
        cycles: 8,
    }, // 0x0E
    OpcodeEntry {
        opcode: Opcode::RrcA,
        length: 1,
        cycles: 4,
    }, // 0x0F
    OpcodeEntry {
        opcode: Opcode::Stop,
        length: 2,
        cycles: 4,
    }, // 0x10
    OpcodeEntry {
        opcode: Opcode::LdR16N16(R16::DE),
        length: 3,
        cycles: 12,
    }, // 0x11
    OpcodeEntry {
        opcode: Opcode::LdPtrR16A(R16::DE),
        length: 1,
        cycles: 8,
    }, // 0x12
    OpcodeEntry {
        opcode: Opcode::IncR16(R16::DE),
        length: 1,
        cycles: 8,
    }, // 0x13
    OpcodeEntry {
        opcode: Opcode::IncR8(R8::D),
        length: 1,
        cycles: 4,
    }, // 0x14
    OpcodeEntry {
        opcode: Opcode::DecR8(R8::D),
        length: 1,
        cycles: 4,
    }, // 0x15
    OpcodeEntry {
        opcode: Opcode::LdR8N8(R8::D),
        length: 2,
        cycles: 8,
    }, // 0x16
    OpcodeEntry {
        opcode: Opcode::RlA,
        length: 1,
        cycles: 4,
    }, // 0x17
    OpcodeEntry {
        opcode: Opcode::JrE8,
        length: 2,
        cycles: 12,
    }, // 0x18
    OpcodeEntry {
        opcode: Opcode::AddHLR16(R16::DE),
        length: 1,
        cycles: 8,
    }, // 0x19
    OpcodeEntry {
        opcode: Opcode::LdAPtrR16(R16::DE),
        length: 1,
        cycles: 8,
    }, // 0x1A
    OpcodeEntry {
        opcode: Opcode::DecR16(R16::DE),
        length: 1,
        cycles: 8,
    }, // 0x1B
    OpcodeEntry {
        opcode: Opcode::IncR8(R8::E),
        length: 1,
        cycles: 4,
    }, // 0x1C
    OpcodeEntry {
        opcode: Opcode::DecR8(R8::E),
        length: 1,
        cycles: 4,
    }, // 0x1D
    OpcodeEntry {
        opcode: Opcode::LdR8N8(R8::E),
        length: 2,
        cycles: 8,
    }, // 0x1E
    OpcodeEntry {
        opcode: Opcode::RrA,
        length: 1,
        cycles: 4,
    }, // 0x1F
    // 0x20–0x3F
    OpcodeEntry {
        opcode: Opcode::JrCCE8(CC::NZ),
        length: 2,
        cycles: 8,
    }, // 0x20 JR NZ,r8 (not taken: 8)
    OpcodeEntry {
        opcode: Opcode::LdR16N16(R16::HL),
        length: 3,
        cycles: 12,
    }, // 0x21 LD HL,nn
    OpcodeEntry {
        opcode: Opcode::LdPtrHLIncA,
        length: 1,
        cycles: 8,
    }, // 0x22 LD (HL+), A
    OpcodeEntry {
        opcode: Opcode::IncR16(R16::HL),
        length: 1,
        cycles: 8,
    }, // 0x23
    OpcodeEntry {
        opcode: Opcode::IncR8(R8::H),
        length: 1,
        cycles: 4,
    }, // 0x24
    OpcodeEntry {
        opcode: Opcode::DecR8(R8::H),
        length: 1,
        cycles: 4,
    }, // 0x25
    OpcodeEntry {
        opcode: Opcode::LdR8N8(R8::H),
        length: 2,
        cycles: 8,
    }, // 0x26
    OpcodeEntry {
        opcode: Opcode::Daa,
        length: 1,
        cycles: 4,
    }, // 0x27
    OpcodeEntry {
        opcode: Opcode::JrCCE8(CC::Z),
        length: 2,
        cycles: 8,
    }, // 0x28 JR Z,r8 (not taken: 8)
    OpcodeEntry {
        opcode: Opcode::AddHLR16(R16::HL),
        length: 1,
        cycles: 8,
    }, // 0x29
    OpcodeEntry {
        opcode: Opcode::LdAPtrHLInc,
        length: 1,
        cycles: 8,
    }, // 0x2A LD A,(HL+)
    OpcodeEntry {
        opcode: Opcode::DecR16(R16::HL),
        length: 1,
        cycles: 8,
    }, // 0x2B
    OpcodeEntry {
        opcode: Opcode::IncR8(R8::L),
        length: 1,
        cycles: 4,
    }, // 0x2C
    OpcodeEntry {
        opcode: Opcode::DecR8(R8::L),
        length: 1,
        cycles: 4,
    }, // 0x2D
    OpcodeEntry {
        opcode: Opcode::LdR8N8(R8::L),
        length: 2,
        cycles: 8,
    }, // 0x2E
    OpcodeEntry {
        opcode: Opcode::Cpl,
        length: 1,
        cycles: 4,
    }, // 0x2F
    OpcodeEntry {
        opcode: Opcode::JrCCE8(CC::NC),
        length: 2,
        cycles: 8,
    }, // 0x30 JR NC,r8 (not taken: 8)
    OpcodeEntry {
        opcode: Opcode::LdSPN16,
        length: 3,
        cycles: 12,
    }, // 0x31 LD SP,nn
    OpcodeEntry {
        opcode: Opcode::LdPtrHLDecA,
        length: 1,
        cycles: 8,
    }, // 0x32 LD (HL-), A
    OpcodeEntry {
        opcode: Opcode::IncSP,
        length: 1,
        cycles: 8,
    }, // 0x33 INC SP
    OpcodeEntry {
        opcode: Opcode::IncR8(R8::HLIndirect),
        length: 1,
        cycles: 12,
    }, // 0x34 INC (HL)
    OpcodeEntry {
        opcode: Opcode::DecR8(R8::HLIndirect),
        length: 1,
        cycles: 12,
    }, // 0x35 DEC (HL)
    OpcodeEntry {
        opcode: Opcode::LdR8N8(R8::HLIndirect),
        length: 2,
        cycles: 12,
    }, // 0x36 LD (HL),n
    OpcodeEntry {
        opcode: Opcode::Scf,
        length: 1,
        cycles: 4,
    }, // 0x37
    OpcodeEntry {
        opcode: Opcode::JrCCE8(CC::C),
        length: 2,
        cycles: 8,
    }, // 0x38 JR C,r8 (not taken: 8)
    OpcodeEntry {
        opcode: Opcode::AddHLSP,
        length: 1,
        cycles: 8,
    }, // 0x39 ADD HL,SP
    OpcodeEntry {
        opcode: Opcode::LdAPtrHLDec,
        length: 1,
        cycles: 8,
    }, // 0x3A LD A,(HL-)
    OpcodeEntry {
        opcode: Opcode::DecSP,
        length: 1,
        cycles: 8,
    }, // 0x3B DEC SP
    OpcodeEntry {
        opcode: Opcode::IncR8(R8::A),
        length: 1,
        cycles: 4,
    }, // 0x3C
    OpcodeEntry {
        opcode: Opcode::DecR8(R8::A),
        length: 1,
        cycles: 4,
    }, // 0x3D
    OpcodeEntry {
        opcode: Opcode::LdR8N8(R8::A),
        length: 2,
        cycles: 8,
    }, // 0x3E
    OpcodeEntry {
        opcode: Opcode::Ccf,
        length: 1,
        cycles: 4,
    }, // 0x3F
    // 0x40–0x5F
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::B, R8::B),
        length: 1,
        cycles: 4,
    }, // 0x40
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::B, R8::C),
        length: 1,
        cycles: 4,
    }, // 0x41
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::B, R8::D),
        length: 1,
        cycles: 4,
    }, // 0x42
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::B, R8::E),
        length: 1,
        cycles: 4,
    }, // 0x43
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::B, R8::H),
        length: 1,
        cycles: 4,
    }, // 0x44
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::B, R8::L),
        length: 1,
        cycles: 4,
    }, // 0x45
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::B, R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0x46
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::B, R8::A),
        length: 1,
        cycles: 4,
    }, // 0x47
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::C, R8::B),
        length: 1,
        cycles: 4,
    }, // 0x48
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::C, R8::C),
        length: 1,
        cycles: 4,
    }, // 0x49
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::C, R8::D),
        length: 1,
        cycles: 4,
    }, // 0x4A
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::C, R8::E),
        length: 1,
        cycles: 4,
    }, // 0x4B
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::C, R8::H),
        length: 1,
        cycles: 4,
    }, // 0x4C
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::C, R8::L),
        length: 1,
        cycles: 4,
    }, // 0x4D
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::C, R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0x4E
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::C, R8::A),
        length: 1,
        cycles: 4,
    }, // 0x4F
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::D, R8::B),
        length: 1,
        cycles: 4,
    }, // 0x50
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::D, R8::C),
        length: 1,
        cycles: 4,
    }, // 0x51
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::D, R8::D),
        length: 1,
        cycles: 4,
    }, // 0x52
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::D, R8::E),
        length: 1,
        cycles: 4,
    }, // 0x53
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::D, R8::H),
        length: 1,
        cycles: 4,
    }, // 0x54
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::D, R8::L),
        length: 1,
        cycles: 4,
    }, // 0x55
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::D, R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0x56
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::D, R8::A),
        length: 1,
        cycles: 4,
    }, // 0x57
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::E, R8::B),
        length: 1,
        cycles: 4,
    }, // 0x58
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::E, R8::C),
        length: 1,
        cycles: 4,
    }, // 0x59
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::E, R8::D),
        length: 1,
        cycles: 4,
    }, // 0x5A
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::E, R8::E),
        length: 1,
        cycles: 4,
    }, // 0x5B
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::E, R8::H),
        length: 1,
        cycles: 4,
    }, // 0x5C
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::E, R8::L),
        length: 1,
        cycles: 4,
    }, // 0x5D
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::E, R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0x5E
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::E, R8::A),
        length: 1,
        cycles: 4,
    }, // 0x5F
    // 0x60–0x7F
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::H, R8::B),
        length: 1,
        cycles: 4,
    }, // 0x60
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::H, R8::C),
        length: 1,
        cycles: 4,
    }, // 0x61
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::H, R8::D),
        length: 1,
        cycles: 4,
    }, // 0x62
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::H, R8::E),
        length: 1,
        cycles: 4,
    }, // 0x63
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::H, R8::H),
        length: 1,
        cycles: 4,
    }, // 0x64
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::H, R8::L),
        length: 1,
        cycles: 4,
    }, // 0x65
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::H, R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0x66
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::H, R8::A),
        length: 1,
        cycles: 4,
    }, // 0x67
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::L, R8::B),
        length: 1,
        cycles: 4,
    }, // 0x68
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::L, R8::C),
        length: 1,
        cycles: 4,
    }, // 0x69
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::L, R8::D),
        length: 1,
        cycles: 4,
    }, // 0x6A
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::L, R8::E),
        length: 1,
        cycles: 4,
    }, // 0x6B
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::L, R8::H),
        length: 1,
        cycles: 4,
    }, // 0x6C
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::L, R8::L),
        length: 1,
        cycles: 4,
    }, // 0x6D
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::L, R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0x6E
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::L, R8::A),
        length: 1,
        cycles: 4,
    }, // 0x6F
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::HLIndirect, R8::B),
        length: 1,
        cycles: 8,
    }, // 0x70 LD (HL),B
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::HLIndirect, R8::C),
        length: 1,
        cycles: 8,
    }, // 0x71
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::HLIndirect, R8::D),
        length: 1,
        cycles: 8,
    }, // 0x72
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::HLIndirect, R8::E),
        length: 1,
        cycles: 8,
    }, // 0x73
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::HLIndirect, R8::H),
        length: 1,
        cycles: 8,
    }, // 0x74
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::HLIndirect, R8::L),
        length: 1,
        cycles: 8,
    }, // 0x75
    OpcodeEntry {
        opcode: Opcode::Halt,
        length: 1,
        cycles: 4,
    }, // 0x76 HALT
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::HLIndirect, R8::A),
        length: 1,
        cycles: 8,
    }, // 0x77
    // 0x78–0x9F (ALU ops)
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::A, R8::B),
        length: 1,
        cycles: 4,
    }, // 0x78 (LD A,B)
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::A, R8::C),
        length: 1,
        cycles: 4,
    }, // 0x79
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::A, R8::D),
        length: 1,
        cycles: 4,
    }, // 0x7A
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::A, R8::E),
        length: 1,
        cycles: 4,
    }, // 0x7B
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::A, R8::H),
        length: 1,
        cycles: 4,
    }, // 0x7C
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::A, R8::L),
        length: 1,
        cycles: 4,
    }, // 0x7D
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::A, R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0x7E
    OpcodeEntry {
        opcode: Opcode::LdR8R8(R8::A, R8::A),
        length: 1,
        cycles: 4,
    }, // 0x7F
    // 0x80–0x9F (ADD / ADC / SUB / SBC)
    OpcodeEntry {
        opcode: Opcode::AddAR8(R8::B),
        length: 1,
        cycles: 4,
    }, // 0x80
    OpcodeEntry {
        opcode: Opcode::AddAR8(R8::C),
        length: 1,
        cycles: 4,
    }, // 0x81
    OpcodeEntry {
        opcode: Opcode::AddAR8(R8::D),
        length: 1,
        cycles: 4,
    }, // 0x82
    OpcodeEntry {
        opcode: Opcode::AddAR8(R8::E),
        length: 1,
        cycles: 4,
    }, // 0x83
    OpcodeEntry {
        opcode: Opcode::AddAR8(R8::H),
        length: 1,
        cycles: 4,
    }, // 0x84
    OpcodeEntry {
        opcode: Opcode::AddAR8(R8::L),
        length: 1,
        cycles: 4,
    }, // 0x85
    OpcodeEntry {
        opcode: Opcode::AddAR8(R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0x86
    OpcodeEntry {
        opcode: Opcode::AddAR8(R8::A),
        length: 1,
        cycles: 4,
    }, // 0x87
    OpcodeEntry {
        opcode: Opcode::AdcAR8(R8::B),
        length: 1,
        cycles: 4,
    }, // 0x88
    OpcodeEntry {
        opcode: Opcode::AdcAR8(R8::C),
        length: 1,
        cycles: 4,
    }, // 0x89
    OpcodeEntry {
        opcode: Opcode::AdcAR8(R8::D),
        length: 1,
        cycles: 4,
    }, // 0x8A
    OpcodeEntry {
        opcode: Opcode::AdcAR8(R8::E),
        length: 1,
        cycles: 4,
    }, // 0x8B
    OpcodeEntry {
        opcode: Opcode::AdcAR8(R8::H),
        length: 1,
        cycles: 4,
    }, // 0x8C
    OpcodeEntry {
        opcode: Opcode::AdcAR8(R8::L),
        length: 1,
        cycles: 4,
    }, // 0x8D
    OpcodeEntry {
        opcode: Opcode::AdcAR8(R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0x8E
    OpcodeEntry {
        opcode: Opcode::AdcAR8(R8::A),
        length: 1,
        cycles: 4,
    }, // 0x8F
    OpcodeEntry {
        opcode: Opcode::SubAR8(R8::B),
        length: 1,
        cycles: 4,
    }, // 0x90
    OpcodeEntry {
        opcode: Opcode::SubAR8(R8::C),
        length: 1,
        cycles: 4,
    }, // 0x91
    OpcodeEntry {
        opcode: Opcode::SubAR8(R8::D),
        length: 1,
        cycles: 4,
    }, // 0x92
    OpcodeEntry {
        opcode: Opcode::SubAR8(R8::E),
        length: 1,
        cycles: 4,
    }, // 0x93
    OpcodeEntry {
        opcode: Opcode::SubAR8(R8::H),
        length: 1,
        cycles: 4,
    }, // 0x94
    OpcodeEntry {
        opcode: Opcode::SubAR8(R8::L),
        length: 1,
        cycles: 4,
    }, // 0x95
    OpcodeEntry {
        opcode: Opcode::SubAR8(R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0x96
    OpcodeEntry {
        opcode: Opcode::SubAR8(R8::A),
        length: 1,
        cycles: 4,
    }, // 0x97
    OpcodeEntry {
        opcode: Opcode::SbcAR8(R8::B),
        length: 1,
        cycles: 4,
    }, // 0x98
    OpcodeEntry {
        opcode: Opcode::SbcAR8(R8::C),
        length: 1,
        cycles: 4,
    }, // 0x99
    OpcodeEntry {
        opcode: Opcode::SbcAR8(R8::D),
        length: 1,
        cycles: 4,
    }, // 0x9A
    OpcodeEntry {
        opcode: Opcode::SbcAR8(R8::E),
        length: 1,
        cycles: 4,
    }, // 0x9B
    OpcodeEntry {
        opcode: Opcode::SbcAR8(R8::H),
        length: 1,
        cycles: 4,
    }, // 0x9C
    OpcodeEntry {
        opcode: Opcode::SbcAR8(R8::L),
        length: 1,
        cycles: 4,
    }, // 0x9D
    OpcodeEntry {
        opcode: Opcode::SbcAR8(R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0x9E
    OpcodeEntry {
        opcode: Opcode::SbcAR8(R8::A),
        length: 1,
        cycles: 4,
    }, // 0x9F
    // 0xA0–0xBF (AND/OR/XOR/CP)
    OpcodeEntry {
        opcode: Opcode::AndAR8(R8::B),
        length: 1,
        cycles: 4,
    }, // 0xA0
    OpcodeEntry {
        opcode: Opcode::AndAR8(R8::C),
        length: 1,
        cycles: 4,
    }, // 0xA1
    OpcodeEntry {
        opcode: Opcode::AndAR8(R8::D),
        length: 1,
        cycles: 4,
    }, // 0xA2
    OpcodeEntry {
        opcode: Opcode::AndAR8(R8::E),
        length: 1,
        cycles: 4,
    }, // 0xA3
    OpcodeEntry {
        opcode: Opcode::AndAR8(R8::H),
        length: 1,
        cycles: 4,
    }, // 0xA4
    OpcodeEntry {
        opcode: Opcode::AndAR8(R8::L),
        length: 1,
        cycles: 4,
    }, // 0xA5
    OpcodeEntry {
        opcode: Opcode::AndAR8(R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0xA6
    OpcodeEntry {
        opcode: Opcode::AndAR8(R8::A),
        length: 1,
        cycles: 4,
    }, // 0xA7
    OpcodeEntry {
        opcode: Opcode::XorAR8(R8::B),
        length: 1,
        cycles: 4,
    }, // 0xA8
    OpcodeEntry {
        opcode: Opcode::XorAR8(R8::C),
        length: 1,
        cycles: 4,
    }, // 0xA9
    OpcodeEntry {
        opcode: Opcode::XorAR8(R8::D),
        length: 1,
        cycles: 4,
    }, // 0xAA
    OpcodeEntry {
        opcode: Opcode::XorAR8(R8::E),
        length: 1,
        cycles: 4,
    }, // 0xAB
    OpcodeEntry {
        opcode: Opcode::XorAR8(R8::H),
        length: 1,
        cycles: 4,
    }, // 0xAC
    OpcodeEntry {
        opcode: Opcode::XorAR8(R8::L),
        length: 1,
        cycles: 4,
    }, // 0xAD
    OpcodeEntry {
        opcode: Opcode::XorAR8(R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0xAE
    OpcodeEntry {
        opcode: Opcode::XorAR8(R8::A),
        length: 1,
        cycles: 4,
    }, // 0xAF
    OpcodeEntry {
        opcode: Opcode::OrAR8(R8::B),
        length: 1,
        cycles: 4,
    }, // 0xB0
    OpcodeEntry {
        opcode: Opcode::OrAR8(R8::C),
        length: 1,
        cycles: 4,
    }, // 0xB1
    OpcodeEntry {
        opcode: Opcode::OrAR8(R8::D),
        length: 1,
        cycles: 4,
    }, // 0xB2
    OpcodeEntry {
        opcode: Opcode::OrAR8(R8::E),
        length: 1,
        cycles: 4,
    }, // 0xB3
    OpcodeEntry {
        opcode: Opcode::OrAR8(R8::H),
        length: 1,
        cycles: 4,
    }, // 0xB4
    OpcodeEntry {
        opcode: Opcode::OrAR8(R8::L),
        length: 1,
        cycles: 4,
    }, // 0xB5
    OpcodeEntry {
        opcode: Opcode::OrAR8(R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0xB6
    OpcodeEntry {
        opcode: Opcode::OrAR8(R8::A),
        length: 1,
        cycles: 4,
    }, // 0xB7
    OpcodeEntry {
        opcode: Opcode::CpAR8(R8::B),
        length: 1,
        cycles: 4,
    }, // 0xB8
    OpcodeEntry {
        opcode: Opcode::CpAR8(R8::C),
        length: 1,
        cycles: 4,
    }, // 0xB9
    OpcodeEntry {
        opcode: Opcode::CpAR8(R8::D),
        length: 1,
        cycles: 4,
    }, // 0xBA
    OpcodeEntry {
        opcode: Opcode::CpAR8(R8::E),
        length: 1,
        cycles: 4,
    }, // 0xBB
    OpcodeEntry {
        opcode: Opcode::CpAR8(R8::H),
        length: 1,
        cycles: 4,
    }, // 0xBC
    OpcodeEntry {
        opcode: Opcode::CpAR8(R8::L),
        length: 1,
        cycles: 4,
    }, // 0xBD
    OpcodeEntry {
        opcode: Opcode::CpAR8(R8::HLIndirect),
        length: 1,
        cycles: 8,
    }, // 0xBE
    OpcodeEntry {
        opcode: Opcode::CpAR8(R8::A),
        length: 1,
        cycles: 4,
    }, // 0xBF
    // 0xC0–0xDF (returns, pops, pushes, jumps, calls)
    OpcodeEntry {
        opcode: Opcode::RetCC(CC::NZ),
        length: 1,
        cycles: 8,
    }, // 0xC0 RET NZ (not taken: 8)
    OpcodeEntry {
        opcode: Opcode::PopR16(R16::BC),
        length: 1,
        cycles: 12,
    }, // 0xC1 POP BC
    OpcodeEntry {
        opcode: Opcode::JpCCN16(CC::NZ),
        length: 3,
        cycles: 12,
    }, // 0xC2 JP NZ,nn (not taken: 12)
    OpcodeEntry {
        opcode: Opcode::JpN16,
        length: 3,
        cycles: 16,
    }, // 0xC3 JP nn
    OpcodeEntry {
        opcode: Opcode::CallCCN16(CC::NZ),
        length: 3,
        cycles: 12,
    }, // 0xC4 CALL NZ,nn (not taken: 12)
    OpcodeEntry {
        opcode: Opcode::PushR16(R16::BC),
        length: 1,
        cycles: 16,
    }, // 0xC5 PUSH BC
    OpcodeEntry {
        opcode: Opcode::AddAN8,
        length: 2,
        cycles: 16,
    }, // 0xC6 ADD A,n8  (note: 0xC6 is ADD A,n8 - adjust if naming different)
    OpcodeEntry {
        opcode: Opcode::Rst(0x00),
        length: 1,
        cycles: 16,
    }, // 0xC7 RST 00h
    OpcodeEntry {
        opcode: Opcode::RetCC(CC::Z),
        length: 1,
        cycles: 8,
    }, // 0xC8 RET Z (not taken: 8)
    OpcodeEntry {
        opcode: Opcode::Ret,
        length: 1,
        cycles: 16,
    }, // 0xC9 RET
    OpcodeEntry {
        opcode: Opcode::JpCCN16(CC::Z),
        length: 3,
        cycles: 12,
    }, // 0xCA JP Z,nn
    OpcodeEntry {
        opcode: Opcode::Prefix,
        length: 1,
        cycles: 4,
    }, // 0xCB CB prefix (we will handle via CB_LOOKUP) - treat as a placeholder
    OpcodeEntry {
        opcode: Opcode::CallCCN16(CC::Z),
        length: 3,
        cycles: 12,
    }, // 0xCC CALL Z,nn
    OpcodeEntry {
        opcode: Opcode::CallN16,
        length: 3,
        cycles: 24,
    }, // 0xCD CALL nn
    OpcodeEntry {
        opcode: Opcode::AdcAN8,
        length: 2,
        cycles: 8,
    }, // 0xCE ADC A,n
    OpcodeEntry {
        opcode: Opcode::Rst(0x08),
        length: 1,
        cycles: 16,
    }, // 0xCF RST 08h
    OpcodeEntry {
        opcode: Opcode::RetCC(CC::NC),
        length: 1,
        cycles: 8,
    }, // 0xD0 RET NC
    OpcodeEntry {
        opcode: Opcode::PopR16(R16::DE),
        length: 1,
        cycles: 12,
    }, // 0xD1 POP DE
    OpcodeEntry {
        opcode: Opcode::JpCCN16(CC::NC),
        length: 3,
        cycles: 12,
    }, // 0xD2 JP NC,nn
    OpcodeEntry {
        opcode: Opcode::Undefined,
        length: 1,
        cycles: 4,
    }, // 0xD3 unused/undefined (placeholder)
    OpcodeEntry {
        opcode: Opcode::CallCCN16(CC::NC),
        length: 3,
        cycles: 12,
    }, // 0xD4 CALL NC,nn
    OpcodeEntry {
        opcode: Opcode::PushR16(R16::DE),
        length: 1,
        cycles: 16,
    }, // 0xD5 PUSH DE
    OpcodeEntry {
        opcode: Opcode::SubAN8,
        length: 2,
        cycles: 8,
    }, // 0xD6 SUB n
    OpcodeEntry {
        opcode: Opcode::Rst(0x10),
        length: 1,
        cycles: 16,
    }, // 0xD7 RST 10h
    OpcodeEntry {
        opcode: Opcode::RetCC(CC::C),
        length: 1,
        cycles: 8,
    }, // 0xD8 RET C
    OpcodeEntry {
        opcode: Opcode::RetI,
        length: 1,
        cycles: 16,
    }, // 0xD9 RetI
    OpcodeEntry {
        opcode: Opcode::JpCCN16(CC::C),
        length: 3,
        cycles: 12,
    }, // 0xDA JP C,nn
    OpcodeEntry {
        opcode: Opcode::Undefined,
        length: 1,
        cycles: 4,
    }, // 0xDB unused/undefined
    OpcodeEntry {
        opcode: Opcode::CallCCN16(CC::C),
        length: 3,
        cycles: 12,
    }, // 0xDC CALL C,nn
    OpcodeEntry {
        opcode: Opcode::Undefined,
        length: 1,
        cycles: 4,
    }, // 0xDD unused/undefined
    OpcodeEntry {
        opcode: Opcode::SbcAN8,
        length: 2,
        cycles: 8,
    }, // 0xDE SBC A,n
    OpcodeEntry {
        opcode: Opcode::Rst(0x18),
        length: 1,
        cycles: 16,
    }, // 0xDF RST 18h
    // 0xE0–0xFF (loads to/from high memory, interrupts, etc)
    OpcodeEntry {
        opcode: Opcode::LDHPtrN8A,
        length: 2,
        cycles: 12,
    }, // 0xE0 LDH (0xFF00+n),A
    OpcodeEntry {
        opcode: Opcode::PopR16(R16::HL),
        length: 1,
        cycles: 12,
    }, // 0xE1 POP HL
    OpcodeEntry {
        opcode: Opcode::LdHAPtrC,
        length: 1,
        cycles: 8,
    }, // 0xE2 LD (0xFF00+C), A
    OpcodeEntry {
        opcode: Opcode::Undefined,
        length: 1,
        cycles: 4,
    }, // 0xE3 unused
    OpcodeEntry {
        opcode: Opcode::Undefined,
        length: 1,
        cycles: 4,
    }, // 0xE4 unused
    OpcodeEntry {
        opcode: Opcode::PushR16(R16::HL),
        length: 1,
        cycles: 16,
    },
    OpcodeEntry {
        opcode: Opcode::AndAN8,
        length: 2,
        cycles: 8,
    }, // 0xE6 AND n
    OpcodeEntry {
        opcode: Opcode::Rst(0x20),
        length: 1,
        cycles: 16,
    }, // 0xE7 RST 20h
    OpcodeEntry {
        opcode: Opcode::AddSPe8,
        length: 2,
        cycles: 8,
    }, // 0xE8 ADD SP, e
    OpcodeEntry {
        opcode: Opcode::JpHL,
        length: 1,
        cycles: 4,
    }, // 0xE9 LD HL,SP+e (JP (HL) variant naming)
    OpcodeEntry {
        opcode: Opcode::LdPtrN16A,
        length: 3,
        cycles: 16,
    }, // 0xEA LD (nn), A? (check naming)
    OpcodeEntry {
        opcode: Opcode::Undefined,
        length: 1,
        cycles: 4,
    }, // 0xEB unused
    OpcodeEntry {
        opcode: Opcode::Undefined,
        length: 1,
        cycles: 4,
    }, // 0xEC unused
    OpcodeEntry {
        opcode: Opcode::Undefined,
        length: 1,
        cycles: 4,
    }, // 0xED unused
    OpcodeEntry {
        opcode: Opcode::XorAN8,
        length: 2,
        cycles: 8,
    }, // 0xEE XOR n
    OpcodeEntry {
        opcode: Opcode::Rst(0x28),
        length: 1,
        cycles: 16,
    }, // 0xEF RST 28h
    OpcodeEntry {
        opcode: Opcode::LDHAPtrN8,
        length: 2,
        cycles: 12,
    }, // 0xF0 LD A,(0xFF00+n)
    OpcodeEntry {
        opcode: Opcode::PopAF,
        length: 1,
        cycles: 12,
    }, // 0xF1 POP AF
    OpcodeEntry {
        opcode: Opcode::LdHAPtrC,
        length: 1,
        cycles: 8,
    }, // 0xF2 LD A,(0xFF00+C)
    OpcodeEntry {
        opcode: Opcode::Di,
        length: 1,
        cycles: 4,
    }, // 0xF3 DI
    OpcodeEntry {
        opcode: Opcode::Undefined,
        length: 1,
        cycles: 4,
    }, // 0xF4 unused
    OpcodeEntry {
        opcode: Opcode::PushAF,
        length: 1,
        cycles: 16,
    }, // 0xF5 PUSH AF
    OpcodeEntry {
        opcode: Opcode::OrAN8,
        length: 2,
        cycles: 8,
    }, // 0xF6 OR n
    OpcodeEntry {
        opcode: Opcode::Rst(0x30),
        length: 1,
        cycles: 16,
    }, // 0xF7 RST 30h
    OpcodeEntry {
        opcode: Opcode::LdHLSPe8,
        length: 2,
        cycles: 12,
    },
    OpcodeEntry {
        opcode: Opcode::LdSPHL,
        length: 1,
        cycles: 8,
    },
    OpcodeEntry {
        opcode: Opcode::LdAPtrN16,
        length: 3,
        cycles: 16,
    }, // 0xFA LD A,(nn)
    OpcodeEntry {
        opcode: Opcode::Ei,
        length: 1,
        cycles: 4,
    }, // 0xFB EI
    OpcodeEntry {
        opcode: Opcode::Undefined,
        length: 1,
        cycles: 4,
    }, // 0xFC unused
    OpcodeEntry {
        opcode: Opcode::Undefined,
        length: 1,
        cycles: 4,
    }, // 0xFD unused
    OpcodeEntry {
        opcode: Opcode::CpAN8,
        length: 2,
        cycles: 8,
    }, // 0xFE CP n
    OpcodeEntry {
        opcode: Opcode::Rst(0x38),
        length: 1,
        cycles: 16,
    }, // 0xFF RST 38h
];

static CB_LOOKUP: [OpcodeEntry; 256] = [
    // 0x00–0x1F : RLC/RRC/RL/RR/SLA/SRA/SWAP/SRL for B,C,D,E,H,L,(HL),A
    OpcodeEntry {
        opcode: Opcode::RlcR8(R8::B),
        length: 2,
        cycles: 8,
    }, // CB00
    OpcodeEntry {
        opcode: Opcode::RlcR8(R8::C),
        length: 2,
        cycles: 8,
    }, // CB01
    OpcodeEntry {
        opcode: Opcode::RlcR8(R8::D),
        length: 2,
        cycles: 8,
    }, // CB02
    OpcodeEntry {
        opcode: Opcode::RlcR8(R8::E),
        length: 2,
        cycles: 8,
    }, // CB03
    OpcodeEntry {
        opcode: Opcode::RlcR8(R8::H),
        length: 2,
        cycles: 8,
    }, // CB04
    OpcodeEntry {
        opcode: Opcode::RlcR8(R8::L),
        length: 2,
        cycles: 8,
    }, // CB05
    OpcodeEntry {
        opcode: Opcode::RlcR8(R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB06 (takes longer for (HL))
    OpcodeEntry {
        opcode: Opcode::RlcR8(R8::A),
        length: 2,
        cycles: 8,
    }, // CB07
    OpcodeEntry {
        opcode: Opcode::RrcR8(R8::B),
        length: 2,
        cycles: 8,
    }, // CB08
    OpcodeEntry {
        opcode: Opcode::RrcR8(R8::C),
        length: 2,
        cycles: 8,
    }, // CB09
    OpcodeEntry {
        opcode: Opcode::RrcR8(R8::D),
        length: 2,
        cycles: 8,
    }, // CB0A
    OpcodeEntry {
        opcode: Opcode::RrcR8(R8::E),
        length: 2,
        cycles: 8,
    }, // CB0B
    OpcodeEntry {
        opcode: Opcode::RrcR8(R8::H),
        length: 2,
        cycles: 8,
    }, // CB0C
    OpcodeEntry {
        opcode: Opcode::RrcR8(R8::L),
        length: 2,
        cycles: 8,
    }, // CB0D
    OpcodeEntry {
        opcode: Opcode::RrcR8(R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB0E
    OpcodeEntry {
        opcode: Opcode::RrcR8(R8::A),
        length: 2,
        cycles: 8,
    }, // CB0F
    OpcodeEntry {
        opcode: Opcode::RlR8(R8::B),
        length: 2,
        cycles: 8,
    }, // CB10
    OpcodeEntry {
        opcode: Opcode::RlR8(R8::C),
        length: 2,
        cycles: 8,
    }, // CB11
    OpcodeEntry {
        opcode: Opcode::RlR8(R8::D),
        length: 2,
        cycles: 8,
    }, // CB12
    OpcodeEntry {
        opcode: Opcode::RlR8(R8::E),
        length: 2,
        cycles: 8,
    }, // CB13
    OpcodeEntry {
        opcode: Opcode::RlR8(R8::H),
        length: 2,
        cycles: 8,
    }, // CB14
    OpcodeEntry {
        opcode: Opcode::RlR8(R8::L),
        length: 2,
        cycles: 8,
    }, // CB15
    OpcodeEntry {
        opcode: Opcode::RlR8(R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB16
    OpcodeEntry {
        opcode: Opcode::RlR8(R8::A),
        length: 2,
        cycles: 8,
    }, // CB17
    OpcodeEntry {
        opcode: Opcode::RrR8(R8::B),
        length: 2,
        cycles: 8,
    }, // CB18
    OpcodeEntry {
        opcode: Opcode::RrR8(R8::C),
        length: 2,
        cycles: 8,
    }, // CB19
    OpcodeEntry {
        opcode: Opcode::RrR8(R8::D),
        length: 2,
        cycles: 8,
    }, // CB1A
    OpcodeEntry {
        opcode: Opcode::RrR8(R8::E),
        length: 2,
        cycles: 8,
    }, // CB1B
    OpcodeEntry {
        opcode: Opcode::RrR8(R8::H),
        length: 2,
        cycles: 8,
    }, // CB1C
    OpcodeEntry {
        opcode: Opcode::RrR8(R8::L),
        length: 2,
        cycles: 8,
    }, // CB1D
    OpcodeEntry {
        opcode: Opcode::RrR8(R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB1E
    OpcodeEntry {
        opcode: Opcode::RrR8(R8::A),
        length: 2,
        cycles: 8,
    }, // CB1F
    // 0x20–0x3F : SLA / SRA / SWAP / SRL
    OpcodeEntry {
        opcode: Opcode::SlaR8(R8::B),
        length: 2,
        cycles: 8,
    }, // CB20
    OpcodeEntry {
        opcode: Opcode::SlaR8(R8::C),
        length: 2,
        cycles: 8,
    }, // CB21
    OpcodeEntry {
        opcode: Opcode::SlaR8(R8::D),
        length: 2,
        cycles: 8,
    }, // CB22
    OpcodeEntry {
        opcode: Opcode::SlaR8(R8::E),
        length: 2,
        cycles: 8,
    }, // CB23
    OpcodeEntry {
        opcode: Opcode::SlaR8(R8::H),
        length: 2,
        cycles: 8,
    }, // CB24
    OpcodeEntry {
        opcode: Opcode::SlaR8(R8::L),
        length: 2,
        cycles: 8,
    }, // CB25
    OpcodeEntry {
        opcode: Opcode::SlaR8(R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB26
    OpcodeEntry {
        opcode: Opcode::SlaR8(R8::A),
        length: 2,
        cycles: 8,
    }, // CB27
    OpcodeEntry {
        opcode: Opcode::SraR8(R8::B),
        length: 2,
        cycles: 8,
    }, // CB28
    OpcodeEntry {
        opcode: Opcode::SraR8(R8::C),
        length: 2,
        cycles: 8,
    }, // CB29
    OpcodeEntry {
        opcode: Opcode::SraR8(R8::D),
        length: 2,
        cycles: 8,
    }, // CB2A
    OpcodeEntry {
        opcode: Opcode::SraR8(R8::E),
        length: 2,
        cycles: 8,
    }, // CB2B
    OpcodeEntry {
        opcode: Opcode::SraR8(R8::H),
        length: 2,
        cycles: 8,
    }, // CB2C
    OpcodeEntry {
        opcode: Opcode::SraR8(R8::L),
        length: 2,
        cycles: 8,
    }, // CB2D
    OpcodeEntry {
        opcode: Opcode::SraR8(R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB2E
    OpcodeEntry {
        opcode: Opcode::SraR8(R8::A),
        length: 2,
        cycles: 8,
    }, // CB2F
    OpcodeEntry {
        opcode: Opcode::SwapR8(R8::B),
        length: 2,
        cycles: 8,
    }, // CB30
    OpcodeEntry {
        opcode: Opcode::SwapR8(R8::C),
        length: 2,
        cycles: 8,
    }, // CB31
    OpcodeEntry {
        opcode: Opcode::SwapR8(R8::D),
        length: 2,
        cycles: 8,
    }, // CB32
    OpcodeEntry {
        opcode: Opcode::SwapR8(R8::E),
        length: 2,
        cycles: 8,
    }, // CB33
    OpcodeEntry {
        opcode: Opcode::SwapR8(R8::H),
        length: 2,
        cycles: 8,
    }, // CB34
    OpcodeEntry {
        opcode: Opcode::SwapR8(R8::L),
        length: 2,
        cycles: 8,
    }, // CB35
    OpcodeEntry {
        opcode: Opcode::SwapR8(R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB36
    OpcodeEntry {
        opcode: Opcode::SwapR8(R8::A),
        length: 2,
        cycles: 8,
    }, // CB37
    OpcodeEntry {
        opcode: Opcode::SrlR8(R8::B),
        length: 2,
        cycles: 8,
    }, // CB38
    OpcodeEntry {
        opcode: Opcode::SrlR8(R8::C),
        length: 2,
        cycles: 8,
    }, // CB39
    OpcodeEntry {
        opcode: Opcode::SrlR8(R8::D),
        length: 2,
        cycles: 8,
    }, // CB3A
    OpcodeEntry {
        opcode: Opcode::SrlR8(R8::E),
        length: 2,
        cycles: 8,
    }, // CB3B
    OpcodeEntry {
        opcode: Opcode::SrlR8(R8::H),
        length: 2,
        cycles: 8,
    }, // CB3C
    OpcodeEntry {
        opcode: Opcode::SrlR8(R8::L),
        length: 2,
        cycles: 8,
    }, // CB3D
    OpcodeEntry {
        opcode: Opcode::SrlR8(R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB3E
    OpcodeEntry {
        opcode: Opcode::SrlR8(R8::A),
        length: 2,
        cycles: 8,
    }, // CB3F
    // bit 0
    OpcodeEntry {
        opcode: Opcode::Bit(0, R8::B),
        length: 2,
        cycles: 8,
    }, // CB40
    OpcodeEntry {
        opcode: Opcode::Bit(0, R8::C),
        length: 2,
        cycles: 8,
    }, // CB41
    OpcodeEntry {
        opcode: Opcode::Bit(0, R8::D),
        length: 2,
        cycles: 8,
    }, // CB42
    OpcodeEntry {
        opcode: Opcode::Bit(0, R8::E),
        length: 2,
        cycles: 8,
    }, // CB43
    OpcodeEntry {
        opcode: Opcode::Bit(0, R8::H),
        length: 2,
        cycles: 8,
    }, // CB44
    OpcodeEntry {
        opcode: Opcode::Bit(0, R8::L),
        length: 2,
        cycles: 8,
    }, // CB45
    OpcodeEntry {
        opcode: Opcode::Bit(0, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB46
    OpcodeEntry {
        opcode: Opcode::Bit(0, R8::A),
        length: 2,
        cycles: 8,
    }, // CB47
    // bit 1
    OpcodeEntry {
        opcode: Opcode::Bit(1, R8::B),
        length: 2,
        cycles: 8,
    }, // CB48
    OpcodeEntry {
        opcode: Opcode::Bit(1, R8::C),
        length: 2,
        cycles: 8,
    }, // CB49
    OpcodeEntry {
        opcode: Opcode::Bit(1, R8::D),
        length: 2,
        cycles: 8,
    }, // CB4A
    OpcodeEntry {
        opcode: Opcode::Bit(1, R8::E),
        length: 2,
        cycles: 8,
    }, // CB4B
    OpcodeEntry {
        opcode: Opcode::Bit(1, R8::H),
        length: 2,
        cycles: 8,
    }, // CB4C
    OpcodeEntry {
        opcode: Opcode::Bit(1, R8::L),
        length: 2,
        cycles: 8,
    }, // CB4D
    OpcodeEntry {
        opcode: Opcode::Bit(1, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB4E
    OpcodeEntry {
        opcode: Opcode::Bit(1, R8::A),
        length: 2,
        cycles: 8,
    }, // CB4F
    // bit 2
    OpcodeEntry {
        opcode: Opcode::Bit(2, R8::B),
        length: 2,
        cycles: 8,
    }, // CB50
    OpcodeEntry {
        opcode: Opcode::Bit(2, R8::C),
        length: 2,
        cycles: 8,
    }, // CB51
    OpcodeEntry {
        opcode: Opcode::Bit(2, R8::D),
        length: 2,
        cycles: 8,
    }, // CB52
    OpcodeEntry {
        opcode: Opcode::Bit(2, R8::E),
        length: 2,
        cycles: 8,
    }, // CB53
    OpcodeEntry {
        opcode: Opcode::Bit(2, R8::H),
        length: 2,
        cycles: 8,
    }, // CB54
    OpcodeEntry {
        opcode: Opcode::Bit(2, R8::L),
        length: 2,
        cycles: 8,
    }, // CB55
    OpcodeEntry {
        opcode: Opcode::Bit(2, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB56
    OpcodeEntry {
        opcode: Opcode::Bit(2, R8::A),
        length: 2,
        cycles: 8,
    }, // CB57
    // bit 3
    OpcodeEntry {
        opcode: Opcode::Bit(3, R8::B),
        length: 2,
        cycles: 8,
    }, // CB58
    OpcodeEntry {
        opcode: Opcode::Bit(3, R8::C),
        length: 2,
        cycles: 8,
    }, // CB59
    OpcodeEntry {
        opcode: Opcode::Bit(3, R8::D),
        length: 2,
        cycles: 8,
    }, // CB5A
    OpcodeEntry {
        opcode: Opcode::Bit(3, R8::E),
        length: 2,
        cycles: 8,
    }, // CB5B
    OpcodeEntry {
        opcode: Opcode::Bit(3, R8::H),
        length: 2,
        cycles: 8,
    }, // CB5C
    OpcodeEntry {
        opcode: Opcode::Bit(3, R8::L),
        length: 2,
        cycles: 8,
    }, // CB5D
    OpcodeEntry {
        opcode: Opcode::Bit(3, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB5E
    OpcodeEntry {
        opcode: Opcode::Bit(3, R8::A),
        length: 2,
        cycles: 8,
    }, // CB5F
    // 0x60–0x7F continued (bits 4..7)
    OpcodeEntry {
        opcode: Opcode::Bit(4, R8::B),
        length: 2,
        cycles: 8,
    }, // CB60
    OpcodeEntry {
        opcode: Opcode::Bit(4, R8::C),
        length: 2,
        cycles: 8,
    }, // CB61
    OpcodeEntry {
        opcode: Opcode::Bit(4, R8::D),
        length: 2,
        cycles: 8,
    }, // CB62
    OpcodeEntry {
        opcode: Opcode::Bit(4, R8::E),
        length: 2,
        cycles: 8,
    }, // CB63
    OpcodeEntry {
        opcode: Opcode::Bit(4, R8::H),
        length: 2,
        cycles: 8,
    }, // CB64
    OpcodeEntry {
        opcode: Opcode::Bit(4, R8::L),
        length: 2,
        cycles: 8,
    }, // CB65
    OpcodeEntry {
        opcode: Opcode::Bit(4, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB66
    OpcodeEntry {
        opcode: Opcode::Bit(4, R8::A),
        length: 2,
        cycles: 8,
    }, // CB67
    OpcodeEntry {
        opcode: Opcode::Bit(5, R8::B),
        length: 2,
        cycles: 8,
    }, // CB68
    OpcodeEntry {
        opcode: Opcode::Bit(5, R8::C),
        length: 2,
        cycles: 8,
    }, // CB69
    OpcodeEntry {
        opcode: Opcode::Bit(5, R8::D),
        length: 2,
        cycles: 8,
    }, // CB6A
    OpcodeEntry {
        opcode: Opcode::Bit(5, R8::E),
        length: 2,
        cycles: 8,
    }, // CB6B
    OpcodeEntry {
        opcode: Opcode::Bit(5, R8::H),
        length: 2,
        cycles: 8,
    }, // CB6C
    OpcodeEntry {
        opcode: Opcode::Bit(5, R8::L),
        length: 2,
        cycles: 8,
    }, // CB6D
    OpcodeEntry {
        opcode: Opcode::Bit(5, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB6E
    OpcodeEntry {
        opcode: Opcode::Bit(5, R8::A),
        length: 2,
        cycles: 8,
    }, // CB6F
    OpcodeEntry {
        opcode: Opcode::Bit(6, R8::B),
        length: 2,
        cycles: 8,
    }, // CB70
    OpcodeEntry {
        opcode: Opcode::Bit(6, R8::C),
        length: 2,
        cycles: 8,
    }, // CB71
    OpcodeEntry {
        opcode: Opcode::Bit(6, R8::D),
        length: 2,
        cycles: 8,
    }, // CB72
    OpcodeEntry {
        opcode: Opcode::Bit(6, R8::E),
        length: 2,
        cycles: 8,
    }, // CB73
    OpcodeEntry {
        opcode: Opcode::Bit(6, R8::H),
        length: 2,
        cycles: 8,
    }, // CB74
    OpcodeEntry {
        opcode: Opcode::Bit(6, R8::L),
        length: 2,
        cycles: 8,
    }, // CB75
    OpcodeEntry {
        opcode: Opcode::Bit(6, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB76
    OpcodeEntry {
        opcode: Opcode::Bit(6, R8::A),
        length: 2,
        cycles: 8,
    }, // CB77
    OpcodeEntry {
        opcode: Opcode::Bit(7, R8::B),
        length: 2,
        cycles: 8,
    }, // CB78
    OpcodeEntry {
        opcode: Opcode::Bit(7, R8::C),
        length: 2,
        cycles: 8,
    }, // CB79
    OpcodeEntry {
        opcode: Opcode::Bit(7, R8::D),
        length: 2,
        cycles: 8,
    }, // CB7A
    OpcodeEntry {
        opcode: Opcode::Bit(7, R8::E),
        length: 2,
        cycles: 8,
    }, // CB7B
    OpcodeEntry {
        opcode: Opcode::Bit(7, R8::H),
        length: 2,
        cycles: 8,
    }, // CB7C
    OpcodeEntry {
        opcode: Opcode::Bit(7, R8::L),
        length: 2,
        cycles: 8,
    }, // CB7D
    OpcodeEntry {
        opcode: Opcode::Bit(7, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB7E
    OpcodeEntry {
        opcode: Opcode::Bit(7, R8::A),
        length: 2,
        cycles: 8,
    }, // CB7F
    // bit 0
    OpcodeEntry {
        opcode: Opcode::Res(0, R8::B),
        length: 2,
        cycles: 8,
    }, // CB80
    OpcodeEntry {
        opcode: Opcode::Res(0, R8::C),
        length: 2,
        cycles: 8,
    }, // CB81
    OpcodeEntry {
        opcode: Opcode::Res(0, R8::D),
        length: 2,
        cycles: 8,
    }, // CB82
    OpcodeEntry {
        opcode: Opcode::Res(0, R8::E),
        length: 2,
        cycles: 8,
    }, // CB83
    OpcodeEntry {
        opcode: Opcode::Res(0, R8::H),
        length: 2,
        cycles: 8,
    }, // CB84
    OpcodeEntry {
        opcode: Opcode::Res(0, R8::L),
        length: 2,
        cycles: 8,
    }, // CB85
    OpcodeEntry {
        opcode: Opcode::Res(0, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB86
    OpcodeEntry {
        opcode: Opcode::Res(0, R8::A),
        length: 2,
        cycles: 8,
    }, // CB87
    // bit 1
    OpcodeEntry {
        opcode: Opcode::Res(1, R8::B),
        length: 2,
        cycles: 8,
    }, // CB88
    OpcodeEntry {
        opcode: Opcode::Res(1, R8::C),
        length: 2,
        cycles: 8,
    }, // CB89
    OpcodeEntry {
        opcode: Opcode::Res(1, R8::D),
        length: 2,
        cycles: 8,
    }, // CB8A
    OpcodeEntry {
        opcode: Opcode::Res(1, R8::E),
        length: 2,
        cycles: 8,
    }, // CB8B
    OpcodeEntry {
        opcode: Opcode::Res(1, R8::H),
        length: 2,
        cycles: 8,
    }, // CB8C
    OpcodeEntry {
        opcode: Opcode::Res(1, R8::L),
        length: 2,
        cycles: 8,
    }, // CB8D
    OpcodeEntry {
        opcode: Opcode::Res(1, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB8E
    OpcodeEntry {
        opcode: Opcode::Res(1, R8::A),
        length: 2,
        cycles: 8,
    }, // CB8F
    // bit 2
    OpcodeEntry {
        opcode: Opcode::Res(2, R8::B),
        length: 2,
        cycles: 8,
    }, // CB90
    OpcodeEntry {
        opcode: Opcode::Res(2, R8::C),
        length: 2,
        cycles: 8,
    }, // CB91
    OpcodeEntry {
        opcode: Opcode::Res(2, R8::D),
        length: 2,
        cycles: 8,
    }, // CB92
    OpcodeEntry {
        opcode: Opcode::Res(2, R8::E),
        length: 2,
        cycles: 8,
    }, // CB93
    OpcodeEntry {
        opcode: Opcode::Res(2, R8::H),
        length: 2,
        cycles: 8,
    }, // CB94
    OpcodeEntry {
        opcode: Opcode::Res(2, R8::L),
        length: 2,
        cycles: 8,
    }, // CB95
    OpcodeEntry {
        opcode: Opcode::Res(2, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB96
    OpcodeEntry {
        opcode: Opcode::Res(2, R8::A),
        length: 2,
        cycles: 8,
    }, // CB97
    // bit 3
    OpcodeEntry {
        opcode: Opcode::Res(3, R8::B),
        length: 2,
        cycles: 8,
    }, // CB98
    OpcodeEntry {
        opcode: Opcode::Res(3, R8::C),
        length: 2,
        cycles: 8,
    }, // CB99
    OpcodeEntry {
        opcode: Opcode::Res(3, R8::D),
        length: 2,
        cycles: 8,
    }, // CB9A
    OpcodeEntry {
        opcode: Opcode::Res(3, R8::E),
        length: 2,
        cycles: 8,
    }, // CB9B
    OpcodeEntry {
        opcode: Opcode::Res(3, R8::H),
        length: 2,
        cycles: 8,
    }, // CB9C
    OpcodeEntry {
        opcode: Opcode::Res(3, R8::L),
        length: 2,
        cycles: 8,
    }, // CB9D
    OpcodeEntry {
        opcode: Opcode::Res(3, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CB9E
    OpcodeEntry {
        opcode: Opcode::Res(3, R8::A),
        length: 2,
        cycles: 8,
    }, // CB9F
    // 0xA0–0xBF (RES bits 4..7)
    OpcodeEntry {
        opcode: Opcode::Res(4, R8::B),
        length: 2,
        cycles: 8,
    }, // CBA0
    OpcodeEntry {
        opcode: Opcode::Res(4, R8::C),
        length: 2,
        cycles: 8,
    }, // CBA1
    OpcodeEntry {
        opcode: Opcode::Res(4, R8::D),
        length: 2,
        cycles: 8,
    }, // CBA2
    OpcodeEntry {
        opcode: Opcode::Res(4, R8::E),
        length: 2,
        cycles: 8,
    }, // CBA3
    OpcodeEntry {
        opcode: Opcode::Res(4, R8::H),
        length: 2,
        cycles: 8,
    }, // CBA4
    OpcodeEntry {
        opcode: Opcode::Res(4, R8::L),
        length: 2,
        cycles: 8,
    }, // CBA5
    OpcodeEntry {
        opcode: Opcode::Res(4, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CBA6
    OpcodeEntry {
        opcode: Opcode::Res(4, R8::A),
        length: 2,
        cycles: 8,
    }, // CBA7
    OpcodeEntry {
        opcode: Opcode::Res(5, R8::B),
        length: 2,
        cycles: 8,
    }, // CBA8
    OpcodeEntry {
        opcode: Opcode::Res(5, R8::C),
        length: 2,
        cycles: 8,
    }, // CBA9
    OpcodeEntry {
        opcode: Opcode::Res(5, R8::D),
        length: 2,
        cycles: 8,
    }, // CBAA
    OpcodeEntry {
        opcode: Opcode::Res(5, R8::E),
        length: 2,
        cycles: 8,
    }, // CBAB
    OpcodeEntry {
        opcode: Opcode::Res(5, R8::H),
        length: 2,
        cycles: 8,
    }, // CBAC
    OpcodeEntry {
        opcode: Opcode::Res(5, R8::L),
        length: 2,
        cycles: 8,
    }, // CBAD
    OpcodeEntry {
        opcode: Opcode::Res(5, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CBAE
    OpcodeEntry {
        opcode: Opcode::Res(5, R8::A),
        length: 2,
        cycles: 8,
    }, // CBAF
    OpcodeEntry {
        opcode: Opcode::Res(6, R8::B),
        length: 2,
        cycles: 8,
    }, // CBB0
    OpcodeEntry {
        opcode: Opcode::Res(6, R8::C),
        length: 2,
        cycles: 8,
    }, // CBB1
    OpcodeEntry {
        opcode: Opcode::Res(6, R8::D),
        length: 2,
        cycles: 8,
    }, // CBB2
    OpcodeEntry {
        opcode: Opcode::Res(6, R8::E),
        length: 2,
        cycles: 8,
    }, // CBB3
    OpcodeEntry {
        opcode: Opcode::Res(6, R8::H),
        length: 2,
        cycles: 8,
    }, // CBB4
    OpcodeEntry {
        opcode: Opcode::Res(6, R8::L),
        length: 2,
        cycles: 8,
    }, // CBB5
    OpcodeEntry {
        opcode: Opcode::Res(6, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CBB6
    OpcodeEntry {
        opcode: Opcode::Res(6, R8::A),
        length: 2,
        cycles: 8,
    }, // CBB7
    OpcodeEntry {
        opcode: Opcode::Res(7, R8::B),
        length: 2,
        cycles: 8,
    }, // CBB8
    OpcodeEntry {
        opcode: Opcode::Res(7, R8::C),
        length: 2,
        cycles: 8,
    }, // CBB9
    OpcodeEntry {
        opcode: Opcode::Res(7, R8::D),
        length: 2,
        cycles: 8,
    }, // CBBA
    OpcodeEntry {
        opcode: Opcode::Res(7, R8::E),
        length: 2,
        cycles: 8,
    }, // CBBB
    OpcodeEntry {
        opcode: Opcode::Res(7, R8::H),
        length: 2,
        cycles: 8,
    }, // CBBC
    OpcodeEntry {
        opcode: Opcode::Res(7, R8::L),
        length: 2,
        cycles: 8,
    }, // CBBD
    OpcodeEntry {
        opcode: Opcode::Res(7, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CBBE
    OpcodeEntry {
        opcode: Opcode::Res(7, R8::A),
        length: 2,
        cycles: 8,
    }, // CBBF
    // 0xC0–0xFF : SET b,r (set bit b)
    OpcodeEntry {
        opcode: Opcode::Set(0, R8::B),
        length: 2,
        cycles: 8,
    }, // CBC0
    OpcodeEntry {
        opcode: Opcode::Set(0, R8::C),
        length: 2,
        cycles: 8,
    }, // CBC1
    OpcodeEntry {
        opcode: Opcode::Set(0, R8::D),
        length: 2,
        cycles: 8,
    }, // CBC2
    OpcodeEntry {
        opcode: Opcode::Set(0, R8::E),
        length: 2,
        cycles: 8,
    }, // CBC3
    OpcodeEntry {
        opcode: Opcode::Set(0, R8::H),
        length: 2,
        cycles: 8,
    }, // CBC4
    OpcodeEntry {
        opcode: Opcode::Set(0, R8::L),
        length: 2,
        cycles: 8,
    }, // CBC5
    OpcodeEntry {
        opcode: Opcode::Set(0, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CBC6
    OpcodeEntry {
        opcode: Opcode::Set(0, R8::A),
        length: 2,
        cycles: 8,
    }, // CBC7
    OpcodeEntry {
        opcode: Opcode::Set(1, R8::B),
        length: 2,
        cycles: 8,
    }, // CBC8
    OpcodeEntry {
        opcode: Opcode::Set(1, R8::C),
        length: 2,
        cycles: 8,
    }, // CBC9
    OpcodeEntry {
        opcode: Opcode::Set(1, R8::D),
        length: 2,
        cycles: 8,
    }, // CBCA
    OpcodeEntry {
        opcode: Opcode::Set(1, R8::E),
        length: 2,
        cycles: 8,
    }, // CBCB
    OpcodeEntry {
        opcode: Opcode::Set(1, R8::H),
        length: 2,
        cycles: 8,
    }, // CBCC
    OpcodeEntry {
        opcode: Opcode::Set(1, R8::L),
        length: 2,
        cycles: 8,
    }, // CBCD
    OpcodeEntry {
        opcode: Opcode::Set(1, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CBCE
    OpcodeEntry {
        opcode: Opcode::Set(1, R8::A),
        length: 2,
        cycles: 8,
    }, // CBCF
    OpcodeEntry {
        opcode: Opcode::Set(2, R8::B),
        length: 2,
        cycles: 8,
    }, // CBD0
    OpcodeEntry {
        opcode: Opcode::Set(2, R8::C),
        length: 2,
        cycles: 8,
    }, // CBD1
    OpcodeEntry {
        opcode: Opcode::Set(2, R8::D),
        length: 2,
        cycles: 8,
    }, // CBD2
    OpcodeEntry {
        opcode: Opcode::Set(2, R8::E),
        length: 2,
        cycles: 8,
    }, // CBD3
    OpcodeEntry {
        opcode: Opcode::Set(2, R8::H),
        length: 2,
        cycles: 8,
    }, // CBD4
    OpcodeEntry {
        opcode: Opcode::Set(2, R8::L),
        length: 2,
        cycles: 8,
    }, // CBD5
    OpcodeEntry {
        opcode: Opcode::Set(2, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CBD6
    OpcodeEntry {
        opcode: Opcode::Set(2, R8::A),
        length: 2,
        cycles: 8,
    }, // CBD7
    OpcodeEntry {
        opcode: Opcode::Set(3, R8::B),
        length: 2,
        cycles: 8,
    }, // CBD8
    OpcodeEntry {
        opcode: Opcode::Set(3, R8::C),
        length: 2,
        cycles: 8,
    }, // CBD9
    OpcodeEntry {
        opcode: Opcode::Set(3, R8::D),
        length: 2,
        cycles: 8,
    }, // CBDA
    OpcodeEntry {
        opcode: Opcode::Set(3, R8::E),
        length: 2,
        cycles: 8,
    }, // CBDB
    OpcodeEntry {
        opcode: Opcode::Set(3, R8::H),
        length: 2,
        cycles: 8,
    }, // CBDC
    OpcodeEntry {
        opcode: Opcode::Set(3, R8::L),
        length: 2,
        cycles: 8,
    }, // CBDD
    OpcodeEntry {
        opcode: Opcode::Set(3, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CBDE
    OpcodeEntry {
        opcode: Opcode::Set(3, R8::A),
        length: 2,
        cycles: 8,
    }, // CBDF
    OpcodeEntry {
        opcode: Opcode::Set(4, R8::B),
        length: 2,
        cycles: 8,
    }, // CBE0
    OpcodeEntry {
        opcode: Opcode::Set(4, R8::C),
        length: 2,
        cycles: 8,
    }, // CBE1
    OpcodeEntry {
        opcode: Opcode::Set(4, R8::D),
        length: 2,
        cycles: 8,
    }, // CBE2
    OpcodeEntry {
        opcode: Opcode::Set(4, R8::E),
        length: 2,
        cycles: 8,
    }, // CBE3
    OpcodeEntry {
        opcode: Opcode::Set(4, R8::H),
        length: 2,
        cycles: 8,
    }, // CBE4
    OpcodeEntry {
        opcode: Opcode::Set(4, R8::L),
        length: 2,
        cycles: 8,
    }, // CBE5
    OpcodeEntry {
        opcode: Opcode::Set(4, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CBE6
    OpcodeEntry {
        opcode: Opcode::Set(4, R8::A),
        length: 2,
        cycles: 8,
    }, // CBE7
    OpcodeEntry {
        opcode: Opcode::Set(5, R8::B),
        length: 2,
        cycles: 8,
    }, // CBE8
    OpcodeEntry {
        opcode: Opcode::Set(5, R8::C),
        length: 2,
        cycles: 8,
    }, // CBE9
    OpcodeEntry {
        opcode: Opcode::Set(5, R8::D),
        length: 2,
        cycles: 8,
    }, // CBEA
    OpcodeEntry {
        opcode: Opcode::Set(5, R8::E),
        length: 2,
        cycles: 8,
    }, // CBEB
    OpcodeEntry {
        opcode: Opcode::Set(5, R8::H),
        length: 2,
        cycles: 8,
    }, // CBEC
    OpcodeEntry {
        opcode: Opcode::Set(5, R8::L),
        length: 2,
        cycles: 8,
    }, // CBED
    OpcodeEntry {
        opcode: Opcode::Set(5, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CBEE
    OpcodeEntry {
        opcode: Opcode::Set(5, R8::A),
        length: 2,
        cycles: 8,
    }, // CBEF
    OpcodeEntry {
        opcode: Opcode::Set(6, R8::B),
        length: 2,
        cycles: 8,
    }, // CBF0
    OpcodeEntry {
        opcode: Opcode::Set(6, R8::C),
        length: 2,
        cycles: 8,
    }, // CBF1
    OpcodeEntry {
        opcode: Opcode::Set(6, R8::D),
        length: 2,
        cycles: 8,
    }, // CBF2
    OpcodeEntry {
        opcode: Opcode::Set(6, R8::E),
        length: 2,
        cycles: 8,
    }, // CBF3
    OpcodeEntry {
        opcode: Opcode::Set(6, R8::H),
        length: 2,
        cycles: 8,
    }, // CBF4
    OpcodeEntry {
        opcode: Opcode::Set(6, R8::L),
        length: 2,
        cycles: 8,
    }, // CBF5
    OpcodeEntry {
        opcode: Opcode::Set(6, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CBF6
    OpcodeEntry {
        opcode: Opcode::Set(6, R8::A),
        length: 2,
        cycles: 8,
    }, // CBF7
    OpcodeEntry {
        opcode: Opcode::Set(7, R8::B),
        length: 2,
        cycles: 8,
    }, // CBF8
    OpcodeEntry {
        opcode: Opcode::Set(7, R8::C),
        length: 2,
        cycles: 8,
    }, // CBF9
    OpcodeEntry {
        opcode: Opcode::Set(7, R8::D),
        length: 2,
        cycles: 8,
    }, // CBFA
    OpcodeEntry {
        opcode: Opcode::Set(7, R8::E),
        length: 2,
        cycles: 8,
    }, // CBFB
    OpcodeEntry {
        opcode: Opcode::Set(7, R8::H),
        length: 2,
        cycles: 8,
    }, // CBFC
    OpcodeEntry {
        opcode: Opcode::Set(7, R8::L),
        length: 2,
        cycles: 8,
    }, // CBFD
    OpcodeEntry {
        opcode: Opcode::Set(7, R8::HLIndirect),
        length: 2,
        cycles: 16,
    }, // CBFE
    OpcodeEntry {
        opcode: Opcode::Set(7, R8::A),
        length: 2,
        cycles: 8,
    }, // CBFF
];

pub enum R8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLIndirect,
}

#[allow(clippy::enum_variant_names)]
#[allow(dead_code)]
pub enum R16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

pub enum CC {
    NZ,
    Z,
    NC,
    C,
}

#[allow(clippy::enum_variant_names)]
#[allow(dead_code)]
pub enum Opcode {
    // Load
    LdR8R8(R8, R8), // load value in right reg to left reg
    LdR8N8(R8),     // load value to reg
    LdR16N16(R16),  // load value in right reg to left reg
    LdPtrR16A(R16),
    LdPtrN16A,
    LdHPtrCA,       // loads a to byte pointer by 0xff00 + c
    LdAPtrR16(R16), // loads value from address reg into A
    LdAPtrN16,      // loads value from address val into A
    LdHAPtrC,
    LDHAPtrN8,
    LDHPtrN8A,
    LdPtrHLIncA, // increment
    LdPtrHLDecA, // decrement
    LdAPtrHLDec,
    LdAPtrHLInc,
    // 8-bit arithmetic
    AdcAR8(R8),
    AdcAN8,
    AddAR8(R8),
    AddAN8,
    CpAR8(R8),
    CpAN8,
    DecR8(R8),
    IncR8(R8),
    SbcAR8(R8),
    SbcAN8,
    SubAR8(R8),
    SubAN8,
    // 16-bit arithmetic
    AddHLR16(R16),
    DecR16(R16),
    IncR16(R16),
    // Bitwise logic
    AndAR8(R8),
    AndAN8,
    Cpl,
    OrAR8(R8),
    OrAN8,
    XorAR8(R8),
    XorAN8,
    // Bit flag
    Bit(u8, R8),
    Set(u8, R8),
    Res(u8, R8),
    // Bit shift
    RlR8(R8),
    RlA,
    RlcR8(R8),
    RlcA,
    RrR8(R8),
    RrA,
    RrcR8(R8),
    RrcA,
    SlaR8(R8),
    SraR8(R8),
    SrlR8(R8),
    SwapR8(R8),
    // Jump / coroutine
    CallN16,
    CallCCN16(CC),
    JpHL,
    JpN16,
    JpCCN16(CC),
    JrE8,
    JrCCE8(CC),
    Ret,
    RetCC(CC),
    RetI,
    Rst(u8),
    // Carry flag
    Scf,
    Ccf,
    // Stack manipulation
    AddHLSP,
    AddSPe8,
    LdHLSPe8, // add signed e8 to SP and result in HL
    DecSP,
    IncSP,
    LdSPN16,
    LdPtrN16SP,
    LdSPHL,
    PopAF,
    PopR16(R16),
    PushAF,
    PushR16(R16),
    // Interrupts
    Di,
    Ei,
    Halt,
    // Misc
    Daa,
    Nop,
    Stop,
    // Undefined
    Undefined,
    // Prefix
    Prefix,
}

pub struct OpcodeEntry {
    pub opcode: Opcode,
    pub length: u8,
    pub cycles: u8,
}

pub fn decode(byte: u8) -> &'static OpcodeEntry {
    &LOOKUP[byte as usize]
}

pub fn decode_cb(byte: u8) -> &'static OpcodeEntry {
    &CB_LOOKUP[byte as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_nop_entry() {
        let entry = decode(0x00);
        assert!(matches!(
            entry,
            &OpcodeEntry {
                opcode: Opcode::Nop,
                length: 1,
                cycles: 4
            }
        ));
    }

    #[test]
    fn decode_prefix_and_cb_table_entries() {
        // 0xCB in the main table is the prefix placeholder
        let pref = decode(0xCB);
        assert!(matches!(
            pref,
            &OpcodeEntry {
                opcode: Opcode::Prefix,
                length: 1,
                cycles: 4
            }
        ));

        // Check a few CB table entries directly
        // CB07 -> RLC A
        assert!(matches!(
            &CB_LOOKUP[0x07],
            &OpcodeEntry {
                opcode: Opcode::RlcR8(R8::A),
                length: 2,
                cycles: 8
            }
        ));

        // CB06 -> RLC (HL) (longer cycles)
        assert!(matches!(
            &CB_LOOKUP[0x06],
            &OpcodeEntry {
                opcode: Opcode::RlcR8(R8::HLIndirect),
                length: 2,
                cycles: 16
            }
        ));

        // CB47 -> BIT 0, A
        assert!(matches!(
            &CB_LOOKUP[0x47],
            &OpcodeEntry {
                opcode: Opcode::Bit(0, R8::A),
                length: 2,
                cycles: 8
            }
        ));
    }

    #[test]
    fn decode_loads_and_halt() {
        // LD (HL),A+ related checks
        let e2 = decode(0x22); // LD (HL+), A
        assert!(matches!(
            e2,
            &OpcodeEntry {
                opcode: Opcode::LdPtrHLIncA,
                length: 1,
                cycles: 8
            }
        ));

        // HALT opcode at 0x76
        let halt = decode(0x76);
        assert!(matches!(
            halt,
            &OpcodeEntry {
                opcode: Opcode::Halt,
                length: 1,
                cycles: 4
            }
        ));
    }

    #[test]
    fn decode_alu_and_misc() {
        // 0x86 is ADD A, (HL) in the table
        let add_hl = decode(0x86);
        assert!(matches!(
            add_hl,
            &OpcodeEntry {
                opcode: Opcode::AddAR8(R8::HLIndirect),
                length: 1,
                cycles: 8
            }
        ));

        // 0xC3 is unconditional JP nn
        let jp = decode(0xC3);
        assert!(matches!(
            jp,
            &OpcodeEntry {
                opcode: Opcode::JpN16,
                length: 3,
                cycles: 16
            }
        ));

        // 0xE2 is the LD (0xFF00+C), A variant in the table
        let e2 = decode(0xE2);
        assert!(matches!(
            e2,
            &OpcodeEntry {
                opcode: Opcode::LdHAPtrC,
                length: 1,
                cycles: 8
            }
        ));
    }
}
