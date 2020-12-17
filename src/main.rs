mod aggregator;
mod counter;
use clap::Clap;
use death_rs::{death::Death, error::Error};
use log::{error, info};
use signal_hook::{SIGINT, SIGTERM};
use simple_logger::SimpleLogger;
use std::time::Duration;

#[derive(Clap, Debug)]
#[clap(
    name = "rust-service-example",
    about = "An example rust service for running multiple threads."
)]
struct Opts {
    #[clap(
        short,
        long,
        default_value = "5",
        about = "Number of counterss that will be counting"
    )]
    num_workers: i32,
}

fn main() -> Result<(), Error> {
    SimpleLogger::new().init().unwrap();
    let opts = Opts::parse();

    info!("{:#?}", opts);

    let mut d = Death::new(&[SIGINT, SIGTERM], Duration::from_millis(800))?;

    let (counter_output, aggregator_input) = crossbeam_channel::bounded(100);

    d.give_life(aggregator::Aggregator::new(aggregator_input));

    for _i in 0..100 {
        d.give_life(counter::Counter::new(counter_output.clone()));
    }

    let errors = d.wait_for_death();
    errors.iter().for_each(|e| error!("{}", e));
    info!("shut down successfully");

    Ok(())
}
