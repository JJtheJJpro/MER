use crate::{byte_stream::ByteStream, executable::Signature};

pub struct MZ {
    pub last_page_bytes: u16,
    pub page_count: u16,
    pub relocation_table_entry_count: u16,
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

    pub oem_id: Option<u16>,
    pub oem_info: Option<u16>,
    pub new_header_start: Option<u32>,

    pub relocation_tables: Vec<RelocationTable>,
    pub header_code: Vec<String>,
}

impl MZ {
    pub fn signature() -> Signature {
        Signature::MZ
    }

    pub fn read(bst: &mut ByteStream) -> Self {
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

        let mut oem_id = None;
        let mut oem_info = None;
        let mut new_header_start = None;
        bst.pos += 8;//bst.check_reserved(8); // skip instead of throw, for linker compatibility reasons
        oem_id = Some(bst.read_word());
        oem_info = Some(bst.read_word());
        bst.pos += 8;//bst.check_reserved(20);
        new_header_start = Some(bst.read_dword());

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

        new_header_start.unwrap(); // more like a "todo!();" call

        

        MZ {
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
            relocation_tables,
            header_code: Vec::new(),
        }
    }
}

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
