use std::{
    fs::File,
    io::{Error, Read},
    path::Path,
};

use mz::MZ;

use crate::byte_stream::ByteStream;

pub mod mz;

/// Used for the interrupt operation (OpCode 0xCD), indicating any changes to the code or the program.
pub enum InteruptChange {
    /// No change.
    None,
    /// Exit the Program (aka Stop executing code).
    Exit,
    /// Indicating that a string was read from beginning position to ending position.
    SkipString(usize, usize),
}

/// Represents the Signature (or Magic Number) of an executable.
pub enum Signature {
    /// Every Windows Executable and DOS program requires the MZ header.
    MZ(MZ),
    /// Linux and other Unix-like systems use the Executable and Linkable Format (ELF).
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
pub fn read<P: AsRef<Path>>(file_name: P) -> Result<Signature, Error> {
    match File::open(file_name) {
        Ok(mut file) => {
            // later, we'll implement a faster and much more efficient system that avoids storing the whole exe in memory.
            // for now, suffer.
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            let mut bst = ByteStream::new(buf);
            match &bst.read_bytes(2)[0..2] {
                b"MZ" => Ok(Signature::MZ(MZ::read(&mut bst))),
                &[0x7F, 0x45] => {
                    if &bst.read_bytes(2)[0..2] == b"LF" {
                        Ok(Signature::ELF)
                    } else {
                        Err(Error::last_os_error())
                    }
                }
                &[0xFE, 0xED] => match &bst.read_bytes(2)[0..2] {
                    &[0xFA, 0xCE] => Ok(Signature::MachO32BE),
                    &[0xFA, 0xCF] => Ok(Signature::MachO64BE),
                    _ => Err(Error::last_os_error()),
                },
                &[0xCE, 0xFA] => {
                    if &bst.read_bytes(2)[0..2] == &[0xED, 0xFE] {
                        Ok(Signature::MachO32LE)
                    } else {
                        Err(Error::last_os_error())
                    }
                }
                &[0xCF, 0xFA] => {
                    if &bst.read_bytes(2)[0..2] == &[0xED, 0xFE] {
                        Ok(Signature::MachO64LE)
                    } else {
                        Err(Error::last_os_error())
                    }
                }
                &[0xCA, 0xFE] => {
                    if &bst.read_bytes(2)[0..2] == &[0xBA, 0xBE] {
                        Ok(Signature::MachOUBBE)
                    } else {
                        Err(Error::last_os_error())
                    }
                }
                &[0xBE, 0xBA] => {
                    if &bst.read_bytes(2)[0..2] == &[0xFE, 0xCA] {
                        Ok(Signature::MachOUBLE)
                    } else {
                        Err(Error::last_os_error())
                    }
                }
                _ => Err(Error::last_os_error()),
            }
        }
        Err(e) => Err(e),
    }
}