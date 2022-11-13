use clap::Parser;
use iris_lib::{
    connect::{ConnectionError, ConnectionManager},
    types::SERVER_NAME,
};
use std::net::IpAddr;

#[derive(Parser)]
struct Arguments {
    #[clap(default_value = "127.0.0.1")]
    ip_address: IpAddr,

    #[clap(default_value = "6991")]
    port: u16,
}

fn main() {
    let arguments = Arguments::parse();
    println!(
        "Launching {} at {}:{}",
        SERVER_NAME,
        arguments.ip_address,
        arguments.port
    );

    let mut connection_manager = ConnectionManager::launch(arguments.ip_address, arguments.port);
    loop {
        // This function call will block until a new client connects!
        let (mut conn_read, mut conn_write) = connection_manager.accept_new_connection();

        println!("New connection from {}", conn_read.id());

        loop {
            println!("Waiting for message...");
            let message = match conn_read.read_message() {
                Ok(message) => message,
                Err(ConnectionError::ConnectionLost | ConnectionError::ConnectionClosed) => {
                    println!("Lost connection.");
                    break;
                }
                Err(_) => {
                    println!("Invalid message received... ignoring message.");
                    continue;
                }
            };

            println!("Received message: {message}");

            let _ = conn_write.write_message("Hello, World!\r\n");
            println!("Sent hello-world message back!");
        }
    }
}
