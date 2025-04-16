use crate::{
    apis::API,
    byte_operation::code::{Architecture, Code, Instruction},
    byte_stream::ByteStream,
};
use ne::NewExecutable;

use super::ExecutableError;

pub mod ne;

/// All MZ extension signatures pertaining to MZ's New Header value.
pub enum MZExtSignature {
    /// The Windows New Executable format, successor to the DOS executables.  NE is always 16-bit.
    NE(NewExecutable),
    /// The Windows Linear Executable format, designed for 32-bit protected mod operating systems and 16-bit executable extensions.
    LE,
    /// The Windows Linear Executable format, except LX is only used in 32-bit environments and was developed specifically for OS/2 Warp, supporting further extensions over the LE format.
    LX,
    /// The Windows Portable Executable format, which uses a COFF header for object files and as a component for the header.  This can be 32-bit or 64-bit.
    PE,
}

/// Named after <a href="https://en.wikipedia.org/wiki/Mark_Zbikowski">Mark Zbikowski</a>, the MZ Header is the main MS-DOS EXE format
/// found in all windows executables and libraries, containing basic information of the whole executable.
pub struct MZ {
    pub last_page_bytes: u16,
    pub page_count: u16,
    pub relocation_table_entry_count: u16,
    /// In paragraphs
    pub header_size: u16,
    pub min_alloc: u16,
    pub max_alloc: u16,
    pub init_ss: u16,
    pub init_sp: u16,
    pub checksum: u16,
    pub init_ip: u16,
    pub init_cs: u16,
    pub relocation_table_offset: u16,
    pub overlay: u16,

    pub oem_id: u16,
    pub oem_info: u16,
    pub new_header_start: u32,

    pub extension: Option<MZExtSignature>,

    pub relocation_tables: Vec<RelocationTable>,
    pub header_code: Code,
}

impl MZ {
    pub fn signature() -> String {
        "MZ".to_owned()
    }

    pub fn read(bst: &mut ByteStream) -> Result<Self, ExecutableError> {
        let last_page_bytes = bst.read_word();
        let page_count = bst.read_word();
        let relocation_table_entry_count = bst.read_word();
        let header_size = bst.read_word();
        let min_alloc = bst.read_word();
        let max_alloc = bst.read_word();
        let init_ss = bst.read_word();
        let init_sp = bst.read_word();
        let checksum = bst.read_word();
        let init_ip = bst.read_word();
        let init_cs = bst.read_word();
        let relocation_table_offset = bst.read_word();
        let overlay = bst.read_word();

        bst.pos += 8; //bst.check_reserved(8); // skip instead of throw, for linker compatibility reasons
        let oem_id = bst.read_word();
        let oem_info = bst.read_word();
        bst.pos += 20; //bst.check_reserved(20);
        let new_header_start = bst.read_dword();

        let mut relocation_tables = Vec::new();
        if bst.pos == relocation_table_offset as usize && relocation_table_entry_count > 0 {
            for _ in 0..relocation_table_entry_count {
                relocation_tables.push(RelocationTable::read(bst));
            }
        }

        if bst.pos < header_size as usize * 16 {
            if !bst.check_reserved((header_size as usize * 16) - bst.pos) {
                panic!();
            }
        }

        let header_byte_code =
            bst.read_bytes((new_header_start - header_size as u32 * 16) as usize);

        let extension = if new_header_start > 0 {
            match &bst.read_bytes(2)[0..2] {
                b"NE" => Some(MZExtSignature::NE(NewExecutable::read(bst, new_header_start as u16))),
                _ => None,
            }
        } else {
            None
        };

        Ok(MZ {
            last_page_bytes,
            page_count,
            relocation_table_entry_count,
            header_size,
            min_alloc,
            max_alloc,
            init_ss,
            init_sp,
            checksum,
            init_ip,
            init_cs,
            relocation_table_offset,
            overlay,

            oem_id,
            oem_info,
            new_header_start,
            extension,
            relocation_tables,
            header_code: Code {
                api: API::DOS,
                arch: Architecture::Sixteen,
                bytes: header_byte_code,
                set: Instruction::X86,
            },
        })
    }
}

/// Represents a single Relocation Table possibly found in the MZ header.
pub struct RelocationTable {
    pub offset: u16,
    pub segment: u16,
}

impl RelocationTable {
    pub fn read(bst: &mut ByteStream) -> Self {
        let offset = bst.read_word();
        let segment = bst.read_word();
        Self { offset, segment }
    }
}
