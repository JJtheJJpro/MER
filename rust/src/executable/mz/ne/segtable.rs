use crate::byte_stream::ByteStream;

pub struct SegmentTable {
    pub data_offset: u16,
    pub length: u16,
    pub flags: u16,
    pub min_alloc_size: u16,
}

impl SegmentTable {
    pub fn read(bst: &mut ByteStream) -> Self {
        Self {
            data_offset: bst.read_word(),
            length: bst.read_word(),
            flags: bst.read_word(),
            min_alloc_size: bst.read_word(),
        }
    }
}
