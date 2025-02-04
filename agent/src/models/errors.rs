use std;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilError {
    #[error("Can't able to Copy the parent executable to target folder: {0} \n{1}")]
    CantCopyFileError(String, std::io::Error),
    #[error("Can't get environment variable for Target exe name")]
    NoTargetExeName,
}
