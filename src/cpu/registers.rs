pub struct Registers
{
    a: u8,
    f: u8, // flags
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16, // stack pointer
    pc: u16, // program counter
}

impl Registers {
    pub fn af(self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    pub fn bc(self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    pub fn de(self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    pub fn hl(self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_af_register() {
        let registers = Registers { a: 0xAB, f: 0xCD, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0 };
        assert_eq!(registers.af(), 0xABCD);
    }

    #[test]
    fn test_bc_register() {
        let registers = Registers { a: 0, f: 0, b: 0x12, c: 0x34, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0 };
        assert_eq!(registers.bc(), 0x1234);
    }

    #[test]
    fn test_de_register() {
        let registers = Registers { a: 0, f: 0, b: 0, c: 0, d: 0x56, e: 0x78, h: 0, l: 0, sp: 0, pc: 0 };
        assert_eq!(registers.de(), 0x5678);
    }

    #[test]
    fn test_hl_register() {
        let registers = Registers { a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0x9A, l: 0xBC, sp: 0, pc: 0 };
        assert_eq!(registers.hl(), 0x9ABC);
    }
}
