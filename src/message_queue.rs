use amiquip::{Channel, Connection, Consumer, ConsumerOptions, Queue, QueueDeclareOptions};
use log::{error, info};
use std::{thread, time};

pub fn open_rabbitmq_or_hunt(url: &str) -> Connection {
    loop {
        match Connection::insecure_open(url) {
            Ok(connection) => {
                // open_channel_or_exit(&mut connection );
                info!("Established RabbitMQ connection.");
                return connection;
            }
            Err(_) => {
                thread::sleep(time::Duration::from_secs(3));
                info!("Looking for RabbitMQ {}... (Hunting Wabbits)", url);
            }
        }
    }
}

pub fn declare_queue_or_exit(channel: &Channel, queue_name: String) -> Queue {
    let options = QueueDeclareOptions {
        durable: true,
        exclusive: false,
        auto_delete: false,
        arguments: Default::default(),
    };

    // let queue = channel.queue_declare("events", QueueDeclareOptions::default())?;
    // declare the queue that we are listening to
    let request_queue = match channel.queue_declare(queue_name, options.clone()) {
        Ok(rq) => rq,
        Err(_) => {
            error!("Could not open queue by name.");
            std::process::exit(42);
        }
    };
    request_queue
}

pub fn create_consumer_or_exit<'a>(request_queue: &'a Queue<'a>) -> Consumer<'a> {
    let options = ConsumerOptions {
        no_local: false,
        no_ack: false,
        exclusive: false,
        arguments: Default::default(),
    };
    let consumer = match request_queue.consume(options) {
        Ok(c) => c,
        Err(_) => {
            error!("Could not create consumer in queue.");
            std::process::exit(42);
        }
    };
    consumer
}
