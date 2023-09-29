use std::{io::Read, path::Path, fs::File};
use std::fmt::Debug;

pub mod error;
pub mod helper;
pub mod prelude;

use error::{RimResult, RimError};
use helper::{U3, U4};

pub const MAGIC: u16 = 0x8bca;

#[inline]
pub fn check_magic(signature: [u8; 2]) -> bool {
    ((signature[0] as u16) << 8) | signature[1] as u16 == MAGIC
}

pub fn read_file<F: AsRef<Path>>(f: F) -> RimResult<Rim> {
    let mut file = File::open(f)?;

    let mut signature = [0; 2];
    file.read_exact(&mut signature)?;
    if !check_magic(signature) {
        return Err(RimError::InvalidMagic);
    }

    let mut instructions = Vec::new();

    let mut buf = [0u8; 12];

    while let Ok(read) = file.read(&mut buf) {
        if read == 0 {
            break;
        }

        for &byte in &buf[0..read] {
            let opcode: Opcode = byte.into();
            let data = opcode.parse_data(byte & 0b1111_1000);

            instructions.push(Instruction(opcode, data));
        }
    }

    Ok(Rim {
        instructions,
        ..Default::default()
    })
}

/// A Rim program.
#[derive(Clone)]
pub struct Rim {
    instructions: Vec<Instruction>,
    pc: usize,

    registers: [u8; 4],
    flags: [bool; 2],
    data: [u8; 4096],
}

impl Rim {
    pub fn run(&mut self) -> RimResult<()> {
        for instruction in self.instructions.clone() {
            match instruction.0 {
                Opcode::Adi => {
                    let imm = instruction.1.as_imm();
                    let res = self.registers[0].wrapping_add(imm);
                    self.registers[0] = res;

                    self.flags[0] = false;
                    self.flags[1] = res == 0;
                }
                Opcode::Add => {
                    let (is_id, src, dest) = instruction.1.as_reg();
                    let (src, dest) = if is_id {
                        (
                            Register::from(self.registers[src as usize]) as usize,
                            Register::from(self.registers[dest as usize]) as usize,
                        )
                    } else {
                        (
                            src as usize,
                            dest as usize,
                        )
                    };

                    let res = self.registers[dest].wrapping_add(self.registers[src]);
                    self.registers[dest] = res;

                    self.flags[0] = false;
                    self.flags[1] = res == 0;
                }
                Opcode::Sub => {
                    let (is_id, src, dest) = instruction.1.as_reg();
                    let (src, dest) = if is_id {
                        (
                            Register::from(self.registers[src as usize]) as usize,
                            Register::from(self.registers[dest as usize]) as usize,
                        )
                    } else {
                        (
                            src as usize,
                            dest as usize,
                        )
                    };

                    let (res, sign) = self.registers[dest].overflowing_sub(self.registers[src]);
                    self.registers[dest as usize] = res;

                    self.flags[0] = sign;
                    self.flags[1] = res == 0;
                }
                Opcode::Jne => {
                    let (is_ptr, addr) = instruction.1.as_mem();
                    let mut addr = ((self.registers[3] as usize) << 4) | addr as usize;
                    if is_ptr {
                        addr = ((self.registers[3] as usize) << 4) | self.data[addr] as usize;
                    }

                    if !self.flags[1] {
                        self.pc = addr;
                    }
                }
                Opcode::Jg => {
                    let (is_ptr, addr) = instruction.1.as_mem();
                    let mut addr = ((self.registers[3] as usize) << 4) | addr as usize;
                    if is_ptr {
                        addr = ((self.registers[3] as usize) << 4) | self.data[addr] as usize;
                    }

                    if self.flags[0] {
                        self.pc = addr;
                    }
                }
                Opcode::Jl => {
                    let (is_ptr, addr) = instruction.1.as_mem();
                    let mut addr = ((self.registers[3] as usize) << 4) | addr as usize;
                    if is_ptr {
                        addr = ((self.registers[3] as usize) << 4) | self.data[addr] as usize;
                    }

                    if !self.flags[0] && !self.flags[1] {
                        self.pc = addr;
                    }
                }
                Opcode::Ioi => {
                    let (device, function) = instruction.1.as_io();
                    if self.io(device, function, self.registers[0])? {
                        return Ok(());
                    }
                }
                Opcode::Ior => {
                    let (device, function) = instruction.1.as_io();
                    if self.io(device, function, self.registers[self.registers[0] as usize])? {
                        return Ok(());
                    }
                }
            }
        }

        Ok(())
    }

    fn io(&mut self, device: Device, function: U3, value: u8) -> RimResult<bool> {
        match device {
            Device::Cpu => match function as u8 {
                0 => return Ok(true),
                1 => {},
                2 => self.registers[0] = 0,
                3 => {
                    let addr = ((self.registers[3] as usize) << 4) | value as usize;
                    self.registers[0] = self.data[addr];
                }
                4 => {
                    let addr = ((self.registers[3] as usize) << 4) | self.registers[0] as usize;
                    self.data[addr] = value;
                }
                5 => {
                    let addr = ((self.registers[3] as usize) << 4) | value as usize;
                    let addr = ((self.registers[3] as usize) << 4) | addr;
                    self.registers[0] = self.data[addr];
                }
                6 => {
                    let addr = ((self.registers[3] as usize) << 4) | self.registers[0] as usize;
                    let addr = ((self.registers[3] as usize) << 4) | addr;
                    self.data[addr] = value;
                }
                7 => {},
                _ => unreachable!()
            },
            Device::Kbd => match function as u8 {
                0 => todo!(),
                1 => todo!(),
                2 => {},
                3 => {},
                4 => {},
                5 => {},
                6 => {},
                7 => {},
                _ => unreachable!()
            },
            Device::Scr => match function as u8 {
                0 => print!("{}[{value};H", 27 as char),
                1 => print!("{}[;{value}H", 27 as char),
                2 => print!("{}", value as char),
                3 => self.registers[0] = 0,
                4 => self.registers[0] = 0,
                5 => println!("{}[2J", 27 as char),
                6 => {},
                7 => {},
                _ => unreachable!()
            },
            Device::Mth => match function as u8 {
                0 => {
                    let res = (self.registers[0] as u16).wrapping_mul(self.registers[value as usize] as u16);
                    self.registers[0] = res as u8;
                    self.registers[1] = (res >> 8) as u8;

                    self.flags[1] = res == 0;
                }
                1 => {
                    let res = self.registers[0] / self.registers[value as usize];
                    self.registers[0] = res;

                    self.flags[1] = res == 0;
                }
                2 => {
                    let res = self.registers[0] & self.registers[0];
                    self.registers[0] = res;

                    self.flags[1] = res == 0;
                }
                3 => {
                    let res = self.registers[0] | self.registers[0];
                    self.registers[0] = res;

                    self.flags[1] = res == 0;
                }
                4 => {
                    let res = self.registers[0] ^ self.registers[0];
                    self.registers[0] = res;

                    self.flags[1] = res == 0;
                }
                5 => {
                    let res = !self.registers[0];
                    self.registers[0] = res;

                    self.flags[1] = res == 0;
                }
                6 => {
                    let mut res = 0;

                    if self.flags[0] {
                        res |= 0b01;
                    }

                    if self.flags[1] {
                        res |= 0b10;
                    }

                    self.registers[res];
                }
                7 => {
                    self.flags[0] = value & 0b01 != 0;
                    self.flags[1] = value & 0b10 != 0;
                }
                _ => unreachable!()
            },
        }

        Ok(false)
    }
}

impl Default for Rim {
    fn default() -> Self {
        Self { instructions: Default::default(), pc: Default::default(), registers: Default::default(), flags: [false; 2], data: [0; 4096] }
    }
}

impl Debug for Rim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rim").field("instructions", &self.instructions).field("pc", &self.pc).field("registers", &self.registers).field("flags", &self.flags).finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction(pub Opcode, pub InstructionData);

impl From<Instruction> for u8 {
    fn from(instruction: Instruction) -> Self {
        let opcode = instruction.0 as u8;
        let data: u8 = instruction.1.into();

        opcode | data
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Adi,
    Add,
    Sub,
    Jne,
    Jg,
    Jl,
    Ioi,
    Ior,
}

impl Opcode {
    pub fn parse_data(&self, data: u8) -> InstructionData {
        match self {
            Opcode::Adi => {
                let imm = data >> 3;
                
                InstructionData::Imm(imm)
            }
            Opcode::Add
            | Opcode::Sub => {
                let is_id = data & 0b0000_1000 != 0;
                let src = Register::from(data >> 4);
                let dest = Register::from(data >> 6);

                InstructionData::Reg {
                    is_id,
                    src,
                    dest,
                }
            }
            Opcode::Jne
            | Opcode::Jg
            | Opcode::Jl => {
                let is_ptr = data & 0b0000_1000 != 0;
                let addr = U4::from(data >> 4);

                InstructionData::Mem {
                    is_ptr,
                    addr,
                }
            }
            Opcode::Ioi
            | Opcode::Ior => {
                let device = Device::from(data >> 3);
                let function = U3::from(data >> 5);

                InstructionData::Io {
                    device,
                    function,
                }
            }
        }
    }
}

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        match opcode & 0b0000_0111 {
            0b000 => Self::Adi,
            0b001 => Self::Add,
            0b010 => Self::Sub,
            0b011 => Self::Jne,
            0b100 => Self::Jg,
            0b101 => Self::Jl,
            0b110 => Self::Ioi,
            0b111 => Self::Ior,

            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionData {
    Imm(u8),
    Reg {
        is_id: bool,
        src: Register,
        dest: Register,
    },
    Mem {
        is_ptr: bool,
        addr: U4,
    },
    Io {
        device: Device,
        function: U3,
    },
}

impl InstructionData {
    pub fn as_imm(self) -> u8 {
        if let Self::Imm(imm) = self {
            imm
        } else {
            panic!("Tried to call as_imm on non-Imm InstructionData")
        }
    }

    pub fn as_reg(self) -> (bool, Register, Register) {
        if let Self::Reg { is_id, src, dest } = self {
            (is_id, src, dest)
        } else {
            panic!("Tried to call as_reg on non-Reg InstructionData")
        }
    }

    pub fn as_mem(self) -> (bool, U4) {
        if let Self::Mem { is_ptr, addr } = self {
            (is_ptr, addr)
        } else {
            panic!("Tried to call as_mem on non-Mem InstructionData")
        }
    }

    pub fn as_io(self) -> (Device, U3) {
        if let Self::Io { device, function } = self {
            (device, function)
        } else {
            panic!("Tried to call as_io on non-Io InstructionData")
        }
    }
}

impl From<InstructionData> for u8 {
    fn from(data: InstructionData) -> Self {
        let mut byte = 0;
        match data {
            InstructionData::Imm(imm) => byte |= imm << 3,
            InstructionData::Reg { is_id, src, dest } => {
                if is_id {
                    byte |= 1 << 3;
                }

                byte |= (src as u8) << 4;
                byte |= (dest as u8) << 6;
            }
            InstructionData::Mem { is_ptr, addr } => {
                if is_ptr {
                    byte |= 1 << 3;
                }

                byte |= (addr as u8) << 4;
            }
            InstructionData::Io { device, function } => {
                byte |= (device as u8) << 3;
                byte |= (function as u8) << 5;
            }
        }

        byte
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Register {
    /// The accumulator and return value.
    Ra,
    /// A general-purpose register.
    Rb,
    /// The stack pointer.
    Rc,
    /// The high end of addresses.
    Rd,
}

impl From<u8> for Register {
    fn from(id: u8) -> Self {
        match id & 0b0000_0011 {
            0 => Self::Ra,
            1 => Self::Rb,
            2 => Self::Rc,
            3 => Self::Rd,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Device {
    Cpu,
    Kbd,
    Scr,
    Mth,
}

impl From<u8> for Device {
    fn from(id: u8) -> Self {
        match id & 0b0000_0011 {
            0 => Self::Cpu,
            1 => Self::Kbd,
            2 => Self::Scr,
            3 => Self::Mth,
            _ => unreachable!()
        }
    }
}
