use executable::Signature;

pub mod apis;
pub mod byte_operation;
pub mod byte_stream;
pub mod executable;

fn log_info(exe: Signature) {
    
}

fn ne_test() {
    const FILE1: &str = "C:/Users/jjthe/Desktop/16-bit Programs/Spelling Jungle/BST.EXE";
    const FILE2: &str = "C:/Users/jjthe/Desktop/16-bit Programs/Spelling Jungle/BSTCDRES.DLL";
    const FILE3: &str = "C:/Users/jjthe/Downloads/American Girls Premiere/DISK1/SETUP.EXE";
    log_info(executable::read(FILE1).unwrap());
    log_info(executable::read(FILE2).unwrap());
    log_info(executable::read(FILE3).unwrap());
}

fn pe_test() {
    const FILE4: &str = "C:/Users/jjthe/Desktop/songplayer.exe";
    log_info(executable::read(FILE4).unwrap());
}

fn main() {
    ne_test();
    //pe_test();
}
