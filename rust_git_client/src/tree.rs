use std::io;
use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use crypto::sha1::Sha1;
use crypto::digest::Digest;

pub struct Blob {
    pub hash: String,
    pub data: Vec<u8>,
}

pub enum Tree {
    BlobEntry { blobName: String, hashCode: String },
    TreeEntry {
        treeName: String,
        hashCode: String,
        children: Vec<Tree>,
    }
}


impl Blob {
    pub fn fromPath(path: &PathBuf) -> io::Result<Blob> {
        let mut bytes = Vec::new();
        let mut f = File::open(path)?;
        
        f.read_to_end(&mut bytes)?;

        let mut sha = Sha1::new();
        sha.input(&bytes);

        Ok(
            Blob {
                hash: sha.result_str(),
                data: bytes,
            }
        )
    }
}