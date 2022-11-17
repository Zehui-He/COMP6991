use crate::connect::ConnectionWrite;

pub struct User {
    ip_address: String,
    nick_name: Option<String>,
    real_name: Option<String>,
    conn_write: ConnectionWrite,
    registered: bool,
    quited: bool,
    joined_channel: Vec<String>
}

impl User {
    pub fn new(ip_address: String, conn_write: ConnectionWrite) -> Self {
        Self {
            ip_address,
            nick_name: None,
            real_name: None,
            conn_write,
            registered: false,
            quited: false,
            joined_channel: Vec::<String>::new()
        }
    }

    pub fn get_id(&self) -> String {
        self.ip_address.clone()
    }

    pub fn get_nick_name(&self) -> Option<String> {
        self.nick_name.clone()
    }

    pub fn get_real_name(&self) -> Option<String> {
        self.real_name.clone()
    }

    pub fn set_nick_name(&mut self, nick_name: String) {
        self.nick_name = Some(nick_name);
    }

    pub fn set_real_name(&mut self, real_name: String) {
        self.real_name = Some(real_name);
    }

    pub fn get_conn_write(&mut self) -> &mut ConnectionWrite {
        &mut self.conn_write
    }

    pub fn nick_name_is_none(&self) -> bool {
        self.nick_name.is_none()
    }

    pub fn real_name_is_none(&self) -> bool {
        self.real_name.is_none()
    }

    pub fn set_registered(&mut self) {
        self.registered = true
    }

    pub fn is_registered(&self) -> bool {
        self.registered
    }

    pub fn is_quit(&self) -> bool {
        self.quited
    }

    pub fn set_quit(&mut self) {
        self.quited = true
    }

    pub fn get_channels(&mut self) -> &mut Vec<String> {
        &mut self.joined_channel
    }
}