use std::{
    fmt::Debug,
    sync::{Arc, RwLock, RwLockWriteGuard},
};

use crate::{
    connect::ConnectionWrite,
    types::{ErrorType, Nick, Reply},
};

pub enum UserPoolErrorType {
    UserNotExist,
}

pub struct User {
    id: String,
    nick: Option<String>,
    real: Option<String>,
    connwrite: ConnectionWrite,
    is_registered: bool,
    has_nick: bool,
    has_real: bool,
    joined_channels: Vec<String>,
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("nick", &self.nick)
            .field("real", &self.real)
            .field("is_registered", &self.is_registered)
            .field("has_nick", &self.has_nick)
            .field("has_real", &self.has_real)
            .finish()
    }
}

impl User {
    pub fn new(id: String, connwrite: ConnectionWrite) -> Self {
        User {
            id,
            nick: None,
            connwrite,
            is_registered: false,
            real: None,
            has_nick: false,
            has_real: false,
            joined_channels: Vec::<String>::new(),
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_nick(&self) -> Nick {
        match &self.nick {
            Some(nick) => Nick(nick.clone()),
            None => Nick(self.get_id()),
        }
    }

    pub fn get_real(&self) -> Nick {
        match &self.real {
            Some(real) => Nick(real.clone()),
            None => Nick(self.get_id()),
        }
    }

    pub fn is_registered(&self) -> bool {
        self.is_registered
    }

    pub fn nick_is_set(&self) -> bool {
        self.has_nick
    }

    pub fn set_nick(&mut self, nick_str: String) {
        self.nick = Some(nick_str);
        self.has_nick = true;

        // The user is registered if both real and nick are not none
        if self.nick_is_set() && self.real_is_set() {
            self.is_registered = true
        }
    }

    pub fn send(&mut self, reply: Reply) {
        self.connwrite.write_message(&format!("{}", reply)).unwrap();
    }

    pub fn send_error(&mut self, err: ErrorType) {
        self.connwrite
            .write_message(&format!("{}\r\n", err))
            .unwrap();
    }

    pub fn set_real(&mut self, real: String) {
        self.real = Some(real);
        self.has_real = true;

        // The user is registered if both real and nick are not none
        if self.nick_is_set() && self.real_is_set() {
            self.is_registered = true
        }
    }

    pub fn real_is_set(&self) -> bool {
        self.has_real
    }

    pub fn get_joined_channels(&self) -> &Vec<String> {
        &self.joined_channels
    }
}

pub struct UserPool {
    users: Arc<RwLock<Vec<User>>>,
}

impl UserPool {
    pub fn new() -> Self {
        UserPool {
            users: Arc::new(RwLock::new(Vec::<User>::new())),
        }
    }

    pub fn insert(&mut self, user: User) {
        self.users.write().unwrap().push(user)
    }

    pub fn nick_exist(&self, nick_str: String) -> bool {
        let users = self.users.as_ref().read().unwrap();
        users.iter().find(|user| user.get_nick() == Nick(nick_str.clone())).is_some()
    }

    pub fn get_nick_by_id(&self, id: String) -> Nick {
        let users = self.users.as_ref().read().unwrap();
        match users.iter().find(|user| user.get_id() == id) {
            Some(user) => user.get_nick(),
            None => Nick(id),
        }
    }

    pub fn get_write_pool(&mut self) -> RwLockWriteGuard<Vec<User>> {
        return self.users.as_ref().write().unwrap();
    }

    pub fn user_is_registered(&self, nick: Nick) -> bool {
        let users = self.users.as_ref().read().unwrap();

        users
            .iter()
            .find(|user| user.get_nick() == nick)
            .unwrap()
            .is_registered()
    }

    pub fn user_join_channel(&mut self, nick: Nick, channel_name: String) {
        let mut users = self.get_write_pool();

        let user = users
            .iter_mut()
            .find(|user| user.get_nick() == nick)
            .unwrap();
        user.joined_channels.push(channel_name);
    }

    pub fn user_quit_channel(&mut self, nick: Nick, channel_name: String) {
        let mut users = self.get_write_pool();
        let user = users.iter_mut().find(|x| x.get_nick() == nick).unwrap();

        user.joined_channels.retain(|x| x != &channel_name);
    }

    pub fn remove_user(&mut self, nick: Nick) {
        let mut users = self.get_write_pool();
        users.retain(|x| x.get_nick() != nick);
    }
}

impl Clone for UserPool {
    fn clone(&self) -> Self {
        Self {
            users: self.users.clone(),
        }
    }
}
