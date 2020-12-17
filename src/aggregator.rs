use crossbeam_channel::{select, Receiver};
use death_rs::death::Life;
use log::info;
use uuid::Uuid;

#[derive(Debug)]
pub struct Aggregator {
    input: Receiver<i32>,
    id: String,
    sum: i32,
}

impl Aggregator {
    pub fn new(input: Receiver<i32>) -> Aggregator {
        Aggregator {
            input,
            id: Uuid::new_v4().to_string(),
            sum: 0,
        }
    }
}

impl Life for Aggregator {
    fn run(&mut self, done: Receiver<()>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            select! {
                recv(done) -> _ => {
                    info!("aggregator finished!");
                    return Ok(());
                }

                recv(self.input) -> incoming => {
                    let i = incoming?;
                    self.sum = self.sum + i;
                    info!("Input received: {} | Sum is now {}", i, self.sum);
                }
            }
        }
    }
}
