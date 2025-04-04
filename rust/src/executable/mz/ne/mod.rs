pub mod restable;
pub mod segtable;

use restable::ResourceTable;
use segtable::SegmentTable;

use crate::byte_stream::ByteStream;

pub struct NewExecutable {
    pub linker_version: String,
    pub entry_table_offset: u16,
    pub entry_table_length: u16,
    pub crc: u32,
    pub flag_word: u16,
    pub auto_data_segment_number: u16,
    pub init_local_heap_size: u16,
    pub init_stack_size: u16,
    pub cs: u16,
    pub ip: u16,
    pub ss: u16,
    pub sp: u16,
    pub entries_in_segment_table: u16,
    pub entries_in_module_ref_table: u16,
    pub non_resident_names_table_size: u16,
    pub segment_table_offset: u16,
    pub resource_table_offset: u16,
    pub resident_names_table_offset: u16,
    pub module_ref_table_offset: u16,
    pub imported_names_table_offset: u16,
    pub non_resident_names_table_offset: u32,
    pub moveable_entry_points: u16,
    pub shift_count: u16,
    pub resource_segment_count: u16,
    pub target_os: u8,
    pub additional_info: u8,
    pub fast_load_offset: u16,
    pub fast_load_length: u16,
    pub expected_win_ver: String,

    // the following are completely variable (no part of it is fixed)
    pub segments: Vec<SegmentTable>,
    pub resource_table: ResourceTable,
}

impl NewExecutable {
    pub fn signature(&self) -> String {
        "NE".to_owned()
    }
    pub fn read(bst: &mut ByteStream, offset: u16) -> Self {
        let linker_version = format!("{}.{}", bst.read_byte(), bst.read_byte());
        let entry_table_offset = bst.read_word();
        let entry_table_length = bst.read_word();
        let crc = bst.read_dword();
        let flag_word = bst.read_word();
        let auto_data_segment_number = bst.read_word();
        let init_local_heap_size = bst.read_word();
        let init_stack_size = bst.read_word();
        let ip = bst.read_word();
        let cs = bst.read_word();
        let sp = bst.read_word();
        let ss = bst.read_word();
        let entries_in_segment_table = bst.read_word();
        let entries_in_module_ref_table = bst.read_word();
        let non_resident_names_table_size = bst.read_word();
        let segment_table_offset = bst.read_word();
        let resource_table_offset = bst.read_word();
        let resident_names_table_offset = bst.read_word();
        let module_ref_table_offset = bst.read_word();
        let imported_names_table_offset = bst.read_word();
        let non_resident_names_table_offset = bst.read_dword() - offset as u32;
        let moveable_entry_points = bst.read_word();
        let shift_count = bst.read_word();
        let resource_segment_count = bst.read_word();
        let target_os = bst.read_byte();

        let additional_info = bst.read_byte();
        let fast_load_offset = bst.read_word();
        let fast_load_length = bst.read_word();
        bst.pos += 2; // bst.check_reserved(2);
        let exp_win_ver_min = bst.read_byte();
        let exp_win_ver_maj = bst.read_byte();

        if bst.pos < (segment_table_offset + offset) as usize {
            let mut rdata = Vec::new();
            while bst.pos < segment_table_offset as usize {
                rdata.push(bst.read_byte());
            }
            panic!();
        } else if bst.pos > (segment_table_offset + offset) as usize {
            panic!();
        }
        let mut segments = Vec::new();
        for _ in 0..entries_in_segment_table {
            segments.push(SegmentTable::read(bst));
        }

        if bst.pos < (resource_table_offset + offset) as usize {
            let mut rdata = Vec::new();
            while bst.pos < resource_table_offset as usize {
                rdata.push(bst.read_byte());
            }
            panic!();
        } else if bst.pos > (resource_table_offset + offset) as usize {
            panic!();
        }
        let resource_table = ResourceTable::read(bst, resident_names_table_offset - resource_table_offset);

        Self {
            linker_version,
            entry_table_offset,
            entry_table_length,
            crc,
            flag_word,
            auto_data_segment_number,
            init_local_heap_size,
            init_stack_size,
            cs,
            ip,
            ss,
            sp,
            entries_in_segment_table,
            entries_in_module_ref_table,
            non_resident_names_table_size,
            segment_table_offset,
            resource_table_offset,
            resident_names_table_offset,
            module_ref_table_offset,
            imported_names_table_offset,
            non_resident_names_table_offset,
            moveable_entry_points,
            shift_count,
            resource_segment_count,
            target_os,
            additional_info,
            fast_load_offset,
            fast_load_length,
            expected_win_ver: format!("{exp_win_ver_maj}.{exp_win_ver_min}"),
            segments,
            resource_table,
        }
    }
}
