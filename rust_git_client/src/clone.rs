   
use git2::{Repository,Cred, Error, RemoteCallbacks};
use std::env;
use std::path::Path;
pub fn clone(url: &str){
    if url.starts_with("https"){
    println!("Running git clone for https!");
    let path="Cloned-folder";
    let _repo = match Repository::clone(url,path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
    }


else{
    print!("Running git clone for git ssh url");
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
      Cred::ssh_key(
        username_from_url.unwrap(),
        None,
        Path::new(&format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap())),
        None,
      )
    });
  
    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);
  
    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);
  
    // Clone the project.
    builder.clone(
      url,
      Path::new("Cloned-Folder"),
    ).expect("Could not clone repo");
    


}
    
  

  
}