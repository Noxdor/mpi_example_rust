If you have [just](https://github.com/casey/just) installed, just run `just` in the
command line to build and run the example.

Otherwise run

1. `cargo build --release`
2. `mpiexec -n 6 target/release/mpi_example`

If 2. should fail for you with an error message telling `there are not enough slots available`,
reduce the number for n.
