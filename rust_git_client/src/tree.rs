#![allow(dead_code)]
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
    BlobEntry { blob_name: String, hash_code: String },
    TreeEntry {
        tree_name: String,
        hash_code: String,
        children: Vec<Tree>,
    }
}


impl Blob {
    pub fn from_path(path: &PathBuf) -> io::Result<Blob> {
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