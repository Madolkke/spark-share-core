
use rand_core::OsRng;
use p256::ecdsa::VerifyingKey;
use p256::ecdsa::SigningKey;
use crate::error::Error;
use std::io::prelude::{Write, BufRead};
use std::path::{Path, PathBuf};

pub struct KeyManager{
    file_path: String,
    signing_key: SigningKey,
    verifying_key: VerifyingKey
}

impl KeyManager{
    fn create_keypair() -> (SigningKey, VerifyingKey){
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);
        (signing_key, verifying_key)
    }

    fn new(path_str: &str) -> KeyManager{
        unimplemented!()
    }

    pub fn new_current_dir(file_name: &str) -> Result<KeyManager, Error>{
        if let Ok(current_dir) = std::env::current_dir(){
            let file_path = current_dir.join(file_name);
            let file_path_string = file_path.to_str().unwrap().to_string();
            match std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(file_path)
            {
                Ok(mut file) => {
                    let keypair = KeyManager::create_keypair();
                    let sk_bs58 = bs58::encode(keypair.0.to_bytes()).into_string();
                    let vk_bs58 = bs58::encode(keypair.1.to_encoded_point(true).to_bytes()).into_string();
                    let content = format!("{}\n{}", sk_bs58, vk_bs58);
                    if let Err(error) = file.write_all(content.as_bytes()){
                        return Err(Error::StdIOError(error))
                    }
                    return Ok(KeyManager{
                        file_path: file_path_string,
                        signing_key: keypair.0,
                        verifying_key: keypair.1
                    })
                }
                Err(error) => { Err(Error::StdIOError(error))}
            }
        }else{ return Err(Error::KeyManagerInvalidWorkingDirError); }
    }

    fn from_path(path: Path) -> Result<KeyManager, Error>{
        if let Ok(file) = std::fs::File::open(path){
            let buf_reader = std::io::BufReader::new(file);
            let signing_key_string = String::new();
            let verifying_key_string = String::new();
        }
        Err(Error::KeyPairFileNotExistError);
    }

    fn from_current_dir_checked(path_str: &str) -> KeyManager{
        unimplemented!()
    }



}