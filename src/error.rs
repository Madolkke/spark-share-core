#[derive(Debug)]
pub enum Error{
    KeyManagerInvalidWorkingDirError,
    KeyPairFileNotExistError,
    StdIOError(std::io::Error)
}