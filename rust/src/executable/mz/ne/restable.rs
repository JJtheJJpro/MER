use crate::byte_stream::ByteStream;

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
    pub fn read(bst: &mut ByteStream, length: u16) -> Self {
        let bst_start = bst.pos;

        let align_shift = bst.read_word();
        let mut types = Vec::new();
        while bst.peek_word() != 0 {
            types.push(TypeInfo::read(bst));
        }
        bst.read_word();

        let mut resource_strings = Vec::new();
        if bst.pos - bst_start < length as usize {
            while bst.peek_byte() != 0 {
                let _t = bst.read_byte();
                resource_strings.push(bst.read_string(_t as usize));
            }
            _ = bst.read_byte();
        }

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
