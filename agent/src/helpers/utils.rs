use std::{env, path::PathBuf};

use crate::models::UtilError;

pub fn copy_exe() -> Result<PathBuf, UtilError> {
    // Get the current executable path
    let current_exe = std::env::current_exe().unwrap();

    // Create a file dialog to select a directory
    let path = get_target_loc();
    let exe_name = env::var("TARGET_EXE_NAME");

    if exe_name.is_err() {
        return Err(UtilError::NoTargetExeName);
    }

    // Construct the new executable path
    let executable_path = path.join(&exe_name.unwrap());

    // Copy the current executable to the selected directory
    match std::fs::copy(&current_exe, &executable_path) {
        Ok(_) => Ok(executable_path),
        Err(err) => Err(UtilError::CantCopyFileError(
            String::from(executable_path.to_str().unwrap()),
            err,
        )),
    }
}

pub fn get_target_loc() -> PathBuf {
    // Create a file dialog to select a directory
    match native_dialog::FileDialog::new()
        .set_location("~") // Default location (home directory)
        .set_title("Select a directory for saving TrackVaul's bin file") // Dialog title
        .show_open_single_dir() // Open a single directory
        .unwrap_or_else(|err| {
            eprintln!("File Error: {}", err);
            std::process::exit(1);
        }) {
        Some(path) => path,
        None => {
            eprintln!("Error selecting folder");
            std::process::exit(1)
        }
    }
}
