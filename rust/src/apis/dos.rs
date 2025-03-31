// as far as i know, DOS is always 16-bit.

use crate::{
    byte_operation::x86_16::{self, get_dx},
    byte_stream::ByteStream,
    executable::InteruptChange,
};

pub fn dos_op_cd(bst: &mut ByteStream, interpret: bool) -> (String, InteruptChange) {
    let vcd = bst.read_byte();
    if interpret {
        let code = format!("int {vcd:X}h");

        match vcd {
            0x21 => match *x86_16::AH.read().unwrap() {
                None => (
                    format!("{code} ; WARNING: registry AH not known here"),
                    InteruptChange::None,
                ),
                Some(ah) => match ah {
                    0x09 => {
                        let begin = get_dx().unwrap() as usize;
                        let end = bst.find_first_byte_from(begin, 0x24);
                        let string = bst.read_string_from_to(begin, end);
                        (format!("{code}\n; printf({});", string.replace("\n", "\\n").replace("\r", "\\r")), InteruptChange::SkipString(begin, end))
                    }
                    0x4C => {
                        match *x86_16::AL.read().unwrap() {
                            None => (format!("{code} ; WARNING registry AL not known here\n; exit(0);"), InteruptChange::Exit),
                            Some(al) => (format!("{code}\n; exit({});", al), InteruptChange::Exit),
                        }
                    }
                    v => panic!("ah value of {v}"),
                },
            },
            _ => panic!(),
        }
    } else {
        let mut code = format!("int {vcd:X}h");

        match vcd {
            0x21 => match x86_16::AH.read().unwrap().unwrap() {
                0x09 => {
                    let begin = get_dx().unwrap();
                    let end = bst.find_first_byte_from(begin as usize, 0x24);
                    let string = bst.read_string_from_to(begin as usize, end);
                    code += format!(
                        "\n; printf({});",
                        string.replace("\n", "\\n").replace("\r", "\\r")
                    )
                    .as_str();
                    //print!("{string}");
                    (code, InteruptChange::SkipString(begin as usize, end))
                }
                0x4C => {
                    code += format!("\n; exit({});", x86_16::AL.read().unwrap().unwrap()).as_str();
                    (code, InteruptChange::Exit)
                }
                v => panic!("ah value of {v}"),
            },
            _ => panic!(),
        }
    }
}
