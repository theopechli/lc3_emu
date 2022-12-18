use std::{convert::TryFrom, env, fs::File, path::PathBuf};

const PC_START: u16 = 0x3000;
const MEMORY_MAX: usize = 1 << 16;

#[repr(u8)]
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
}

impl TryFrom<u8> for Register {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Register::Rr0 as u8 => Ok(Register::Rr0),
            x if x == Register::Rr1 as u8 => Ok(Register::Rr1),
            x if x == Register::Rr2 as u8 => Ok(Register::Rr2),
            x if x == Register::Rr3 as u8 => Ok(Register::Rr3),
            x if x == Register::Rr4 as u8 => Ok(Register::Rr4),
            x if x == Register::Rr5 as u8 => Ok(Register::Rr5),
            x if x == Register::Rr6 as u8 => Ok(Register::Rr6),
            x if x == Register::Rr7 as u8 => Ok(Register::Rr7),
            x if x == Register::Rpc as u8 => Ok(Register::Rpc),
            x if x == Register::Rcond as u8 => Ok(Register::Rcond),
            _ => Err(()),
        }
    }
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
            _ => panic!("Invalid register index"),
        }
    }

    pub fn get(self, reg: Register) -> u16 {
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
            _ => panic!("Invalid register index"),
        }
    }
}

#[repr(u8)]
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

impl TryFrom<u16> for Instruction {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == Instruction::OpBr as u16 => Ok(Instruction::OpBr),
            x if x == Instruction::OpAdd as u16 => Ok(Instruction::OpAdd),
            x if x == Instruction::OpLd as u16 => Ok(Instruction::OpLd),
            x if x == Instruction::OpSt as u16 => Ok(Instruction::OpSt),
            x if x == Instruction::OpJsr as u16 => Ok(Instruction::OpJsr),
            x if x == Instruction::OpAnd as u16 => Ok(Instruction::OpAnd),
            x if x == Instruction::OpLdr as u16 => Ok(Instruction::OpLdr),
            x if x == Instruction::OpStr as u16 => Ok(Instruction::OpStr),
            x if x == Instruction::OpRti as u16 => Ok(Instruction::OpRti),
            x if x == Instruction::OpNot as u16 => Ok(Instruction::OpNot),
            x if x == Instruction::OpLdi as u16 => Ok(Instruction::OpLdi),
            x if x == Instruction::OpSti as u16 => Ok(Instruction::OpSti),
            x if x == Instruction::OpRes as u16 => Ok(Instruction::OpRes),
            x if x == Instruction::OpLea as u16 => Ok(Instruction::OpLea),
            x if x == Instruction::OpTrap as u16 => Ok(Instruction::OpTrap),
            _ => Err(()),
        }
    }
}

struct Instructions {
    op_br: fn(),
    op_add: fn(),
    op_ld: fn(),
    op_st: fn(),
    op_jsr: fn(),
    op_and: fn(),
    op_ldr: fn(),
    op_str: fn(),
    op_rti: fn(),
    op_not: fn(),
    op_ldi: fn(),
    op_sti: fn(),
    op_res: fn(),
    op_lea: fn(),
    op_trap: fn(),
}

impl Instructions {
    pub fn new() -> Self {
        Instructions {
            op_br: help,
            op_add: help,
            op_ld: help,
            op_st: help,
            op_jsr: help,
            op_and: help,
            op_ldr: help,
            op_str: help,
            op_rti: help,
            op_not: help,
            op_ldi: help,
            op_sti: help,
            op_res: help,
            op_lea: help,
            op_trap: help,
        }
    }

    pub fn call(&self, instr: Instruction) {
        match instr {
            Instruction::OpBr => (self.op_br)(),
            Instruction::OpAdd => (self.op_add)(),
            Instruction::OpLd => (self.op_ld)(),
            Instruction::OpSt => (self.op_st)(),
            Instruction::OpJsr => (self.op_jsr)(),
            Instruction::OpAnd => (self.op_and)(),
            Instruction::OpLdr => (self.op_ldr)(),
            Instruction::OpStr => (self.op_str)(),
            Instruction::OpRti => (self.op_rti)(),
            Instruction::OpNot => (self.op_not)(),
            Instruction::OpLdi => (self.op_ldi)(),
            Instruction::OpSti => (self.op_sti)(),
            Instruction::OpRes => (self.op_res)(),
            Instruction::OpLea => (self.op_lea)(),
            Instruction::OpTrap => (self.op_trap)(),
        }
    }
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
    pub instructions: Instructions,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            memory: Mmu::new(),
            registers: Registers::new(),
            instructions: Instructions::new(),
        }
    }
}

fn help() {
    println!(
        "Usage: lc3_emu <binary>

        Options:
            <binary>    Binary to emulate."
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let binary: PathBuf = PathBuf::from(args[1].parse::<String>().unwrap());
            println!("Emulation target is: '{}'", binary.display());
            {
                File::open(&binary).unwrap_or_else(|err| {
                    panic!("Could not open file '{}': {}", binary.display(), err)
                });
            }
        }
        _ => {
            help();
            panic!("Invalid arguments");
        }
    }

    // TODO load binary

    let mut emu = Emulator::new();

    emu.registers.update(
        Register::Rcond,
        ConditionFlag::get_cflag_value(ConditionFlag::FlZro),
    );

    emu.registers.update(Register::Rpc, PC_START);

    let mut running = true;
    let mut instr: u16;
    let mut op: Result<Instruction, ()>;

    while running {
        // TODO fetch instruction and match op

        instr = 0b0001000001000011;
        op = Instruction::try_from(instr >> 12);

        match op {
            Ok(op) => {
                println!("{:?}", op);
                emu.instructions.call(op);
                running = false;
            }
            Err(_) => eprintln!("Invalid instruction"),
        }
    }

    println!("{:?}", emu.registers);
}
