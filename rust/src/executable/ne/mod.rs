use super::{ExecutableType, Signature};

pub struct NewExecutable {

}

impl ExecutableType for NewExecutable {
    fn signature(&self) -> Signature where Self: Sized {
        Signature::NE
    }
    fn read(bst: &mut crate::byte_stream::ByteStream) -> Self where Self: Sized {
        NewExecutable {}
    }
}