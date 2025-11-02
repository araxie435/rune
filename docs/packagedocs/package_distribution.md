# Package distribution

Now there is no scripts to check package, so be careful

## Build steps:

1. Create a folder with package name and version

   ```sh
   mkdir my_package-0.1.0
   cd my_package-0.1.0
   ```

2. Place all files according to the structure defined in the [structure](structure.md).

3. Create a manifest file as described in the [manifest](manifest.md) documentation.

4. Compress folder into .rune file

   ```sh
   tar --zstd -cf my_package-0.1.0.rune -C my_package-0.1.0 .
   ```

## Deploy package to official mirror

***Later***