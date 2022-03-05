use git2::Repository;
//use std::env;
//use std::path::Path;
pub fn clone(){

    let url = "https://github.com/sushruth13/Airbnb.git";
    // let current_dir = std::env::current_dir();
    // let current_dir=String::from(current_dir);
    let _repo = match Repository::clone(url, "test-tmp") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
   
   


  
}