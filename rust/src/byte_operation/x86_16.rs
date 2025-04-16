/*
use std::sync::RwLock;

use crate::{
    apis::{dos::dos_op_cd, API},
    byte_stream::ByteStream,
    executable::ExecutableError,
};

const OPS: [&str; 8] = ["add", "or", "adc", "sbb", "and", "sub", "xor", "cmp"];
const REG_NAMES: [&str; 8] = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const REG_NAMES_8: [&str; 8] = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const RM_NAMES: [&str; 8] = ["bx+si", "bx+di", "bp+si", "bp+di", "si", "di", "bp", "bx"];
const SEG_REG_NAMES: [&str; 4] = ["es", "cs", "ss", "ds"];

fn modrm_byte_handling(
    bst: &mut ByteStream,
) -> Result<(u8, u8, u8, u8, u16, Option<u16>, Option<String>), ExecutableError> {
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
                    v = match rm {
                        0 => match SI.read() {
                            Ok(good_si) => match get_bx() {
                                Ok(good_bx) => match (good_bx, *good_si) {
                                    (None, None) => None,
                                    (Some(bx), None) => {
                                        eprintln!("WARNING: bx has known value while si has unknown value, assuming si is 0.");
                                        Some(bx)
                                    },
                                    (None, Some(si)) => {
                                        eprintln!("WARNING: si has known value while bx has unknown value, assuming bx is 0.");
                                        Some(si)
                                    },
                                    (Some(bx), Some(si)) => Some(bx + si),
                                },
                                Err(e) => return Err(e),
                            },
                            Err(e) => return Err(ExecutableError::from_inner(e))
                        },
                        1 => match DI.read() {
                            Ok(good_di) => match get_bx() {
                                Ok(good_bx) => match (good_bx, *good_di) {
                                    (None, None) => None,
                                    (Some(bx), None) => {
                                        eprintln!("WARNING: bx has known value while di has unknown value, assuming di is 0.");
                                        Some(bx)
                                    },
                                    (None, Some(di)) => {
                                        eprintln!("WARNING: di has known value while bx has unknown value, assuming bx is 0.");
                                        Some(di)
                                    },
                                    (Some(bx), Some(di)) => Some(bx + di),
                                },
                                Err(e) => return Err(e),
                            },
                            Err(e) => return Err(ExecutableError::from_inner(e))
                        },
                        2 => match BP.read() {
                            Ok(good_bp) => match SI.read() {
                                Ok(good_si) => match (*good_bp, *good_si) {
                                    (None, None) => None,
                                    (Some(bp), None) => {
                                        eprintln!("WARNING: bp has known value while si has unknown value, assuming si is 0.");
                                        Some(bp)
                                    },
                                    (None, Some(si)) => {
                                        eprintln!("WARNING: si has known value while bp has unknown value, assuming bp is 0.");
                                        Some(si)
                                    },
                                    (Some(bp), Some(si)) => Some(bp + si),
                                },
                                Err(e) => return Err(ExecutableError::from_inner(e)),
                            },
                            Err(e) => return Err(ExecutableError::from_inner(e))
                        },
                        3 => match BP.read() {
                            Ok(good_bp) => match DI.read() {
                                Ok(good_di) => match (*good_bp, *good_di) {
                                    (None, None) => None,
                                    (Some(bp), None) => {
                                        eprintln!("WARNING: bp has known value while di has unknown value, assuming di is 0.");
                                        Some(bp)
                                    },
                                    (None, Some(di)) => {
                                        eprintln!("WARNING: di has known value while bp has unknown value, assuming bp is 0.");
                                        Some(di)
                                    },
                                    (Some(bp), Some(di)) => Some(bp + di),
                                },
                                Err(e) => return Err(ExecutableError::from_inner(e)),
                            },
                            Err(e) => return Err(ExecutableError::from_inner(e)),
                        },
                        4 => match SI.read() {
                            Ok(good) => *good,
                            Err(e) => return Err(ExecutableError::from_inner(e)),
                        },
                        5 => match DI.read() {
                            Ok(good) => *good,
                            Err(e) => return Err(ExecutableError::from_inner(e)),
                        },
                        7 => match get_bx() {
                            Ok(good) => good,
                            Err(e) => return Err(e),
                        },
                        _ => unreachable!("impossible value reached: modrm_byte_handling -> mod_s == 0 -> rm == {rm} !!!ILLEGAL VALUE!!!"), // literally impossible
                    };
                    v_s = Some(format!("[{}]", RM_NAMES[rm as usize]));
                }
            }
            1 | 2 => {
                displacement = if mod_s == 1 {
                    bst.read_byte() as u16
                } else {
                    bst.read_word()
                };
                v = match rm {
                    0 => match SI.read() {
                            Ok(good_si) => match get_bx() {
                                Ok(good_bx) => match (good_bx, *good_si) {
                                    (None, None) => None,
                                    (Some(bx), None) => {
                                        eprintln!("WARNING: bx has known value while si has unknown value, assuming si is 0.");
                                        Some(bx)
                                    },
                                    (None, Some(si)) => {
                                        eprintln!("WARNING: si has known value while bx has unknown value, assuming bx is 0.");
                                        Some(si)
                                    },
                                    (Some(bx), Some(si)) => Some(bx + si),
                                },
                                Err(e) => return Err(e),
                            },
                            Err(e) => return Err(ExecutableError::from_inner(e))
                    },
                    1 => match DI.read() {
                            Ok(good_di) => match get_bx() {
                                Ok(good_bx) => match (good_bx, *good_di) {
                                    (None, None) => None,
                                    (Some(bx), None) => {
                                        eprintln!("WARNING: bx has known value while di has unknown value, assuming di is 0.");
                                        Some(bx)
                                    },
                                    (None, Some(di)) => {
                                        eprintln!("WARNING: di has known value while bx has unknown value, assuming bx is 0.");
                                        Some(di)
                                    },
                                    (Some(bx), Some(di)) => Some(bx + di),
                                },
                                Err(e) => return Err(e),
                            },
                            Err(e) => return Err(ExecutableError::from_inner(e))
                    },
                    2 => match BP.read() {
                            Ok(good_bp) => match SI.read() {
                                Ok(good_si) => match (*good_bp, *good_si) {
                                    (None, None) => None,
                                    (Some(bp), None) => {
                                        eprintln!("WARNING: bp has known value while si has unknown value, assuming si is 0.");
                                        Some(bp)
                                    },
                                    (None, Some(si)) => {
                                        eprintln!("WARNING: si has known value while bp has unknown value, assuming bp is 0.");
                                        Some(si)
                                    },
                                    (Some(bp), Some(si)) => Some(bp + si),
                                },
                                Err(e) => return Err(ExecutableError::from_inner(e)),
                            },
                            Err(e) => return Err(ExecutableError::from_inner(e))
                    },
                    3 => match BP.read() {
                            Ok(good_bp) => match DI.read() {
                                Ok(good_di) => match (*good_bp, *good_di) {
                                    (None, None) => None,
                                    (Some(bp), None) => {
                                        eprintln!("WARNING: bp has known value while di has unknown value, assuming di is 0.");
                                        Some(bp)
                                    },
                                    (None, Some(di)) => {
                                        eprintln!("WARNING: di has known value while bp has unknown value, assuming bp is 0.");
                                        Some(di)
                                    },
                                    (Some(bp), Some(di)) => Some(bp + di),
                                },
                                Err(e) => return Err(ExecutableError::from_inner(e)),
                            },
                            Err(e) => return Err(ExecutableError::from_inner(e)),
                    },
                    4 => match SI.read() {
                            Ok(good) => *good,
                            Err(e) => return Err(ExecutableError::from_inner(e)),
                    },
                    5 => match DI.read() {
                            Ok(good) => *good,
                            Err(e) => return Err(ExecutableError::from_inner(e)),
                    },
                    6 => match BP.read() {
                            Ok(good) => *good,
                            Err(e) => return Err(ExecutableError::from_inner(e)),
                    },
                    7 => match get_bx() {
                            Ok(good) => good,
                            Err(e) => return Err(e),
                    },
                    _ => unreachable!("impossible value reached: modrm_byte_handling -> mod_s == {mod_s} -> rm == {rm} !!!ILLEGAL VALUE!!!"), // literally impossible
                };
                match v {
                    Some(value) => v = Some(value + displacement),
                    None => {
                        eprintln!("WARNING: v value is unknown; using displacement value to avoid mod usage confusion.");
                        v = Some(displacement);
                    }
                }
                v_s = Some(format!("[{}+0x{displacement:X}]", RM_NAMES[rm as usize]));
            }
            _ => unreachable!("checked from 0, 1, and 2; matched {mod_s} afterwards"),
        }
    }

    Ok((mod_byte, mod_s, reg, rm, displacement, v, v_s))
}
fn get_parsed_reg(reg: &String) -> Result<Option<u16>, ExecutableError> {
    match reg.as_str() {
        "ax" => match get_ax() {
            Ok(v) => Ok(v),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "cx" => match get_cx() {
            Ok(v) => Ok(v),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "dx" => match get_dx() {
            Ok(v) => Ok(v),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "bx" => match get_bx() {
            Ok(v) => Ok(v),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "sp" => match SP.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "bp" => match BP.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "si" => match SI.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "di" => match DI.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        n => Err(ExecutableError::from_message(format!(
            "invalid registry name '{n}' (must be ax, bx, cx, dx, sp, bp, si, or di)"
        ))),
    }
}
fn set_parsed_reg(reg: &String, v: u16) -> Result<(), ExecutableError> {
    match reg.as_str() {
        "ax" => set_ax(v),
        "bx" => set_bx(v),
        "cx" => set_cx(v),
        "dx" => set_dx(v),
        "sp" => match SP.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "bp" => match BP.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "si" => match SI.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "di" => match DI.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        n => Err(ExecutableError::from_message(format!(
            "invalid registry name '{n}' (must be ax, bx, cx, dx, sp, bp, si, or di)"
        ))),
    }
}

fn get_parsed_seg_reg(seg_reg: &String) -> Result<Option<u16>, ExecutableError> {
    match seg_reg.as_str() {
        "es" => match ES.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "cs" => match CS.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "ss" => match SS.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "ds" => match DS.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        n => Err(ExecutableError::from_message(format!(
            "invalid registry name '{n}' (must be es, cs, ss, or ds)"
        ))),
    }
}
fn set_parsed_seg_reg(seg_reg: &String, v: u16) -> Result<(), ExecutableError> {
    match seg_reg.as_str() {
        "es" => match ES.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "cs" => match CS.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "ss" => match SS.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "ds" => match DS.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        n => Err(ExecutableError::from_message(format!(
            "invalid registry name '{n}' (must be es, cs, ss, or ds)"
        ))),
    }
}

fn get_parsed_reg_8(reg: &String) -> Result<Option<u8>, ExecutableError> {
    match reg.as_str() {
        "al" => match AL.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "cl" => match CL.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "dl" => match DL.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "bl" => match BL.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "ah" => match AH.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "ch" => match CH.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "dh" => match DH.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "bh" => match BH.read() {
            Ok(vr) => Ok(*vr),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        n => Err(ExecutableError::from_message(format!(
            "invalid registry name '{n}' (must be al, cl, dl, bl, ah, ch, dh, or bh)"
        ))),
    }
}
fn set_parsed_reg_8(reg: &String, v: u8) -> Result<(), ExecutableError> {
    match reg.as_str() {
        "al" => match AL.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "cl" => match CL.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "dl" => match DL.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "bl" => match BL.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "ah" => match AH.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "ch" => match CH.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "dh" => match DH.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        "bh" => match BH.write() {
            Ok(mut vr) => Ok(*vr = Some(v)),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        n => Err(ExecutableError::from_message(format!(
            "invalid registry name '{n}' (must be al, cl, dl, bl, ah, ch, dh, or bh)"
        ))),
    }
}

pub fn op_00(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let (_, mod_s, reg, rm, _, v, v_s) = match modrm_byte_handling(bst) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let src = REG_NAMES_8[reg as usize].to_owned();
    let dest = v_s.unwrap_or(REG_NAMES_8[rm as usize].to_owned());

    match v {
        Some(v) => {
            let index =
                ((if rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod_s == 0) || rm == 7
                {
                    match DS.read() {
                        Ok(vr) => match *vr {
                            Some(vrg) => vrg,
                            None => {
                                eprintln!(
                                    "WARNING: ds is unknown; assuming value of 0 (opcode 0x00, v)"
                                );
                                0
                            }
                        },
                        Err(e) => return Err(ExecutableError::from_inner(e)),
                    }
                } else if rm == 2 || rm == 3 || (rm == 6 && mod_s != 0) {
                    match SS.read() {
                        Ok(vr) => match *vr {
                            Some(vrg) => vrg,
                            None => {
                                eprintln!(
                                    "WARNING: ss is unknown; assuming value of 0 (opcode 0x00, v)"
                                );
                                0
                            }
                        },
                        Err(e) => return Err(ExecutableError::from_inner(e)),
                    }
                } else {
                    unreachable!(
                        "impossible value reached: op_00 -> rm == {rm} !!!ILLEGAL VALUE!!!"
                    )
                } as u32)
                    << 4)
                    + v as u32;
            let r_bst = bst.read_byte_at(index as usize);
            bst.replace_byte(index as usize, r_bst);
        }
        None => {
            let srcv = match get_parsed_reg_8(&src) {
                Ok(vr) => match vr {
                    Some(v) => v,
                    None => {
                        eprintln!(
                            "WARNING: {src} is unknown; assuming value of 0 (opcode 0x00, !v)"
                        );
                        0
                    }
                },
                Err(e) => return Err(e),
            };
            let destv = match get_parsed_reg_8(&dest) {
                Ok(vr) => match vr {
                    Some(v) => v,
                    None => {
                        eprintln!(
                            "WARNING: {dest} is unknown; assuming value of 0 (opcode 0x00, !v)"
                        );
                        0
                    }
                },
                Err(e) => return Err(e),
            };
            if let Err(e) = set_parsed_reg_8(&dest, destv + srcv) {
                return Err(e);
            }
        }
    }

    Ok(format!("add {dest},{src}"))
}
// 01-0d
pub fn op_0e() -> Result<String, ExecutableError> {
    match STACK.write() {
        Ok(mut good) => match CS.read() {
            Ok(cs) => good.push(*cs),
            Err(e) => return Err(ExecutableError::from_inner(e)),
        },
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }

    Ok("push cs".to_owned())
}
// 0f-1e
pub fn op_1f() -> Result<String, ExecutableError> {
    match STACK.write() {
        Ok(mut good) => match DS.write() {
            Ok(mut ds) => match good.pop() {
                Some(v) => *ds = v,
                None => return Err(ExecutableError::from_message("stack is empty")),
            },
            Err(e) => return Err(ExecutableError::from_inner(e)),
        },
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }

    Ok("pop ds".to_owned())
}
// 20-32
pub fn op_33(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let (_, mod_s, reg, rm, _, v, v_s) = match modrm_byte_handling(bst) {
        Ok(good) => good,
        Err(e) => return Err(e),
    };
    let destreg = REG_NAMES[reg as usize].to_owned();

    let regv = match get_parsed_reg(&destreg) {
        Ok(good) => good,
        Err(e) => return Err(e),
    };
    match v {
        Some(value) => {
            let index =
                ((if rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod_s == 0) || rm == 7
                {
                    match DS.read() {
                        Ok(vr) => match *vr {
                            Some(vrg) => vrg,
                            None => {
                                eprintln!(
                                    "WARNING: ds is unknown; assuming value of 0 (opcode 0x33, v)"
                                );
                                0
                            }
                        },
                        Err(e) => return Err(ExecutableError::from_inner(e)),
                    }
                } else if rm == 2 || rm == 3 || (rm == 6 && mod_s != 0) {
                    match SS.read() {
                        Ok(vr) => match *vr {
                            Some(vrg) => vrg,
                            None => {
                                eprintln!(
                                    "WARNING: ss is unknown; assuming value of 0 (opcode 0x33, v)"
                                );
                                0
                            }
                        },
                        Err(e) => return Err(ExecutableError::from_inner(e)),
                    }
                } else {
                    unreachable!()
                } as u32)
                    << 4)
                    + value as u32;
            let r_bst = bst.read_word_at(index as usize);
            match regv {
                Some(reg_val) => {
                    if let Err(e) = set_parsed_reg(&destreg, reg_val ^ r_bst) {
                        return Err(e);
                    }
                }
                None => {
                    eprintln!("WARNING: {destreg} is unknown; assuming value is 0.");
                    if let Err(e) = set_parsed_reg(&destreg, 0 ^ r_bst) {
                        return Err(e);
                    }
                }
            }
        }
        None => {
            let reg_name = REG_NAMES[rm as usize].to_owned();
            match get_parsed_reg(&reg_name) {
                Ok(r1) => match r1 {
                    Some(rr1) => match regv {
                        Some(reg_val) => {
                            if let Err(e) = set_parsed_reg(&destreg, reg_val ^ rr1) {
                                return Err(e);
                            }
                        }
                        None => {
                            eprintln!("WARNING: {destreg} is unknown; assuming value is 0.");
                            if let Err(e) = set_parsed_reg(&destreg, 0 ^ rr1) {
                                return Err(e);
                            }
                        }
                    },
                    None => {
                        eprintln!("WARNING: {reg_name} is unknown; assuming value is 0.");
                        match regv {
                            Some(reg_val) => {
                                if let Err(e) = set_parsed_reg(&destreg, reg_val ^ 0) {
                                    return Err(e);
                                }
                            }
                            None => {
                                eprintln!("WARNING: {destreg} is unknown; assuming value is 0.");
                                if let Err(e) = set_parsed_reg(&destreg, 0 ^ 0) {
                                    return Err(e);
                                }
                            }
                        }
                    }
                },
                Err(e) => return Err(e),
            }
        }
    }

    Ok(format!(
        "xor {destreg},{}",
        v_s.unwrap_or(REG_NAMES[rm as usize].to_owned())
    ))
}
// 34-3b
pub fn op_3c(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let imm8 = bst.read_byte();
    let al = match AL.read() {
        Ok(good_al) => match *good_al {
            Some(al) => al,
            None => {
                eprintln!("WARNING: al is unknown; assuming value is 0 (opcode 0x3C)");
                0
            }
        },
        Err(e) => return Err(ExecutableError::from_inner(e)),
    };
    let res = al.wrapping_sub(imm8);

    match ZF.write() {
        Ok(mut good_zf) => *good_zf = Some(res == 0),
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }
    match SF.write() {
        Ok(mut good_sf) => *good_sf = Some((res & 0x80) != 0),
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }
    match CF.write() {
        Ok(mut good_cf) => *good_cf = Some(al < imm8),
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }
    match OF.write() {
        Ok(mut good_of) => {
            *good_of = Some(
                (((al & 0x80) != 0) != ((imm8 & 0x80) != 0))
                    && (((res & 0x80) != 0) != ((al & 0x80) != 0)),
            )
        }
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }
    match PF.write() {
        Ok(mut good_pf) => *good_pf = Some((res.count_ones() % 2) == 0),
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }

    Ok(format!("cmp al,0x{imm8:02X}"))
}
// 3d-4f
pub fn op_50() -> Result<String, ExecutableError> {
    match STACK.write() {
        Ok(mut good) => match get_ax() {
            Ok(ax) => good.push(ax),
            Err(e) => return Err(e),
        },
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }

    Ok("push ax".to_owned())
}
// 51-54
pub fn op_55() -> Result<String, ExecutableError> {
    match STACK.write() {
        Ok(mut good) => match BP.read() {
            Ok(bp) => good.push(*bp),
            Err(e) => return Err(ExecutableError::from_inner(e)),
        },
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }

    Ok("push bp".to_owned())
}
pub fn op_56() -> Result<String, ExecutableError> {
    match STACK.write() {
        Ok(mut good) => match SI.read() {
            Ok(si) => good.push(*si),
            Err(e) => return Err(ExecutableError::from_inner(e)),
        },
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }

    Ok("push si".to_owned())
}
// 57-5c
pub fn op_5d() -> Result<String, ExecutableError> {
    match STACK.write() {
        Ok(mut good) => match BP.write() {
            Ok(mut bp) => match good.pop() {
                Some(v) => *bp = v,
                None => return Err(ExecutableError::from_message("stack is empty")),
            },
            Err(e) => return Err(ExecutableError::from_inner(e)),
        },
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }

    Ok("pop bp".to_owned())
}
// 5e-73
pub fn op_74(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let disp8 = bst.read_byte();
    match ZF.read() {
        Ok(good_zf) => match *good_zf {
            Some(zf) => {
                if zf {
                    bst.pos = ((bst.pos as u16).wrapping_add(disp8 as u16)) as usize;
                }
            }
            None => {
                eprintln!("WARNING: zf is unknown; assuming value is 0 (opcode 0x74)");
            }
        },
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }
    Ok(format!("je 0x{disp8:02X}"))
}
// 75-80
pub fn op_81(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let (_, _, reg, rm, _, v, v_s) = match modrm_byte_handling(bst) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let immediate = bst.read_word();
    let mnemonic = OPS[reg as usize];

    match mnemonic {
        "sub" => match v {
            Some(v) => {
                let vt = bst.read_word_at(v as usize);
                bst.replace_word(v as usize, vt.wrapping_sub(immediate));
            }
            None => {
                let sregv = REG_NAMES[rm as usize].to_owned();
                match get_parsed_reg(&sregv) {
                    Ok(regv) => if let Err(e) = set_parsed_reg(&sregv, {match regv {
                        Some(regv) => regv,
                        None => {
                            eprintln!("WARNING: {sregv} is unknown; assuming value is 0 (opcode 0x81, sub)");
                            0u16
                        }
                    }}.wrapping_sub(immediate)) {
                        return Err(e);
                    }
                    Err(e) => return Err(e),
                };
            }
        },
        &_ => todo!("need to create mnemonic '{mnemonic}' in opcode 0x81"),
    }

    Ok(format!(
        "{} {},0x{immediate:X}",
        mnemonic,
        v_s.unwrap_or(REG_NAMES[rm as usize].to_owned())
    ))
}
// 82
pub fn op_83(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let (_, _, reg, rm, _, v, v_s) = match modrm_byte_handling(bst) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let immediate = bst.read_byte();
    let mnemonic = OPS[reg as usize];

    match mnemonic {
        "sub" => match v {
            Some(v) => {
                let vt = bst.read_byte_at(v as usize);
                bst.replace_byte(v as usize, vt.wrapping_sub(immediate));
            }
            None => {
                let sregv = REG_NAMES[rm as usize].to_owned();
                match get_parsed_reg(&sregv) {
                    Ok(regv) => if let Err(e) = set_parsed_reg(&sregv, {match regv {
                        Some(regv) => regv,
                        None => {
                            eprintln!("WARNING: {sregv} is unknown; assuming value is 0 (opcode 0x81, sub)");
                            0u16
                        }
                    }}.wrapping_sub(immediate as u16)) {
                        return Err(e);
                    }
                    Err(e) => return Err(e),
                }
            }
        },
        &_ => todo!("need to create mnemonic '{mnemonic}' in opcode 0x83"),
    }

    Ok(format!(
        "{} {},0x{immediate:X}",
        mnemonic,
        v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
    ))
}
// 84-8a
pub fn op_8b(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let (_, mod_s, reg, rm, _, v, v_s) = match modrm_byte_handling(bst) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let destreg = REG_NAMES[reg as usize].to_owned();

    match v {
        Some(v) => {
            let index =
                if rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod_s == 0) || rm == 7 {
                    match DS.read() {
                        Ok(vr) => match *vr {
                            Some(vrg) => vrg,
                            None => {
                                eprintln!(
                                    "WARNING: ds is unknown; assuming value of 0 (opcode 0x00, v)"
                                );
                                0
                            }
                        },
                        Err(e) => return Err(ExecutableError::from_inner(e)),
                    }
                } else if rm == 2 || rm == 3 || (rm == 6 && mod_s != 0) {
                    match SS.read() {
                        Ok(vr) => match *vr {
                            Some(vrg) => vrg,
                            None => {
                                eprintln!(
                                    "WARNING: ss is unknown; assuming value of 0 (opcode 0x00, v)"
                                );
                                0
                            }
                        },
                        Err(e) => return Err(ExecutableError::from_inner(e)),
                    }
                } else {
                    panic!()
                } << 4 + v;

            if let Err(e) = set_parsed_reg(&destreg, bst.read_word_at(index as usize)) {
                return Err(e);
            }
        }
        None => match get_parsed_reg(&REG_NAMES[rm as usize].to_owned()) {
            Ok(regv) => {
                if let Err(e) = set_parsed_reg(&destreg, {
                    match regv {
                        Some(regv) => regv,
                        None => {
                            eprintln!(
                                "WARNING: {} is unknown; assuming value is 0 (opcode 0x8B)",
                                REG_NAMES[rm as usize]
                            );
                            0
                        }
                    }
                }) {
                    return Err(e);
                }
            }
            Err(e) => return Err(e),
        },
    }

    Ok(format!(
        "mov {destreg},{}",
        v_s.unwrap_or(REG_NAMES[rm as usize].to_owned())
    ))
}
pub fn op_8c(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let (_, mod_s, reg, rm, _, v, v_s) = match modrm_byte_handling(bst) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let seg_reg = SEG_REG_NAMES[reg as usize].to_owned();

    match v {
        Some(v) => {
            let index =
                if rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod_s == 0) || rm == 7 {
                    match DS.read() {
                        Ok(vr) => match *vr {
                            Some(vrg) => vrg,
                            None => {
                                eprintln!(
                                    "WARNING: ds is unknown; assuming value of 0 (opcode 0x00, v)"
                                );
                                0
                            }
                        },
                        Err(e) => return Err(ExecutableError::from_inner(e)),
                    }
                } else if rm == 2 || rm == 3 || (rm == 6 && mod_s != 0) {
                    match SS.read() {
                        Ok(vr) => match *vr {
                            Some(vrg) => vrg,
                            None => {
                                eprintln!(
                                    "WARNING: ss is unknown; assuming value of 0 (opcode 0x00, v)"
                                );
                                0
                            }
                        },
                        Err(e) => return Err(ExecutableError::from_inner(e)),
                    }
                } else {
                    panic!()
                } << 4 + v;

            match get_parsed_seg_reg(&seg_reg) {
                Ok(regv) => {
                    match regv {
                        Some(regv) => bst.replace_word(index as usize, regv),
                        None => {
                            eprintln!("WARNING: {seg_reg} is unknown; assuming value is 0 (opcode 0x8C, v)");
                            bst.replace_word(index as usize, 0);
                        }
                    }
                }
                Err(e) => return Err(e),
            }
        }
        None => match get_parsed_seg_reg(&seg_reg) {
            Ok(regv) => {
                if let Err(e) = set_parsed_reg(&REG_NAMES[rm as usize].to_owned(), {
                    match regv {
                        Some(regv) => regv,
                        None => {
                            eprintln!("WARNING: {seg_reg} is unknown; assuming value is 0 (opcode 0x8C, !v)");
                            0
                        }
                    }
                }) {
                    return Err(e);
                }
            }
            Err(e) => return Err(e),
        },
    }

    Ok(format!(
        "mov {},{seg_reg}",
        v_s.unwrap_or(REG_NAMES[rm as usize].to_owned())
    ))
}
pub fn op_8d(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let (_, _, reg, rm, _, v, v_s) = match modrm_byte_handling(bst) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    if let Err(e) = set_parsed_reg(
        &REG_NAMES[reg as usize].to_owned(),
        match v {
            Some(v) => v,
            None => match get_parsed_reg(&REG_NAMES[rm as usize].to_owned()) {
                Ok(regv) => match regv {
                    Some(regv) => regv,
                    None => {
                        eprintln!(
                            "WARNING: {} is unknown; assuming value is 0 (opcode 0x8D)",
                            REG_NAMES[rm as usize]
                        );
                        0
                    }
                },
                Err(e) => return Err(e),
            },
        },
    ) {
        return Err(e);
    }
    Ok(format!(
        "lea {},{}",
        REG_NAMES[reg as usize],
        v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
    ))
}
pub fn op_8e(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let (_, mod_s, reg, rm, _, v, v_s) = match modrm_byte_handling(bst) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let seg_reg = SEG_REG_NAMES[reg as usize].to_owned();

    match v {
        Some(v) => {
            let index =
                if rm == 0 || rm == 1 || rm == 4 || rm == 5 || (rm == 6 && mod_s == 0) || rm == 7 {
                    match DS.read() {
                        Ok(vr) => match *vr {
                            Some(vrg) => vrg,
                            None => {
                                eprintln!(
                                    "WARNING: ds is unknown; assuming value of 0 (opcode 0x00, v)"
                                );
                                0
                            }
                        },
                        Err(e) => return Err(ExecutableError::from_inner(e)),
                    }
                } else if rm == 2 || rm == 3 || (rm == 6 && mod_s != 0) {
                    match SS.read() {
                        Ok(vr) => match *vr {
                            Some(vrg) => vrg,
                            None => {
                                eprintln!(
                                    "WARNING: ss is unknown; assuming value of 0 (opcode 0x00, v)"
                                );
                                0
                            }
                        },
                        Err(e) => return Err(ExecutableError::from_inner(e)),
                    }
                } else {
                    panic!()
                } << 4 + v;

            let regv = bst.read_word_at(index as usize);
            if let Err(e) = set_parsed_seg_reg(&seg_reg, regv) {
                return Err(e);
            }
        }
        None => match get_parsed_reg(&REG_NAMES[rm as usize].to_owned()) {
            Ok(regv) => {
                if let Err(e) = set_parsed_seg_reg(&seg_reg, {
                    match regv {
                        Some(regv) => regv,
                        None => {
                            eprintln!(
                                "WARNING: {} is unknown; assuming value of 0 (opcode 0x8E)",
                                REG_NAMES[rm as usize]
                            );
                            0
                        }
                    }
                }) {
                    return Err(e);
                }
            }
            Err(e) => return Err(e),
        },
    }

    Ok(format!(
        "mov {seg_reg},{}",
        v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
    ))
}
// 8f
pub fn op_90() -> String {
    "nop".to_owned()
}
// 91-ab
pub fn op_ac(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let ptr = (match DS.read() {
        Ok(good_ds) => match *good_ds {
            Some(ds) => ds,
            None => {
                eprintln!("WARNING: ds is unknown; assuming value is 0 (opcode 0xAC)");
                0
            }
        },
        Err(e) => return Err(ExecutableError::from_inner(e)),
    } << 4)
        + match SI.read() {
            Ok(good_si) => match *good_si {
                Some(si) => si,
                None => {
                    eprintln!("WARNING: si is unknown; assuming value is 0 (opcode 0xAC)");
                    0
                }
            },
            Err(e) => return Err(ExecutableError::from_inner(e)),
        };

    match AL.write() {
        Ok(mut good_al) => *good_al = Some(bst.read_byte_at(ptr as usize)),
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }

    match DF.read() {
        Ok(good_df) => match SI.write() {
            Ok(mut good_si) => match *good_df {
                Some(df) => {
                    if df {
                        *good_si = Some(good_si.unwrap_or(0) - 1);
                    } else {
                        *good_si = Some(good_si.unwrap_or(0) + 1);
                    }
                }
                None => {
                    eprintln!("WARNING: df is unknown; assuming value is 0 (opcode 0xAC)");
                    *good_si = Some(good_si.unwrap_or(0) + 1);
                }
            },
            Err(e) => return Err(ExecutableError::from_inner(e)),
        },
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }

    Ok("lodsb".to_owned())
}
// ad
pub fn op_ae(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let ptr_val = bst.read_byte_at(match ES.read() {
        Ok(good_es) => match DI.read() {
            Ok(good_di) => match (*good_es, *good_di) {
                (None, None) => {
                    eprintln!(
                        "WARNING: es and di are unknown; assuming values are 0 (opcode 0xAE)"
                    );
                    0
                }
                (Some(es), None) => {
                    eprintln!("WARNING: es is unknown; assuming value is 0 (opcode 0xAE)");
                    (es as u32) << 4
                }
                (None, Some(di)) => {
                    eprintln!("WARNING: di is unknown; assuming value is 0 (opcode 0xAE)");
                    di as u32
                }
                (Some(es), Some(di)) => ((es as u32) << 4) + di as u32,
            },
            Err(e) => return Err(ExecutableError::from_inner(e)),
        },
        Err(e) => return Err(ExecutableError::from_inner(e)),
    } as usize);

    match ZF.write() {
        Ok(mut good_zf) => match AL.read() {
            Ok(good_al) => match *good_al {
                Some(al) => *good_zf = Some(ptr_val == al),
                None => {
                    eprintln!("WARNING: al is unknown; assuming value is 0 (opcode 0xAE)");
                    *good_zf = Some(ptr_val == 0)
                }
            },
            Err(e) => return Err(ExecutableError::from_inner(e)),
        },
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }

    match DI.write() {
        Ok(mut good_di) => {
            match DF.read() {
                Ok(good_df) => {
                    match *good_df {
                        Some(df) => {
                            if df {
                                *good_di = Some(
                                    match *good_di {
                                        Some(di) => di,
                                        None => {
                                            eprintln!("WARNING: di is unknown; assuming value is 0xFFFF (opcode 0xAE)");
                                            0xFFFF
                                        }
                                    } - 1,
                                );
                            } else {
                                *good_di = Some(
                                    match *good_di {
                                        Some(di) => di,
                                        None => {
                                            eprintln!("WARNING: di is unknown; assuming value is 0 (opcode 0xAE)");
                                            0
                                        }
                                    } + 1,
                                );
                            }
                        }
                        None => {
                            eprintln!("WARNING: df is unknown; assuming value is 0 (opcode 0xAE)");
                            *good_di = Some(
                                match *good_di {
                                    Some(di) => di,
                                    None => {
                                        eprintln!("WARNING: di is unknown; assuming value is 0 (opcode 0xAE)");
                                        0
                                    }
                                } + 1,
                            );
                        }
                    }
                }
                Err(e) => return Err(ExecutableError::from_inner(e)),
            }
        }
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }

    Ok("scasb".to_owned())
}
// af
pub fn op_b0(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match AL.write() {
        Ok(mut good_al) => Ok(format!("mov al,0x{:X}", {
            let b = bst.read_byte();
            *good_al = Some(b);
            b
        })),
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn op_b1(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match CL.write() {
        Ok(mut good_cl) => Ok(format!("mov cl,0x{:X}", {
            let b = bst.read_byte();
            *good_cl = Some(b);
            b
        })),
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn op_b2(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match DL.write() {
        Ok(mut good_dl) => Ok(format!("mov dl,0x{:X}", {
            let b = bst.read_byte();
            *good_dl = Some(b);
            b
        })),
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn op_b3(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match BL.write() {
        Ok(mut good_bl) => Ok(format!("mov bl,0x{:X}", {
            let b = bst.read_byte();
            *good_bl = Some(b);
            b
        })),
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn op_b4(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match AH.write() {
        Ok(mut good_ah) => Ok(format!("mov ah,0x{:X}", {
            let b = bst.read_byte();
            *good_ah = Some(b);
            b
        })),
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn op_b5(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match CH.write() {
        Ok(mut good_ch) => Ok(format!("mov ch,0x{:X}", {
            let b = bst.read_byte();
            *good_ch = Some(b);
            b
        })),
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn op_b6(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match DH.write() {
        Ok(mut good_dh) => Ok(format!("mov dh,0x{:X}", {
            let b = bst.read_byte();
            *good_dh = Some(b);
            b
        })),
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn op_b7(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match BH.write() {
        Ok(mut good_bh) => Ok(format!("mov bh,0x{:X}", {
            let b = bst.read_byte();
            *good_bh = Some(b);
            b
        })),
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn op_b8(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    Ok(format!("mov ax,0x{:X}", {
        let w = bst.read_word();
        if let Err(e) = set_ax(w) {
            return Err(e);
        }
        w
    }))
}
pub fn op_b9(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    Ok(format!("mov cx,0x{:X}", {
        let w = bst.read_word();
        if let Err(e) = set_cx(w) {
            return Err(e);
        }
        w
    }))
}
pub fn op_ba(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    Ok(format!("mov dx,0x{:X}", {
        let w = bst.read_word();
        if let Err(e) = set_dx(w) {
            return Err(e);
        }
        w
    }))
}
pub fn op_bb(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    Ok(format!("mov bx,0x{:X}", {
        let w = bst.read_word();
        if let Err(e) = set_bx(w) {
            return Err(e);
        }
        w
    }))
}
pub fn op_bc(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match SP.write() {
        Ok(mut good_sp) => Ok(format!("mov sp,0x{:X}", {
            let b = bst.read_word();
            *good_sp = Some(b);
            b
        })),
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn op_bd(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match BP.write() {
        Ok(mut good_bp) => Ok(format!("mov bp,0x{:X}", {
            let b = bst.read_word();
            *good_bp = Some(b);
            b
        })),
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn op_be(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match SI.write() {
        Ok(mut good_si) => Ok(format!("mov si,0x{:X}", {
            let b = bst.read_word();
            *good_si = Some(b);
            b
        })),
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn op_bf(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match DI.write() {
        Ok(mut good_di) => Ok(format!("mov di,0x{:X}", {
            let b = bst.read_word();
            *good_di = Some(b);
            b
        })),
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
// c0-c2
pub fn op_c3(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match STACK.write() {
        Ok(mut good) => match good.pop() {
            Some(v) => match v {
                Some(v) => bst.pos = v as usize,
                None => return Err(ExecutableError::from_message("popped value from stack is unknown; cannot move on due to ambiguity (opcode 0xC3)")),
            },
            None => return Err(ExecutableError::from_message("stack is empty")),
        },
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }
    bst.pos = STACK.write().unwrap().pop().unwrap().unwrap() as usize;

    Ok("ret".to_owned())
}
// c4-cb
pub fn op_cd(bst: &mut ByteStream, api: &API) -> Result<String, ExecutableError> {
    match api {
        API::DOS => dos_op_cd(bst, false),
        _ => Ok(format!("int {:X}h", bst.read_byte())),
    }
}
// ce-e7
pub fn op_e8(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    match STACK.write() {
        Ok(mut good) => good.push(Some(bst.pos as u16)),
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }
    bst.pos += bst.read_sword() as usize;
    Ok(format!("call 0x{:04X}", bst.pos))
}
pub fn op_e9(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let displacement = bst.read_word();

    bst.pos = ((bst.pos as u16).wrapping_add(displacement)) as usize;

    Ok(format!("jmp 0x{:04X}", bst.pos))
}
// ea-f1
pub fn op_f2(bst: &mut ByteStream, api: &API) -> Result<String, ExecutableError> {
    let fixed_pos = bst.pos;
    let r = match run_byte_code(bst, api) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    match get_cx() {
        Ok(good_cx) => {
            if let Err(e) = set_cx(match good_cx {
                Some(v) => v - 1,
                None => {
                    eprintln!("WARNING: cx is unknown; assuming value is 0xFFFF (opcode 0xF2)");
                    0xFFFF - 1
                }
            }) {
                return Err(e);
            }
        }
        Err(e) => return Err(e),
    }

    match ZF.write() {
        Ok(mut temp1) => {
            if let None = *temp1 {
                eprintln!("WARNING: zf is unknown; assuming value is 0 (opcode 0xF2)");
                *temp1 = Some(false);
            }
        }
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }

    while match get_cx() {
        Ok(good_cx) => good_cx.unwrap(), // this panicking should be unreachable
        Err(e) => return Err(e),
    } > 0
        || match ZF.read() {
            Ok(good_zf) => match *good_zf {
                Some(zf) => zf,
                None => {
                    eprintln!("WARNING: zf is unknown after repitition; assuming value is 0 (opcode 0xF2)");
                    false
                }
            },
            Err(e) => return Err(ExecutableError::from_inner(e)),
        } == false
    {
        bst.pos = fixed_pos;
        if match run_byte_code(bst, api) {
            Ok(res) => res,
            Err(e) => return Err(e),
        } != r
        {
            panic!(); // idk
        }
    }
    Ok(format!("repne {r}"))
}
pub fn op_f3(bst: &mut ByteStream, api: &API) -> Result<String, ExecutableError> {
    let fixed_pos = bst.pos;
    let r = match run_byte_code(bst, api) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    match get_cx() {
        Ok(good_cx) => {
            if let Err(e) = set_cx(match good_cx {
                Some(v) => v - 1,
                None => {
                    eprintln!("WARNING: cx is unknown; assuming value is 0xFFFF (opcode 0xF3)");
                    0xFFFF - 1
                }
            }) {
                return Err(e);
            }
        }
        Err(e) => return Err(e),
    }

    match ZF.write() {
        Ok(mut temp1) => {
            if let None = *temp1 {
                eprintln!("WARNING: zf is unknown; assuming value is 1 (opcode 0xF3)");
                *temp1 = Some(false);
            }
        }
        Err(e) => return Err(ExecutableError::from_inner(e)),
    }

    while match get_cx() {
        Ok(good_cx) => good_cx.unwrap(), // this panicking should be unreachable
        Err(e) => return Err(e),
    } > 0
        || match ZF.read() {
            Ok(good_zf) => match *good_zf {
                Some(zf) => zf,
                None => {
                    eprintln!("WARNING: zf is unknown after repitition; assuming value is 1 (opcode 0xF3)");
                    false
                }
            },
            Err(e) => return Err(ExecutableError::from_inner(e)),
        } == true
    {
        bst.pos = fixed_pos;
        if match run_byte_code(bst, api) {
            Ok(res) => res,
            Err(e) => return Err(e),
        } != r
        {
            panic!(); // idk
        }
    }
    Ok(format!("rep {r}"))
}
// f4-f6
pub fn op_f7(bst: &mut ByteStream) -> Result<String, ExecutableError> {
    let (_, _, reg, rm, _, v, v_s) = match modrm_byte_handling(bst) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    match reg {
        0 => {
            let imm16 = bst.read_word();
            let res = match v {
                Some(v) => v,
                None => {
                    match get_parsed_reg(&REG_NAMES[rm as usize].to_owned()) {
                        Ok(regv) => match regv {
                            Some(regv) => regv,
                            None => {
                                eprintln!("WARNING: {} is unknown; assuming value of 0 (opcode 0xF7, reg=0)", REG_NAMES[rm as usize]);
                                0
                            }
                        },
                        Err(e) => return Err(e),
                    }
                }
            };
            match CF.write() {
                Ok(mut good_cf) => *good_cf = Some(false),
                Err(e) => return Err(ExecutableError::from_inner(e)),
            }
            match OF.write() {
                Ok(mut good_of) => *good_of = Some(false),
                Err(e) => return Err(ExecutableError::from_inner(e)),
            }
            match ZF.write() {
                Ok(mut good_zf) => *good_zf = Some(res == 0),
                Err(e) => return Err(ExecutableError::from_inner(e)),
            }
            match SF.write() {
                Ok(mut good_sf) => *good_sf = Some((res >> 15) == 1),
                Err(e) => return Err(ExecutableError::from_inner(e)),
            }
            match PF.write() {
                Ok(mut good_pf) => *good_pf = Some((res & 0xFF).count_ones() % 2 == 0),
                Err(e) => return Err(ExecutableError::from_inner(e)),
            }
            Ok(format!(
                "test {},0x{imm16:04X}",
                v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
            ))
        }
        2 => {
            match v {
                Some(v) => {
                    let w = bst.read_word_at(v as usize);
                    bst.replace_word(v as usize, !w);
                }
                None => match get_parsed_reg(&REG_NAMES[rm as usize].to_owned()) {
                    Ok(w) => {
                        if let Err(e) = set_parsed_reg(&REG_NAMES[rm as usize].to_owned(), !{
                            match w {
                                Some(w) => w,
                                None => {
                                    eprintln!("WARNING: {} is unknown; assuming value of 0 (opcode 0xF7, reg=2)", REG_NAMES[rm as usize]);
                                    0u16
                                }
                            }
                        }) {
                            return Err(e);
                        }
                    }
                    Err(e) => return Err(e),
                },
            }

            Ok(format!(
                "not {}",
                v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
            ))
        }
        3 => {
            let res = match v {
                Some(v) => {
                    let w = bst.read_word_at(v as usize);
                    bst.replace_word(v as usize, 0u16.wrapping_sub(w));
                    bst.read_word_at(v as usize)
                }
                None => {
                    match get_parsed_reg(&REG_NAMES[rm as usize].to_owned()) {
                        Ok(w) => {
                            if let Err(e) = set_parsed_reg(
                                &REG_NAMES[rm as usize].to_owned(),
                                0u16.wrapping_sub({
                                    match w {
                                        Some(w) => w,
                                        None => {
                                            eprintln!("WARNING: {} is unknown; assuming value of 0 (opcode 0xF7, reg=3)", REG_NAMES[rm as usize]);
                                            0u16
                                        }
                                    }
                                }),
                            ) {
                                return Err(e);
                            }
                        }
                        Err(e) => return Err(e),
                    }
                    match get_parsed_reg(&REG_NAMES[rm as usize].to_owned()) {
                        Ok(v) => v.unwrap(), // we literally just set it, so we chillin
                        Err(e) => return Err(e),
                    }
                }
            };
            *CF.write().unwrap() = Some(res != 0);
            *OF.write().unwrap() = Some(res == 0x8000);
            *ZF.write().unwrap() = Some(res == 0);
            *SF.write().unwrap() = Some((res >> 15) & 0b1 == 1);
            *PF.write().unwrap() = Some((res & 0xFF).count_ones() % 2 == 0);
            // AF?
            Ok(format!(
                "neg {}",
                v_s.unwrap_or_else(|| REG_NAMES[rm as usize].to_owned())
            ))
        }

        _ => todo!("need to implement reg={reg} in opcode 0xF7"),
    }
}
// f8-ff

pub fn run_byte_code(bst: &mut ByteStream, api: &API) -> Result<String, ExecutableError> {
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
        0x74 => op_74(bst),
        0x81 => op_81(bst),
        0x83 => op_83(bst),
        0x8B => op_8b(bst),
        0x8C => op_8c(bst),
        0x8D => op_8d(bst),
        0x8E => op_8e(bst),
        0x90 => Ok(op_90()),
        0xAE => op_ae(bst),
        0xAC => op_ac(bst),
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
        0xCD => op_cd(bst, api),
        0xE8 => op_e8(bst),
        0xE9 => op_e9(bst),
        0xF2 => op_f2(bst, api),
        0xF3 => op_f3(bst, api),
        0xF7 => op_f7(bst),
        _ => Err(ExecutableError::from_message(format!(
            "opcode {byte} not yet implemented"
        ))),
    }
}

/// Executes given code with a specific API set.
pub fn run_code(bytes: &Vec<u8>, api: &API) -> Result<Vec<String>, ExecutableError> {
    let mut bst = ByteStream::new(bytes.clone());

    let mut code = Vec::new();

    while bst.available() {
        match run_byte_code(&mut bst, api) {
            Ok(v) => code.push(v),
            Err(e) => return Err(e),
        }
    }

    reset_regs();
    Ok(code)
}

pub fn single_interpret_code(bst: &mut ByteStream, api: &API) -> Result<String, ExecutableError> {
    match bst.read_byte() {
        0x0E => {
            const CMD: &str = "push cs";
            match STACK.write() {
                Ok(mut good) => match CS.read() {
                    Ok(good_cs) => Ok({
                        good.push(*good_cs);
                        match *good_cs {
                            Some(_) => format!("{CMD}"),
                            None => format!("{CMD} ; WARNING: cs is unknown"),
                        }
                    }),
                    Err(e) => Err(ExecutableError::from_inner(e)),
                },
                Err(e) => Err(ExecutableError::from_inner(e)),
            }
        }
        0x1F => {
            const CMD: &str = "pop ds";

            match STACK.write() {
                Ok(mut good) => match DS.write() {
                    Ok(mut good_ds) => Ok(match good.pop() {
                        Some(v) => {
                            *good_ds = v;
                            match v {
                                Some(_) => format!("{CMD}"),
                                None => format!("{CMD} ; WARNING: popped value not known here"),
                            }
                        }
                        None => format!("{CMD} ; WARNING: stack is empty"),
                    }),
                    Err(e) => Err(ExecutableError::from_inner(e)),
                },
                Err(e) => Err(ExecutableError::from_inner(e)),
            }
        }
        0x3C => {
            let imm8 = bst.read_byte();
            let mut cmd = format!("cmp al,0x{imm8:02X}");
            let al = match AL.read() {
                Ok(good_al) => match *good_al {
                    Some(al) => al,
                    None => {
                        cmd += " ; WARNING: al is unknown; assuming value is 0 (opcode 0x3C)";
                        0
                    }
                },
                Err(e) => return Err(ExecutableError::from_inner(e)),
            };
            let res = al.wrapping_sub(imm8);

            match ZF.write() {
                Ok(mut good_zf) => *good_zf = Some(res == 0),
                Err(e) => return Err(ExecutableError::from_inner(e)),
            }
            match SF.write() {
                Ok(mut good_sf) => *good_sf = Some((res & 0x80) != 0),
                Err(e) => return Err(ExecutableError::from_inner(e)),
            }
            match CF.write() {
                Ok(mut good_cf) => *good_cf = Some(al < imm8),
                Err(e) => return Err(ExecutableError::from_inner(e)),
            }
            match OF.write() {
                Ok(mut good_of) => {
                    *good_of = Some(
                        (((al & 0x80) != 0) != ((imm8 & 0x80) != 0))
                            && (((res & 0x80) != 0) != ((al & 0x80) != 0)),
                    )
                }
                Err(e) => return Err(ExecutableError::from_inner(e)),
            }
            match PF.write() {
                Ok(mut good_pf) => *good_pf = Some((res.count_ones() % 2) == 0),
                Err(e) => return Err(ExecutableError::from_inner(e)),
            }

            Ok(cmd)
        }
        0x74 => Ok(format!("je 0x{:02X}", bst.read_byte())),
        0x90 => Ok("nop".to_owned()),
        0xAC => {
            let mut probs = String::new();
            let ptr = (match DS.read() {
                Ok(good_ds) => match *good_ds {
                    Some(ds) => ds,
                    None => {
                        probs += "; WARNING: ds is unknown; assuming value is 0 (opcode 0xAC)\n";
                        0
                    }
                },
                Err(e) => return Err(ExecutableError::from_inner(e)),
            } << 4)
                + match SI.read() {
                    Ok(good_si) => match *good_si {
                        Some(si) => si,
                        None => {
                            probs +=
                                "; WARNING: si is unknown; assuming value is 0 (opcode 0xAC)\n";
                            0
                        }
                    },
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                };

            match AL.write() {
                Ok(mut good_al) => *good_al = Some(bst.read_byte_at(ptr as usize)),
                Err(e) => return Err(ExecutableError::from_inner(e)),
            }

            match DF.read() {
                Ok(good_df) => match SI.write() {
                    Ok(mut good_si) => match *good_df {
                        Some(df) => {
                            if df {
                                *good_si = Some(good_si.unwrap_or(0) - 1);
                            } else {
                                *good_si = Some(good_si.unwrap_or(0) + 1);
                            }
                        }
                        None => {
                            probs +=
                                "; WARNING: df is unknown; assuming value is 0 (opcode 0xAC)\n";
                            *good_si = Some(good_si.unwrap_or(0) + 1);
                        }
                    },
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                },
                Err(e) => return Err(ExecutableError::from_inner(e)),
            }

            probs.pop();
            Ok(format!(
                "lodsb{}",
                if probs.is_empty() {
                    "".to_owned()
                } else {
                    "\n".to_owned() + &probs
                }
            ))
        }
        0xB0 => {
            const CMD: &str = "mov al,";
            if bst.available() {
                let v = bst.read_byte();
                match AL.write() {
                    Ok(mut al) => *al = Some(v),
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xB1 => {
            const CMD: &str = "mov cl,";
            if bst.available() {
                let v = bst.read_byte();
                match CL.write() {
                    Ok(mut cl) => *cl = Some(v),
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xB2 => {
            const CMD: &str = "mov dl,";
            if bst.available() {
                let v = bst.read_byte();
                match DL.write() {
                    Ok(mut dl) => *dl = Some(v),
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xB3 => {
            const CMD: &str = "mov bl,";
            if bst.available() {
                let v = bst.read_byte();
                match BL.write() {
                    Ok(mut bl) => *bl = Some(v),
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xB4 => {
            const CMD: &str = "mov ah,";
            if bst.available() {
                let v = bst.read_byte();
                match AH.write() {
                    Ok(mut ah) => *ah = Some(v),
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xB5 => {
            const CMD: &str = "mov ch,";
            if bst.available() {
                let v = bst.read_byte();
                match CH.write() {
                    Ok(mut ch) => *ch = Some(v),
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xB6 => {
            const CMD: &str = "mov dh,";
            if bst.available() {
                let v = bst.read_byte();
                match DH.write() {
                    Ok(mut dh) => *dh = Some(v),
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xB7 => {
            const CMD: &str = "mov bh,";
            if bst.available() {
                let v = bst.read_byte();
                match BH.write() {
                    Ok(mut bh) => *bh = Some(v),
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xB8 => {
            const CMD: &str = "mov ax,";
            if bst.available() {
                let v = bst.read_word();
                if let Err(e) = set_ax(v) {
                    return Err(e);
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xB9 => {
            const CMD: &str = "mov cx,";
            if bst.available() {
                let v = bst.read_word();
                if let Err(e) = set_cx(v) {
                    return Err(e);
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xBA => {
            const CMD: &str = "mov dx,";
            if bst.available() {
                let v = bst.read_word();
                if let Err(e) = set_dx(v) {
                    return Err(e);
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xBB => {
            const CMD: &str = "mov bx,";
            if bst.available() {
                let v = bst.read_word();
                if let Err(e) = set_bx(v) {
                    return Err(e);
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xBC => {
            const CMD: &str = "mov sp,";
            if bst.available() {
                let v = bst.read_word();
                match SP.write() {
                    Ok(mut sp) => *sp = Some(v),
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xBD => {
            const CMD: &str = "mov bp,";
            if bst.available() {
                let v = bst.read_word();
                match BP.write() {
                    Ok(mut bp) => *bp = Some(v),
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xBE => {
            const CMD: &str = "mov si,";
            if bst.available() {
                let v = bst.read_word();
                match SI.write() {
                    Ok(mut si) => *si = Some(v),
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }
        0xBF => {
            const CMD: &str = "mov di,";
            if bst.available() {
                let v = bst.read_word();
                match DI.write() {
                    Ok(mut si) => *si = Some(v),
                    Err(e) => return Err(ExecutableError::from_inner(e)),
                }
                Ok(format!("{CMD}0x{v:02X}"))
            } else {
                Ok(format!("{CMD}? ; WARNING: end of code"))
            }
        }

        0xCD => match api {
            API::DOS => {
                let v = dos_op_cd(bst, true);
                v
            }
            _ => Ok(format!("int {:X}h", bst.read_byte())),
        },
        0xE9 => {
            let displacement = bst.read_sword();

            if displacement >= 0 {
                bst.pos += displacement as usize;
            } else {
                bst.pos -= -displacement as usize;
            }

            Ok(format!("jmp 0x{:04X}", bst.pos))
        }
        b => Ok(format!(
            "; byte code 0x{b:02X} at 0x{:04X} not yet implemented",
            bst.pos - 1
        )),
    }
}

/// Parses code without executing it (NOTE: there are instructions that can manipulate the machine code during runtime).
pub fn interpret_code(bytes: &Vec<u8>, api: &API) -> Result<Vec<String>, ExecutableError> {
    let mut bst = ByteStream::new(bytes.clone());

    let mut code = Vec::new();

    while bst.available() {
        let p = bst.pos;
        match single_interpret_code(&mut bst, api) {
            Ok(v) => code.push(format!("0x{p:04X} -> {v}")),
            Err(e) => return Err(e),
        }
    }

    reset_regs();
    Ok(code)
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

pub fn get_ax() -> Result<Option<u16>, ExecutableError> {
    let ah = match AH.read() {
        Ok(v) => *v,
        Err(e) => return Err(ExecutableError::from_inner(e)),
    };
    let al = match AL.read() {
        Ok(v) => *v,
        Err(e) => return Err(ExecutableError::from_inner(e)),
    };
    return match (ah, al) {
        (None, None) => Ok(None),
        (Some(vah), None) => {
            eprintln!("WARNING: registry value AL is not known here, assuming value of 0.");
            Ok(Some((vah as u16) << 8))
        }
        (None, Some(val)) => {
            eprintln!("WARNING: registry value AH is not known here, assuming value of 0.");
            Ok(Some(val as u16))
        }
        (Some(vah), Some(val)) => Ok(Some(((vah as u16) << 8) | (val as u16))),
    };
}
pub fn set_ax(v: u16) -> Result<(), ExecutableError> {
    match AH.write() {
        Ok(mut ah) => match AL.write() {
            Ok(mut al) => Ok({
                *al = Some((v & 0xFF) as u8);
                *ah = Some((v >> 8) as u8);
            }),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn get_bx() -> Result<Option<u16>, ExecutableError> {
    let bh = match BH.read() {
        Ok(v) => *v,
        Err(e) => return Err(ExecutableError::from_inner(e)),
    };
    let bl = match BL.read() {
        Ok(v) => *v,
        Err(e) => return Err(ExecutableError::from_inner(e)),
    };
    return match (bh, bl) {
        (None, None) => Ok(None),
        (Some(vbh), None) => {
            eprintln!("WARNING: registry value BL is not known here, assuming value of 0.");
            Ok(Some((vbh as u16) << 8))
        }
        (None, Some(vbl)) => {
            eprintln!("WARNING: registry value BH is not known here, assuming value of 0.");
            Ok(Some(vbl as u16))
        }
        (Some(vbh), Some(vbl)) => Ok(Some(((vbh as u16) << 8) | (vbl as u16))),
    };
}
pub fn set_bx(v: u16) -> Result<(), ExecutableError> {
    match BH.write() {
        Ok(mut bh) => match BL.write() {
            Ok(mut bl) => Ok({
                *bl = Some((v & 0xFF) as u8);
                *bh = Some((v >> 8) as u8);
            }),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn get_cx() -> Result<Option<u16>, ExecutableError> {
    let ch = match CH.read() {
        Ok(v) => *v,
        Err(e) => return Err(ExecutableError::from_inner(e)),
    };
    let cl = match CL.read() {
        Ok(v) => *v,
        Err(e) => return Err(ExecutableError::from_inner(e)),
    };
    return match (ch, cl) {
        (None, None) => Ok(None),
        (Some(vch), None) => {
            eprintln!("WARNING: registry value CL is not known here, assuming value of 0.");
            Ok(Some((vch as u16) << 8))
        }
        (None, Some(vcl)) => {
            eprintln!("WARNING: registry value CH is not known here, assuming value of 0.");
            Ok(Some(vcl as u16))
        }
        (Some(vch), Some(vcl)) => Ok(Some(((vch as u16) << 8) | (vcl as u16))),
    };
}
pub fn set_cx(v: u16) -> Result<(), ExecutableError> {
    match CH.write() {
        Ok(mut ch) => match CL.write() {
            Ok(mut cl) => Ok({
                *cl = Some((v & 0xFF) as u8);
                *ch = Some((v >> 8) as u8);
            }),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
pub fn get_dx() -> Result<Option<u16>, ExecutableError> {
    let dh = match DH.read() {
        Ok(v) => *v,
        Err(e) => return Err(ExecutableError::from_inner(e)),
    };
    let dl = match DL.read() {
        Ok(v) => *v,
        Err(e) => return Err(ExecutableError::from_inner(e)),
    };
    return match (dh, dl) {
        (None, None) => Ok(None),
        (Some(vdh), None) => {
            eprintln!("WARNING: registry value DL is not known here, assuming value of 0.");
            Ok(Some((vdh as u16) << 8))
        }
        (None, Some(vdl)) => {
            eprintln!("WARNING: registry value DH is not known here, assuming value of 0.");
            Ok(Some(vdl as u16))
        }
        (Some(vdh), Some(vdl)) => Ok(Some(((vdh as u16) << 8) | (vdl as u16))),
    };
}
pub fn set_dx(v: u16) -> Result<(), ExecutableError> {
    match DH.write() {
        Ok(mut dh) => match DL.write() {
            Ok(mut dl) => Ok({
                *dl = Some((v & 0xFF) as u8);
                *dh = Some((v >> 8) as u8);
            }),
            Err(e) => Err(ExecutableError::from_inner(e)),
        },
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}

*/