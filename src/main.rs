use mpi::traits::*;

fn main() {
    // init
    let universe = mpi::initialize().unwrap();
    // group of all processes
    let world = universe.world();
    // size of all processes
    let size = world.size();
    // process' own rank
    let rank = world.rank();

    println!("started process with rank {rank}");

    if rank != 0 {
        let msg = format!("Hello from {rank}");
        // sends message to process with rank 0
        world.process_at_rank(0).send(msg.as_bytes());
    } else {
        // receive buffer to be filled with received message
        let mut buf = [0u8; 256];

        // iterate over all other process ranks
        for src in 1..size {
            // receive from "src" into buffer "buf"
            world.process_at_rank(src).receive_into(&mut buf);
            println!(
                "received \"{}\" from process with rank {src}",
                std::str::from_utf8(&buf).unwrap()
            );
            // zero out buffer for new msg
            buf.fill(0);
        }
    }
}
