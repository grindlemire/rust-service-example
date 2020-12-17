use crossbeam_channel::tick;
use crossbeam_channel::{select, Receiver, Sender};
use death_rs::death::Life;
use log::error;
use rand::Rng;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug)]
pub struct Counter {
    output: Sender<i32>,
    id: String,
}

impl Counter {
    pub fn new(output: Sender<i32>) -> Counter {
        Counter {
            output,
            id: Uuid::new_v4().to_string(),
        }
    }
}

impl Life for Counter {
    fn run(&mut self, done: Receiver<()>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut rng = rand::thread_rng();
        let ticker = tick(Duration::from_millis(100 * rng.gen_range(1, 50)));

        loop {
            select! {
                recv(done) -> _ => {
                    return Ok(());
                }

                recv(ticker) -> _ => {
                    match self.output.send(rng.gen_range(1, 10)) {
                        Ok(())=>(),
                        Err(e)=>error!("Error sending on output channel: {}", e)
                    }
                }
            }
        }
    }
}
