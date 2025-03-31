use std::{io::Error, sync::RwLock};

use crate::{
    apis::{dos::dos_op_cd, API},
    byte_stream::ByteStream,
    executable::InteruptChange,
};

const OPS: [&str; 8] = ["add", "or", "adc", "sbb", "and", "sub", "xor", "cmp"];
const REG_NAMES: [&str; 8] = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const REG_NAMES_8: [&str; 8] = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
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
                        0 => get_bx().unwrap() + SI.read().unwrap().unwrap(),
                        1 => get_bx().unwrap() + DI.read().unwrap().unwrap(),
                        2 => BP.read().unwrap().unwrap() + SI.read().unwrap().unwrap(),
                        3 => BP.read().unwrap().unwrap() + DI.read().unwrap().unwrap(),
                        4 => SI.read().unwrap().unwrap(),
                        5 => DI.read().unwrap().unwrap(),
                        7 => get_bx().unwrap(),
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
                        0 => get_bx().unwrap() + SI.read().unwrap().unwrap(),
                        1 => get_bx().unwrap() + DI.read().unwrap().unwrap(),
                        2 => BP.read().unwrap().unwrap() + SI.read().unwrap().unwrap(),
                        3 => BP.read().unwrap().unwrap() + DI.read().unwrap().unwrap(),
                        4 => SI.read().unwrap().unwrap(),
                        5 => DI.read().unwrap().unwrap(),
                        6 => BP.read().unwrap().unwrap(),
                        7 => get_bx().unwrap(),
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
        "ax" => Ok(get_ax().unwrap()),
        "bx" => Ok(get_bx().unwrap()),
        "cx" => Ok(get_cx().unwrap()),
        "dx" => Ok(get_dx().unwrap()),
        "sp" => Ok(SP.read().unwrap().unwrap()),
        "bp" => Ok(BP.read().unwrap().unwrap()),
        "si" => Ok(SI.read().unwrap().unwrap()),
        "di" => Ok(DI.read().unwrap().unwrap()),
        &_ => Err(Error::last_os_error()),
    }
}
fn set_parsed_reg(reg: &String, v: u16) -> Result<(), Error> {
    match reg.as_str() {
        "ax" => Ok(set_ax(v)),
        "bx" => Ok(set_bx(v)),
        "cx" => Ok(set_cx(v)),
        "dx" => Ok(set_dx(v)),
        "sp" => Ok(*SP.write().unwrap() = Some(v)),
        "bp" => Ok(*BP.write().unwrap() = Some(v)),
        "si" => Ok(*SI.write().unwrap() = Some(v)),
        "di" => Ok(*DI.write().unwrap() = Some(v)),
        &_ => Err(Error::last_os_error()),
    }
}

fn get_parsed_seg_reg(seg_reg: &String) -> Result<u16, Error> {
    match seg_reg.as_str() {
        "es" => Ok(ES.read().unwrap().unwrap()),
        "cs" => Ok(CS.read().unwrap().unwrap()),
        "ss" => Ok(SS.read().unwrap().unwrap()),
        "ds" => Ok(DS.read().unwrap().unwrap()),
        &_ => Err(Error::last_os_error()),
    }
}
fn set_parsed_seg_reg(seg_reg: &String, v: u16) -> Result<(), Error> {
    match seg_reg.as_str() {
        "es" => Ok(*ES.write().unwrap() = Some(v)),
        "cs" => Ok(*CS.write().unwrap() = Some(v)),
        "ss" => Ok(*SS.write().unwrap() = Some(v)),
        "ds" => Ok(*DS.write().unwrap() = Some(v)),
        &_ => Err(Error::last_os_error()),
    }
}

fn get_parsed_reg_8(reg: &String) -> Result<u8, Error> {
    match reg.as_str() {
        "al" => Ok(AL.read().unwrap().unwrap()),
        "cl" => Ok(CL.read().unwrap().unwrap()),
        "dl" => Ok(DL.read().unwrap().unwrap()),
        "bl" => Ok(BL.read().unwrap().unwrap()),
        "ah" => Ok(AH.read().unwrap().unwrap()),
        "ch" => Ok(CH.read().unwrap().unwrap()),
        "dh" => Ok(DH.read().unwrap().unwrap()),
        "bh" => Ok(BH.read().unwrap().unwrap()),
        &_ => Err(Error::last_os_error()),
    }
}
fn set_parsed_reg_8(reg: &String, v: u8) -> Result<(), Error> {
    match reg.as_str() {
        "al" => Ok(*AL.write().unwrap() = Some(v)),
        "cl" => Ok(*CL.write().unwrap() = Some(v)),
        "dl" => Ok(*DL.write().unwrap() = Some(v)),
        "bl" => Ok(*BL.write().unwrap() = Some(v)),
        "ah" => Ok(*AH.write().unwrap() = Some(v)),
        "ch" => Ok(*CH.write().unwrap() = Some(v)),
        "dh" => Ok(*DH.write().unwrap() = Some(v)),
        "bh" => Ok(*BH.write().unwrap() = Some(v)),
        &_ => Err(Error::last_os_error()),
    }
}

pub fn op_00(bst: &mut ByteStream) -> String {
    let (_, mod_s, reg, rm, _, v, v_s) = modrm_byte_handling(bst);
    let src = REG_NAMES_8[reg as usize].to_owned();
    let dest = v_s.unwrap_or(REG_NAMES_8[rm as usize].to_owned());

    if v.is_some() {
        let index =
            ((if rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod_s == 0) || rm == 7 {
                DS.read().unwrap().unwrap()
            } else if rm == 2 || rm == 3 || (rm == 6 && mod_s != 0) {
                SS.read().unwrap().unwrap()
            } else {
                panic!()
            } as u32)
                << 4)
                + v.unwrap() as u32;
        let r_bst = bst.read_byte_at(index as usize);
        bst.replace_byte(index as usize, r_bst);
    } else {
        let srcv = get_parsed_reg_8(&src).unwrap();
        let destv = get_parsed_reg_8(&dest).unwrap();
        set_parsed_reg_8(&dest, destv + srcv).unwrap();
    }

    format!("add {dest},{src}")
}
// 01-0d
pub fn op_0e() -> String {
    STACK.write().unwrap().push(*CS.read().unwrap());
    "push cs".to_owned()
}
// 0f-1e
pub fn op_1f() -> String {
    *DS.write().unwrap() = STACK.write().unwrap().pop().unwrap();
    "pop ds".to_owned()
}
// 20-32
pub fn op_33(bst: &mut ByteStream) -> String {
    let (_, mod_s, reg, rm, _, v, v_s) = modrm_byte_handling(bst);
    let destreg = REG_NAMES[reg as usize].to_owned();

    let regv = get_parsed_reg(&destreg).unwrap();
    if v.is_some() {
        let index =
            ((if rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod_s == 0) || rm == 7 {
                DS.read().unwrap().unwrap()
            } else if rm == 2 || rm == 3 || (rm == 6 && mod_s != 0) {
                SS.read().unwrap().unwrap()
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

    format!(
        "xor {destreg},{}",
        v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
    )
}
// 34-4f
pub fn op_50() -> String {
    STACK.write().unwrap().push(get_ax());
    "push ax".to_owned()
}
// 51-54
pub fn op_55() -> String {
    STACK.write().unwrap().push(*BP.read().unwrap());
    "push bp".to_owned()
}
pub fn op_56() -> String {
    STACK.write().unwrap().push(*SI.read().unwrap());
    "push si".to_owned()
}
// 57-5c
pub fn op_5d() -> String {
    *BP.write().unwrap() = STACK.write().unwrap().pop().unwrap();
    "pop bp".to_owned()
}
// 5e-80
pub fn op_81(bst: &mut ByteStream) -> String {
    let (_, _, reg, rm, _, v, v_s) = modrm_byte_handling(bst);

    let immediate = bst.read_word();
    let mnemonic = OPS[reg as usize];

    match mnemonic {
        "sub" => {
            if v.is_some() {
                let vt = bst.read_word_at(v.unwrap() as usize);
                bst.replace_word(v.unwrap() as usize, vt.wrapping_sub(immediate));
            } else {
                let sregv = REG_NAMES[rm as usize].to_owned();
                let regv = get_parsed_reg(&sregv).unwrap();
                set_parsed_reg(&sregv, regv.wrapping_sub(immediate)).unwrap();
            }
        }
        &_ => panic!(),
    }

    format!(
        "{} {},0x{immediate:X}",
        mnemonic,
        v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
    )
}
// 82
pub fn op_83(bst: &mut ByteStream) -> String {
    let (_, _, reg, rm, _, v, v_s) = modrm_byte_handling(bst);

    let immediate = bst.read_byte();
    let mnemonic = OPS[reg as usize];

    match mnemonic {
        "sub" => {
            if v.is_some() {
                let vt = bst.read_byte_at(v.unwrap() as usize);
                bst.replace_byte(v.unwrap() as usize, vt.wrapping_sub(immediate));
            } else {
                let sregv = REG_NAMES[rm as usize].to_owned();
                let regv = get_parsed_reg(&sregv).unwrap();
                set_parsed_reg(&sregv, regv.wrapping_sub(immediate as u16)).unwrap();
            }
        }
        &_ => panic!(),
    }

    format!(
        "{} {},0x{immediate:X}",
        mnemonic,
        v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
    )
}
// 84-8a
pub fn op_8b(bst: &mut ByteStream) -> String {
    let (_, mod_s, reg, rm, _, v, v_s) = modrm_byte_handling(bst);
    let destreg = REG_NAMES[reg as usize].to_owned();

    if v.is_some() {
        let index =
            if rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod_s == 0) || rm == 7 {
                DS.read().unwrap().unwrap()
            } else if rm == 2 || rm == 3 || (rm == 6 && mod_s != 0) {
                SS.read().unwrap().unwrap()
            } else {
                panic!()
            } << 4 + v.unwrap();

        set_parsed_reg(&destreg, bst.read_word_at(index as usize)).unwrap();
    } else {
        let regv = get_parsed_reg(&REG_NAMES[rm as usize].to_owned()).unwrap();
        set_parsed_reg(&destreg, regv).unwrap();
    }

    format!(
        "mov {destreg},{}",
        v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
    )
}
pub fn op_8c(bst: &mut ByteStream) -> String {
    let (_, mod_s, reg, rm, _, v, v_s) = modrm_byte_handling(bst);
    let seg_reg = SEG_REG_NAMES[reg as usize].to_owned();

    if v.is_some() {
        let index =
            if rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod_s == 0) || rm == 7 {
                DS.read().unwrap().unwrap()
            } else if rm == 2 || rm == 3 || (rm == 6 && mod_s != 0) {
                SS.read().unwrap().unwrap()
            } else {
                panic!()
            } << 4 + v.unwrap();

        let regv = get_parsed_seg_reg(&seg_reg).unwrap();
        bst.replace_word(index as usize, regv);
    } else {
        let regv = get_parsed_seg_reg(&seg_reg).unwrap();
        set_parsed_reg(&REG_NAMES[rm as usize].to_owned(), regv).unwrap();
    }

    format!(
        "mov {},{seg_reg}",
        v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
    )
}
pub fn op_8d(bst: &mut ByteStream) -> String {
    let (_, _, reg, rm, _, v, v_s) = modrm_byte_handling(bst);
    set_parsed_reg(
        &REG_NAMES[reg as usize].to_owned(),
        v.unwrap_or_else(|| get_parsed_reg(&REG_NAMES[rm as usize].to_owned()).unwrap()),
    )
    .unwrap();
    format!(
        "lea {},{}",
        REG_NAMES[reg as usize],
        v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
    )
}
pub fn op_8e(bst: &mut ByteStream) -> String {
    let (_, mod_s, reg, rm, _, v, v_s) = modrm_byte_handling(bst);
    let seg_reg = SEG_REG_NAMES[reg as usize].to_owned();

    if v.is_some() {
        let index =
            if rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod_s == 0) || rm == 7 {
                DS.read().unwrap().unwrap()
            } else if rm == 2 || rm == 3 || (rm == 6 && mod_s != 0) {
                SS.read().unwrap().unwrap()
            } else {
                panic!()
            } << 4 + v.unwrap();

        let regv = bst.read_word_at(index as usize);
        set_parsed_seg_reg(&seg_reg, regv).unwrap();
    } else {
        let regv = get_parsed_reg(&REG_NAMES[rm as usize].to_owned()).unwrap();
        set_parsed_seg_reg(&seg_reg, regv).unwrap();
    }

    format!(
        "mov {seg_reg},{}",
        v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
    )
}
// 8f
pub fn op_90() -> String {
    "nop".to_owned()
}
// 91-ad
pub fn op_ae(bst: &mut ByteStream) -> String {
    let ptr_val = bst.read_byte_at(
        (((ES.read().unwrap().unwrap() as u32) << 4) + DI.read().unwrap().unwrap() as u32) as usize,
    );
    *ZF.write().unwrap() = Some(ptr_val == AL.read().unwrap().unwrap());
    if DF.read().unwrap().unwrap() {
        *DI.write().unwrap() = Some(DI.read().unwrap().unwrap() - 1);
    } else {
        *DI.write().unwrap() = Some(DI.read().unwrap().unwrap() + 1);
    }

    "scasb".to_owned()
}
// af
pub fn op_b0(bst: &mut ByteStream) -> String {
    format!("mov al,0x{:X}", {
        let b = bst.read_byte();
        *AL.write().unwrap() = Some(b);
        b
    })
}
pub fn op_b1(bst: &mut ByteStream) -> String {
    format!("mov cl,0x{:X}", {
        let b = bst.read_byte();
        *CL.write().unwrap() = Some(b);
        b
    })
}
pub fn op_b2(bst: &mut ByteStream) -> String {
    format!("mov dl,0x{:X}", {
        let b = bst.read_byte();
        *DL.write().unwrap() = Some(b);
        b
    })
}
pub fn op_b3(bst: &mut ByteStream) -> String {
    format!("mov bl,0x{:X}", {
        let b = bst.read_byte();
        *BL.write().unwrap() = Some(b);
        b
    })
}
pub fn op_b4(bst: &mut ByteStream) -> String {
    format!("mov ah,0x{:X}", {
        let b = bst.read_byte();
        *AH.write().unwrap() = Some(b);
        b
    })
}
pub fn op_b5(bst: &mut ByteStream) -> String {
    format!("mov ch,0x{:X}", {
        let b = bst.read_byte();
        *CH.write().unwrap() = Some(b);
        b
    })
}
pub fn op_b6(bst: &mut ByteStream) -> String {
    format!("mov dh,0x{:X}", {
        let b = bst.read_byte();
        *DH.write().unwrap() = Some(b);
        b
    })
}
pub fn op_b7(bst: &mut ByteStream) -> String {
    format!("mov bh,0x{:X}", {
        let b = bst.read_byte();
        *BH.write().unwrap() = Some(b);
        b
    })
}
pub fn op_b8(bst: &mut ByteStream) -> String {
    format!("mov ax,0x{:X}", {
        let w = bst.read_word();
        set_ax(w);
        w
    })
}
pub fn op_b9(bst: &mut ByteStream) -> String {
    format!("mov cx,0x{:X}", {
        let w = bst.read_word();
        set_cx(w);
        w
    })
}
pub fn op_ba(bst: &mut ByteStream) -> String {
    format!("mov dx,0x{:X}", {
        let w = bst.read_word();
        set_dx(w);
        w
    })
}
pub fn op_bb(bst: &mut ByteStream) -> String {
    format!("mov bx,0x{}", {
        let w = bst.read_word();
        set_bx(w);
        w
    })
}
pub fn op_bc(bst: &mut ByteStream) -> String {
    format!("mov sp,0x{}", {
        let w = bst.read_word();
        *SP.write().unwrap() = Some(w);
        w
    })
}
pub fn op_bd(bst: &mut ByteStream) -> String {
    format!("mov bp,0x{}", {
        let w = bst.read_word();
        *BP.write().unwrap() = Some(w);
        w
    })
}
pub fn op_be(bst: &mut ByteStream) -> String {
    format!("mov si,0x{}", {
        let w = bst.read_word();
        *SI.write().unwrap() = Some(w);
        w
    })
}
pub fn op_bf(bst: &mut ByteStream) -> String {
    format!("mov di,0x{}", {
        let w = bst.read_word();
        *DI.write().unwrap() = Some(w);
        w
    })
}
// c0-c2
pub fn op_c3(bst: &mut ByteStream) -> String {
    bst.pos = STACK.write().unwrap().pop().unwrap().unwrap() as usize;
    "ret".to_owned()
}
// c4-cb
pub fn op_cd(bst: &mut ByteStream, api: &API) -> (String, InteruptChange) {
    match api {
        API::DOS => dos_op_cd(bst, false),
        _ => (format!("int {:X}h", bst.read_byte()), InteruptChange::None),
    }
}
// ce-e7
pub fn op_e8(bst: &mut ByteStream) -> String {
    STACK.write().unwrap().push(Some(bst.pos as u16));
    bst.pos += bst.read_sword() as usize;
    format!("call 0x{:04X}", bst.pos)
}
// e9-f1
pub fn op_f2(bst: &mut ByteStream, api: &API) -> String {
    let fixed_pos = bst.pos;
    let r = run_byte_code(bst, api);
    set_cx(get_cx().unwrap() - 1);
    while get_cx().unwrap() > 0 || ZF.read().unwrap().unwrap() == false {
        bst.pos = fixed_pos;
        if run_byte_code(bst, api) != r {
            panic!();
        }
    }
    format!("repne {r}")
}
pub fn op_f3(bst: &mut ByteStream, api: &API) -> String {
    let fixed_pos = bst.pos;
    let r = run_byte_code(bst, api);
    set_cx(get_cx().unwrap() - 1);
    while get_cx().unwrap() > 0 || ZF.read().unwrap().unwrap() == true {
        bst.pos = fixed_pos;
        if run_byte_code(bst, api) != r {
            panic!();
        }
    }
    format!("rep {r}")
}
// f4-f6
pub fn op_f7(bst: &mut ByteStream) -> String {
    let (_, _, reg, rm, _, v, v_s) = modrm_byte_handling(bst);

    match reg {
        0 => {
            let imm16 = bst.read_word();
            let res =
                v.unwrap_or_else(|| get_parsed_reg(&REG_NAMES[rm as usize].to_owned()).unwrap());
            *CF.write().unwrap() = Some(false);
            *OF.write().unwrap() = Some(false);
            *ZF.write().unwrap() = Some(res == 0);
            *SF.write().unwrap() = Some((res >> 15) == 1);
            *PF.write().unwrap() = Some((res & 0xFF).count_ones() % 2 == 0);
            format!(
                "test {},0x{imm16:04X}",
                v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
            )
        }
        2 => {
            if let Some(v1) = v {
                let w = bst.read_word_at(v1 as usize);
                bst.replace_word(v1 as usize, !w);
            } else {
                let w = get_parsed_reg(&REG_NAMES[rm as usize].to_owned()).unwrap();
                set_parsed_reg(&REG_NAMES[rm as usize].to_owned(), !w).unwrap();
            }

            format!(
                "not {}",
                v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
            )
        }
        3 => {
            let res = if let Some(v1) = v {
                let w = bst.read_word_at(v1 as usize);
                bst.replace_word(v1 as usize, 0u16.wrapping_sub(w));
                bst.read_word_at(v1 as usize)
            } else {
                let w = get_parsed_reg(&REG_NAMES[rm as usize].to_owned()).unwrap();
                set_parsed_reg(&REG_NAMES[rm as usize].to_owned(), 0u16.wrapping_sub(w)).unwrap();
                get_parsed_reg(&REG_NAMES[rm as usize].to_owned()).unwrap()
            };
            *CF.write().unwrap() = Some(res != 0);
            *OF.write().unwrap() = Some(res == 0x8000);
            *ZF.write().unwrap() = Some(res == 0);
            *SF.write().unwrap() = Some((res >> 15) & 0b1 == 1);
            *PF.write().unwrap() = Some((res & 0xFF).count_ones() % 2 == 0);
            // AF?
            format!(
                "neg {}",
                v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
            )
        }

        _ => panic!(),
    }
}
// f8-ff

pub fn run_byte_code(bst: &mut ByteStream, api: &API) -> String {
    let byte = bst.read_byte();

    match byte {
        0x00 => op_00(bst),
        0x0E => op_0e(),
        0x1F => op_1f(),
        0x33 => op_33(bst),
        0x50 => op_50(),
        0x55 => op_55(),
        0x56 => op_56(),
        0x5D => op_5d(),
        0x81 => op_81(bst),
        0x83 => op_83(bst),
        0x8B => op_8b(bst),
        0x8C => op_8c(bst),
        0x8D => op_8d(bst),
        0x8E => op_8e(bst),
        0xAE => op_ae(bst),
        0xB0 => op_b0(bst),
        0xB1 => op_b1(bst),
        0xB2 => op_b2(bst),
        0xB3 => op_b3(bst),
        0xB4 => op_b4(bst),
        0xB5 => op_b5(bst),
        0xB6 => op_b6(bst),
        0xB7 => op_b7(bst),
        0xB8 => op_b8(bst),
        0xB9 => op_b9(bst),
        0xBA => op_ba(bst),
        0xBB => op_bb(bst),
        0xBC => op_bc(bst),
        0xBD => op_bd(bst),
        0xBE => op_be(bst),
        0xBF => op_bf(bst),
        0xC3 => op_c3(bst),
        0xCD => {
            let (s, int) = op_cd(bst, api);
            match int {
                InteruptChange::None => {}
                InteruptChange::Exit => {
                    bst.exit();
                }
                InteruptChange::SkipString(b, e) => {
                    bst.skip_bytes_at(&(b..e).map(|x| x).collect::<Vec<usize>>());
                }
            }
            s
        }
        0xE8 => op_e8(bst),
        0xF2 => op_f2(bst, api),
        0xF3 => op_f3(bst, api),
        0xF7 => op_f7(bst),
        _ => panic!(),
    }
}

/// Executes given code with a specific API set.
pub fn run_code(bytes: &Vec<u8>, api: &API) -> Vec<String> {
    let mut bst = ByteStream::new(bytes.clone());

    let mut code = Vec::new();

    while bst.available() {
        code.push(run_byte_code(&mut bst, api));
    }

    reset_regs();
    code
}

pub fn single_interpret_code(bst: &mut ByteStream, api: &API) -> String {
    match bst.read_byte() {
        0x0E => {
            const CMD: &str = "push cs";
            let cs = *CS.read().unwrap();
            STACK.write().unwrap().push(cs);
            match cs {
                None => format!("{CMD} ; WARNING: register CS not known here"),
                Some(_) => format!("{CMD}"),
            }
        }
        0x1F => {
            const CMD: &str = "pop ds";
            match STACK.write().unwrap().pop() {
                None => format!("{CMD} ; WARNING: alloc stack empty"),
                Some(v) => {
                    *DS.write().unwrap() = v;
                    match v {
                        None => format!("{CMD} ; WARNING: popped value not known here"),
                        Some(_) => format!("{CMD}"),
                    }
                }
            }
        }
        0x90 => "nop".to_owned(),
        0xB0 => {
            const CMD: &str = "mov al,";
            if bst.available() {
                let v = bst.read_byte();
                *AL.write().unwrap() = Some(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xB1 => {
            const CMD: &str = "mov cl,";
            if bst.available() {
                let v = bst.read_byte();
                *CL.write().unwrap() = Some(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xB2 => {
            const CMD: &str = "mov dl,";
            if bst.available() {
                let v = bst.read_byte();
                *DL.write().unwrap() = Some(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xB3 => {
            const CMD: &str = "mov bl,";
            if bst.available() {
                let v = bst.read_byte();
                *BL.write().unwrap() = Some(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xB4 => {
            const CMD: &str = "mov ah,";
            if bst.available() {
                let v = bst.read_byte();
                *AH.write().unwrap() = Some(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xB5 => {
            const CMD: &str = "mov ch,";
            if bst.available() {
                let v = bst.read_byte();
                *CH.write().unwrap() = Some(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xB6 => {
            const CMD: &str = "mov dh,";
            if bst.available() {
                let v = bst.read_byte();
                *DH.write().unwrap() = Some(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xB7 => {
            const CMD: &str = "mov bh,";
            if bst.available() {
                let v = bst.read_byte();
                *BH.write().unwrap() = Some(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xB8 => {
            const CMD: &str = "mov ax,";
            if bst.available() {
                let v = bst.read_word();
                set_ax(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xB9 => {
            const CMD: &str = "mov cx,";
            if bst.available() {
                let v = bst.read_word();
                set_cx(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xBA => {
            const CMD: &str = "mov dx,";
            if bst.available() {
                let v = bst.read_word();
                set_dx(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xBB => {
            const CMD: &str = "mov bx,";
            if bst.available() {
                let v = bst.read_word();
                set_bx(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xBC => {
            const CMD: &str = "mov sp,";
            if bst.available() {
                let v = bst.read_word();
                *SP.write().unwrap() = Some(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xBD => {
            const CMD: &str = "mov bp,";
            if bst.available() {
                let v = bst.read_word();
                *BP.write().unwrap() = Some(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xBE => {
            const CMD: &str = "mov si,";
            if bst.available() {
                let v = bst.read_word();
                *SI.write().unwrap() = Some(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        0xBF => {
            const CMD: &str = "mov di,";
            if bst.available() {
                let v = bst.read_word();
                *DI.write().unwrap() = Some(v);
                format!("{CMD}0x{v:02X}")
            } else {
                format!("{CMD}? ; WARNING: end of code")
            }
        }
        
        0xCD => {
            match api {
                API::DOS => {
                    let v = dos_op_cd(bst, true);
                    match v.1 {
                        InteruptChange::Exit => {
                            bst.exit();
                        }
                        _ => {}
                    }
                    v.0
                },
                _ => format!("int {:X}h", bst.read_byte()),
            }
        }
        b => format!("byte code 0x{b:02X} at 0x{:04X} not implemented", bst.pos),
    }
}

/// Parses code without executing it (NOTE: there are instructions that can manipulate the machine code during runtime).
pub fn interpret_code(bytes: &Vec<u8>, api: &API) -> Vec<String> {
    let mut bst = ByteStream::new(bytes.clone());

    let mut code = Vec::new();

    while bst.available() {
        code.push(single_interpret_code(&mut bst, api));
    }

    reset_regs();
    code
}

fn reset_regs() {
    *AH.write().unwrap() = None;
    *AL.write().unwrap() = None;
    *BH.write().unwrap() = None;
    *BL.write().unwrap() = None;
    *CH.write().unwrap() = None;
    *CL.write().unwrap() = None;
    *DH.write().unwrap() = None;
    *DL.write().unwrap() = None;

    *SI.write().unwrap() = None;
    *DI.write().unwrap() = None;
    *BP.write().unwrap() = None;
    *SP.write().unwrap() = None;

    *CS.write().unwrap() = None;
    *DS.write().unwrap() = None;
    *SS.write().unwrap() = None;
    *ES.write().unwrap() = None;

    *IP.write().unwrap() = None;

    *CF.write().unwrap() = None;
    *PF.write().unwrap() = None;
    *AF.write().unwrap() = None;
    *ZF.write().unwrap() = None;
    *SF.write().unwrap() = None;
    *TF.write().unwrap() = None;
    *IF.write().unwrap() = None;
    *DF.write().unwrap() = None;
    *OF.write().unwrap() = None;
    *IOPL.write().unwrap() = None;
    *NT.write().unwrap() = None;
}

pub static STACK: RwLock<Vec<Option<u16>>> = RwLock::new(Vec::new());

pub static AH: RwLock<Option<u8>> = RwLock::new(None);
pub static AL: RwLock<Option<u8>> = RwLock::new(None);
pub static BH: RwLock<Option<u8>> = RwLock::new(None);
pub static BL: RwLock<Option<u8>> = RwLock::new(None);
pub static CH: RwLock<Option<u8>> = RwLock::new(None);
pub static CL: RwLock<Option<u8>> = RwLock::new(None);
pub static DH: RwLock<Option<u8>> = RwLock::new(None);
pub static DL: RwLock<Option<u8>> = RwLock::new(None);

pub static SI: RwLock<Option<u16>> = RwLock::new(None);
pub static DI: RwLock<Option<u16>> = RwLock::new(None);
pub static BP: RwLock<Option<u16>> = RwLock::new(None);
pub static SP: RwLock<Option<u16>> = RwLock::new(None);

pub static CS: RwLock<Option<u16>> = RwLock::new(None);
pub static DS: RwLock<Option<u16>> = RwLock::new(None);
pub static SS: RwLock<Option<u16>> = RwLock::new(None);
pub static ES: RwLock<Option<u16>> = RwLock::new(None);

pub static IP: RwLock<Option<u16>> = RwLock::new(None);

pub static CF: RwLock<Option<bool>> = RwLock::new(None);
pub static PF: RwLock<Option<bool>> = RwLock::new(None);
pub static AF: RwLock<Option<bool>> = RwLock::new(None);
pub static ZF: RwLock<Option<bool>> = RwLock::new(None);
pub static SF: RwLock<Option<bool>> = RwLock::new(None);
pub static TF: RwLock<Option<bool>> = RwLock::new(None);
pub static IF: RwLock<Option<bool>> = RwLock::new(None);
pub static DF: RwLock<Option<bool>> = RwLock::new(None);
pub static OF: RwLock<Option<bool>> = RwLock::new(None);
pub static IOPL: RwLock<Option<(bool, bool)>> = RwLock::new(None);
pub static NT: RwLock<Option<bool>> = RwLock::new(None);

pub unsafe fn get_flags() -> Option<u16> {
    Some(
        (if NT.read().unwrap().unwrap() {
            1 << 14
        } else {
            0
        }) | (if IOPL.read().unwrap().unwrap().0 {
            1 << 13
        } else {
            0
        }) | (if IOPL.read().unwrap().unwrap().1 {
            1 << 12
        } else {
            0
        }) | (if OF.read().unwrap().unwrap() {
            1 << 11
        } else {
            0
        }) | (if DF.read().unwrap().unwrap() {
            1 << 10
        } else {
            0
        }) | (if IF.read().unwrap().unwrap() {
            1 << 9
        } else {
            0
        }) | (if TF.read().unwrap().unwrap() {
            1 << 8
        } else {
            0
        }) | (if SF.read().unwrap().unwrap() {
            1 << 7
        } else {
            0
        }) | (if ZF.read().unwrap().unwrap() {
            1 << 6
        } else {
            0
        }) | (if AF.read().unwrap().unwrap() {
            1 << 4
        } else {
            0
        }) | (if PF.read().unwrap().unwrap() {
            1 << 2
        } else {
            0
        }) | (if CF.read().unwrap().unwrap() { 1 } else { 0 }),
    )
}
pub unsafe fn set_flags(v: u16) {
    *CF.write().unwrap() = Some((v & 1) == 1);
    *PF.write().unwrap() = Some(((v >> 2) & 1) == 1);
    *AF.write().unwrap() = Some(((v >> 4) & 1) == 1);
    *ZF.write().unwrap() = Some(((v >> 6) & 1) == 1);
    *SF.write().unwrap() = Some(((v >> 7) & 1) == 1);
    *TF.write().unwrap() = Some(((v >> 8) & 1) == 1);
    *IF.write().unwrap() = Some(((v >> 9) & 1) == 1);
    *DF.write().unwrap() = Some(((v >> 10) & 1) == 1);
    *OF.write().unwrap() = Some(((v >> 11) & 1) == 1);
    *IOPL.write().unwrap() = Some((((v >> 13) & 1) == 1, ((v >> 12) & 1) == 1));
    *NT.write().unwrap() = Some(((v >> 14) & 1) == 1);
}

pub fn get_ax() -> Option<u16> {
    Some(((AH.read().unwrap().unwrap() as u16) << 8) | (AL.read().unwrap().unwrap() as u16))
}
pub fn set_ax(v: u16) {
    *AH.write().unwrap() = Some((v >> 8) as u8);
    *AL.write().unwrap() = Some((v & 0xFF) as u8);
}
pub fn get_bx() -> Option<u16> {
    Some(((BH.read().unwrap().unwrap() as u16) << 8) | (BL.read().unwrap().unwrap() as u16))
}
pub fn set_bx(v: u16) {
    *BH.write().unwrap() = Some((v >> 8) as u8);
    *BL.write().unwrap() = Some((v & 0xFF) as u8);
}
pub fn get_cx() -> Option<u16> {
    Some(((CH.read().unwrap().unwrap() as u16) << 8) | (CL.read().unwrap().unwrap() as u16))
}
pub fn set_cx(v: u16) {
    *CH.write().unwrap() = Some((v >> 8) as u8);
    *CL.write().unwrap() = Some((v & 0xFF) as u8);
}
pub fn get_dx() -> Option<u16> {
    Some(((DH.read().unwrap().unwrap() as u16) << 8) | (DL.read().unwrap().unwrap() as u16))
}
pub fn set_dx(v: u16) {
    *DH.write().unwrap() = Some((v >> 8) as u8);
    *DL.write().unwrap() = Some((v & 0xFF) as u8);
}
