use std::process::Command;
use std::path::Path;

pub fn editor(){
    let _start_vim=Command::new("/usr/bin/bash")
    .arg("-c")
    .arg(concat!("vim Cloned-folder/"))
    .spawn()
    .expect("Error: Failed to run editor")
    .wait()
    .expect("Error: Editor returned a non-zero status");

}