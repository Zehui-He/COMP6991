/// Plugin module for the IRC server.
///
/// To add a plugin into the server, you can just simply define your function in this
/// file and add a new line of code into the Plugin::new function.
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::user::User;

type PluginFunc =
    fn(Arc<Mutex<Vec<User>>>, Arc<Mutex<HashMap<String, Vec<String>>>>, String, String) -> ();

/// Struct that is used for storing all the plugin functions with assiciated name.
pub struct Plugin {
    plugins: HashMap<String, Box<PluginFunc>>,
}

impl Plugin {
    /// This function initilize the plugin object with given plugin name and functions.
    ///
    /// If you want to add a function named "send_message_at_2pm".
    /// You should modify your code as following:
    ///
    /// ```
    /// pub fn new() -> Self {
    ///     let plugin_hashmap = HashMap::<String, Box<PluginFunc>>::new();
    ///     let mut plugin_obj = Plugin { plugins: plugin_hashmap };
    ///
    ///     plugin_obj.insert_plugin("send_message_at_2pm".to_string(), send_msg_plug_in);
    ///
    ///     plugin_obj
    /// }
    /// ```
    ///
    ///
    ///
    pub fn new() -> Self {
        let plugin_hashmap = HashMap::<String, Box<PluginFunc>>::new();
        let mut plugin_obj = Plugin {
            plugins: plugin_hashmap,
        };

        // This is where you insert your plugin name and function
        plugin_obj.insert_plugin("reminder".to_string(), reminder_plug_in);
        plugin_obj.insert_plugin("checktime".to_string(), checktime);

        plugin_obj
    }

    /// Check if the receiver name is a plugin name.
    pub fn is_a_plugin(&self, receiver_name: &String) -> bool {
        let x = self.plugins.get(receiver_name);
        if let Some(_) = x {
            return true;
        } else {
            return false;
        }
    }

    /// Insert a function to the plugin object.
    fn insert_plugin(&mut self, plugin_name: String, plugin_func: PluginFunc) {
        self.plugins.insert(plugin_name, Box::new(plugin_func));
    }

    /// Generate a plugin thread with a given plugin name.
    pub fn generate_thread(
        &self,
        users: Arc<Mutex<Vec<User>>>,
        channels: Arc<Mutex<HashMap<String, Vec<String>>>>,
        sender_id: String,
        message: String,
        plugin_name: String,
    ) {
        let f = *self.plugins.get(&plugin_name).unwrap().clone();

        std::thread::spawn(move || f(users, channels, sender_id, message));
    }
}

/// Example plugin 1.
///
/// Remind the user of something for every 60 seconds. The reminding message is the input message.
///
/// In this case the receiver and the sender are the same person.
fn reminder_plug_in(
    users: Arc<Mutex<Vec<User>>>,
    _channels: Arc<Mutex<HashMap<String, Vec<String>>>>,
    sender_id: String,
    message: String,
) {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));

        // Find the receiver
        let mut users = users.as_ref().lock().unwrap();
        let receiver = users.iter_mut().find(|usr| usr.get_id() == *sender_id);

        // If the receiver(sender) doesn't exist break the loop
        let receiver = match receiver {
            Some(res) => res,
            None => break,
        };

        // The remin
        let message = &format!("{message}\r\n",);

        // Send the message to receiver
        let _ = receiver.get_conn_write().write_message(message);
    }
}

/// Example plugin 2.
///
/// Return the current date and time.
fn checktime(
    users: Arc<Mutex<Vec<User>>>,
    _channels: Arc<Mutex<HashMap<String, Vec<String>>>>,
    sender_id: String,
    _message: String,
) {
    // Find the receiver
    let mut users = users.as_ref().lock().unwrap();
    let receiver = users.iter_mut().find(|usr| usr.get_id() == *sender_id);

    // If the receiver(sender) doesn't exist, return
    let receiver = match receiver {
        Some(res) => res,
        None => {
            return;
        }
    };

    let time_str = chrono::offset::Utc::now().format("%Y-%m-%d %H:%M:%S");

    let message = &format!("{time_str}\r\n",);

    let _ = receiver.get_conn_write().write_message(message);
}
