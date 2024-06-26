use crate::util::database::fns::{get_remote_package, search_for_package};

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
	let repo_unwrap = repo.unwrap();
	let remote_package = get_remote_package(&i, &repo_unwrap);
	if remote_package.is_err() {
            eprintln!("ERR> {} was not found!", &i);
	    continue;
	}
        else {
            println!("{} exists!", i)
        }
    }
}
