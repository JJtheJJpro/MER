use crate::byte_stream::ByteStream;
use mz::MZ;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    fs::File,
    io::Read,
    path::Path,
};

pub mod mz;

#[derive(Debug)]
pub enum ExecutableError {
    Message(String),
    InnerError(Box<dyn Error>),
}
impl ExecutableError {
    pub fn from_message(msg: impl Into<String>) -> Self {
        ExecutableError::Message(msg.into())
    }
    pub fn from_inner(inner: impl Error + 'static) -> Self {
        ExecutableError::InnerError(Box::new(inner))
    }
}
impl Error for ExecutableError {}
impl Display for ExecutableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExecutableError::Message(msg) => msg.clone(),
                ExecutableError::InnerError(err) => format!("{err}"),
            }
        )
    }
}

///// Used for the interrupt operation (OpCode 0xCD), indicating any changes to the code or the program.
//pub enum InteruptChange {
//    /// No change.
//    None,
//    /// Exit the Program (aka Stop executing code).
//    Exit,
//    /// Indicating that a string was read from beginning position to ending position.
//    SkipString(usize, usize),
//}

/// Represents the Signature (or Magic Number) of an executable.
pub enum Signature {
    /// Every Windows Executable and DOS program requires the MZ header.
    MZ(MZ),
    /// Linux and other Unix-like systems (except MacOS) use the Executable and Linkable Format (ELF).
    ELF,
    /// MacOS and iOS use the Mach-O format.  This is for 32-bit architectures that read in big endian.
    MachO32BE,
    /// MacOS and iOS use the Mach-O format.  This is for 32-bit architectures that read in little endian.
    MachO32LE,
    /// MacOS and iOS use the Mach-O format.  This is for 64-bit architectures that read in big endian.
    MachO64BE,
    /// MacOS and iOS use the Mach-O format.  This is for 64-bit architectures that read in little endian.
    MachO64LE,
    /// MacOS and iOS use the Mach-O format.  This is for universal binary architectures that read in big endian.
    MachOUBBE,
    /// MacOS and iOS use the Mach-O format.  This is for universal binary architectures that read in little endian.
    MachOUBLE,
}

/// Reads a given executable file.
pub fn read<P: AsRef<Path> + Clone>(file_name: P) -> Result<Signature, ExecutableError> {
    match File::open(file_name) {
        Ok(mut file) => {
            // later, we'll implement a faster and much more efficient system that avoids storing the whole exe in memory.
            // for now, suffer.
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            let mut bst = ByteStream::new(buf);
            match &bst.read_bytes(2)[0..2] {
                b"MZ" => match MZ::read(&mut bst) {
                    Ok(v) => Ok(Signature::MZ(v)),
                    Err(e) => Err(e),
                },
                &[0x7F, 0x45] => {
                    if &bst.read_bytes(2)[0..2] == b"LF" {
                        Ok(Signature::ELF)
                    } else {
                        Err(ExecutableError::from_message("Unknown executable header"))
                    }
                }
                &[0xFE, 0xED] => match &bst.read_bytes(2)[0..2] {
                    &[0xFA, 0xCE] => Ok(Signature::MachO32BE),
                    &[0xFA, 0xCF] => Ok(Signature::MachO64BE),
                    _ => Err(ExecutableError::from_message("Unknown executable header")),
                },
                &[0xCE, 0xFA] => {
                    if &bst.read_bytes(2)[0..2] == &[0xED, 0xFE] {
                        Ok(Signature::MachO32LE)
                    } else {
                        Err(ExecutableError::from_message("Unknown executable header"))
                    }
                }
                &[0xCF, 0xFA] => {
                    if &bst.read_bytes(2)[0..2] == &[0xED, 0xFE] {
                        Ok(Signature::MachO64LE)
                    } else {
                        Err(ExecutableError::from_message("Unknown executable header"))
                    }
                }
                &[0xCA, 0xFE] => {
                    if &bst.read_bytes(2)[0..2] == &[0xBA, 0xBE] {
                        Ok(Signature::MachOUBBE)
                    } else {
                        Err(ExecutableError::from_message("Unknown executable header"))
                    }
                }
                &[0xBE, 0xBA] => {
                    if &bst.read_bytes(2)[0..2] == &[0xFE, 0xCA] {
                        Ok(Signature::MachOUBLE)
                    } else {
                        Err(ExecutableError::from_message("Unknown executable header"))
                    }
                }
                _ => Err(ExecutableError::from_message("Unknown executable header")),
            }
        }
        Err(e) => Err(ExecutableError::from_inner(e)),
    }
}
