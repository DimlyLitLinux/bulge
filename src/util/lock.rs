use std::{fs, fs::File};
use std::path::Path;
use crate::util::macros::continue_prompt;

/// Creates a lock file indicating that bulge is open
pub fn create_lock() -> std::io::Result<()> {
    File::create("/var/lock/bulge.lock")?;
    Ok(())
}

/// Deletes the lock file
pub fn remove_lock() -> std::io::Result<()>{
    fs::remove_file("/var/lock/bulge.lock")?;
    Ok(())
}

/// Returns true if the lock file exists on the file system
pub fn check_lock() -> bool {
    Path::new("/var/lock/bulge.lock").exists()
}

/// Check if a bulge instance is already running and give the option of removing the lock file
pub fn lock_exists() {
    if check_lock() {
        println!("An instance of bulge is already running.");
        println!("Delete lock file? (Only do this when the other process is frozen)");
        if continue_prompt() {
            remove_lock().expect("Failed to delete lock file.");
        } else {
            std::process::exit(1);
        }
    }
}