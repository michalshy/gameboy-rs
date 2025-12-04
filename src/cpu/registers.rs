#[derive(Clone, Copy)]
pub enum Flags {
    Z = 0b1000_0000,
    N = 0b0100_0000,
    H = 0b0010_0000,
    C = 0b0001_0000,
}

impl Flags {
    fn bit(self) -> u8 {
        self as u8
    }
}


pub struct Registers {
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
    pub fn new() -> Self {
        Registers { 
            a: 0, 
            f: 0, 
            b: 0, 
            c: 0, 
            d: 0, 
            e: 0, 
            h: 0, 
            l: 0, 
            sp: 0xFFFE, 
            pc: 0x0100 
        }
    }
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val as u8) & 0xF0; // keep lower nibble empty
    }
    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }
    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }
    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }
    pub fn set_flags(&mut self, flags: &[Flags]) {
        for flag in flags {
            self.f |= flag.bit();
        }
    }
    pub fn clear_flags(&mut self, flags: &[Flags]) {
        for flag in flags {
            self.f &= !flag.bit();
        }
    }
    pub fn get_flag(&self, flag: Flags) -> bool {
        self.f & flag.bit() != 0
    }
    pub fn clear_flags_all(&mut self) {
        self.f = 0;
    }
    pub fn reset(&mut self) { // resets to dmg state
       self.set_af(0x01B0); 
       self.set_bc(0x0013); 
       self.set_de(0x00D8); 
       self.set_hl(0x014D);
       self.sp = 0xFFFE;
       self.pc = 0x0100; 
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    // Generic registers tests
    #[test]
    fn test_af_register() {
        let mut registers = Registers::new();
        registers.a = 0xAB;
        registers.f = 0xCD;
        assert_eq!(registers.af(), 0xABCD);
    }
    #[test]
    fn test_bc_register() {
        let mut registers = Registers::new();
        registers.b = 0x12;
        registers.c = 0x34;
        assert_eq!(registers.bc(), 0x1234);
    }
    #[test]
    fn test_de_register() {
        let mut registers = Registers::new();
        registers.d = 0x56;
        registers.e = 0x78;
        assert_eq!(registers.de(), 0x5678);
    }
    #[test]
    fn test_hl_register() {
        let mut registers = Registers::new();
        registers.h = 0x9A;
        registers.l = 0xBC;
        assert_eq!(registers.hl(), 0x9ABC);
    }
    #[test]
    fn test_set_af_register() {
        let mut registers = Registers::new();
        registers.set_af(0xABCD);
        assert_eq!(registers.a, 0xAB);
        assert_eq!(registers.f, 0xC0);
    }
    #[test]
    fn test_set_bc_register() {
        let mut registers = Registers::new();
        registers.set_bc(0x1234);
        assert_eq!(registers.b, 0x12);
        assert_eq!(registers.c, 0x34);
    }

    #[test]
    fn test_set_de_register() {
        let mut registers = Registers::new();
        registers.set_de(0x5678);
        assert_eq!(registers.d, 0x56);
        assert_eq!(registers.e, 0x78);
    }
    #[test]
    fn test_set_hl_register() {
        let mut registers = Registers::new();
        registers.set_hl(0x9ABC);
        assert_eq!(registers.h, 0x9A);
        assert_eq!(registers.l, 0xBC);
    }
    #[test]
    fn test_set_flags() {
        let mut registers = Registers::new();
        registers.set_flags(&[Flags::Z, Flags::C]);
        assert_eq!(registers.f, 0b1001_0000);
    }

    #[test]
    fn test_clear_flags() {
        let mut registers = Registers::new();
        registers.f = 0xF0;
        registers.clear_flags(&[Flags::Z, Flags::N]);
        assert_eq!(registers.f, 0b0011_0000);
    }

    #[test]
    fn test_get_flag_set() {
        let mut registers = Registers::new();
        registers.set_flags(&[Flags::H]);
        assert!(registers.get_flag(Flags::H));
        assert!(!registers.get_flag(Flags::Z));
    }

    #[test]
    fn test_get_flag_clear() {
        let mut registers = Registers::new();
        registers.set_flags(&[Flags::N]);
        assert!(registers.get_flag(Flags::N));
        registers.clear_flags(&[Flags::N]);
        assert!(!registers.get_flag(Flags::N));
    }

    #[test]
    fn test_multiple_flag_operations() {
        let mut registers = Registers::new();
        registers.set_flags(&[Flags::Z, Flags::N, Flags::H, Flags::C]);
        assert!(registers.get_flag(Flags::Z));
        assert!(registers.get_flag(Flags::N));
        registers.clear_flags(&[Flags::N, Flags::H]);
        assert!(registers.get_flag(Flags::Z));
        assert!(!registers.get_flag(Flags::N));
        assert!(!registers.get_flag(Flags::H));
        assert!(registers.get_flag(Flags::C));
    }
    #[test]
    fn test_reset_to_dmg_state() {
        let mut registers = Registers::new();
        registers.a = 0xFF;
        registers.f = 0xFF;
        registers.b = 0xFF;
        registers.c = 0xFF;
        registers.d = 0xFF;
        registers.e = 0xFF;
        registers.h = 0xFF;
        registers.l = 0xFF;
        registers.sp = 0x0000;
        registers.pc = 0x0000;
        
        registers.reset();
        
        assert_eq!(registers.af(), 0x01B0);
        assert_eq!(registers.bc(), 0x0013);
        assert_eq!(registers.de(), 0x00D8);
        assert_eq!(registers.hl(), 0x014D);
        assert_eq!(registers.sp, 0xFFFE);
        assert_eq!(registers.pc, 0x0100);
    }

    #[test]
    fn test_clear_flags_all() {
        let mut registers = Registers::new();
        registers.set_flags(&[Flags::Z, Flags::N, Flags::H, Flags::C]);
        assert_eq!(registers.f, 0xF0);
        
        registers.clear_flags_all();
        
        assert_eq!(registers.f, 0);
        assert!(!registers.get_flag(Flags::Z));
        assert!(!registers.get_flag(Flags::N));
        assert!(!registers.get_flag(Flags::H));
        assert!(!registers.get_flag(Flags::C));
    }
}
