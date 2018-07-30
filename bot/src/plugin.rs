use libloading::Library;
use irc::client::prelude::*;
use std::collections::HashMap;
/// Plugins represent dynamically loaded libraries which are expected to implement several methods.
pub struct Plugin <'a> {
    path: &'a str,
    lib: Library,
}


impl<'a> Plugin<'a> {
    /// Load a plugin from the provided path, with the provided name. TODO: Make error handling better.
    pub fn new(path: &'a str) -> Self {
        Plugin {
            path: path,
            lib:Library::new(path).unwrap_or_else(|error| panic!("{}", error)),
        }
    }
    /// Passes message events through to a plugin.
    pub fn handle_message(&self, client: &IrcClient, message: &Message) {
        unsafe {
            let f = self.lib.get::<fn(&IrcClient, &Message)> (
                b"handle_message\0"
            ).unwrap();
            f(client, message);
        }
    }
    /// Allows a plugin to run code upon initialization
    pub fn initialize(&self, client: &IrcClient) {
        unsafe {
            let f = self.lib.get::<fn (&IrcClient)> (
                b"initialize\0"
            ).unwrap();
            f(client);
        }
    }
    /// Allows a plugin to finalize itself.
    pub fn finalize(&self) {
        unsafe {
            let f = self.lib.get::<fn ()> (
                b"finalize\0"
            ).unwrap();
            f();
        }
    }
    /// Reloads a plugin.
    pub fn reload_plugin(&mut self) {
        self.lib = Library::new(self.path).unwrap_or_else(|error| panic!("{}", error));
    }

}

/// Plugin manager for IRC bot.
pub struct PluginManager<'a> {
    plugins: HashMap<&'a str, Plugin<'a>>,
}

impl<'a> PluginManager<'a> {
    /// Initialize a plugin manager with a given irc client.
    pub fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }
    /// Load a plugin from the given library path with the given name.
    pub fn load_plugin(&mut self, client: &IrcClient, path: &'a str, name: &'a str) {
        let p = Plugin::new(path);
        p.initialize(client);
        self.plugins.insert(name, p);
    }
    /// Reload the named plugin.
    pub fn reload_plugin(&mut self, client: &IrcClient, name: &str) {
        let p = self.plugins.get_mut(name).unwrap();
        p.finalize();
        p.reload_plugin();
        p.initialize(client);
    }
    /// Reload all plugins.
    pub fn reload_all(&mut self, client: &IrcClient) {
        self.plugins.values_mut().for_each(|plugin| {
            plugin.finalize();
            plugin.reload_plugin();
            plugin.initialize(client);
        });
    }
    /// Unload a plugin.
    pub fn unload_plugin(&mut self, name: &str) {
        let p = self.plugins.remove(name).unwrap();
        p.finalize();
    }

    /// Pass a message to all plugins.
    pub fn handle_message(&self, client: &IrcClient, message: &Message) {
        self.plugins.values().for_each(move |x| {
            x.handle_message(client, message)
        })
    }
}
