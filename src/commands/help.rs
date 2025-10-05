pub fn help(parameter: &str) {
    match parameter {
        "--everything" => println!("Rune usage:\n\nrune install [scope] <packages>\t\tInstall package\nrune update\t\t\t\tUpdate package list\nrune upgrade [scope] [packages]\t\tUpgrade certain packages or all packages\nrune uninstall [scope] <packages>\tUninstall package\nrune reinstall [scope] <packages>\tReinstall package\nrune tree [scope]\t\t\tShow tree of packages\nrune info [scope] <package>\t\tShow information about package\nrune help [argument]\t\t\tShow help about command\n\nArguments:\n--group <group>\t\t\t\tScope special group\n--user <user>\t\t\t\tScope special user\nhelp --everything\t\t\tShow full help"),
        "install" => println!("Install usage:\nrune install [scope] <packages>\tInstall package\nArguments:\n--user <user>\tInstall into user scope\n--group <group>\tInstall into group scope"),
        "update" => println!("Update usage:\nrune update\tUpdate package list"),
        "upgrade" => println!("Upgrade usage:\nrune upgrade [scope] [packages]\tUpgrade certain packages or all packages\nArguments:\n--user <user>\tUpgrade in user scope\n--group <group>\tUpgrade in group scope"),
        "uninstall" => println!("Uninstall usage:\nrune uninstall [scope] <packages>\tUninstall package\nArguments:\n--user <user>\tUninstall from user scope\n--group <group>\tUninstall from group scope"),
        "reinstall" => println!("Reinstall usage:\nrune reinstall [scope] <packages>\tReinstall package\nArguments:\n--user <user>\tReinstall in user scope\n--group <group>\tReinstall in group scope"),
        "tree" => println!("Tree usage:\nrune tree [scope]\tShow tree of packages\nArguments:\n--user <user>\tShow tree of user packages\n--group <group>\tShow tree of group packages"),
        "info" => println!("Info usage:\nrune info [scope] <package>\tShow information about package\nArguments:\n--user <user>\tShow information of user package\n--group <group>\tShow information of group package"),
        "help" => println!("Help usage:\nrune help [argument]\tShow help about command\nArguments:\n--everything\tShow full help"),
        _ => help("--everything")
    }
}