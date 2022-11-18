use std::{sync::{Arc, Mutex}, collections::HashMap};

use crate::{types::{ParsedMessage, ErrorType, NickMsg, UserMsg, Nick, Reply, WelcomeReply, QuitMsg, QuitReply, PrivMsg, Target, PrivReply, Message, JoinMsg, PartMsg, PartReply, JoinReply, Channel}, user::User};

/// Process the parsed message
pub fn parsed_msg_handler(parsed_msg: ParsedMessage, users: &mut Arc<Mutex<Vec<User>>>, channels: &mut Arc<Mutex<HashMap<String, Vec<String>>>>, id: String) -> Result<(), ErrorType> {
    match parsed_msg.message {
        Message::Nick(nickmsg) => Ok({
           nick_msg_handler(nickmsg, users, id);
        }),
        Message::User(usermsg) => Ok({
            user_msg_handler(usermsg, users, id);
        }),
        Message::PrivMsg(privmsg) => {
            priv_msg_handler(privmsg, users, channels, id)
        },
        Message::Ping(message) => {
            ping_msg_handler(message, users, id)
        },
        Message::Join(joinmsg) => {
            join_msg_handler(joinmsg, users, channels, id)
        },
        Message::Part(partmsg) => {
            part_msg_handler(partmsg, users, channels, id)
        },
        Message::Quit(quitmsg) => Ok({
            quit_msg_handler(quitmsg, users, channels, id);
        }),
    }
}

/// Function that used to give back a reply to the user.
fn reply_handler(reply: Reply, user: &mut User) {
    let message = &format!("{}", reply);
    let _ = user.get_conn_write().write_message(message);
}

/// Function that used to give back am error to the user.
pub fn parse_error_handler(err: ErrorType, users: &mut Arc<Mutex<Vec<User>>>, id: String) {
    // Find the receiving user
    let mut users = users.as_ref().lock().unwrap();
    let user = users.iter_mut().find(|usr| usr.get_id() == id).unwrap();

    let message = &format!("{}\r\n", err);
    let _ = user.get_conn_write().write_message(message);
}

// Nick name may collide
fn nick_msg_handler(nickmsg: NickMsg, users: &mut Arc<Mutex<Vec<User>>>, id: String) {
    let mut users = users.as_ref().lock().unwrap();
    let user = users.iter_mut().find(|usr| usr.get_id() == id).unwrap();
    // The user is not allowed to change the nick name
    if !user.nick_name_is_none() {
        return;
    }
    user.set_nick_name(nickmsg.nick.0);
    let message = &format!("You set your nick name as: {}.\r\n", Nick(user.get_nick_name().unwrap()));
    let _ = user.get_conn_write().write_message(&message);
}

fn user_msg_handler(usermsg: UserMsg, users: &mut Arc<Mutex<Vec<User>>>, id: String) {
    let mut users = users.as_ref().lock().unwrap();
    let user = users.iter_mut().find(|usr| usr.get_id() == id).unwrap();
    // Users are not allowed to change real name
    if !user.real_name_is_none() {
        return;
    }
    user.set_real_name(usermsg.real_name);

    // If the nick name and real name of the user are set, they are registered on server
    if !user.nick_name_is_none() && !user.real_name_is_none() {
        user.set_registered();
    }
    
    let reply = Reply::Welcome(
        WelcomeReply {
            target_nick: Nick(user.get_nick_name().unwrap()),
            message: user.get_real_name().unwrap(),
        }
    );
    reply_handler(reply, user);
}

fn priv_msg_handler(privmsg: PrivMsg, users: &mut Arc<Mutex<Vec<User>>>, channels: &mut Arc<Mutex<HashMap<String, Vec<String>>>>, sender_id: String) -> Result<(), ErrorType> {
    let mut users = users.as_ref().lock().unwrap();
    let sender = users.iter_mut().find(|usr| usr.get_id() == sender_id).unwrap();
    
    // The sender cannot ping if they are not registered, throw an error 
    if !sender.is_registered() {
        return Err(ErrorType::NoOrigin);
    }
    let sender_nick = Nick(sender.get_nick_name().unwrap());

    match privmsg.target {
        Target::Channel(Channel(ref receiving_channel_name)) => {
            // Throw error if the user is not in channel
            if !sender.get_channels().contains(&receiving_channel_name) {
                return Err(ErrorType::NoSuchChannel);
            }

            // Send the message to everyone in channel
            let channels = channels.as_ref().lock().unwrap();
            let receiving_channel = channels.get(receiving_channel_name);
            match receiving_channel {
                Some(id_in_channel) => {
                    for id in id_in_channel {
                        if *id == sender_id {
                            continue;
                        }
                        let reply = Reply::PrivMsg(PrivReply {
                            message: privmsg.clone(),
                            sender_nick: sender_nick.clone()
                        });
                        let receiver = users.iter_mut().find(|usr| usr.get_id() == *id).unwrap();
                        reply_handler(reply, receiver);
                    }
                    Ok(())
                },
                None => return Err(ErrorType::NoSuchChannel),
            }
        },
        Target::User(Nick(ref receiver_nick)) => {
            // Find receiver by Nick
            // Throw error if receiver doesn't exist
            let receiver = users.iter_mut().find(|usr| usr.get_nick_name() == Some(receiver_nick.clone()));
            match receiver {
                Some(receiver) => {
                    let reply = Reply::PrivMsg(
                        PrivReply {
                            message: privmsg,
                            sender_nick,
                        }
                    );
                    reply_handler(reply, receiver);
                    Ok(())
                },
                None => return Err(ErrorType::NoSuchNick),
            }
        },
    }
}

fn ping_msg_handler(msg: String, users: &mut Arc<Mutex<Vec<User>>>, id: String) -> Result<(), ErrorType> {
    let mut users = users.as_ref().lock().unwrap();
    let user = users.iter_mut().find(|usr| usr.get_id() == id).unwrap();
    // The user cannot ping if they are not registered, throw an error 
    if !user.is_registered() {
        return Err(ErrorType::NoOrigin);
    }

    let reply = Reply::Pong(
        msg
    );
    reply_handler(reply, user);
    Ok(())
}


fn quit_msg_handler(quit_msg: QuitMsg, users: &mut Arc<Mutex<Vec<User>>>, channels: &mut Arc<Mutex<HashMap<String, Vec<String>>>>, sender_id: String) {
    let mut users = users.as_ref().lock().unwrap();
    let sender = users.iter_mut().find(|usr| usr.get_id() == sender_id).unwrap();

    let reply = Reply::Quit(
        QuitReply {
            message: quit_msg,
            sender_nick: Nick(sender.get_nick_name().unwrap()),
        }
    );

    // Remove the sender from all channels and send messages to users in the channel
    let sender_joined_channels = sender.get_channels().clone();
    let mut channels = channels.as_ref().lock().unwrap();
    for channel in sender_joined_channels {
        // Remove the sender from the channel list
        channels.entry(channel.to_string()).and_modify(|x| x.retain(|usr|  usr != &sender_id));
        
        // Send message to all user in all channels
        let id_in_channels = channels.get(&channel).unwrap();
        for id in id_in_channels {
            let receiver = users.iter_mut().find(|usr| usr.get_id() == *id).unwrap();
            reply_handler(reply.clone(), receiver);
        }
    }

    // Remove the user from user list
    users.retain(|usr|  usr.get_id() != sender_id);
}

fn join_msg_handler(join_msg: JoinMsg, users: &mut Arc<Mutex<Vec<User>>>, channels: &mut Arc<Mutex<HashMap<String, Vec<String>>>>, id: String) -> Result<(), ErrorType> {
    let sender_id = id;
    let mut users = users.as_ref().lock().unwrap();
    let sender = users.iter_mut().find(|usr| usr.get_id() == sender_id).unwrap();
    // The user cannot join if they are not registered, throw an error 
    if !sender.is_registered() {
        return Err(ErrorType::NoOrigin);
    }

    // Do nothing if the user is already in the channel
    if sender.get_channels().contains(&join_msg.channel.0) {
        return Ok(());
    }

    // If the channel doesn't exist, create one
    let mut channels = channels.as_ref().lock().unwrap();
    channels.entry(join_msg.channel.0.clone()).and_modify(|channel| channel.push(sender_id.clone())).or_insert(vec![sender_id.clone()]);

    // Save the channel for user
    sender.get_channels().push(join_msg.channel.0.clone());

    let sender_nick = Nick(sender.get_nick_name().unwrap());

    // Send message to every other user in the channel
    let id_in_channel = channels.get(&join_msg.channel.0).unwrap();
    for id in id_in_channel {
        if id == &sender_id {
            continue;
        }
        let reply = Reply::Join(JoinReply {
            message: join_msg.clone(),
            sender_nick: sender_nick.clone()
        });
        let receiver = users.iter_mut().find(|usr| usr.get_id() == *id).unwrap();
        reply_handler(reply, receiver);
    }
    Ok(())
}

fn part_msg_handler(part_msg: PartMsg, users: &mut Arc<Mutex<Vec<User>>>, channels: &mut Arc<Mutex<HashMap<String, Vec<String>>>>, id: String) -> Result<(), ErrorType> {
    let mut users = users.as_ref().lock().unwrap();
    let sender = users.iter_mut().find(|usr| usr.get_id() == id).unwrap();
    // The sender cannot join if they are not registered, throw an error 
    if !sender.is_registered() {
        return Err(ErrorType::NoOrigin);
    }

    // If the sender doesn't joined the channel, throw error
    if !sender.get_channels().contains(&part_msg.channel.0) {
        return Err(ErrorType::NoSuchChannel);
    }

    // Remove joined channel for sender
    sender.get_channels().retain(|chanl| chanl != &part_msg.channel.0);

    // Remove sender from channel
    let mut channels = channels.as_ref().lock().unwrap();
    channels.entry(part_msg.channel.0.clone()).and_modify(|x| x.retain(|usr|  usr != &id));

    let sender_nick = Nick(sender.get_nick_name().unwrap());

    // Send message to every user in the channel
    let id_in_channel = channels.get(&part_msg.channel.0).unwrap();
    for id in id_in_channel {
        let reply = Reply::Part(PartReply {
            message: part_msg.clone(),
            sender_nick: sender_nick.clone()
        });
        let receiver = users.iter_mut().find(|usr| usr.get_id() == *id).unwrap();
        reply_handler(reply, receiver);
    }
    Ok(())
}