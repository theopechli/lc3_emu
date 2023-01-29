mod emulator;
mod instruction;
mod opcode;
mod register;
use emulator::*;
use instruction::*;
use opcode::*;
use register::*;

use std::{convert::TryFrom, env, fs::File, path::PathBuf, process};
use termios::*;

fn main() {
    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin).unwrap();

    // make a mutable copy of termios
    // that we will modify
    let mut new_termios = termios.clone();
    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode

    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

    let args: Vec<String> = env::args().collect();
    let mut emu: Emulator;

    match args.len() {
        2 => {
            let binary: PathBuf = PathBuf::from(args[1].parse::<String>().unwrap());
            println!("Emulation target is: '{}'", binary.display());
            {
                let file = File::open(&binary).unwrap_or_else(|err| {
                    panic!("Could not open file '{}': {}", binary.display(), err)
                });

                emu = Emulator::new();

                read_image_file(file, &mut emu);
            }
        }
        _ => {
            help();
            panic!("Invalid arguments");
        }
    }

    emu.registers.update(
        Register::Rcond,
        ConditionFlag::get_cflag_value(ConditionFlag::FlZro),
    );
    emu.registers.update(Register::Rpc, PC_START);

    let mut instr: u16;
    let mut op: Result<Opcode, ()>;

    while emu.running {
        instr = emu
            .memory
            .read(emu.registers.get_value(Register::Rpc) as usize);
        emu.registers
            .update(Register::Rpc, emu.registers.get_value(Register::Rpc) + 1);
        op = Opcode::try_from(instr >> 12);

        if let Ok(op) = op {
            emu.opcodes.clone().call(op, &mut emu, instr);
        } else {
            eprintln!("Invalid instruction");
            emu.running = false;
        }
    }

    tcsetattr(stdin, TCSANOW, &termios).unwrap();
    process::exit(1);
}
