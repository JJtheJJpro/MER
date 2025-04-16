// as far as i know, DOS is always 16-bit.

/*
use crate::{byte_operation::x86_16, byte_stream::ByteStream, executable::ExecutableError};

pub fn dos_op_cd(bst: &mut ByteStream, interpret: bool) -> Result<String, ExecutableError> {
    let vcd = bst.read_byte();
    if interpret {
        let code = format!("int {vcd:X}h");

        match vcd {
            0x21 => {
                match x86_16::AH.read() {
                    Ok(good_ah) => match *good_ah {
                        None => Ok(format!("{code} ; WARNING: registry AH not known here")),
                        Some(ah) => match ah {
                            0x09 => {
                                let begin = match x86_16::get_dx() {
                                    Ok(v) => match v {
                                        Some(v) => v,
                                        None => {
                                            return Err(ExecutableError::from_message(
                                                "dx is unknown; required in INT 21h calls with AH value 0x09",
                                            ))
                                        }
                                    },
                                    Err(e) => return Err(e),
                                } as usize;
                                let end = bst.find_first_byte_from(begin, 0x24);
                                let string = bst.read_string_from_to(begin, end);
                                Ok(format!(
                                    "{code}\n; printf(\"{}\");",
                                    string.replace("\n", "\\n").replace("\r", "\\r")
                                ))
                            }
                            0x4C => {
                                bst.exit();
                                match x86_16::AL.read() {
                                    Ok(al) => match *al {
                                        None => Ok(format!("{code} ; WARNING: registry AL not known here\n; exit(0);")),
                                        Some(al) => Ok(format!("{code}\n; exit({});", al)),
                                    },
                                    Err(e) => Err(ExecutableError::from_inner(e)),
                                }
                            }
                            v => Ok(format!("{code} ; WARNING: ah value of {v} not yet implemented")),
                        },
                    },
                    Err(e) => Err(ExecutableError::from_inner(e)),
                }
            }
            _ => panic!(),
        }
    } else {
        let mut code = format!("int {vcd:X}h");

        match vcd {
            0x21 => match x86_16::AH.read() {
                Ok(good_ah) => match *good_ah {
                    Some(ah) => match ah {
                        0x09 => {
                            let begin = match x86_16::get_dx() {
                                Ok(v) => match v {
                                    Some(v) => v,
                                    None => {
                                        return Err(ExecutableError::from_message(
                                            "dx is unknown; required in api calls",
                                        ))
                                    }
                                },
                                Err(e) => return Err(e),
                            };
                            let end = bst.find_first_byte_from(begin as usize, 0x24);
                            let string = bst.read_string_from_to(begin as usize, end);
                            code += format!(
                                "\n; printf(\"{}\");",
                                string.replace("\n", "\\n").replace("\r", "\\r")
                            )
                            .as_str();
                            //print!("{string}");
                            Ok(code)
                        }
                        0x4C => {
                            bst.exit();
                            code += format!("\n; exit({});", x86_16::AL.read().unwrap().unwrap()).as_str();
                            Ok(code)
                        }
                        v => Ok(format!("{code} ; WARNING: ah value of {v} not yet implemented")),
                    },
                    None => Ok(format!("{code} ; WARNING: registry AH not known here")),
                },
                Err(e) => Err(ExecutableError::from_inner(e)),
            },
            _ => panic!(),
        }
    }
}
*/