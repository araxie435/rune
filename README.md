<div align="center" >
    <img height=150 width=550 src="docs/pictures/logo.svg">
</div>
<br>

The package manager 

***Pre αlphα***

## Key features
- Installation in different scopes: user, group, global
- Clean package removal

## Installation
***In future***

## Usage
***Now most of features wont work***
|Command                                  |Description                                                 |
|-----------------------------------------|------------------------------------------------------------|
|'rune install <packages>'                |Install package(s) globally                                 |
|'rune install --group <group> <packages>'|Scope special group *(overrides global version of package)* |
|'rune install --user <user> <packages>'  |Scope special user *(overrides everything above)*           |
|'rune uninstall <packages>'              |Clear uninstall of packages                                 |
|'rune uninstall --keep-data <packages>'  |Uninstall without removing configs and data                 |
|'rune reinstall <packages>'              |Clear reinstall. Similar to uninstall and then install again|
|'rune reinstall --keep-data <packages>'  |Reinstall without removing configs and data                 |
|'rune update'                            |Update package list                                         |
|'rune upgrade [packages]'                |Upgrade certain packages or all packages                    |

## Future

Roadmap:
- [] Help command
- [] Example package
- [] Global installation (offline)
- [] Group & User installation (offline)
- [] Uninstallation
- [] Package update feature
- [] Mirrors
- [] Online installation
- [] Release 0.1.0

Plans:
- Better configuration
- More commands
- Official mirror & site
