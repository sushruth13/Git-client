use std::fs::File;
use std::io::prelude::*;

pub fn config(usrname: &str, email: &str)  -> std::io::Result<()> {
    let name=usrname.to_string();
    let mail=email.to_string();
    let mut file = File::create("cred.env")?;
    write!(file,"{}\n",name);
    write!(file,"{}\n",mail);

    Ok(())
}
