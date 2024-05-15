use std::fs;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

use isahc::http::StatusCode;
use isahc::ReadResponseExt;

use crate::util::database::fns::{get_remote_package, search_for_package};
use crate::util::database::structs::Source;
use crate::util::mirrors::load_mirrors;

pub fn search(args: Vec<String>) {

    if args.len() < 3 {
        eprintln!("Please provide a package to find. (Check bulge --help for usage)");

        std::process::exit(1);
    }

    let requested_packages: Vec<String> = args.clone().drain(2..).collect();

    println!("==> Searching...");
    for i in &requested_packages {
        let repo = search_for_package(&i);

        if repo.is_err() {
            eprintln!("ERR> {} was not found!", i)
        }
        else {
            println!("{} exists!", i)
        }
}