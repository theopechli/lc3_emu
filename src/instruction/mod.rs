use crate::emulator::*;
use crate::opcode::Trap;
use crate::register::*;

use std::{
    fs::File,
    io::{stdin, stdout, BufReader, Read, Write},
    process::exit,
};

pub fn help() {
    println!(
        "Usage: lc3_emu <binary>

        Options:
            <binary>    Binary to emulate."
    );
}

pub fn read_n<R>(reader: R, bytes_to_read: u64) -> Vec<u8>
where
    R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    chunk.read_to_end(&mut buf).expect("Not enough bytes read");
    buf
}

pub fn be_to_le(buf: &mut Vec<u8>) {
    let mut tmp: u8;
    for i in (0..buf.len()).step_by(2) {
        tmp = buf[i + 1];
        buf[i + 1] = buf[i];
        buf[i] = tmp;
    }
}

pub fn vec_u8_to_vec_u16(buf: Vec<u8>) -> Vec<u16> {
    let mut other: Vec<u16> = vec![0; buf.len() / 2];
    let mut j = 0;

    for i in (0..buf.len()).step_by(2) {
        other[j] = ((buf[i + 1] as u16) << 8) | buf[i] as u16;
        j += 1;
    }

    other
}

pub fn read_image_file(file: File, emu: &mut Emulator) {
    let mut reader = BufReader::new(file);

    // origin seems to be the PC_START
    let mut origin = read_n(reader.by_ref(), 2);
    be_to_le(&mut origin);

    let mut rest = read_n(
        reader.by_ref(),
        (MEMORY_MAX - origin.len()).try_into().unwrap(),
    );
    be_to_le(&mut rest);

    let tmp: String = String::from_utf8(origin).unwrap();

    let buf = vec_u8_to_vec_u16(rest);
    let pc_start = match tmp.parse::<usize>() {
        Ok(i) => i,
        Err(_e) => PC_START as usize,
    };

    let mut address: usize = pc_start;

    for i in buf.clone() {
        emu.memory.write(address, i);
        address += 1;
    }
}

pub fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}

pub fn update_flags(emu: &mut Emulator, reg: u16) {
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

pub fn op_add(emu: &mut Emulator, instr: u16) {
    let dr: u16 = (instr >> 9) & 0x7;
    let sr1: u16 = (instr >> 6) & 0x7;
    let r1: u16 = emu.registers.get_value(Register::try_from(sr1).unwrap());
    let imm_flag: u16 = (instr >> 5) & 0x1;

    if imm_flag == 0 {
        let sr2: u16 = instr & 0x7;
        let r2: u16 = emu.registers.get_value(Register::try_from(sr2).unwrap());
        let val: u32 = r1 as u32 + r2 as u32;

        emu.registers
            .update(Register::try_from(dr).unwrap(), val as u16);
    } else {
        let imm5: u16 = sign_extend(instr & 0x1F, 5);
        let val: u32 = r1 as u32 + imm5 as u32;

        emu.registers
            .update(Register::try_from(dr).unwrap(), val as u16);
    }

    update_flags(emu, dr);
}

pub fn op_and(emu: &mut Emulator, instr: u16) {
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

pub fn op_jmp(emu: &mut Emulator, instr: u16) {
    let base_r: u16 = (instr >> 6) & 0x7;
    emu.registers.update(
        Register::Rpc,
        emu.registers.get_value(Register::try_from(base_r).unwrap()),
    );
}

pub fn op_res() {}

pub fn op_jsr(emu: &mut Emulator, instr: u16) {
    emu.registers
        .update(Register::Rr7, emu.registers.get_value(Register::Rpc));

    let flag = (instr >> 11) & 1;

    if flag != 0 {
        let pc_offset: u16 = sign_extend(instr & 0x7FF, 11);
        let value: u32 = emu.registers.get_value(Register::Rpc) as u32 + pc_offset as u32;

        emu.registers.update(Register::Rpc, value as u16);
    } else {
        let base_r: u16 = (instr >> 6) & 0x7;

        emu.registers.update(Register::Rpc, base_r);
    }
}

pub fn op_br(emu: &mut Emulator, instr: u16) {
    let cond_flag: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    let r_cond = emu.registers.get_value(Register::Rcond);

    if cond_flag & r_cond != 0 {
        let val: u32 = emu.registers.get_value(Register::Rpc) as u32 + pc_offset as u32;
        emu.registers.update(Register::Rpc, val as u16);
    }
}

pub fn op_ld(emu: &mut Emulator, instr: u16) {
    let dr: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    let mem: u32 = Register::Rpc as u32 + pc_offset as u32;

    let value = emu.memory.read(mem as usize);

    emu.registers.update(Register::try_from(dr).unwrap(), value);

    update_flags(emu, dr);
}

pub fn op_ldi(emu: &mut Emulator, instr: u16) {
    let dr: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);

    let x: u16 = emu
        .memory
        .read((emu.registers.get_value(Register::Rpc) as u16 + pc_offset).into());
    emu.registers
        .update(Register::try_from(dr).unwrap(), emu.memory.read(x.into()));

    update_flags(emu, dr);
}

pub fn op_ldr(emu: &mut Emulator, instr: u16) {
    let dr: u16 = (instr >> 9) & 0x7;
    let base_r: u16 = (instr >> 6) & 0x7;
    let offset: u16 = sign_extend(instr & 0x3F, 6);
    let value: u32 = Register::try_from(base_r).unwrap() as u32 + offset as u32;

    emu.registers.update(
        Register::try_from(dr).unwrap(),
        emu.memory.read(value as usize),
    );

    update_flags(emu, dr);
}

pub fn op_lea(emu: &mut Emulator, instr: u16) {
    let dr: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    let value: u32 = emu.registers.get_value(Register::Rpc) as u32 + pc_offset as u32;

    emu.registers
        .update(Register::try_from(dr).unwrap(), value as u16);

    update_flags(emu, dr);
}

pub fn op_rti() {}

pub fn op_not(emu: &mut Emulator, instr: u16) {
    let dr: u16 = (instr >> 9) & 0x7;
    let sr: u16 = (instr >> 6) & 0x7;

    emu.registers.update(
        Register::try_from(dr).unwrap(),
        !emu.registers.get_value(Register::try_from(sr).unwrap()),
    );

    update_flags(emu, dr);
}

pub fn op_st(emu: &mut Emulator, instr: u16) {
    let sr: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    let value: u32 = emu.registers.get_value(Register::Rpc) as u32 + pc_offset as u32;
    let value: u16 = value as u16;

    emu.memory.write(
        value as usize,
        emu.registers.get_value(Register::try_from(sr).unwrap()),
    );
}

pub fn op_sti(emu: &mut Emulator, instr: u16) {
    let sr: u16 = (instr >> 9) & 0x7;
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);

    let value: u32 = emu.registers.get_value(Register::Rpc) as u32 + pc_offset as u32;
    let value: u16 = value as u16;
    let address: usize = emu.memory.read(value as usize) as usize;

    emu.memory.write(
        address,
        emu.registers.get_value(Register::try_from(sr).unwrap()),
    );
}

pub fn op_str(emu: &mut Emulator, instr: u16) {
    let sr: u16 = (instr >> 9) & 0x7;
    let base_r: u16 = (instr >> 6) & 0x7;
    let offset: u16 = sign_extend(instr & 0x3F, 6);

    let value: u32 =
        emu.registers.get_value(Register::try_from(base_r).unwrap()) as u32 + offset as u32;
    let value: u16 = value as u16;

    emu.memory.write(
        value as usize,
        emu.registers.get_value(Register::try_from(sr).unwrap()),
    );
}

pub fn op_trap(emu: &mut Emulator, instr: u16) {
    let trap = Trap::try_from(instr & 0xFF);
    if let Ok(trap) = trap {
        emu.traps.clone().call(trap, emu);
    }
}

pub fn trap_getc(emu: &mut Emulator) {
    let value: u16 = stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u16)
        .unwrap();

    emu.registers.update(Register::Rr0, value);
    // update_flags(emu, 0);
}

pub fn trap_out(emu: &mut Emulator) {
    let c: u8 = emu.registers.get_value(Register::Rr0) as u8;
    println!("{:?}", c as char);
}

pub fn trap_puts(emu: &mut Emulator) {
    let mut i: usize = (emu.registers.get_value(Register::Rr0) as u16).into();
    let mut c: u16 = emu.memory.read(i);

    loop {
        if c == 0 {
            break;
        }

        print!("{}", c as u8 as char);
        i += 1;
        c = emu.memory.read(i);
    }

    stdout().flush().expect("Failed to flush");
}

pub fn trap_in(emu: &mut Emulator) {
    println!("Enter a character: ");
    stdout().flush().expect("Failed to flush");

    let value: u16 = stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u16)
        .unwrap();

    emu.registers.update(Register::Rr0, value);
    // update_flags(emu, 0);
}

pub fn trap_putsp(emu: &mut Emulator) {
    let mut i: usize = (emu.registers.get_value(Register::Rr0) as u16).into();
    let mut c: u16 = emu.memory.read(i);

    let mut c1: u8;
    let mut c2: u8;

    loop {
        if c == 0 {
            break;
        }

        c1 = c as u8;
        print!("{}", c1 as char);

        c2 = (c >> 8) as u8;
        if c2 != 0 {
            print!("{}", c2 as char);
        }

        i += 1;
        c = emu.memory.read(i);
    }
    stdout().flush().expect("Failed to flush");
}

pub fn trap_halt(emu: &mut Emulator) {
    println!("HALT");
    stdout().flush().expect("Failed to flush");
    emu.running = false;
}
