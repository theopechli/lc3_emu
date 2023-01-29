#[repr(u16)]
pub enum Register {
    Rr0 = 0,
    Rr1,
    Rr2,
    Rr3,
    Rr4,
    Rr5,
    Rr6,
    Rr7,
    Rpc,
    Rcond,
}

impl TryFrom<u16> for Register {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == Register::Rr0 as u16 => Ok(Register::Rr0),
            x if x == Register::Rr1 as u16 => Ok(Register::Rr1),
            x if x == Register::Rr2 as u16 => Ok(Register::Rr2),
            x if x == Register::Rr3 as u16 => Ok(Register::Rr3),
            x if x == Register::Rr4 as u16 => Ok(Register::Rr4),
            x if x == Register::Rr5 as u16 => Ok(Register::Rr5),
            x if x == Register::Rr6 as u16 => Ok(Register::Rr6),
            x if x == Register::Rr7 as u16 => Ok(Register::Rr7),
            x if x == Register::Rpc as u16 => Ok(Register::Rpc),
            x if x == Register::Rcond as u16 => Ok(Register::Rcond),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Registers {
    r_r0: u16,
    r_r1: u16,
    r_r2: u16,
    r_r3: u16,
    r_r4: u16,
    r_r5: u16,
    r_r6: u16,
    r_r7: u16,
    r_pc: u16,
    r_cond: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            r_r0: 0,
            r_r1: 0,
            r_r2: 0,
            r_r3: 0,
            r_r4: 0,
            r_r5: 0,
            r_r6: 0,
            r_r7: 0,
            r_pc: 0,
            r_cond: 0,
        }
    }

    pub fn update(&mut self, reg: Register, value: u16) {
        match reg {
            Register::Rr0 => self.r_r0 = value,
            Register::Rr1 => self.r_r1 = value,
            Register::Rr2 => self.r_r2 = value,
            Register::Rr3 => self.r_r3 = value,
            Register::Rr4 => self.r_r4 = value,
            Register::Rr5 => self.r_r5 = value,
            Register::Rr6 => self.r_r6 = value,
            Register::Rr7 => self.r_r7 = value,
            Register::Rpc => self.r_pc = value,
            Register::Rcond => self.r_cond = value,
        }
    }

    pub fn get_value(self, reg: Register) -> u16 {
        match reg {
            Register::Rr0 => self.r_r0,
            Register::Rr1 => self.r_r1,
            Register::Rr2 => self.r_r2,
            Register::Rr3 => self.r_r3,
            Register::Rr4 => self.r_r4,
            Register::Rr5 => self.r_r5,
            Register::Rr6 => self.r_r6,
            Register::Rr7 => self.r_r7,
            Register::Rpc => self.r_pc,
            Register::Rcond => self.r_cond,
        }
    }
}

#[repr(u8)]
pub enum ConditionFlag {
    FlPos,
    FlZro,
    FlNeg,
}

impl ConditionFlag {
    pub fn get_cflag_value(cflag: ConditionFlag) -> u16 {
        match cflag {
            ConditionFlag::FlPos => 1 << 0,
            ConditionFlag::FlZro => 1 << 1,
            ConditionFlag::FlNeg => 1 << 2,
        }
    }
}

pub enum MemoryMappedRegister {
    Kbsr = 0xFE00,
    Kbdr = 0xFE02,
}
