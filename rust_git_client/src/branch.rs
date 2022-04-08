use git2::{Repository, BranchType, Branch};
use chrono::{Utc, TimeZone, DateTime};
use std::option::Option as Option;
use colored::*;

struct BranchCommitTime {
    branch: String,
    last_commit_time: DateTime<Utc>,
    Key_hash: String,
}

pub fn find_all_locals(repo: &Repository) -> git2::Branches {
    let branches_list = match repo.branches(Option::Some(BranchType::Local)) {
        Ok(br) => br,
        Err(e) => panic!("failed to init: {}", e),
    };

    return branches_list;
}

pub fn get_hash(name: &str) -> String {
    let mut hashed_name = String::from("");
    for (pos, e) in name.chars().enumerate() {
        match pos {
            0..=2 => hashed_name.push(e),
            _ => hashed_name.push(e)
        };
    }
    return hashed_name
}

pub fn find_name_of_branch(branch: &Branch) -> Option<String> {
    let name = match branch.name() {
        Ok(r) => r.map(|n| String::from(n)),
        Err(_) => None
    };

    return name;
}

pub fn display_time_branch(dir: &str) {

    let repo = match Repository::open(dir) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };


    let branches_list = find_all_locals(&repo);
    let bct_list = branches_list.filter_map(|branch| {


        return match branch {
            Ok((branch, _)) => {
                let branch_name = find_name_of_branch(&branch);
                let last_commit = get_branch_last_commit(&branch, &repo);

                
                let bct = branch_name
                    .and_then(|n| last_commit.map(|t|
                        BranchCommitTime { branch: n, last_commit_time: t.1, Key_hash: t.0 }
                    ));

                    ;

                bct
            }
            Err(_) => None
        };
    });


    let mut bct_v: Vec<_> = bct_list.collect();
    bct_v.sort_by(|a, b| b.last_commit_time.cmp(&a.last_commit_time));
    bct_v.iter().for_each(|bc| {

        let hashed_name = get_hash(bc.branch.as_str());
        println!("{}",  hashed_name.blue());
        }
    );
}


pub fn get_branch_last_commit(branch: &Branch, repo: &Repository) -> Option<(String,DateTime<Utc>)> {
    let p = branch.get().target().and_then(|oid| {
        let commit = repo.find_commit(oid);

        let t = commit.map(|c|
            (c.id().to_string(), Utc.timestamp(c.time().seconds(), 0))
        );

        let res = t.ok();
        res
    });
    return p;
}

