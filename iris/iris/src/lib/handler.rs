use crate::{types::{ParsedMessage, ErrorType, NickMsg, UserMsg, Nick, Reply, WelcomeReply, QuitMsg, QuitReply}, user::User};

pub fn parsed_msg_handler(parsed_msg: ParsedMessage, user: &mut User) {
    match parsed_msg.message {
        crate::types::Message::Nick(nickmsg) => {
           nick_msg_handler(nickmsg, user);
        },
        crate::types::Message::User(usermsg) => {
            user_msg_handler(usermsg, user);
        },
        crate::types::Message::PrivMsg(_privmsg) => {
            todo!()
            // priv_msg_handler(privmsg, user, parsed_msg.sender_nick);
        },
        crate::types::Message::Ping(message) => {
            ping_msg_handler(message, user);
        },
        crate::types::Message::Join(_joinmsg) => {
            todo!()
        },
        crate::types::Message::Part(_) => todo!(),
        crate::types::Message::Quit(quit_msg) => {
            quit_msg_handler(quit_msg, user);
        },
    }
}

/// Function that used to give back a reply to the user.
pub fn reply_handler(reply: Reply, user: &mut User) {
    let message = &format!("{}", reply);
    let _ = user.get_conn_write().write_message(message);
}

/// Function that used to give back am error to the user.
pub fn parse_error_handler(err: ErrorType, user: &mut User) {
    let message = &format!("{}\r\n", err);
    let _ = user.get_conn_write().write_message(message);
}

fn nick_msg_handler(nickmsg: NickMsg, user: &mut User) {
    // The user is not allowed to change the nick name
    if !user.nick_name_is_none() {
        return;
    }
    user.set_nick_name(nickmsg.nick.0);
    let message = &format!("You set your nick name as: {}.\r\n", Nick(user.get_nick_name().unwrap()));
    let _ = user.get_conn_write().write_message(&message);
}

fn user_msg_handler(usermsg: UserMsg, user: &mut User) {
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

// TODO: The message is send back to the sender!!!
// fn priv_msg_handler(privmsg: PrivMsg, conn_write: &mut ConnectionWrite, sender_nick: Nick) {
//     println!("You send a message to {} with content: {}.", privmsg.target, privmsg.message);
//     let message = &format!("{}: {}\r\n", sender_nick, privmsg.message);
//     let _ = conn_write.write_message(&message);
// }

fn ping_msg_handler(msg: String, user: &mut User) {
    let reply = Reply::Pong(
        msg
    );
    reply_handler(reply, user);
}


// TODO: The quit message should send to all users in the channel
// Only send to user now
fn quit_msg_handler(quit_msg: QuitMsg, user: &mut User) {
    let reply = Reply::Quit(
        QuitReply {
            message: quit_msg,
            sender_nick: Nick(user.get_nick_name().unwrap()),
        }
    );
    user.set_quit();
    reply_handler(reply, user);
}