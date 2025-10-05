# Package structure

- package.rune (renamed .zst)
    - manifest.yaml
    - bin/
    - scripts/
    - configs/
    - other/

## elements

### manifest.yaml

Manifest file of package.

More about it you can find [there](manifest.md).

### bin/

Directory with executables. Automatically installs into standard directories.

### scripts/

Directory with scripts for package needs.

More about them you can find [there](scipts.md).

### configs/

Directory with basic config files. Automatically copies into standard directory.

### other/

Files for non-standard usage.