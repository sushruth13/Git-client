use git2::{Commit, ObjectType, Repository};
pub fn log(path: &str){
    let repo_root = path;
    let repo = Repository::open(repo_root).expect("Couldn't open repository");
    let commit = find_last_commit(&repo).expect("Couldn't find last commit");
    display_commit(&commit);

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn display_commit(commit: &Commit) {
    // let timestamp = commit.time().seconds();
    println!("commit {}\nAuthor:    {}\n\n    {}",
             commit.id(),
             commit.author(),
             //tm.rfc822(),
             commit.message().unwrap_or("no commit message"));
}
}