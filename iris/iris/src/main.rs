use clap::Parser;
use iris_lib::{
    connect::{ConnectionError, ConnectionManager},
    types::{SERVER_NAME, UnparsedMessage, Nick, ParsedMessage}, handler::parse_error_handler, user::User,
};
use std::net::IpAddr;
use iris_lib::handler::parsed_msg_handler;

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

    // Vec to store all of the users
    // let mut users = Vec::<User>::new();

    let mut connection_manager = ConnectionManager::launch(arguments.ip_address, arguments.port);
    loop {
        // This function call will block until a new client connects!
        let (conn_read, conn_write) = connection_manager.accept_new_connection();

        let mut user = User::new(conn_read.id(), conn_write, conn_read);
        // users.push(user);

        println!("New connection from {}", user.conn_read.id());

        loop {
            println!("Waiting for message...");
            let message = match user.conn_read.read_message() {
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

            // Deal with the incoming message
            // If the sender doesn't have a nick name, use id as nick name
            let sender_nick = match user.get_nick_name() {
                Some(nick) => nick,
                None => user.get_id(),
            };
            match ParsedMessage::try_from(UnparsedMessage{
                sender_nick: Nick(sender_nick),
                message: &message,
            }) {
                Ok(msg) => parsed_msg_handler(msg, &mut user),
                Err(err) => parse_error_handler(err, &mut user)
            };

            // If the user quits, kill the connection
            if user.is_quit() {
                break;
            }
        }
    }
}
