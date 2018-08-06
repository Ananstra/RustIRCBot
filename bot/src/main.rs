extern crate irc;
extern crate libloading;
extern crate regex;
mod plugin;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate text_io;

use irc::client::prelude::*;
use plugin::PluginManager;
use regex::Regex;
use std::thread;
use std::sync::{Arc, Mutex};

const LIB_PATH: &'static str = "target/debug/libprint_plugin.so"; 

lazy_static! {
    static ref RELOAD_REGEX: Regex = Regex::new(r#"reload (.*)"#).unwrap();
    static ref UNLOAD_REGEX: Regex = Regex::new(r#"unload (.*)"#).unwrap();
    static ref LOAD_REGEX: Regex = Regex::new(r#"load (.*) (.*)"#).unwrap();
    static ref PLUGIN_MANAGER: Arc<Mutex<PluginManager<'static>>> = Arc::new(Mutex::new(PluginManager::new()));
}

/// Plugin Management Terminal
fn term() {
    loop {
        println!("Enter a command.");
        let cmd: String = read!("{}\n");

        if cmd == "exit" || cmd == "quit" || cmd == "halt" {
            std::process::exit(0);
        }

        if cmd == "reload_all" {
            PLUGIN_MANAGER.lock().unwrap().reload_all();
            continue;
        }
        let caps = RELOAD_REGEX.captures(&cmd).unwrap();
        if let Some(plugin_name) = caps.get(1) {
            PLUGIN_MANAGER.lock().unwrap().reload_plugin(&plugin_name.as_str());
            continue;
        }
        let caps = UNLOAD_REGEX.captures(&cmd).unwrap();
        if let Some(plugin_name) = caps.get(1) {
            PLUGIN_MANAGER.lock().unwrap().unload_plugin(&plugin_name.as_str());
            continue;
        }
        // let caps = LOAD_REGEX.captures(&cmd).unwrap();
        // if let Some(plugin_name) = caps.get(1) {
        //     if let Some (plugin_path) = caps.get(2) {
        //         PLUGIN_MANAGER.lock().unwrap().load_plugin(&plugin_path.as_str(), &plugin_name.as_str());
        //     }
        // }
    }
}

/// Main Loop
fn main() {
    // Load Configuration
    let config = Config::load("config.toml").unwrap();
    // Initialize IRC client
    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();
    PLUGIN_MANAGER.lock().unwrap().load_plugin("target/debug/libalive_plugin.so", &"status");
    // Register Handler
    reactor.register_client_with_handler(client, |client, message| {
        PLUGIN_MANAGER.lock().unwrap().handle_message(client, &message);
        Ok(())
    });

    // Kick off terminal
    thread::spawn(move || {
        term();
    });

    // Kick off IRC Client
    reactor.run().unwrap();
}
