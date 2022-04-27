#![allow(unused_variables, unused_imports)]
use git2::{Index, Error, DiffLineType, DiffOptions, Oid, Signature, Time, Repository, Tree};
use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn diff_basic(path:  &str){
	//let mut old_tree = Tree::id(Oid);
	//let path_root = Path::new("$HOME/Git-client/rust_git_client/.GitClient/");
	//let path_root = Repository::workdir();
	//let repo_root = Repository::discover(path_root).unwrap();
    let repo = Repository::open(path).expect("Couldn't open repository");
	//let cloned_tree = Tree::is_empty();
	let mut opts = DiffOptions::new();
	opts.include_untracked(true);
	let diff = repo.diff_tree_to_workdir(None, Some(&mut opts)).unwrap();
	//let diff = repo.diff_index_to_workdir(None, Some(&mut opts)).unwrap();
	println!("lenght of deltas ----- {:}",diff.deltas().len());
	let stats = diff.stats().unwrap();
	println!("Number of insertions ----- {:}",stats.insertions());
	println!("Number of deletions ------ {:}",stats.deletions());
	println!("Total  ------------------- {:}",stats.files_changed());

	//let path_root = Path::new(path);
	//let mut index = repo.index().unwrap();
	// index.add_path(path_root);
	// let mut diff2 = repo.diff_tree_to_index(None, Some(&index), Some(&mut opts));
	
}

