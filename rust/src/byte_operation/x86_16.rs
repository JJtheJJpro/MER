use std::{io::Error, sync::RwLock};

use crate::byte_stream::ByteStream;

const OPS: [&str; 8] = ["add", "or", "adc", "sbb", "and", "sub", "xor", "cmp"];
const REG_NAMES: [&str; 8] = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const RM_NAMES: [&str; 8] = ["bx+si", "bx+di", "bp+si", "bp+di", "si", "di", "bp", "bx"];
const SEG_REG_NAMES: [&str; 4] = ["es", "cs", "ss", "ds"];

fn modrm_byte_handling(bst: &mut ByteStream) -> (u8, u8, u8, u8, u16, Option<u16>, Option<String>) {
    let mod_byte = bst.read_byte();
    let mod_s = mod_byte >> 6;
    let reg = (mod_byte >> 3) & 0b111;
    let rm = mod_byte & 0b111;

    let mut displacement = 0;
    let mut v = None;
    let mut v_s = None;

    if mod_s == 0 || mod_s == 1 || mod_s == 2 {
        match mod_s {
            0 => {
                if rm == 6 {
                    v = Some({
                        displacement = bst.read_word();
                        displacement
                    });
                    v_s = Some(format!("[0x{displacement:X}]"));
                } else {
                    v = Some(match rm {
                        0 => get_bx() + *SI.read().unwrap(),
                        1 => get_bx() + *DI.read().unwrap(),
                        2 => *BP.read().unwrap() + *SI.read().unwrap(),
                        3 => *BP.read().unwrap() + *DI.read().unwrap(),
                        4 => *SI.read().unwrap(),
                        5 => *DI.read().unwrap(),
                        7 => get_bx(),
                        _ => panic!(), // literally impossible
                    });
                    v_s = Some(format!("[{}]", RM_NAMES[rm as usize]));
                }
            }
            1 | 2 => {
                displacement = if mod_s == 1 {
                    bst.read_byte() as u16
                } else {
                    bst.read_word()
                };
                v = Some(
                    match rm {
                        0 => get_bx() + *SI.read().unwrap(),
                        1 => get_bx() + *DI.read().unwrap(),
                        2 => *BP.read().unwrap() + *SI.read().unwrap(),
                        3 => *BP.read().unwrap() + *DI.read().unwrap(),
                        4 => *SI.read().unwrap(),
                        5 => *DI.read().unwrap(),
                        6 => *BP.read().unwrap(),
                        7 => get_bx(),
                        _ => panic!(), // literally impossible
                    } + displacement,
                );
                v_s = Some(format!("[{}+0x{displacement:X}]", RM_NAMES[rm as usize]));
            }
            _ => panic!(), // literally impossible
        }
    }

    (mod_byte, mod_s, reg, rm, displacement, v, v_s)
}
fn get_parsed_reg(reg: &String) -> Result<u16, Error> {
    match reg.as_str() {
        "ax" => Ok(get_ax()),
        "bx" => Ok(get_bx()),
        "cx" => Ok(get_cx()),
        "dx" => Ok(get_dx()),
        "sp" => Ok(*SP.read().unwrap()),
        "bp" => Ok(*BP.read().unwrap()),
        "si" => Ok(*SI.read().unwrap()),
        "di" => Ok(*DI.read().unwrap()),
        &_ => Err(Error::last_os_error()),
    }
}
fn set_parsed_reg(reg: &String, v: u16) -> Result<(), Error> {
    match reg.as_str() {
        "ax" => Ok(set_ax(v)),
        "bx" => Ok(set_bx(v)),
        "cx" => Ok(set_cx(v)),
        "dx" => Ok(set_dx(v)),
        "sp" => Ok(*SP.write().unwrap() = v),
        "bp" => Ok(*BP.write().unwrap() = v),
        "si" => Ok(*SI.write().unwrap() = v),
        "di" => Ok(*DI.write().unwrap() = v),
        &_ => Err(Error::last_os_error()),
    }
}

fn get_parsed_seg_reg(seg_reg: &String) -> Result<u16, Error> {
    match seg_reg.as_str() {
        "es" => Ok(*ES.read().unwrap()),
        "cs" => Ok(*CS.read().unwrap()),
        "ss" => Ok(*SS.read().unwrap()),
        "ds" => Ok(*DS.read().unwrap()),
        &_ => Err(Error::last_os_error()),
    }
}
fn set_parsed_seg_reg(seg_reg: &String, v: u16) -> Result<(), Error> {
    match seg_reg.as_str() {
        "es" => Ok(*ES.write().unwrap() = v),
        "cs" => Ok(*CS.write().unwrap() = v),
        "ss" => Ok(*SS.write().unwrap() = v),
        "ds" => Ok(*DS.write().unwrap() = v),
        &_ => Err(Error::last_os_error()),
    }
}

pub fn op00() -> String {
    "nop".to_owned()
}
pub fn op0e(execute: bool) -> String {
    if execute {
        STACK.write().unwrap().push(*CS.read().unwrap());
    }
    "push cs".to_owned()
}
pub fn op1f(execute: bool) -> String {
    if execute {
        *DS.write().unwrap() = STACK.write().unwrap().pop().unwrap();
    }
    "pop ds".to_owned()
}
pub fn op33(execute: bool, bst: &mut ByteStream) -> String {
    let (_, mod_s, reg, rm, _, v, v_s) = modrm_byte_handling(bst);
    let destreg = REG_NAMES[reg as usize].to_owned();

    if execute {
        let regv = get_parsed_reg(&destreg).unwrap();
        if v.is_some() {
            let index = ((if rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod_s == 0) || rm == 7 {
                *DS.read().unwrap()
            } else if rm == 2 || rm == 3 || (rm == 6 && mod_s != 0) {
                *SS.read().unwrap()
            } else {
                panic!()
            } as u32)
                << 4)
                + v.unwrap() as u32;
            let r_bst = bst.read_word_at(index as usize);
            set_parsed_reg(&destreg, regv ^ r_bst).unwrap();
        } else {
            let reg_name = REG_NAMES[rm as usize].to_owned();
            set_parsed_reg(&destreg, regv ^ get_parsed_reg(&reg_name).unwrap()).unwrap();
        }
    }

    format!(
        "xor {destreg},{}",
        v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
    )
}
pub fn op50(execute: bool) -> String {
    if execute {
        STACK.write().unwrap().push(get_ax());
    }
    "push ax".to_owned()
}
pub fn op55(execute: bool) -> String {
    if execute {
        STACK.write().unwrap().push(*BP.read().unwrap());
    }
    "push bp".to_owned()
}
pub fn op56(execute: bool) -> String {
    if execute {
        STACK.write().unwrap().push(*SI.read().unwrap());
    }
    "push si".to_owned()
}
pub fn op5d(execute: bool) -> String {
    if execute {
        *BP.write().unwrap() = STACK.write().unwrap().pop().unwrap();
    }
    "pop bp".to_owned()
}
pub fn op81(execute: bool, bst: &mut ByteStream) -> String {
    let (_, _, reg, rm, _, v, v_s) = modrm_byte_handling(bst);

    let immediate = bst.read_word();
    let mnemonic = OPS[reg as usize];

    if execute {
        match mnemonic {
            "sub" => {
                if v.is_some() {
                    let vt = bst.read_word_at(v.unwrap() as usize);
                    bst.replace_word(v.unwrap() as usize, vt - immediate);
                } else {
                    let sregv = REG_NAMES[rm as usize].to_owned();
                    let regv = get_parsed_reg(&sregv).unwrap();
                    set_parsed_reg(&sregv, regv - immediate).unwrap();
                }
            }
            &_ => panic!(),
        }
    }

    format!("{} {},0x{immediate:X}", mnemonic, v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned()))
}
pub fn op83(execute: bool, bst: &mut ByteStream) -> String {
    let (_, _, reg, rm, _, v, v_s) = modrm_byte_handling(bst);

    let immediate = bst.read_sbyte();
    let mnemonic = OPS[reg as usize];

    if execute {
        match mnemonic {
            "sub" => {
                if v.is_some() {
                    let vt = bst.read_byte_at(v.unwrap() as usize);
                    bst.replace_byte(v.unwrap() as usize, ((vt as i16) - (immediate as i16)) as i8 as u8);
                } else {
                    let sregv = REG_NAMES[rm as usize].to_owned();
                    let regv = get_parsed_reg(&sregv).unwrap();
                    set_parsed_reg(&sregv, ((regv as i16) - immediate)).unwrap();
                }
            }
            &_ => panic!(),
        }
    }

    format!("{} {},0x{immediate:X}", mnemonic, v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned()))
}

/// <p>Parses code and converts it into 16-bit assembly code with the x86 instruction set.</p>
pub fn parse_byte_code(bytes: &Vec<u8>) -> Vec<String> {
    let mut bst = ByteStream::new(bytes.clone());

    let mut code = Vec::new();

    while bst.available() {
        let byte = bst.read_byte();

        match byte {
            0x00 => code.push(op00()),
            0x0E => code.push(op0e(false)),
            0x1F => code.push(op1f(false)),
            0x33 => code.push(op33(false, &mut bst)),
            _ => {}
        }
    }

    code
}

pub fn execute_byte_code(bytes: &mut Vec<u8>) -> Vec<String> {
    let mut bst = ByteStream::new(bytes.clone());

    let mut code = Vec::new();

    while bst.available() {
        let byte = bst.read_byte();

        match byte {
            0x00 => code.push(op00()),
            0x0E => code.push(op0e(true)),
            0x1F => code.push(op1f(true)),
            _ => {}
        }
    }

    code
}

pub static STACK: RwLock<Vec<u16>> = RwLock::new(Vec::new());

pub static AH: RwLock<u8> = RwLock::new(0);
pub static AL: RwLock<u8> = RwLock::new(0);
pub static BH: RwLock<u8> = RwLock::new(0);
pub static BL: RwLock<u8> = RwLock::new(0);
pub static CH: RwLock<u8> = RwLock::new(0);
pub static CL: RwLock<u8> = RwLock::new(0);
pub static DH: RwLock<u8> = RwLock::new(0);
pub static DL: RwLock<u8> = RwLock::new(0);

pub static SI: RwLock<u16> = RwLock::new(0);
pub static DI: RwLock<u16> = RwLock::new(0);
pub static BP: RwLock<u16> = RwLock::new(0);
pub static SP: RwLock<u16> = RwLock::new(0);

pub static CS: RwLock<u16> = RwLock::new(0);
pub static DS: RwLock<u16> = RwLock::new(0);
pub static SS: RwLock<u16> = RwLock::new(0);
pub static ES: RwLock<u16> = RwLock::new(0);

pub static IP: RwLock<u16> = RwLock::new(0);

pub static CF: RwLock<bool> = RwLock::new(false);
pub static PF: RwLock<bool> = RwLock::new(false);
pub static AF: RwLock<bool> = RwLock::new(false);
pub static ZF: RwLock<bool> = RwLock::new(false);
pub static SF: RwLock<bool> = RwLock::new(false);
pub static TF: RwLock<bool> = RwLock::new(false);
pub static IF: RwLock<bool> = RwLock::new(false);
pub static DF: RwLock<bool> = RwLock::new(false);
pub static OF: RwLock<bool> = RwLock::new(false);
pub static IOPL: RwLock<(bool, bool)> = RwLock::new((false, false));
pub static NT: RwLock<bool> = RwLock::new(false);

pub unsafe fn get_flags() -> u16 {
    (if *NT.read().unwrap() { 1 << 14 } else { 0 })
        | (if IOPL.read().unwrap().0 { 1 << 13 } else { 0 })
        | (if IOPL.read().unwrap().1 { 1 << 12 } else { 0 })
        | (if *OF.read().unwrap() { 1 << 11 } else { 0 })
        | (if *DF.read().unwrap() { 1 << 10 } else { 0 })
        | (if *IF.read().unwrap() { 1 << 9 } else { 0 })
        | (if *TF.read().unwrap() { 1 << 8 } else { 0 })
        | (if *SF.read().unwrap() { 1 << 7 } else { 0 })
        | (if *ZF.read().unwrap() { 1 << 6 } else { 0 })
        | (if *AF.read().unwrap() { 1 << 4 } else { 0 })
        | (if *PF.read().unwrap() { 1 << 2 } else { 0 })
        | (if *CF.read().unwrap() { 1 } else { 0 })
}
pub unsafe fn set_flags(v: u16) {
    *CF.write().unwrap() = (v & 1) == 1;
    *PF.write().unwrap() = ((v >> 2) & 1) == 1;
    *AF.write().unwrap() = ((v >> 4) & 1) == 1;
    *ZF.write().unwrap() = ((v >> 6) & 1) == 1;
    *SF.write().unwrap() = ((v >> 7) & 1) == 1;
    *TF.write().unwrap() = ((v >> 8) & 1) == 1;
    *IF.write().unwrap() = ((v >> 9) & 1) == 1;
    *DF.write().unwrap() = ((v >> 10) & 1) == 1;
    *OF.write().unwrap() = ((v >> 11) & 1) == 1;
    IOPL.write().unwrap().1 = ((v >> 12) & 1) == 1;
    IOPL.write().unwrap().0 = ((v >> 13) & 1) == 1;
    *NT.write().unwrap() = ((v >> 14) & 1) == 1;
}

pub fn get_ax() -> u16 {
    ((*AH.read().unwrap() as u16) << 8) | (*AL.read().unwrap() as u16)
}
pub fn set_ax(v: u16) {
    *AH.write().unwrap() = (v >> 8) as u8;
    *AL.write().unwrap() = (v & 0xFF) as u8;
}
pub fn get_bx() -> u16 {
    ((*BH.read().unwrap() as u16) << 8) | (*BL.read().unwrap() as u16)
}
pub fn set_bx(v: u16) {
    *BH.write().unwrap() = (v >> 8) as u8;
    *BL.write().unwrap() = (v & 0xFF) as u8;
}
pub fn get_cx() -> u16 {
    ((*CH.read().unwrap() as u16) << 8) | (*CL.read().unwrap() as u16)
}
pub fn set_cx(v: u16) {
    *CH.write().unwrap() = (v >> 8) as u8;
    *CL.write().unwrap() = (v & 0xFF) as u8;
}
pub fn get_dx() -> u16 {
    ((*DH.read().unwrap() as u16) << 8) | (*DL.read().unwrap() as u16)
}
pub fn set_dx(v: u16) {
    *DH.write().unwrap() = (v >> 8) as u8;
    *DL.write().unwrap() = (v & 0xFF) as u8;
}
