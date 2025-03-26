use std::{fs::File, io::{Error, Read}, path::Path};

use ne::NewExecutable;

use crate::{byte_stream::ByteStream, mz::MZ};

pub mod ne;

pub enum InteruptChange {
    None,
    String(u16, u16),
}

pub enum Signature {
    MZ,
    NE,
    LE,
    LX,
    PE,
}

pub trait ExecutableType {
    fn signature(&self) -> Signature where Self: Sized;
    fn read(bst: &mut ByteStream) -> Self where Self: Sized;
}

pub struct Executable {
    pub header: MZ,
    pub executable: Box<dyn ExecutableType>,
}

impl Executable {
    pub fn read<P: AsRef<Path>>(file_name: P) -> Result<Executable, Error> {
        match File::open(file_name) {
            Ok(mut file) => {
                let mut buf = Vec::new();
                file.read_to_end(&mut buf).unwrap();
                let mut bst = ByteStream::new(buf);
                let mz = MZ::read(&mut bst);
                let exe_magic = bst.peek_word().to_ne_bytes();
                let executable = Box::new(match &exe_magic {
                    b"NE" => {
                        NewExecutable::read(&mut bst)
                    }
                    &_ => {
                        return Err(Error::last_os_error())
                    }
                });
                Ok(Executable { header: mz, executable })
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}
