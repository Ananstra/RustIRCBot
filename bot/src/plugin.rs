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
        Plugins {
            plugins: Vec::new(),
        }
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
            UpdateState::ReloadFailed(e) => {
                error!("Failed to reload plugins in reload_callback with {:?}", e)
            }
        }
    }

    /// Pass through an IrcClient to a plugin for initialization
    pub fn initialize_plugin(&self, plugin: &Arc<Lib>, client: &IrcClient) {
        unsafe {
            match plugin.lib.get::<fn(&IrcClient)>(b"initialize\0") {
                Ok(f) => {
                    f(client);
                }
                Err(_) => {
                    warn!(
                        "Attempted to call initialize from {:?}, but was unable to load symbol.",
                        plugin.original_path
                    );
                }
            };
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
            match plugin.lib.get::<fn()>(b"finalize\0") {
                Ok(f) => {
                    f();
                }
                Err(_) => {
                    warn!(
                        "Attempted to call finalize from {:?}, but was unable to load symbol",
                        plugin.original_path
                    );
                }
            };
        }
    }
    /// Finalize all plugins
    pub fn finalize_all(&self) {
        for plugin in &self.plugins {
            self.finalize_plugin(plugin);
        }
    }

    /// Print plugin descriptions to a channel.
    pub fn print_descriptions(&self, client: &IrcClient, channel: &str) {
        for plugin in &self.plugins {
            unsafe {
                match plugin
                    .lib
                    .get::<fn(&IrcClient, &str)>(b"print_description\0")
                {
                    Ok(f) => {
                        f(client, channel);
                    }
                    Err(_) => {
                        warn!("Attempted to call print_description from {:?}, but was unable to load symbol", plugin.original_path);
                    }
                };
            }
        }
    }

    /// Pass a message through to all plugins
    pub fn handle_message(&self, client: &IrcClient, message: &Message) {
        for plugin in &self.plugins {
            unsafe {
                match plugin
                    .lib
                    .get::<fn(&IrcClient, &Message)>(b"handle_message\0")
                {
                    Ok(f) => {
                        f(client, message);
                    }
                    Err(_) => {
                        warn!("Attempted to call handle_message from {:?}, but was unable to load symbol", plugin.original_path);
                    }
                };
            }
        }
    }
}
