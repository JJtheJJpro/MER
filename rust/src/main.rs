//use executable::Signature;

pub mod apis;
pub mod byte_operation;
pub mod byte_stream;
pub mod executable;

//fn log_info(exe: Signature) {}

fn mz_ne_test() {
    const FILE1: &str = "../ref/BST.EXE";
    const FILE2: &str = "../ref/BSTCDRES.DLL";
    const FILE3: &str = "../ref/SETUPBST.EXE";
    const FILE4: &str = "../ref/SETUP.EXE";
    println!("Testing MZ NE-extended files...");
    //log_info(executable::read(FILE1).unwrap());
    match executable::read(FILE1) {
        Ok(_) => println!("BST.EXE Success"),
        Err(e) => eprintln!("BST.EXE Fail: {e}"),
    }
    //log_info(executable::read(FILE2).unwrap());
    match executable::read(FILE2) {
        Ok(_) => println!("BST.EXE Success"),
        Err(e) => eprintln!("BSTCDRES.DLL Fail: {e}"),
    }
    //log_info(executable::read(FILE3).unwrap());
    match executable::read(FILE3) {
        Ok(_) => println!("SETUPBST.EXE Success"),
        Err(e) => eprintln!("SETUPBST.EXE Fail: {e}"),
    }
    //log_info(executable::read(FILE4).unwrap());
    match executable::read(FILE4) {
        Ok(_) => println!("SETUP.EXE Success"),
        Err(e) => eprintln!("SETUP.EXE Fail: {e}"),
    }
}

//fn mz_pe_test() {
//    const FILE1: &str = "../ref/songplayer.exe";
//    //log_info(executable::read(FILE1).unwrap());
//}

fn main() {
    mz_ne_test();
    //mz_pe_test();
}
