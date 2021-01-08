use std::hash::Hash;

use p256::ecdsa::VerifyingKey;

use crate::file::size::Size;
use p256::ecdsa::{Signature};
use sha3::Sha3_512;
type Hash = Sha3_512;

pub struct FileTree;
enum ResourceConfig{

}
pub struct Resource{
    name: Option<String>,
    default_tracker: Option<String>,
    pub_key: Option<VerifyingKey>,
    size: Option<Size>,
    blocks: Option<usize>,
    file_tree: Option<FileTree>,
    block_hashes: Option<Vec<Hash>>,
    resource_hash: Option<Hash>,
    signature: Option<Signature>,
}