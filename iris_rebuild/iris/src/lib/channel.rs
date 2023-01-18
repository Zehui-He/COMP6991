//! Struct represants all channels in the IRC server. 

use std::collections::HashMap;

use crate::types::Nick;

pub struct ChannelPool {
    channel_hashmap: HashMap<String, Vec<String>>,
}

impl ChannelPool {
    pub fn new() -> Self {
        ChannelPool {
            channel_hashmap: HashMap::<String, Vec<String>>::new(),
        }
    }

    pub fn channel_exist(&self, channel_name: &String) -> bool {
        self.channel_hashmap.contains_key(channel_name)
    }

    pub fn create_channel(&mut self, channel_name: String) {
        // Do nothing if the channel already exist
        if self.channel_exist(&channel_name) {
            return;
        }

        self.channel_hashmap
            .insert(channel_name, Vec::<String>::new());
    }

    pub fn get_channel(&mut self, channel_name: &String) -> Option<&mut Vec<String>> {
        self.channel_hashmap.get_mut(channel_name)
    }

    /// When a user join a non-exist channel, the channel will be created.
    pub fn join_channel(&mut self, channel_name: &String, user_nick: Nick) {
        // Create the channel if it doesn't exist
        if !self.channel_exist(channel_name) {
            self.create_channel(channel_name.clone())
        }

        // Push the user into the channel
        let channel = self.get_channel(&channel_name).unwrap();
        channel.push(user_nick.0)
    }

    pub fn user_in_channel(&mut self, channel_name: &String, user_nick: Nick) -> bool {
        // Return false if the channel doesn't exist
        if !self.channel_exist(channel_name) {
            return false;
        }

        // Find if the use in the channel
        let channel = self.get_channel(channel_name).unwrap();
        match channel.iter().find(|x| x == &&user_nick.0) {
            Some(_) => return true,
            None => return false,
        }
    }

    pub fn quit_channel(&mut self, channel_name: &String, user_nick: Nick) {
        let channel = self.get_channel(channel_name).unwrap();
        channel.retain(|x| x != &user_nick.0);
    }

    pub fn remove_user(&mut self, user_nick: Nick) {
        for (_, channel) in self.channel_hashmap.iter_mut() {
            channel.retain(|x| x != &user_nick.0);
        }
    }
}
