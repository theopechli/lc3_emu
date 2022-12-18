const MEMORY_MAX: usize = 1 << 16;

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
    r_count: u16,
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
            r_count: 0,
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

enum Instruction {
    OP_BR = 0,
    OP_ADD,
    OP_LD,
    OP_ST,
    OP_JSR,
    OP_AND,
    OP_LDR,
    OP_STR,
    OP_RTI,
    OP_NOT,
    OP_LDI,
    OP_STI,
    OP_RES,
    OP_LEA,
    OP_TRAP,
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
    println!("Hello, world!");
}
