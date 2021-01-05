extern crate getopts;

fn main() {
    let cmd_flags: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "show this information");
    opts.optflag("i", "install", "install software");
    opts.optflag("r", "remote", "install software from network");
    opts.optflag("S", "sync", "syncs all of the repository databases");
    opts.optflag("u", "update", "updates the current system software");
    opts.optflag("c", "clean", "cleans the local temp package directories");
    opts.optflag("R", "remove", "completely removes a package");
    let matches = opts.parse(&cmd_flags[1..]).unwrap();
    
    if matches.opt_present("h") {
        print_help();
        return;
    }
    if matches.opt_present("i") {
        install();
        return;
    }
    if matches.opt_present("r") {
        remote();
        return;
    }
    if matches.opt_present("S") {
        sync();
        return;
    }
    if matches.opt_present("u") {
        update();
        return;
    }
    if matches.opt_present("c") {
        clean();
        return;
    }
    if matches.opt_present("R") {
        purge();
        return;
    }
}

fn print_help() {
    println!("Thanks for choosing the Alfheim Universal Installer!");
    println!("It is designed to run on any Linux distribution to install
    software from multiple distribution repositories and even network addresses");
    println!("");
    println!("As requested here is the help for using this installer::");
    println!("  -h, --help    : shows this information");
    println!("  -i, --install : installs a package, altenatively you can run
             'aui <packagename>' to get a list of packages to install
             and then enter the number or range of numbers to install");
    println!("  -r, --remote  : install software from the network");
    println!("  -S, --sync    : syncs all of the repository databases");
    println!("  -u, --update  : updates the current system software");
    println!("  -c, --clean   : cleans the local temp package directories");
    println!("  -R, --remove  : completely removes a package");
    println!("");
    println!("By issuing aui without any options the program will automatically
              sync all of the repository databases and update the current
              software on the system :: it is the same as issuing an
              'aui -Su' command on the system");
}

fn install() {
    // TODO //
    // This function will install software //
    // it should query all available repositories and install //
    // the requested package(s) //
    println!("The install function has not yet been implemented");
}

fn remote() {
    // TODO //
    // This function will install software from a network address //
    println!("The remote function has not yet been implemented");
}

fn sync() {
    // TODO //
    // This function will sync the repository databases //
    // with the local database //
    println!("The sync function has not yet been implemented");
}

fn update() {
    // TODO //
    // This function will update all of the installed software //
    // on the current system //
    println!("The update function has not yet been implemented");
}

fn clean() {
    // TODO //
    // This function will clean the temp working directory for the //
    // aui tool, and all package temp directories //
    println!("The clean function has not yet been implemented");
}

fn purge() {
    // TODO //
    // This function will remove the requested package and all //
    // dependencies that are not required by other installed packages //
    println!("The remove function has not yet been implemented");
}
