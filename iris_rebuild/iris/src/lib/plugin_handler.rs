//! This module defines a struct for the plugin system. 
//! This system simply provide functions when a plugin name is given. 

use std::{collections::HashMap};

use crate::{user::UserPool, types::{Nick, ErrorType}, plugin::reminder_handler};

type FunctionName = Nick;
type PluginFunc = fn(&mut UserPool, String, Nick) -> Result<(), ErrorType>;

pub struct PluginStruct {
    plugins: HashMap<FunctionName, Box<PluginFunc>>
}

impl PluginStruct {
    /// This is where the user can add a new plugin. 
    /// To add a plugin into the system, the user only need to add a 
    /// new entry into the hashmap. 
    pub fn new() -> Self {
        let mut plugin_obj = PluginStruct { plugins: HashMap::new()};

        // This is an example plugin
        plugin_obj.add_func(Nick("reminder".to_string()), reminder_handler);

        plugin_obj
    }

    pub fn get_plugin(&self, func_name: Nick) -> Box<PluginFunc> {
        self.plugins.get(&func_name).unwrap().clone()
    }

    /// Function to check if the receiver is a plugin name.
    pub fn is_plugin(&self, func_name: Nick) -> bool {
        self.plugins.get(&func_name).is_some()
    }

    fn add_func(&mut self, name: Nick, func: PluginFunc) {
        self.plugins.insert(name, Box::new(func));
    }
}