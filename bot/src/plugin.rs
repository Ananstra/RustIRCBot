extern crate dynamic_reload;
extern crate irc;

use self::dynamic_reload::{Lib, UpdateState};
use irc::client::prelude::*;
use std::sync::Arc;

pub struct Plugins {
    plugins: Vec<Arc<Lib>>,
}

impl Plugins {
    pub fn new() -> Self {
        Plugins { plugins: Vec::new() }
    }

    pub fn add_plugin(&mut self, plugin: &Arc<Lib>) {
        self.plugins.push(plugin.clone());
    }

    pub fn unload_plugin(&mut self, lib: &Arc<Lib>) {
        for i in (0..self.plugins.len()).rev() {
            if &self.plugins[i] == lib {
                self.plugins.swap_remove(i);
            }
        }
    }

    pub fn reload_plugin(&mut self, lib: &Arc<Lib>) {
        Self::add_plugin(self, lib);
    }

    // called when a lib needs to be reloaded.
    pub fn reload_callback(&mut self, state: UpdateState, lib: Option<&Arc<Lib>>) {
        match state {
            UpdateState::Before => Self::unload_plugin(self, lib.unwrap()),
            UpdateState::After => Self::reload_plugin(self, lib.unwrap()),
            UpdateState::ReloadFailed(_) => println!("Failed to reload"),
        }
    }

    pub fn handle_message(&mut self, client: &IrcClient, message: &Message) {
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
