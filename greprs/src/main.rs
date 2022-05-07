use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use std::path::Path;


fn main() -> io::Result<()> {
    // println!("Hello, world!");
    // let mut var :i32;
    // let mut ip = String::new();
    // io::stdin().read_line(&mut ip).expect("Value please");

    // var = ip.trim().parse().expect("Only numbers");
    // println!("{}", var);
    let currentdir = std::env::current_dir()?;

    let file_loc = Path::new("src/main.rs");
    let file_full = currentdir.join("src/main.rs");
    // let mut f = File::open("main.rs")?;

    // let mut buff = [0; 10];

    // let n = f.read(&mut buff)?;
    // println!("{:?}", &buff[..n]);
    println!("Path {}", file_full.display());
    println!("Exists {}", file_full.is_file());



    let args: Vec<String> = std::env::args().collect();
    let search_str = &args[1];
    println!("Args {}", search_str);

    
    Ok(())
    
}
