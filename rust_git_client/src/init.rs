use super::error::GitClientError;


use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;



pub fn init() -> Result<(), GitClientError> {
    let dir = Path::new("/home/naina-13/test_git/.GitClient");

    fs::create_dir(dir)?;
    fs::create_dir(dir.join("refs"))?;
    fs::create_dir(dir.join("objects"))?;
    fs::create_dir(dir.join("refs").join("heads"))?;

    let mut head = File::create(dir.join("HEAD"))?;
    head.write_all("refs: refs/heads/master".as_bytes())?;
    Ok(())
}