#[macro_use]
extern crate lazy_static;

mod add;
mod commit;
mod error;
mod file;
mod index;
mod init;
mod tree;
mod auth;
mod clone;
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
        .about("To use ssh keys with the git repo,please follow the instructions in https://docs.github.com/en/authentication/connecting-to-github-with-ssh/about-ssh"),

    )
    .subcommand(
        SubCommand::with_name("check-auth")
        .about("Checking if the ssh-keys generated are valid")
    )
    .subcommand(
        SubCommand::with_name("clone")
        .about("To clone a repo on github.come, please pass the ssh url")

    )
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

    .get_matches();
    match m.subcommand() {
        ("init", Some(..)) => match init::init() {
            Ok(()) => println!("Repository initialized"),
            Err(..) => println!("Already initialized"),
        },
        ("add", Some(submatch)) => {
            match add::add_all(&submatch.values_of("file").unwrap().collect()) {
                Ok(()) => (),
                Err(e) => println!("Error: {}", e),
            }
        }
        ("commit", Some(..)) => {
            match commit::commit() {
                Ok(()) => (),
                Err(e) => println!("Error: {}", e)
            }
        }
        ("clone",Some(..)) =>{
           clone::clone();
           println!("The cloning is completed");
        }
        ("check-auth",Some(..))=>{
            auth::check_auth();
            println!("Checking ssh keys");
        }
        
        _ => println!("Command not recognized."),
    }
      
       
}