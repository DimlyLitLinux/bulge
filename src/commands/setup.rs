use std::fs;
use std::io::Write;

use crate::commands::setup_files::static_files::{default_config, default_mirrorlist};
use crate::commands::sync::sync;
use crate::util::database::fns::init_database;
use crate::util::lock::{create_lock, lock_exists, remove_lock};
use crate::util::macros::get_root;

pub fn init() {
    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /var/lock/bulge.lock already exist?)");

    println!("Welcome to bulge!");
    println!("We'll be creating the necessary folders on root.");
    println!();

    println!("Creating {}/etc/bulge", get_root());
    fs::create_dir_all(get_root() + "/etc/bulge").expect("Failed to create /etc/bulge");

    println!("Creating {}/etc/bulge/databases", get_root());
    fs::create_dir_all(get_root() + "/etc/bulge/databases").expect("Failed to create /etc/bulge/databases");

    println!("Creating {}/etc/bulge/databases/cache", get_root());
    fs::create_dir_all(get_root() +"/etc/bulge/databases/cache").expect("Failed to create /etc/bulge/databases/cache");

    println!();
    println!("We'll now create some default files.");
    println!();

    println!("Creating default configuration file.");
    fs::File::create(get_root() + "/etc/bulge/config.json")
        .expect("Failed to create /etc/bulge/config.json")
        .write_all(default_config().as_ref())
        .expect("Failed to insert default configuration.");

    println!("Creating default mirror list for yiffOS.");
    fs::File::create(get_root() + "/etc/bulge/mirrors")
        .expect("Failed to create /etc/bulge/mirrors")
        .write_all(default_mirrorlist().as_ref())
        .expect("Failed to insert default mirror list.");

    println!("Creating default databases.");
    init_database();

    println!();
    println!("The databases will now be synced with the mirrors.");
    println!();
    remove_lock().expect("Failed to remove lock?"); // Work around for lock issues
    sync();

    println!();
    println!("Setup complete!");
}