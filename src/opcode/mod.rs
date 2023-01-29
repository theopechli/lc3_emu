use crate::emulator::*;
use crate::instruction::*;

#[derive(Debug)]
#[repr(u16)]
pub enum Opcode {
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
    OpJmp,
    OpRes,
    OpLea,
    OpTrap,
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
            x if x == Opcode::OpJmp as u16 => Ok(Opcode::OpJmp),
            x if x == Opcode::OpRes as u16 => Ok(Opcode::OpRes),
            x if x == Opcode::OpLea as u16 => Ok(Opcode::OpLea),
            x if x == Opcode::OpTrap as u16 => Ok(Opcode::OpTrap),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub struct Opcodes {
    op_br: fn(&mut Emulator, u16),
    op_add: fn(&mut Emulator, u16),
    op_ld: fn(&mut Emulator, u16),
    op_st: fn(&mut Emulator, u16),
    op_jsr: fn(&mut Emulator, u16),
    op_and: fn(&mut Emulator, u16),
    op_ldr: fn(&mut Emulator, u16),
    op_str: fn(&mut Emulator, u16),
    op_rti: fn(),
    op_not: fn(&mut Emulator, u16),
    op_ldi: fn(&mut Emulator, u16),
    op_sti: fn(&mut Emulator, u16),
    op_jmp: fn(&mut Emulator, u16),
    op_res: fn(),
    op_lea: fn(&mut Emulator, u16),
    op_trap: fn(&mut Emulator, u16),
}

impl Opcodes {
    pub fn new() -> Self {
        Opcodes {
            op_br,
            op_add,
            op_ld,
            op_st,
            op_jsr,
            op_and,
            op_ldr,
            op_str,
            op_rti,
            op_not,
            op_ldi,
            op_sti,
            op_jmp,
            op_res,
            op_lea,
            op_trap,
        }
    }

    pub fn call(&self, op: Opcode, emu: &mut Emulator, instr: u16) {
        match op {
            Opcode::OpBr => (self.op_br)(emu, instr),
            Opcode::OpAdd => (self.op_add)(emu, instr),
            Opcode::OpLd => (self.op_ld)(emu, instr),
            Opcode::OpSt => (self.op_st)(emu, instr),
            Opcode::OpJsr => (self.op_jsr)(emu, instr),
            Opcode::OpAnd => (self.op_and)(emu, instr),
            Opcode::OpLdr => (self.op_ldr)(emu, instr),
            Opcode::OpStr => (self.op_str)(emu, instr),
            Opcode::OpRti => (self.op_rti)(),
            Opcode::OpNot => (self.op_not)(emu, instr),
            Opcode::OpLdi => (self.op_ldi)(emu, instr),
            Opcode::OpSti => (self.op_sti)(emu, instr),
            Opcode::OpJmp => (self.op_jmp)(emu, instr),
            Opcode::OpRes => (self.op_res)(),
            Opcode::OpLea => (self.op_lea)(emu, instr),
            Opcode::OpTrap => (self.op_trap)(emu, instr),
        }
    }
}

#[repr(u8)]
pub enum Trap {
    TrapGetc = 0x20,
    TrapOut,
    TrapPuts,
    TrapIn,
    TrapPutsp,
    TrapHalt,
}

impl TryFrom<u16> for Trap {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == Trap::TrapGetc as u16 => Ok(Trap::TrapGetc),
            x if x == Trap::TrapOut as u16 => Ok(Trap::TrapOut),
            x if x == Trap::TrapPuts as u16 => Ok(Trap::TrapPuts),
            x if x == Trap::TrapIn as u16 => Ok(Trap::TrapIn),
            x if x == Trap::TrapPutsp as u16 => Ok(Trap::TrapPutsp),
            x if x == Trap::TrapHalt as u16 => Ok(Trap::TrapHalt),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub struct Traps {
    trap_getc: fn(&mut Emulator),
    trap_out: fn(&mut Emulator),
    trap_puts: fn(&mut Emulator),
    trap_in: fn(&mut Emulator),
    trap_putsp: fn(&mut Emulator),
    trap_halt: fn(&mut Emulator),
}

impl Traps {
    pub fn new() -> Self {
        Traps {
            trap_getc,
            trap_out,
            trap_puts,
            trap_in,
            trap_putsp,
            trap_halt,
        }
    }

    pub fn call(&self, trap: Trap, emu: &mut Emulator) {
        match trap {
            Trap::TrapGetc => (self.trap_getc)(emu),
            Trap::TrapOut => (self.trap_out)(emu),
            Trap::TrapPuts => (self.trap_puts)(emu),
            Trap::TrapIn => (self.trap_in)(emu),
            Trap::TrapPutsp => (self.trap_putsp)(emu),
            Trap::TrapHalt => (self.trap_halt)(emu),
        }
    }
}
