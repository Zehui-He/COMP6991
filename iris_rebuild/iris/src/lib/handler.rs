use crate::{
    channel::ChannelPool,
    types::{ErrorType, Message, Nick, PrivMsg, PrivReply, Reply, Target, UserMsg},
    types::{NickMsg, ParsedMessage, WelcomeReply, JoinMsg, PartMsg, Channel, JoinReply, QuitMsg, PartReply, QuitReply},
    user::UserPool,
};

pub fn error_message_handler(user_pool: &mut UserPool, err: ErrorType, sender_nick: Nick) {
    // Find the user by ID
    let mut users = user_pool.get_write_pool();
    let sender = users
        .iter_mut()
        .find(|user| user.get_nick() == sender_nick)
        .unwrap();

    sender.send_error(err);
}

pub fn parsed_message_handler(
    user_pool: &mut UserPool,
    channel_pool: &mut ChannelPool,
    parsed_msg: ParsedMessage,
) -> Result<(), ErrorType> {
    match parsed_msg.message {
        Message::Nick(nick_msg) => nick_msg_handler(user_pool, nick_msg, parsed_msg.sender_nick),
        Message::User(user_msg) => user_msg_handler(user_pool, user_msg, parsed_msg.sender_nick),
        Message::PrivMsg(priv_msg) => {
            priv_msg_handler(user_pool, channel_pool, priv_msg, parsed_msg.sender_nick)
        }
        Message::Ping(ping_msg) => ping_msg_handler(user_pool, ping_msg, parsed_msg.sender_nick),
        Message::Join(join_msg) => join_msg_handler(user_pool, channel_pool, join_msg, parsed_msg.sender_nick),
        Message::Part(part_msg) => part_msg_handler(user_pool, channel_pool, part_msg, parsed_msg.sender_nick),
        Message::Quit(quit_msg) => quit_msg_handler(user_pool, channel_pool, quit_msg, parsed_msg.sender_nick),
    }
}

fn nick_msg_handler(
    user_pool: &mut UserPool,
    nick_msg: NickMsg,
    sender_nick: Nick,
) -> Result<(), ErrorType> {
    let nick_str = nick_msg.nick.0;

    // If the nick name is already in used
    if user_pool.nick_exist(nick_str.clone()) {
        return Err(ErrorType::NickCollision);
    }

    // Find the user by NICK(ID at initial state)
    // Note: the sender nick is the user id if the nick name is not set
    let mut users = user_pool.get_write_pool();
    let user = users
        .iter_mut()
        .find(|user| user.get_id() == sender_nick.0)
        .unwrap();

    // If the nick name of the user is not set, set the nick name
    if !user.nick_is_set() {
        user.set_nick(nick_str);

        if user.is_registered() {
            user.send(Reply::Welcome(WelcomeReply {
                target_nick: user.get_real(),
                message: String::from("Welcome to the server!"),
            }))
        }
    }

    Ok(())
}

fn ping_msg_handler(
    user_pool: &mut UserPool,
    ping_msg: String,
    sender_nick: Nick,
) -> Result<(), ErrorType> {
    // Find the user by NICK
    let mut users = user_pool.get_write_pool();
    let sender = users
        .iter_mut()
        .find(|user| user.get_nick() == sender_nick)
        .unwrap();

    println!("{:?}", sender);

    // If the user is not registered
    if !sender.is_registered() {
        return Err(ErrorType::UserNotRegistered);
    }

    // Construct the message
    let reply = Reply::Pong(ping_msg);

    // Send back the message
    sender.send(reply);

    Ok(())
}

fn user_msg_handler(
    user_pool: &mut UserPool,
    user_msg: UserMsg,
    sender_nick: Nick,
) -> Result<(), ErrorType> {
    let real_str = user_msg.real_name;

    // Find the user by NICK(ID)
    let mut users = user_pool.get_write_pool();
    let user = users
        .iter_mut()
        .find(|user| user.get_nick() == sender_nick)
        .unwrap();

    // If the real name of the user is not set, set the real name
    if !user.real_is_set() {
        user.set_real(real_str);

        if user.is_registered() {
            user.send(Reply::Welcome(WelcomeReply {
                target_nick: user.get_real(),
                message: String::from("Welcome to the server!"),
            }))
        }
    }

    Ok(())
}

fn priv_msg_handler(
    user_pool: &mut UserPool,
    channel_pool: &mut ChannelPool,
    priv_msg: PrivMsg,
    sender_nick: Nick,
) -> Result<(), ErrorType> {
    let mut users = user_pool.get_write_pool();

    // The sender cannot send messgae if not registered
    let sender = users
        .iter_mut()
        .find(|user| user.get_nick() == sender_nick)
        .unwrap();

    if !sender.is_registered() {
        return Err(ErrorType::UserNotRegistered);
    }

    let target = priv_msg.target;
    let message = priv_msg.message;

    match target {
        Target::Channel(channel_obj) => {
            let channel_name = channel_obj.0;

            // If the channel doesn't exist, throw error
            if !channel_pool.channel_exist(&channel_name) {
                return Err(ErrorType::NoSuchChannel);
            }

            // If the user is not in the channel, throw error
            if !channel_pool.user_in_channel(&channel_name, sender_nick.clone()) {
                return  Err(ErrorType::UserNotJoinChannel);
            }

            // Send the message to every user in the channel
            let channel = channel_pool.get_channel(&channel_name).unwrap();

            for user_name in channel.iter() {
                let receiver_nick = user_name.clone();

                let receiver = users
                    .iter_mut()
                    .find(|user| user.get_nick() == Nick(receiver_nick.clone()))
                    .unwrap();

                receiver.send(Reply::PrivMsg(PrivReply {
                    message: PrivMsg {
                        target: Target::Channel(Channel(channel_name.clone())),
                        message: message.clone(),
                    },
                    sender_nick: sender_nick.clone(),
                }));
            }

            Ok(())
        }
        Target::User(receiver_nick) => {
            let receiver = users
                .iter_mut()
                .find(|user| user.get_nick() == receiver_nick);

            // Throw error if the recepient doesn't exist
            if receiver.is_none() {
                return Err(ErrorType::NoSuchNick);
            }

            let receiver = receiver.unwrap();
            receiver.send(Reply::PrivMsg(PrivReply {
                message: PrivMsg {
                    target: Target::User(receiver.get_nick()),
                    message,
                },
                sender_nick,
            }));

            Ok(())
        }
    }
}

fn join_msg_handler(user_pool: &mut UserPool,
    channel_pool: &mut ChannelPool,
    join_msg: JoinMsg,
    sender_nick: Nick,
) -> Result<(), ErrorType> {
    if !user_pool.user_is_registered(sender_nick.clone()) {
        return Err(ErrorType::UserNotRegistered);
    }

    let channel_name = join_msg.channel.0;

    // Throw error if the user is already in channel
    if channel_pool.user_in_channel(&channel_name, sender_nick.clone()) {
        return Err(ErrorType::UserAlreadyInChannel);
    }

    // User join channel
    user_pool.user_join_channel(sender_nick.clone(), channel_name.clone());

    // Channel recored user
    channel_pool.join_channel(&channel_name, sender_nick.clone());

    // Send the message to every user in the channel
    let channel = channel_pool.get_channel(&channel_name).unwrap();
    let mut users = user_pool.get_write_pool();

    for user_name in channel.iter() {
        let receiver_nick = user_name.clone();

        let receiver = users
            .iter_mut()
            .find(|user| user.get_nick() == Nick(receiver_nick.clone()))
            .unwrap();

        receiver.send(Reply::Join(JoinReply{
            message: JoinMsg { channel: Channel(channel_name.clone()) },
            sender_nick: sender_nick.clone(),
        }))
    }

    Ok(())
}

fn part_msg_handler(user_pool: &mut UserPool,
    channel_pool: &mut ChannelPool,
    part_msg: PartMsg,
    sender_nick: Nick,
) -> Result<(), ErrorType> {
    // Ignore message if not registered
    if !user_pool.user_is_registered(sender_nick.clone()) {
        return Err(ErrorType::UserNotRegistered);
    }

    let channel_name = part_msg.channel.0;

    // Throw error if the channel not exist
    if !channel_pool.channel_exist(&channel_name) {
        return Err(ErrorType::NoSuchChannel);
    }
    
    // Throw error if the user not in the channel
    if !channel_pool.user_in_channel(&channel_name, sender_nick.clone()) {
        return Err(ErrorType::UserNotJoinChannel);
    }

    // Send the message to every user in the channel
    let channel = channel_pool.get_channel(&channel_name).unwrap();
    let mut users = user_pool.get_write_pool();

    for user_name in channel.iter() {
        let receiver_nick = user_name.clone();

        let receiver = users
            .iter_mut()
            .find(|user| user.get_nick() == Nick(receiver_nick.clone()))
            .unwrap();

        receiver.send(Reply::Part(PartReply{
            message: PartMsg { channel: Channel(channel_name.clone()) },
            sender_nick: sender_nick.clone(),
        }))
    }

    drop(users);

    // Remove the user from the channel
    channel_pool.quit_channel(&channel_name, sender_nick.clone());

    // User remove the channel
    user_pool.user_quit_channel(sender_nick, channel_name);

    Ok(())
}

fn quit_msg_handler(user_pool: &mut UserPool,
    channel_pool: &mut ChannelPool,
    quit_msg: QuitMsg,
    sender_nick: Nick,
) -> Result<(), ErrorType> {
    // Send message to all the channel that the user joined
    let users = user_pool.get_write_pool();
    let user = users.iter().find(|x| x.get_nick() == sender_nick).unwrap();
    
    // Send quit message to all channel that the user joined
    let channels = user.get_joined_channels().clone();
    drop(users);

    for channel_name in channels.iter() {
        let channel = channel_pool.get_channel(&channel_name).unwrap();
        let mut users = user_pool.get_write_pool();

        for user_name in channel.iter() {
            let receiver_nick = user_name.clone();

            let receiver = users
                .iter_mut()
                .find(|user| user.get_nick() == Nick(receiver_nick.clone()))
                .unwrap();

            receiver.send(Reply::Quit(QuitReply{
                message: quit_msg.clone(),
                sender_nick: sender_nick.clone(),
            }))
        }
    }
    
    // Remove user from all channels
    channel_pool.remove_user(sender_nick.clone());

    // Remove user from the user pool
    user_pool.remove_user(sender_nick.clone());

    Ok(())
}