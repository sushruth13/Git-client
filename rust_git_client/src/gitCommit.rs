use git2::{Commit, ObjectType, Repository,Oid, Signature,Direction};
use std::path::Path;

pub fn commit(path: &str, fpath:&str){
    let repo_root = path;
    let fpath= Path::new(fpath);
    let repo = Repository::open(repo_root).expect("Couldn't open repository");
    let commit = find_last_commit(&repo).expect("Couldn't find last commit");
     let commit_id = add_and_commit(&repo,&fpath, "Added example file")
    .expect("Couldn't add file to repo");
    println!("New commit: {}", commit_id);
    println!("{} state={:?}", repo.path().display(), repo.state());


fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}
fn add_and_commit(repo: &Repository, path: &Path, message: &str) -> Result<Oid, git2::Error> {
    let mut index = repo.index()?;
    index.add_path(path)?;
    let oid = index.write_tree()?;
    let signature = Signature::now("Test User", "test@testEmail.com")?;
    let parent_commit = find_last_commit(&repo)?;
    let tree = repo.find_tree(oid)?;
    repo.commit(Some("HEAD"), //  point HEAD to our new commit
                &signature, // author
                &signature, // committer
                message, // commit message
                &tree, // tree
                &[&parent_commit]) // parents
}
}