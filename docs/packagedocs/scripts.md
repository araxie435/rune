# Scripts

List:
- preinstall
- install
- postinstall
- preuninstall
- uninstall
- postuninstall

Install and uninstall scripts override most of standard logic.

Template:
``` bash
#!/bin/bash

# Variables
SKIP_CONFIGS= false
SCOPE= "Global"
#...#

# Parse
while [[ $# -gt 0 ]]; do
  case $1 in
    --skip-configs)
      SKIP_CONFIGS= true
      shift
      ;;
    --scope)
      SCOPE= $2
      shift
      shift
  esac
done

# Your code

if [[ SKIP_CONFIGS != true ]]; then
    ## Work with configs
fi

# Your code
```