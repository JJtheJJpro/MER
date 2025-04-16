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
    pub resident_names: Vec<ResidentNameTable>,
    pub module_references: Vec<ModuleReferenceTable>,
    pub imported_names: Vec<ImportedNameTable>,
    pub entry_tables: Vec<EntryTable>,
    pub non_resident_names: Vec<NonResidentNameTable>,
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
            segments.push(SegmentTable::read(bst, shift_count));
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
        let resource_table = ResourceTable::read(bst);

        bst.pos = (resident_names_table_offset + offset) as usize;
        let mut resident_name_tables = Vec::new();
        while bst.pos < (module_ref_table_offset + offset) as usize {
            if let Some(r) = ResidentNameTable::read(bst) {
                resident_name_tables.push(r);
            }
        }

        bst.pos = (module_ref_table_offset + offset) as usize;
        let mut module_ref_tables = Vec::new();
        while bst.pos < (imported_names_table_offset + offset) as usize {
            module_ref_tables.push(ModuleReferenceTable::read(bst));
        }

        bst.pos = (imported_names_table_offset + offset) as usize;
        let mut imported_name_tables = Vec::new();
        while bst.pos < (entry_table_offset + offset) as usize {
            if let Some(r) = ImportedNameTable::read(bst) {
                imported_name_tables.push(r);
            }
        }

        bst.pos = (entry_table_offset + offset) as usize;
        let mut entry_tables = Vec::new();
        while bst.pos < (entry_table_offset + offset + entry_table_length) as usize {
            if bst.peek_byte() == 0 {
                _ = bst.read_byte();
            } else {
                entry_tables.push(EntryTable::read(bst));
            }
        }

        bst.pos = (non_resident_names_table_offset as u16 + offset) as usize;
        let mut non_resident_name_tables = Vec::new();
        while bst.pos
            < (non_resident_names_table_offset as u16 + offset + non_resident_names_table_size)
                as usize
        {
            non_resident_name_tables.push(NonResidentNameTable::read(bst));
        }

        println!("bst.pos = 0x{:04X}", bst.pos);
        println!("zero check before segments: {}", bst.check_reserved((segments[0].data_offset << shift_count) as usize - bst.pos));
        println!("bst.pos = 0x{:04X}", bst.pos);

        let mut ordered_segments = segments.clone();
        ordered_segments.sort_by(|a, b| a.data_offset.cmp(&b.data_offset));
        for i in 0..ordered_segments.len() {
            let s = &ordered_segments[i];
            if s.length > 0 && s.data_offset > 0 {
                if bst.pos < (s.data_offset << shift_count) as usize {
                    println!("extra info between segments - {} bytes", (s.data_offset << shift_count) as usize - bst.pos);
                    bst.pos = ((s.data_offset as u32) << shift_count as u32) as usize;
                } else if bst.pos > (s.data_offset << shift_count) as usize {
                    panic!("bst.pos passed shifted offset value (segment {i}, offset shifted is 0x{:04X}, bst.pos is 0x{:04X})", s.data_offset << shift_count, bst.pos);
                }
                println!("seg{i} check");
                println!("seg{i} 0x{:08X}-0x{:08X}", (s.data_offset as u32) << shift_count as u32, ((s.data_offset as u32) << shift_count as u32) + s.length as u32);
                println!("pos 0x{:08X}", bst.pos);
                bst.pos += s.raw_data.len() as usize;
                println!("pos 0x{:08X}", bst.pos);
            }
        }

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
            resident_names: resident_name_tables,
            module_references: module_ref_tables,
            imported_names: imported_name_tables,
            entry_tables,
            non_resident_names: non_resident_name_tables,
        }
    }
}

pub enum ResourceTypes {
    Cursor = 0x8001,
    Bitmap,
    Icon,
    Menu,
    Dialog,
    String,
    FontDir,
    Font,
    Accelerator,
    RCData,
    MessageTable,
    GroupCursor,
    GroupIcon = 0x800E,
    Version = 0x8010,
    DLGInclude,
    PlugPlay = 0x8013,
    VXD,
    AniCursor,
    AniIcon,
    HTML,
    Manifest,
}

pub struct ResourceTable {
    pub align_shift: u16,
    pub types: Vec<TypeInfo>,
    pub resource_strings: Vec<String>,
}
impl ResourceTable {
    pub fn read(bst: &mut ByteStream) -> Self {
        let align_shift = bst.read_word();
        let mut types = Vec::new();
        while bst.peek_word() != 0 {
            types.push(TypeInfo::read(bst));
        }
        _ = bst.read_word();

        let mut resource_strings = Vec::new();
        while bst.peek_byte() != 0 {
            let _t = bst.read_byte();
            resource_strings.push(bst.read_string(_t as usize));
        }
        _ = bst.read_byte();

        Self {
            align_shift,
            types,
            resource_strings,
        }
    }
}

pub struct TypeInfo {
    pub type_id_or_offset: u16,
    pub res_count: u16,
    pub name_info: Vec<NameInfo>,
}
impl TypeInfo {
    pub fn read(bst: &mut ByteStream) -> Self {
        let type_id_or_offset = bst.read_word();
        let res_count = bst.read_word();
        bst.pos += 4; // bst.check_reserved(4);
        let mut name_info = Vec::new();
        for _ in 0..res_count {
            name_info.push(NameInfo::read(bst));
        }
        Self {
            type_id_or_offset,
            res_count,
            name_info,
        }
    }
}

pub struct NameInfo {
    pub offset: u16,
    pub length: u16,
    pub flags: u16,
    pub id: u16,
}
impl NameInfo {
    pub fn read(bst: &mut ByteStream) -> Self {
        let offset = bst.read_word();
        let length = bst.read_word();
        let flags = bst.read_word();
        let id = bst.read_word();
        bst.pos += 4; // bst.check_reserved(4);
        Self {
            offset,
            length,
            flags,
            id,
        }
    }
}

#[derive(Clone)]
pub struct SegmentTable {
    pub data_offset: u16,
    pub length: u16,
    pub flags: u16,
    pub min_alloc_size: u16,
    pub raw_data: Vec<u8>,
}
impl SegmentTable {
    pub fn read(bst: &mut ByteStream, shift: u16) -> Self {
        let data_offset = bst.read_word();
        let length = bst.read_word();
        let flags = bst.read_word();
        let min_alloc_size = bst.read_word();

        Self {
            data_offset,
            length,
            flags,
            min_alloc_size,
            raw_data: bst.read_bytes_at(length as usize, (data_offset << shift) as usize),
        }
    }
}

pub struct ResidentNameTable {
    pub length: u8,
    pub text: String,
    pub ordinal: u16,
}
impl ResidentNameTable {
    pub fn read(bst: &mut ByteStream) -> Option<Self> {
        let length = bst.read_byte();
        if length == 0 {
            return None;
        }
        let text = bst.read_string(length as usize);
        let ordinal = bst.read_word();

        Some(Self {
            length,
            text,
            ordinal,
        })
    }
}

pub struct ModuleReferenceTable {
    pub offset: u16,
}
impl ModuleReferenceTable {
    pub fn read(bst: &mut ByteStream) -> Self {
        Self {
            offset: bst.read_word(),
        }
    }
}

pub struct ImportedNameTable {
    pub length: u8,
    pub text: String,
}
impl ImportedNameTable {
    pub fn read(bst: &mut ByteStream) -> Option<Self> {
        let length = bst.read_byte();

        if length == 0 {
            return None;
        }

        Some(Self {
            length,
            text: bst.read_string(length as usize),
        })
    }
}

pub struct EntryTable {
    pub entry_count: u8,
    pub seg_indicator: u8,
    pub entry_type: EntryType,
}
impl EntryTable {
    pub fn read(bst: &mut ByteStream) -> Self {
        let entry_count = bst.read_byte();
        let seg_indicator = bst.read_byte();

        let entry_type = match seg_indicator {
            0x00 => EntryType::Unused,
            0x01..=0xFE => {
                let flag_word = bst.read_byte();

                EntryType::Fixed {
                    flag_word,
                    offset: bst.read_word(),
                }
            }
            0xFF => {
                let flag_word = bst.read_byte();
                if bst.read_word() != 0x3FCD {
                    unreachable!();
                }
                let seg_num = bst.read_byte();

                EntryType::Moveable {
                    flag_word,
                    seg_num,
                    offset: bst.read_word(),
                }
            }
        };

        Self {
            entry_count,
            seg_indicator,
            entry_type,
        }
    }
}

pub enum EntryType {
    Unused,
    Fixed {
        flag_word: u8,
        offset: u16,
    },
    Moveable {
        flag_word: u8,
        seg_num: u8,
        offset: u16,
    },
}

pub struct NonResidentNameTable {
    pub length: u8,
    pub text: String,
    pub ordinal: u16,
}
impl NonResidentNameTable {
    pub fn read(bst: &mut ByteStream) -> Self {
        let length = bst.read_byte();
        let text = if length > 0 { bst.read_string(length as usize) } else { String::new() };
        let ordinal = if length > 0 { bst.read_word() } else { 0 };

        Self {
            length,
            text,
            ordinal,
        }
    }
}
