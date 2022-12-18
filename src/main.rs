const PC_START: u16 = 0x3000;
const MEMORY_MAX: usize = 1 << 16;

enum Register {
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
    Rcount,
}

#[derive(Debug)]
struct Registers {
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

    pub fn update(&mut self, index: Register, value: u16) {
        match index {
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
            _ => panic!("Invalid register index"),
        }
    }

    pub fn get(self, index: Register) -> u16 {
        match index {
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
            _ => panic!("Invalid register index"),
        }
    }
}

enum ConditionFlag {
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

#[derive(Debug)]
#[repr(u16)]
enum Instruction {
    OpBr = 0,
    OpAdd,
    OpLd,
    OpSt,
    OpJsr,
    OpAnd,
    OpLdr,
    OpStr,
    OpRti,
    OpNot,
    OpLdi,
    OpSti,
    OpRes,
    OpLea,
    OpTrap,
}

struct Mmu {
    memory: Vec<u16>,
}

impl Mmu {
    pub fn new() -> Self {
        Mmu {
            memory: vec![0; MEMORY_MAX],
        }
    }
}

struct Emulator {
    pub memory: Mmu,
    pub registers: Registers,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            memory: Mmu::new(),
            registers: Registers::new(),
        }
    }
}

fn main() {
    let mut emu = Emulator::new();

    emu.registers.update(
        Register::Rcond,
        ConditionFlag::get_cflag_value(ConditionFlag::FlZro),
    );

    emu.registers.update(Register::Rpc, PC_START);

    let mut running = true;
    let mut op: Instruction = Instruction::OpLdr;

    while running {
        match op {
            Instruction::OpBr => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpAdd => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpLd => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpSt => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpJsr => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpAnd => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpLdr => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpStr => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpRti => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpNot => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpLdi => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpSti => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpRes => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpLea => {
                println!("{:?}", op);
                running = false;
            }
            Instruction::OpTrap => {
                println!("{:?}", op);
                running = false;
            }
            _ => println!("Invalid instruction"),
        }
    }

    println!("{:?}", emu.registers);
}
