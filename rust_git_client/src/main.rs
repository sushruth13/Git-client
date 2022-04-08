// #[allow(unused_variables, unused_assignments)]
#[macro_use]
extern crate lazy_static;


//use std::env;
use std::process::exit;





mod add;
mod commit;
mod error;
mod file;
mod index;
mod init;
mod tree;
mod auth;
mod clone;
mod checkauth;
mod branch;
use clap::{App, Arg, SubCommand};
use std::fs::canonicalize;
use git2::Repository;
use std::path::Path;
use git2::{ ObjectType, Blob, Commit, Tree};
use git2::{Oid, Signature};
use std::fs::File;
use std::io::Write;
use git2::{Cred, RemoteCallbacks};
use std::env;
use colored::Colorize;
use getopts::Options;

fn main() {


    let m =  App::new("GitClient")
    .about("A custom Rust-based git client")
    // //.subcommand_required(true)
    // .arg_required_else_help(true)
    // .allow_external_subcommands(true)
    // .allow_invalid_utf8_for_external_subcommands(true)
    .subcommand(
        SubCommand::with_name("auth")
        .about("Generates ssh keys for git authentication,please generate keys and add keys to the repo once done"),

    )
    .subcommand(
        SubCommand::with_name("check-auth")
        .about("Checking if the ssh-keys generated are valid")
    )
    .subcommand(
        SubCommand::with_name("clone")
        .about("To clone a repo on github.come, please pass the ssh url")

    )
    .subcommand(SubCommand::with_name("commit").about("Comits the file"))
    .subcommand(
        SubCommand::with_name("add").about("Add a file").arg(
            Arg::with_name("file")
                .help("File to add")
                .index(1)
                .multiple(true)
                .required(true),
        )
    )
    .subcommand(SubCommand::with_name("init").about("Initializes the repository"))
    .subcommand(SubCommand::with_name("branch").about("Displays branches in a repository"))
    //.subcommand(SubCommand::with_name("new").about("Displays branches in a repository"))

    .get_matches();
    match m.subcommand() {
        ("init", Some(..)) => match init::init() {
            Ok(()) => println!("Repository initialized"),
            Err(..) => println!("Already initialized"),
        },
        ("add", Some(submatch)) => {
            match add::add_all(&submatch.values_of("file").unwrap().collect()) {
                Ok(()) => println!("file/s Added to stagging area"),
                Err(e) => println!("Error: {}", e),
            }
        }
        ("commit", Some(..)) => {
            match commit::commit() {
                Ok(()) => (),
                Err(e) => println!("Error: {}", e)
            }
        }
        //need to pass url to the clone function
        /*("clone",Some(..)) =>{
           clone::clone();
           println!("The cloning is completed");
        } */
        ("auth",Some(..))=>{
            auth::genKeys();
            println!("Generating ssh keys,ssh keys are saved in the default location ~/.ssh");
        }
        ("check-auth",Some(..))=>{
            checkauth::check_auth();
            println!("Ssh keys are valid!");
        }
        ("branch", Some(submatch)) => {
            match branch::show_branches_and_time(&submatch.values_of("branch").unwrap().collect())
            {
                Ok(()) => println!("file/s Added to stagging area"),
                Err(e) => println!("Error: {}", e),
            }

        }

        _ => println!("Please enter valid gitclient command"),
        }

    //Backup code
    /*
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let i = args[1].as_str();
    //println!("{}", i);
    let submatch = Some(args[2]);

    match i {
        "init" => match init::init() {
            Ok(()) => println!("Repository initialized"),
            Err(..) => println!("Already initialized"),
        },
        "add" => match add::add_all(&submatch.values_of("file").unwrap().collect()) {
            Ok(()) => println!("file/s Added to stagging area"),
            Err(e) => println!("Error: {}", e),
        },
        "commit" => match commit::commit() {
            Ok(()) => (),
            Err(e) => println!("Error: {}", e)
        },
        "clone" => {
            clone::clone(&args[2]);
            println!("The cloning is completed");
         },
        "auth" => {
            auth::genKeys();
            println!("Generating ssh keys,ssh keys are saved in the default location ~/.ssh");
        },
        "check-auth" => {
            checkauth::check_auth();
            println!("Ssh keys are valid!");
        },
        "branch" => branch::show_branches_and_time(&args[2]),
        _ => println!("enter valid command"),
    };
   */


}