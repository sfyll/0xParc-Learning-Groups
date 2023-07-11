use clap::Parser;
use halo2_scaffold::scaffold::cmd::Cli;
use halo2_scaffold::scaffold::run;

mod fibonacci;
use fibonacci::fibonacci::compute_fibonacci;

fn main() {
    env_logger::init();

    let args = Cli::parse();

    // run different zk commands based on the command line arguments
    run(compute_fibonacci, args);
}
