//! This module defines the implementatiton of the plugin functions.
//! This module provides an example reminder plugin for demonstration 
//! of adding a plugin into the system. 

use std::time::Duration;

use crate::{
    types::{ErrorType, Nick, PrivMsg, PrivReply, Reply, Target},
    user::UserPool,
};

/// This is an example plugin
/// This plugin can be called by the following command: PRIVMSG reminder: time in second:things to remind.
/// 
/// If the time is not given or the time is invalid(for example: negative time), the function would return
/// an error saying the parameter being given is invalid. 
/// 
/// It should be noted that all the plugin functions should be defined in the type fn(&mut Userpool, String, Nick) 
/// -> Result<(), ErrorType>.
/// 
/// The user only need to add an entry in the "fn new()" in plugin_handler.rs. 
/// Then, the plugin should be able to use in the chatting system. 
pub fn reminder_handler(
    user_pool: &mut UserPool,
    msg: String,
    receiver_nick: Nick,
) -> Result<(), ErrorType> {
    let sender_nick = Nick(String::from("reminder"));

    // Check if the value being given are valid
    let (duration, msg) = match msg.split_once(":") {
        Some((d, t)) => (d.to_string(), t.to_string()),
        None => return Err(ErrorType::InvalidPluginCommand),
    };

    // Check if the duration can be parsed into u64
    let duration = match duration.parse::<u64>() {
        Ok(t) => t,
        Err(_) => return Err(ErrorType::InvalidPluginCommand),
    };

    // Generate a separate thread for timing
    let mut user_pool = user_pool.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(duration));

        let mut users = user_pool.get_write_pool();
        let receiver = users
            .iter_mut()
            .find(|x| x.get_nick() == receiver_nick)
            .unwrap();

        receiver.send(Reply::PrivMsg(PrivReply {
            message: PrivMsg {
                target: Target::User(sender_nick.clone()),
                message: msg,
            },
            sender_nick: sender_nick.clone(),
        }));
    });
    Ok(())
}
