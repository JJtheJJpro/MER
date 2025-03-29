// as far as i know, DOS is always 16-bit.

use crate::{
    byte_operation::x86_16::{self, get_dx},
    byte_stream::ByteStream,
    executable::InteruptChange,
};

pub fn dos_op_cd(bst: &mut ByteStream) -> (String, InteruptChange) {
    let vcd = bst.read_byte();
    let mut code = format!("int {vcd:X}h");

    match vcd {
        0x21 => match *x86_16::AH.read().unwrap() {
            0x09 => {
                let begin = get_dx();
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
                code += format!("\n; exit({});", *x86_16::AL.read().unwrap()).as_str();
                (code, InteruptChange::Exit)
            }
            v => panic!("ah value of {v}"),
        },
        _ => panic!(),
    }
}
