use crate::opcode::*;
use crate::register::*;

use std::{io::stdin, io::Read};

pub const PC_START: u16 = 0x3000;
pub const MEMORY_MAX: usize = 1 << 16;

#[derive(Clone)]
pub struct Mmu {
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
        if address == MemoryMappedRegister::Kbsr as usize {
            self.handle_keyboard();
        }
        return self.memory[address];
    }

    fn handle_keyboard(&mut self) {
        let mut buf = [0; 1];
        stdin().read_exact(&mut buf).unwrap();
        if buf[0] != 0 {
            self.write(MemoryMappedRegister::Kbsr as usize, 1 << 15);
            self.write(MemoryMappedRegister::Kbdr as usize, buf[0] as u16);
        } else {
            self.write(MemoryMappedRegister::Kbsr as usize, 0);
        }
    }
}

#[derive(Clone)]
pub struct Emulator {
    pub memory: Mmu,
    pub registers: Registers,
    pub opcodes: Opcodes,
    pub traps: Traps,
    pub running: bool,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            memory: Mmu::new(),
            registers: Registers::new(),
            opcodes: Opcodes::new(),
            traps: Traps::new(),
            running: true,
        }
    }
}
