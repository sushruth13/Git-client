use std::process::Command;

pub fn check_auth(){
    let mut run_ssh=Command::new("bash");
    run_ssh.arg("-c")
    .arg("ssh -T git@github.com");
    let check_ssh = run_ssh.output().expect("failed to execute process");
            
    
}