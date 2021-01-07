use rand_core::OsRng;
use p256::ecdsa::VerifyingKey;
use p256::ecdsa::SigningKey;
use crate::error::{self, Error};
use std::io::prelude::{Write, BufRead};
use std::path::{Path};
extern crate ecdsa;

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

    fn from_path(path: &Path) -> Result<KeyManager, Error>{
        if let Ok(file) = std::fs::File::open(path){
            let mut buf_reader = std::io::BufReader::new(file);
            let mut signing_key_bs58_string = String::new();
            let mut verifying_key_bs58_string = String::new();
            let sk_res = buf_reader.read_line(&mut signing_key_bs58_string)
                .or_else(crate::util::std_io_error_pack)?;
            let vk_res = buf_reader.read_line(&mut verifying_key_bs58_string)
                .or_else(crate::util::std_io_error_pack)?;
            if sk_res == 0 || vk_res == 0 { return Err(Error::KeyPairNotFoundInFileError)}
            let mut sk_bytes = bs58::decode(signing_key_bs58_string.trim()).into_vec()
                .or_else(|error|Err(Error::BS58DecodeError(error)))?;
            let mut vk_bytes = bs58::decode(verifying_key_bs58_string.trim()).into_vec()
                .or_else(|error|Err(Error::BS58DecodeError(error)))?;
            let sk = SigningKey::from_bytes(&sk_bytes)
                .or_else(|error|Err(Error::KeyPairFromBytesError(error)))?;
            let vk = VerifyingKey::from_encoded_point(&ecdsa::EncodedPoint::from_bytes(&vk_bytes)
                .or_else(|error|Err(Error::VKFromEncodedPointError(error)))?)
                .or_else(|error|Err(Error::KeyPairFromBytesError(error)))?;
            return Ok(KeyManager{
                file_path: path.to_str().unwrap().to_string(),
                signing_key: sk,
                verifying_key: vk,
            })
        }
        Err(Error::KeyPairFileNotExistError)
    }

    fn from_path_checked(path: &Path) -> Result<KeyManager, Error>{
        let km = KeyManager::from_path(path)?;
        if !crate::util::verify_keypair(&km.signing_key, &km.verifying_key) { return Err(Error::KeyPairVerifyFailError); }
        return Ok(km);
    }

    pub fn from_current_dir_checked(file_name: &str) -> Result<KeyManager, Error>{
        let dir = std::env::current_dir()
            .or_else(|error|Err(Error::KeyManagerInvalidWorkingDirError))?;
        let path = dir.join(file_name);
        KeyManager::from_path_checked(&path)
    }

}