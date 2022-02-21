use std::fmt;
use std::io;

pub enum GitClientError {
    IoError(io::Error),
    DirectoryNotFound,
    CommitInvalid,
    IndexCorrupt,
}
//Displaying error messages
impl fmt::Display for GitClientError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &GitClientError::IoError(ref a) => a.fmt(formatter),
            &GitClientError::DirectoryNotFound => formatter.write_str("Directory Not Found"),
            &GitClientError::CommitInvalid => formatter.write_str("Invalid Commit"),
            &GitClientError::IndexCorrupt => formatter.write_str("Corrupt Index"),
        }
    }
}

impl From<io::Error> for GitClientError {
    fn from(err: io::Error) -> GitClientError {
        GitClientError::IoError(err)
    }
}