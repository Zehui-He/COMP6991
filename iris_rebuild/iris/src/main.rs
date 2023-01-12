use clap::Parser;
use iris_lib::{
    connect::{ConnectionError, ConnectionManager},
    handler::{error_message_handler, parsed_message_handler},
    types::{ErrorType, Nick, ParsedMessage, UnparsedMessage, SERVER_NAME, Message},
    user::{User, UserPool}, channel::ChannelPool,
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
        SERVER_NAME, arguments.ip_address, arguments.port
    );

    let mut user_pool = UserPool::new();

    // Create a channel to send send and receive message
    let (sender, receiver) = std::sync::mpsc::channel::<Result<ParsedMessage, (ErrorType, Nick)>>();

    // Create a thread for sending message
    {
        let mut user_pool = user_pool.clone();
        std::thread::spawn(move || {
            let mut channel_pool = ChannelPool::new();

            for recv in receiver {
                match recv {
                    Ok(parsed_msg) => {
                        let sender_nick = parsed_msg.sender_nick.clone();
                        match parsed_message_handler(&mut user_pool, &mut channel_pool, parsed_msg) {
                            Ok(_) => println!("Send back a message."),
                            Err(err) => {
                                error_message_handler(&mut user_pool, err, sender_nick);
                                println!("Send back an error.")
                            }
                        }
                    }
                    Err((e, sender_nick)) => error_message_handler(&mut user_pool, e, sender_nick),
                }
            }
        });
    }

    let mut connection_manager = ConnectionManager::launch(arguments.ip_address, arguments.port);

    // Client listening thread
    loop {
        // This function call will block until a new client connects!
        let (mut conn_read, conn_write) = connection_manager.accept_new_connection();

        // Create a user for the new connection
        let user = User::new(conn_read.id(), conn_write);

        // Push the user to the user pool
        user_pool.insert(user);

        println!("New connection from {}", conn_read.id());

        // Create a thread for listening client message
        {
            let user_pool = user_pool.clone();
            let sender = sender.clone();
            std::thread::spawn(move || {
                let mut user_quited = false;
                loop {
                    if user_quited {
                        break;
                    }

                    let message = match conn_read.read_message() {
                        Ok(message) => message,
                        Err(
                            ConnectionError::ConnectionLost | ConnectionError::ConnectionClosed,
                        ) => {
                            println!("Lost connection.");
                            break;
                        }
                        Err(_) => {
                            println!("Invalid message received... ignoring message.");
                            continue;
                        }
                    };

                    println!("Received message: {message}");

                    // Find the nick name of the user
                    // Use id as nickname if not specified
                    let sender_nick = user_pool.get_nick_by_id(conn_read.id());

                    // Parse the string message, panic if there is any error
                    let parsed_msg = match ParsedMessage::try_from(UnparsedMessage {
                        sender_nick: sender_nick.clone(),
                        message: &message,
                    }) {
                        Ok(parsed_msg) => parsed_msg,
                        // The message cannot be parsed
                        Err(e) => {
                            sender.send(Err((e, sender_nick))).unwrap();
                            println!("{:?}", e);
                            continue;
                        }
                    };

                    println!("{:?}", parsed_msg);
                    
                    // Break the loop if the user send a quit message
                    match parsed_msg.message {
                        Message::Quit(_) => user_quited = true,
                        _ => {}
                    }

                    // Send the parsed messgae to the sending thread
                    sender.send(Ok(parsed_msg)).unwrap();

                }
            });
        }
    }
}
