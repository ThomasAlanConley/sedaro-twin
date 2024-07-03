// use amiquip::{
//     ConsumerMessage,
//     Exchange,
//     Publish,
// };
use log::info;
use sedaro::cli::Cli;
use sedaro::sedaro_twin::SedaroTwin;
// use sedaro::twin::TwinMode;
// use sedaro::message_queue::{open_rabbitmq_or_hunt,declare_queue_or_exit,create_consumer_or_exit};
// use std::fs::File;
// use std::io::BufRead;
// use std::path::Path;
// use std::io;

fn main() {
    let cli = Cli::custom_parse();
    info!("Running twin in {:?} mode...", cli.twin_mode);

    // TODO
    let twin = SedaroTwin::new(cli.twin_mode);
    twin.run(&cli);

    // match cli.twin_mode {
    //     TwinMode::Analog => {
    //         let mut connection = open_rabbitmq_or_hunt(cli.url_rabbitmq.as_str());
    //         let channel = connection.open_channel(None).expect("connection");
    //         let exchange = Exchange::direct(&channel);
    //
    //         loop {
    //             if let Ok(lines) = read_lines("./sedaro-gui/app/public/data.jsonl") {
    //                 // let lines_buffer = Vec::<String>::new();
    //                 for line in lines.map_while(Result::ok) {
    //                     exchange
    //                         .publish(Publish::new(line.as_bytes(), &cli.emitter_queue))
    //                         .expect("publish");
    //                 }
    //             }
    //         }
    //     }
    //     TwinMode::Digital => {
    //         let mut connection = open_rabbitmq_or_hunt(cli.url_rabbitmq.as_str());
    //         let channel = connection.open_channel(None).expect("connection");
    //
    //         let request_queue = declare_queue_or_exit(&channel, cli.emitter_queue.to_owned());
    //         let consumer = create_consumer_or_exit(&request_queue);
    //         info!(
    //             "Waiting for messages on queue: {} ... ",
    //             cli.emitter_queue.to_owned()
    //         );
    //
    //         for message in consumer.receiver().iter() {
    //             if let ConsumerMessage::Delivery(delivery) = message {
    //                 let body = String::from_utf8_lossy(&delivery.body);
    //                 _ = consumer.ack(delivery.clone()); // ignore any errors
    //
    //                 // publish state to the GUI
    //                 info!("{}", body);
    //             }
    //         }
    //     }
    //     TwinMode::Simulant => {
    //         // simulate_data();
    //     }
    // }
}

// fn open_rabbitmq_or_hunt(url: &str) -> Connection {
//     loop {
//         match Connection::insecure_open(url) {
//             Ok(connection) => {
//                 // open_channel_or_exit(&mut connection );
//                 info!("Established RabbitMQ connection.");
//                 return connection;
//             }
//             Err(_) => {
//                 thread::sleep(time::Duration::from_secs(3));
//                 info!("Looking for RabbitMQ {}... (Hunting Wabbits)", url);
//             }
//         }
//     }
// }

// fn open_channel_or_exit(connection: &mut Connection) -> Channel {
//     match connection.open_channel(None) {
//         Ok(channel) => channel,
//         Err(_) => {
//             error!("Could not open channel in queue.");
//             std::process::exit(42);
//         }
//     }
// }

// fn declare_queue_or_exit(channel: &Channel, queue_name: String) -> Queue {
//     // Declare the queue options (note: must match DL options)
//     let options = QueueDeclareOptions {
//         durable: true,
//         exclusive: false,
//         auto_delete: false,
//         arguments: Default::default(),
//     };
//
//     // let queue = channel.queue_declare("events", QueueDeclareOptions::default())?;
//     // declare the queue that we are listening to
//     let request_queue = match channel.queue_declare(queue_name, options.clone()) {
//         Ok(rq) => rq,
//         Err(_) => {
//             error!("Could not open queue by name.");
//             std::process::exit(42);
//         }
//     };
//     request_queue
// }
//
// fn create_consumer_or_exit<'a>(request_queue: &'a Queue<'a>) -> Consumer<'a> {
//     let options = ConsumerOptions {
//         no_local: false,
//         no_ack: false,
//         exclusive: false,
//         arguments: Default::default(),
//     };
//     let consumer = match request_queue.consume(options) {
//         Ok(c) => c,
//         Err(_) => {
//             error!("Could not create consumer in queue.");
//             std::process::exit(42);
//         }
//     };
//     consumer
// }
