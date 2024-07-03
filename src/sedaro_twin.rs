use crate::cli::Cli;
use crate::message_queue::{create_consumer_or_exit, declare_queue_or_exit, open_rabbitmq_or_hunt};
use amiquip::{ConsumerMessage, Exchange, Publish};
use log::info;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead};
use std::path::Path;
use std::{fs, io, time};
use clap::ValueEnum;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct SedaroTwin {
    mode: TwinMode,
    config: SedaroConfig,
    state: SedaroState,
}
impl SedaroTwin {
    pub fn new(mode: TwinMode) -> Self {
        SedaroTwin {
            mode,
            config: Default::default(),
            state: Default::default(),
        }
    }
    pub fn run(&self, cli: &Cli) {
        match cli.twin_mode {
            TwinMode::Analog => {
                let mut connection = open_rabbitmq_or_hunt(cli.url_rabbitmq.as_str());
                let channel = connection.open_channel(None).expect("connection");
                let exchange = Exchange::direct(&channel);

                loop {
                    if let Ok(lines) = read_lines("./sedaro-gui/app/public/data.jsonl") {
                        for line in lines.map_while(Result::ok) {
                            exchange
                                .publish(Publish::new(line.as_bytes(), &cli.emitter_queue))
                                .expect("publish");
                            std::thread::sleep(time::Duration::from_millis(50));
                        }
                    }
                }
            }

            TwinMode::Digital => {
                let mut connection = open_rabbitmq_or_hunt(cli.url_rabbitmq.as_str());
                let channel = connection.open_channel(None).expect("connection");

                let request_queue = declare_queue_or_exit(&channel, cli.emitter_queue.to_owned());
                let consumer = create_consumer_or_exit(&request_queue);
                info!(
                    "Waiting for messages on queue: {} ... ",
                    cli.emitter_queue.to_owned()
                );

                let mut lines_buffer = Vec::<String>::new();
                let mut counter = 0;

                for message in consumer.receiver().iter() {
                    if let ConsumerMessage::Delivery(delivery) = message {
                        counter += 1;
                        let body = String::from_utf8_lossy(&delivery.body);
                        if body.starts_with('[') {
                            lines_buffer.push(body.to_string());
                        }

                        // update the GUI via file... periodically
                        if counter % 50 == 0 {
                            let msg = format!("[{}]", lines_buffer.join(","));
                            fs::write("/public/data.json", msg).expect("fs::write");
                            std::thread::sleep(time::Duration::from_millis(100));
                        }
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
struct SedaroConfig {}
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
struct SedaroState {}

#[derive(Serialize, Deserialize, Copy, Clone, ValueEnum, Debug)]
pub enum TwinMode {
    Analog,
    Digital,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub time: f64,
    pub time_step: f64,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
}
impl State {
    pub fn propagate(&mut self) -> &mut Self {
        self.x += self.vx * self.time_step;
        self.y += self.vy * self.time_step;
        self
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
