use std::process::Command;

pub fn check_auth(){
    let check=Command::new("ssh -T git@github.com")
            .output()
            .expect("failed run ssh");
    // println!("status: {}", check.status());
    // println!("stdout: {}", String::from_utf8_lossy(&check.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&check.stderr));
    // if check.status == 0
    // {

    // }
            
    
}