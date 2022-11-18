use clap::Parser;
use iris_lib::{
    connect::{ConnectionError, ConnectionManager},
    types::{SERVER_NAME, UnparsedMessage, Nick, ParsedMessage, Message}, user::User, handler::parse_error_handler,
};
use std::{net::IpAddr, sync::{self, Arc, Mutex}, collections::HashMap};
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
    let users = Arc::new(Mutex::new(Vec::<User>::new()));

    // Hashmap to store all channels
    let channels = Arc::new(Mutex::new(HashMap::<String, Vec<String>>::new()));

    // Channel to pass reply or error between threads
    let (sender, receiver) = sync::mpsc::channel::<(ParsedMessage, String)>();

    // Reply thread
    {
        let mut users = users.clone();
        let mut channels = channels.clone();
        std::thread::spawn(move || {
            loop {
                match receiver.recv() {
                    Ok((parsed_msg, id)) => {
                        // Error may occur during replying
                        match parsed_msg_handler(parsed_msg, &mut users, &mut channels, id.clone()) {
                            Ok(_) => {
                                println!("I received a message and send a reply!");
                            },
                            Err(err) => {
                                parse_error_handler(err, &mut users, id);
                            },
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        });
    }

    let mut connection_manager = ConnectionManager::launch(arguments.ip_address, arguments.port);
    
    // Listening thread
    loop {
        // This function call will block until a new client connects!
        let (mut conn_read, conn_write) = connection_manager.accept_new_connection();

        // Record the user in the user list
        let user = User::new(conn_read.id(), conn_write);
        users.as_ref().lock().unwrap().push(user);

        println!("New connection from {}", conn_read.id());

        // Generate a thread for the client with JUST THE READER
        {
            let mut users = users.clone();
            let sender = sender.clone();

            // Flag of thread continuation
            let mut user_quited = false;

            std::thread::spawn(move || {
                loop {
                    // Kill the thread if the user quited
                    if user_quited {
                        break;
                    }

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

                    // If the sender doesn't have a nick name, use id as nick name
                    let mut sender_nick = conn_read.id();
                    for user in users.as_ref().lock().unwrap().iter() {
                        if user.get_id() == conn_read.id() {
                            sender_nick = match user.get_nick_name() {
                                Some(nick) => nick,
                                None => conn_read.id(),
                            };
                        }
                    }

                    // Process the incoming message with parsing
                    let processed_msg = ParsedMessage::try_from(UnparsedMessage{
                        sender_nick: Nick(sender_nick),
                        message: &message,
                    });
                    // Send back an error message if there is a parsing error, otherwise send to the reply thread for further process
                    match processed_msg {
                        Ok(processed_msg) => {
                            // Set quit signal for thread
                            match processed_msg.message {
                                Message::Quit(_) => {
                                    user_quited = true;
                                },
                                _ => {}
                            }
                            sender.send((processed_msg, conn_read.id())).unwrap();
                        },
                        Err(err) => parse_error_handler(err, &mut users, conn_read.id()),
                    }
                }
            });
        }
    }
}
