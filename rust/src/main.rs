use executable::Executable;

pub mod byte_stream;
pub mod executable;
pub mod mz;
pub mod byte_operation;
pub mod apis;

fn log_info(exe: Executable) {}

fn main() {
    const FILE1: &str = "C:/Users/jjthe/Desktop/16-bit Programs/Spelling Jungle/BST.EXE";
    const FILE2: &str = "C:/Users/jjthe/Desktop/16-bit Programs/Spelling Jungle/BSTCDRES.DLL";
    const FILE3: &str = "C:/Users/jjthe/Downloads/American Girls Premiere/DISK1/SETUP.EXE";

    log_info(executable::Executable::read(FILE1).unwrap());
    log_info(executable::Executable::read(FILE2).unwrap());
    log_info(executable::Executable::read(FILE3).unwrap());
}
