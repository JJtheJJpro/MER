// as far as i know, DOS is always 16-bit.

use crate::{byte_operation::x86_16::{self, get_dx}, byte_stream::ByteStream, executable::InteruptChange};

pub fn dos_op_cd(execute: bool, bst: &mut ByteStream) -> (String, InteruptChange) {
    let vcd = bst.read_byte();
    let mut code = format!("int {vcd:X}h");

    match vcd {
        0x21 => match *x86_16::AH.read().unwrap() {
            0x09 => {
                let begin = get_dx();
                let end = bst.find_first_byte_from(begin as usize, 0x24);
                let string = bst.read_string_from_to(begin as usize, end);
                code += format!("\n; printf({});", string.replace("\n", "\\n").replace("\r", "\\r")).as_str();
                if execute {
                    print!("{string}");
                }
                (code, InteruptChange::String(begin, end as u16))
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
}