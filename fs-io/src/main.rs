use std::io::{Error, Read};
use std::{fs, future::Ready, fs::File};



fn read_user_name() -> Result<String, Error> {

    let mut cur_dir = std::env::current_dir().unwrap();


    let file_path = cur_dir.join("default1.txt");


    let f = fs::File::open(file_path);

    let mut f = match f {
        Ok(file) => file,
        // Err(error) => match error.kind() {
        //     ErrorKind::NotFound => match File::create("default.txt") {
        //         Ok(fc) => fc,
        //         Err(e) => panic!("Problem creating the file: {:?}", e),
        //     },
        //     other_error => {
        //         panic!("Problem opening the file: {:?}", other_error)
        //     }
        // },
        Err(error) => return Err(error)
        };


    let mut data = String::new();


    match f.read_to_string(&mut data) {
        Ok(_) => Ok(data),
        Err(e) => Err(e),
    }

}

fn main() {


    let data = read_user_name().unwrap();
    

    println!("User name {}", data);
        
}
    


