use std::process;
use std::env;
use rfsearch::*;
fn main() {
    let mut args=env::args();
    args.next();
    let dir_path=args.next().unwrap_or_else(||{
        eprintln!("rfsearch [path] [regex] ?[parallel?]");
        eprintln!("specify the directorry path please!");
        process::exit(1);
    });
    let regex=args.next().unwrap_or_else(||{
        eprintln!("rfsearch [path] [regex] ?[parallel?]");
        eprintln!("specify the search string please!");
        process::exit(1);
    });
    let mut mode=ProcessOption::One;
    let mut file_ext=String::from("_");
    if let Some(ext)=args.next(){
        file_ext=ext;
    }
    if let Some(_mode)=args.next(){
        mode = ProcessOption::Parallel;
    }
    parse_files(dir_path.clone(), regex.clone(), mode, file_ext);
}
