# Example of using shared library

1. Build `blockmesh-cli`:

   `cargo b -p blockmesh-cli`.
2. Compile the example:

   `gcc main.c -o test-lib`
3. Use the full path to the compiled shared library:

   `./test-lib "....../block-mesh-monorepo/target/debug/libblockmesh_cli.dylib" "url" "email" "password"`
