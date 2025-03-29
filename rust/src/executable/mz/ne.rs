use crate::byte_stream::ByteStream;

pub struct NewExecutable {}

impl NewExecutable {
    pub fn signature(&self) -> String {
        "NE".to_owned()
    }
    pub fn read(bst: &mut ByteStream) -> Self {
        NewExecutable {}
    }
}
