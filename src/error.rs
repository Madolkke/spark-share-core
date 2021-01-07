#[derive(Debug)]
pub enum Error{
    KeyManagerInvalidWorkingDirError,
    KeyPairFileNotExistError,
    KeyPairNotFoundInFileError,
    KeyPairFromBytesError(ecdsa::Error),
    VKFromEncodedPointError(ecdsa::elliptic_curve::Error),
    KeyPairVerifyFailError,
    StdIOError(std::io::Error),
    BS58DecodeError(bs58::decode::Error)
}