use git2::Repository;
use std::path::Path;


pub fn init(path: &str){
    let fpath= Path::new(path);
    std::fs::create_dir(path).unwrap();
    // std::fs::create_dir(fpath.join("refs"));
    // std::fs::create_dir(fpath.join("objects"));
    // std::fs::create_dir(fpath.join("refs").join("heads"));

    let repo = match Repository::init(fpath) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

    // std::fs::create_dir(path).unwrap();
    // std::fs::create_dir(fpath.join("refs"));
    // std::fs::create_dir(fpath.join("objects"));
    // std::fs::create_dir(fpath.join("refs").join("heads"));

   // let mut head = std::fs::File::create(fpath.join("HEAD"));
    //head.write_all("refs: refs/heads/master".as_bytes())?;

}