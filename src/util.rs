use p256::ecdsa::{signature::Signer, signature::Verifier};

pub(crate) fn std_io_error_pack(error: std::io::Error) -> Result<usize, crate::error::Error>{
    Err(crate::error::Error::StdIOError(error))
}

pub(crate) fn verify_keypair(sk: &p256::ecdsa::SigningKey, vk: &p256::ecdsa::VerifyingKey) -> bool{
    let sig = sk.sign(b"");
    vk.verify(b"", &sig).is_ok()
}