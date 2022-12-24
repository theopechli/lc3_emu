use std::{convert::TryFrom, env, fs::File, io::BufReader, io::Read, path::PathBuf};

const PC_START: u16 = 0x3000;
const MEMORY_MAX: usize = 1 << 16;

#[repr(u16)]
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
enum Opcode {
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
    OpLea = 14,
    OpTrap = 15,
}

impl TryFrom<u16> for Opcode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == Opcode::OpBr as u16 => Ok(Opcode::OpBr),
            x if x == Opcode::OpAdd as u16 => Ok(Opcode::OpAdd),
            x if x == Opcode::OpLd as u16 => Ok(Opcode::OpLd),
            x if x == Opcode::OpSt as u16 => Ok(Opcode::OpSt),
            x if x == Opcode::OpJsr as u16 => Ok(Opcode::OpJsr),
            x if x == Opcode::OpAnd as u16 => Ok(Opcode::OpAnd),
            x if x == Opcode::OpLdr as u16 => Ok(Opcode::OpLdr),
            x if x == Opcode::OpStr as u16 => Ok(Opcode::OpStr),
            x if x == Opcode::OpRti as u16 => Ok(Opcode::OpRti),
            x if x == Opcode::OpNot as u16 => Ok(Opcode::OpNot),
            x if x == Opcode::OpLdi as u16 => Ok(Opcode::OpLdi),
            x if x == Opcode::OpSti as u16 => Ok(Opcode::OpSti),
            x if x == Opcode::OpRes as u16 => Ok(Opcode::OpRes),
            x if x == Opcode::OpLea as u16 => Ok(Opcode::OpLea),
            x if x == Opcode::OpTrap as u16 => Ok(Opcode::OpTrap),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct Opcodes {
    op_br: fn(&mut Emulator, u16),
    op_add: fn(&mut Emulator, u16),
    op_ld: fn(&mut Emulator, u16),
    op_st: fn(),
    op_jsr: fn(&mut Emulator, u16),
    op_and: fn(&mut Emulator, u16),
    op_ldr: fn(&mut Emulator, u16),
    op_str: fn(),
    op_rti: fn(),
    op_not: fn(&mut Emulator, u16),
    op_ldi: fn(&mut Emulator, u16),
    op_sti: fn(),
    op_res: fn(&mut Emulator, u16),
    op_lea: fn(&mut Emulator, u16),
    op_trap: fn(),
}

impl Opcodes {
    pub fn new() -> Self {
        Opcodes {
            op_br,
            op_add,
            op_ld,
            op_st: help,
            op_jsr,
            op_and,
            op_ldr,
            op_str: help,
            op_rti: help,
            op_not,
            op_ldi,
            op_sti: help,
            op_res,
            op_lea,
            op_trap: help,
        }
    }

    pub fn call(&self, op: Opcode, emu: &mut Emulator, instr: u16) {
        match op {
            Opcode::OpBr => (self.op_br)(emu, instr),
            Opcode::OpAdd => (self.op_add)(emu, instr),
            Opcode::OpLd => (self.op_ld)(emu, instr),
            Opcode::OpSt => (self.op_st)(),
            Opcode::OpJsr => (self.op_jsr)(emu, instr),
            Opcode::OpAnd => (self.op_and)(emu, instr),
            Opcode::OpLdr => (self.op_ldr)(emu, instr),
            Opcode::OpStr => (self.op_str)(),
            Opcode::OpRti => (self.op_rti)(),
            Opcode::OpNot => (self.op_not)(emu, instr),
            Opcode::OpLdi => (self.op_ldi)(emu, instr),
            Opcode::OpSti => (self.op_sti)(),
            Opcode::OpRes => (self.op_res)(emu, instr),
            Opcode::OpLea => (self.op_lea)(emu, instr),
            Opcode::OpTrap => (self.op_trap)(),
        }
    }
}

#[derive(Clone)]
struct Mmu {
    memory: Vec<u16>,
}

impl Mmu {
    pub fn new() -> Self {
        Mmu {
            memory: vec![0; MEMORY_MAX],
        }
    }

    pub fn write(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }

    pub fn read(&mut self, address: usize) -> u16 {
        let value = self.memory[address];
        value
    }
}

#[derive(Clone)]
struct Emulator {
    pub memory: Mmu,
    pub registers: Registers,
    pub opcodes: Opcodes,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            memory: Mmu::new(),
            registers: Registers::new(),
            opcodes: Opcodes::new(),
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

fn read_n<R>(reader: R, bytes_to_read: u64) -> Vec<u8>
where
    R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    chunk.read_to_end(&mut buf).expect("Not enough bytes read");
    buf
}

fn be_to_le(buf: &mut Vec<u8>) {
    let mut tmp: u8;
    for i in (0..buf.len()).step_by(2) {
        tmp = buf[i + 1];
        buf[i + 1] = buf[i];
        buf[i] = tmp;
    }
}

fn read_image_file(file: File) {
    let mut reader = BufReader::new(file);

    let mut origin = read_n(reader.by_ref(), 2);
    be_to_le(&mut origin);

    let mut rest = read_n(
        reader.by_ref(),
        (MEMORY_MAX - origin.len()).try_into().unwrap(),
    );
    be_to_le(&mut rest);
}

fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}

fn update_flags(emu: &mut Emulator, reg: u16) {
    let r: u16 = emu.registers.get_value(Register::try_from(reg).unwrap());

    if r == 0 {
        emu.registers.update(
            Register::Rcond,
            ConditionFlag::get_cflag_value(ConditionFlag::FlZro),
        );
    } else if r >> 15 == 1 {
        emu.registers.update(
            Register::Rcond,
            ConditionFlag::get_cflag_value(ConditionFlag::FlNeg),
        );
    } else {
        emu.registers.update(
            Register::Rcond,
            ConditionFlag::get_cflag_value(ConditionFlag::FlPos),
        );
    }
}

fn op_add(emu: &mut Emulator, instr: u16) {
    let dr: u16 = (instr >> 9) & 0x7;
    let sr1: u16 = (instr >> 6) & 0x7;
    let r1: u16 = emu.registers.get_value(Register::try_from(sr1).unwrap());
    let imm_flag: u16 = (instr >> 5) & 0x1;

    if imm_flag == 0 {
        let sr2: u16 = instr & 0x7;
        let r2: u16 = emu.registers.get_value(Register::try_from(sr2).unwrap());
        emu.registers
            .update(Register::try_from(dr).unwrap(), r1 + r2);
    } else {
        let imm5: u16 = sign_extend(instr & 0x1F, 5);
        emu.registers
            .update(Register::try_from(dr).unwrap(), r1 + imm5);
    }

    update_flags(emu, dr);
}

fn op_and(emu: &mut Emulator, instr: u16) {
    let dr: u16 = (instr >> 9) & 0x7;
    let sr1: u16 = (instr >> 6) & 0x7;
    let r1: u16 = emu.registers.get_value(Register::try_from(sr1).unwrap());
    let imm_flag: u16 = instr & 0x20;

    if imm_flag == 0 {
        let sr2: u16 = instr & 0x7;
        let r2: u16 = emu.registers.get_value(Register::try_from(sr2).unwrap());
        emu.registers
            .update(Register::try_from(dr).unwrap(), r1 & r2);
    } else {
        let imm5: u16 = sign_extend(instr & 0x1F, 5);
        emu.registers
            .update(Register::try_from(dr).unwrap(), r1 & imm5);
    }

    update_flags(emu, dr);
}

fn op_res(emu: &mut Emulator, instr: u16) {
    let base_r: u16 = (instr >> 6) & 0x7;
    emu.registers.update(
        Register::Rpc,
        emu.registers.get_value(Register::try_from(base_r).unwrap()),
    );
}

fn op_jsr(emu: &mut Emulator, instr: u16) {
    emu.registers
        .update(Register::Rr7, emu.registers.get_value(Register::Rpc));

    if instr & 0xB != 0 {
        let pc_offset: u16 = sign_extend(instr & 0x3FF, 11);
        emu.registers.update(
            Register::Rpc,
            emu.registers.get_value(Register::Rpc) + pc_offset,
        );
    } else {
        let base_r: u16 = (instr >> 6) & 0x7;
        emu.registers.update(Register::Rpc, base_r);
    }
}

fn op_br(emu: &mut Emulator, instr: u16) {
    let cond_flag: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    let r_cond = emu.registers.get_value(Register::Rcond);

    if cond_flag & r_cond != 0 {
        emu.registers.update(
            Register::Rpc,
            emu.registers.get_value(Register::Rpc) + pc_offset,
        );
    }
}

fn op_ld(emu: &mut Emulator, instr: u16) {
    let dr: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);

    emu.registers.update(
        Register::try_from(dr).unwrap(),
        emu.memory.read((Register::Rpc as u16 + pc_offset).into()),
    );

    update_flags(emu, dr);
}

fn op_ldi(emu: &mut Emulator, instr: u16) {
    let dr: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);

    let x: u16 = emu.memory.read((Register::Rpc as u16 + pc_offset).into());
    emu.registers
        .update(Register::try_from(dr).unwrap(), emu.memory.read(x.into()));

    update_flags(emu, dr);
}

fn op_ldr(emu: &mut Emulator, instr: u16) {
    let dr: u16 = (instr >> 9) & 0x7;
    let base_r: u16 = (instr >> 6) & 0x7;
    let offset: u16 = sign_extend(instr & 0x3F, 6);

    emu.registers.update(
        Register::try_from(dr).unwrap(),
        emu.memory
            .read((Register::try_from(base_r).unwrap() as u16 + offset).into()),
    );

    update_flags(emu, dr);
}

fn op_lea(emu: &mut Emulator, instr: u16) {
    let dr: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);

    emu.registers.update(
        Register::try_from(dr).unwrap(),
        emu.registers.get_value(Register::Rpc) + pc_offset,
    );

    update_flags(emu, dr);
}

fn op_not(emu: &mut Emulator, instr: u16) {
    let dr: u16 = (instr >> 9) & 0x7;
    let sr: u16 = (instr >> 6) & 0x7;

    emu.registers.update(
        Register::try_from(dr).unwrap(),
        !emu.registers.get_value(Register::try_from(sr).unwrap()),
    );

    update_flags(emu, dr);
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
    emu.registers.update(Register::Rr1, 10);
    emu.registers.update(Register::Rr2, 11);

    emu.registers.update(Register::Rpc, PC_START);

    let mut running = true;
    let mut instr: u16;
    let mut op: Result<Opcode, ()>;

    while running {
        // TODO fetch instruction and match op

        instr = 0b1001011101010101;
        op = Opcode::try_from(instr >> 12);

        println!("Instruction {:b}", instr);

        if let Ok(op) = op {
            println!("Opcode {:?}", op);
            emu.opcodes.clone().call(op, &mut emu, instr);
            running = false;
        } else {
            eprintln!("Invalid instruction")
        }

        println!("{:?}", emu.registers);
    }
}
