run: build
  mpiexec -n 6 target/release/mpi_example
  

build:
  cargo build --release

