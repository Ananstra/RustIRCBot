extern crate dynamic_reload;
extern crate irc;

use self::dynamic_reload::{Lib, UpdateState};
use irc::client::prelude::*;
use std::sync::Arc;

/// A struct to hold a collection of loaded "plugins", which are wrapped dynamically loaded libraries.
pub struct Plugins {
    plugins: Vec<Arc<Lib>>,
}

impl Plugins {
    /// Create an empty list of plugins
    pub fn new() -> Self {
        Plugins { plugins: Vec::new() }
    }

    /// Add a wrapped library to the collection
    pub fn add_plugin(&mut self, plugin: &Arc<Lib>) {
        self.plugins.push(plugin.clone());
    }

    /// Remove a library from the collection
    pub fn unload_plugin(&mut self, lib: &Arc<Lib>) {
        for i in (0..self.plugins.len()).rev() {
            if &self.plugins[i] == lib {
                self.plugins.swap_remove(i);
            }
        }
    }

    /// "Reload"" a library
    pub fn reload_plugin(&mut self, lib: &Arc<Lib>) {
        Self::add_plugin(self, lib);
    }

    /// called when a lib needs to be reloaded.
    pub fn reload_callback(&mut self, state: UpdateState, lib: Option<&Arc<Lib>>) {
        match state {
            UpdateState::Before => Self::unload_plugin(self, lib.unwrap()),
            UpdateState::After => Self::reload_plugin(self, lib.unwrap()),
            UpdateState::ReloadFailed(_) => println!("Failed to reload"),
        }
    }

    /// Pass through an IrcClient to a plugin for initialization
    pub fn initialize_plugin(&self, plugin: &Arc<Lib>, client: &IrcClient) {
        unsafe {
            let f = plugin.lib.get::<fn(&IrcClient)> (
                b"initialize\0"
            ).unwrap();
            f(client);
        }
    }

    /// initialize all plugins
    pub fn initialize_all(&self, client: &IrcClient) {
        for plugin in &self.plugins {
            self.initialize_plugin(plugin, client);
        }
    }

    /// Finalize a plugin
    pub fn finalize_plugin(&self, plugin: &Arc<Lib>) {
        unsafe {
            let f = plugin.lib.get::<fn()> (
                b"finalize\0"
            ).unwrap();
            f();
        }
    }
    /// Finalize all plugins
    pub fn finalize_all(&self) {
        for plugin in &self.plugins {
            self.finalize_plugin(plugin);
        }
    }

    /// Pass a message through to all plugins
    pub fn handle_message(&self, client: &IrcClient, message: &Message) {
        for plugin in &self.plugins {
            unsafe {
                let f = plugin.lib.get::<fn(&IrcClient, &Message)> (
                    b"handle_message\0"
                ).unwrap();
                f(client,message);
            }
        }
    }
}
