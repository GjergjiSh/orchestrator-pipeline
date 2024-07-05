#![allow(warnings)]

extern crate libloading;
use orchestrator_pipeline::prelude::*;
use std::{path::Path, thread, time::Duration};

fn run_publisher(context: Context) {
    let publisher = context.socket(PUB).unwrap();

    // Bind to the specified address
    publisher.bind("tcp://127.0.0.1:5555").unwrap();

    println!("Publisher started, sending messages with topic 'Motors'");

    let mut count = 0;
    loop {
        let topic = "Motors";
        let message = format!("Motor status update {}", count);

        // Send the topic first
        publisher.send(topic, SNDMORE).unwrap();
        // Then send the message
        publisher.send(&message, 0).unwrap();

        println!("Sent: {} - {}", topic, message);

        count += 1;
        thread::sleep(Duration::from_secs(1));
    }
}

fn run_subscriber(context: Context, topic: &str) {
    let subscriber = context.socket(SUB).unwrap();

    // Connect to the publisher
    subscriber.connect("tcp://127.0.0.1:5555").unwrap();

    // Subscribe to the specific topic
    subscriber.set_subscribe(topic.as_bytes()).unwrap();

    println!("Subscriber started, listening for topic: {}", topic);

    loop {
        // Receive the topic
        let topic = subscriber.recv_string(0).unwrap().unwrap();
        // Receive the message
        let message = subscriber.recv_string(0).unwrap().unwrap();

        println!("Received: {} - {}", topic, message);
    }
}

fn main() {
    let cfg_path = Path::new(r"C:\Users\gshkurti\Repos\orchestrator-pipeline\cfg\module-config.yaml");
    let connection_cfg_path = Path::new(r"C:\Users\gshkurti\Repos\orchestrator-pipeline\cfg\connection-config.yaml");

    let module_config = ModuleConfig::new(cfg_path);
    let connection_config = ConnectionConfig::new(connection_cfg_path);

    let modules = load_modules(&module_config);

    let mut orchestrator = Orchestrator::new();
    let connection = Connection::new(&connection_config);
    for module in modules {
        orchestrator.register_module(module)
    }

    // let shared_data = orchestrator.trigger_cycle();
    let context = connection.context();

    let subscriber_thread = thread::spawn({
        let context = context.clone();
        move || run_subscriber(context, "Motors")
        });

    for _ in 1..100 {
        connection.send(orchestrator.shared_data());
    }
    // connection.send(orchestrator.shared_data());
    subscriber_thread.join().unwrap();
}
