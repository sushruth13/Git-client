#![allow(unused_variables, unused_assignments)]
#[macro_use]
extern crate lazy_static;

use std::env;
//use std::process::exit;


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
mod log;
mod git_commit;
mod editor;
use clap::{App, Arg, SubCommand};

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
    .subcommand(SubCommand::with_name("branch").about("Displays branches in a repository"));
    //.subcommand(SubCommand::with_name("new").about("Displays branches in a repository"))

    // .get_matches();
    // match m.subcommand() {
    //     ("init", Some(..)) => match init::init() {
    //         Ok(()) => println!("Repository initialized"),
    //         Err(..) => println!("Already initialized"),
    //     },
    //     ("add", Some(submatch)) => {
    //         match add::add_all(&submatch.values_of("file").unwrap().collect()) {
    //             Ok(()) => println!("file/s Added to stagging area"),
    //             Err(e) => println!("Error: {}", e),
    //         }
    //     }
    //     ("commit", Some(..)) => {
    //         match commit::commit() {
    //             Ok(()) => (),
    //             Err(e) => println!("Error: {}", e)
    //         }
    //     }
    //     //need to pass url to the clone function
    //     /*("clone",Some(..)) =>{
    //        clone::clone();
    //        println!("The cloning is completed");
    //     } */
    //     ("auth",Some(..))=>{
    //         auth::genKeys();
    //         println!("Generating ssh keys,ssh keys are saved in the default location ~/.ssh");
    //     }
    //     ("check-auth",Some(..))=>{
    //         checkauth::check_auth();
    //         println!("Ssh keys are valid!");
    //     }
    //     ("branch", Some(submatch)) => {
    //         match branch::display_time_branch(&submatch.values_of("branch").unwrap().collect())
    //         {
    //             Ok(()) => println!("file/s Added to stagging area"),
    //             Err(e) => println!("Error: {}", e),
    //         }

    //     }

    //     _ => println!("Please enter valid gitclient command"),
    //     }

    //Backup code
    
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let i = args[1].as_str();
    let path=args[2].as_str();
    

    match i {
        "init" => match init::init() {
            Ok(()) => println!("Repository initialized"),
            Err(..) => println!("Already initialized"),
        },
        "add" => match add::add_all(&vec![args[2].as_str()]){
            Ok(()) => println!("file/s Added to stagging area"),
            Err(e) => println!("Error: {}", e),
        },
        "commit" => match commit::commit() {
            Ok(()) => (),
            Err(e) => println!("Error: {}", e)
        },
        "clone" =>  {
            match args.get(2){
                Some(_x) => clone::clone(&args[2]),
                None => println!("URL missing please add url to run clone!Currently support only exists for https and git")
            }
        },
        "auth" => {
            auth::gen_keys();
            println!("Generating ssh keys,ssh keys are saved in the default location ~/.ssh");
        },
        "check-auth" => {
            checkauth::check_auth();
            println!("Ssh keys are valid!");
        },
        "branch" => branch::display_time_branch(&args[2]),
        "log"=> {
            match args.get(2){
                Some(_x) => log::log(&args[2]),
                None => println!("Missing Repo path,please add repo path")

            }
        },
        "git_commit"=> {
            match args.get(2){
            Some(_x)=>git_commit::commit(&args[2],&args[3]),
            None => print!("Missing repo name in cmd, please add it")
            }
        }
        "editor"=>{
            match args.get(2){
                Some(_x)=>editor::editor(),
                None => print!("Unable to open vim editor")

            }
        }
        ,
        
        
        _ => println!("enter valid command"),

    }
}