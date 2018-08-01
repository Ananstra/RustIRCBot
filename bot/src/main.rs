extern crate irc;
extern crate libloading;
extern crate regex;
mod plugin;
#[macro_use]
extern crate lazy_static;

use irc::client::prelude::*;
use plugin::PluginManager;
use regex::Regex;
use std::thread;
use std::sync::{Arc, Mutex};

const LIB_PATH: &'static str = "target/debug/libprint_plugin.so"; 

lazy_static! {
    static ref RELOAD_REGEX: Regex = Regex::new(r#"!reload (.*)"#).unwrap();
    static ref UNLOAD_REGEX: Regex = Regex::new(r#"!unload (.*)"#).unwrap();
    static ref LOAD_REGEX: Regex = Regex::new(r#"!load (.*) (.*)"#).unwrap();
    static ref LIB_STRINGS: Vec<String> = vec![];
    static ref PLUGIN_MANAGER: Arc<Mutex<PluginManager<'static>>> = Arc::new(Mutex::new(PluginManager::new()));
}

fn term() {
}

fn main() {
    let config = Config::load("config.toml").unwrap();
    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();
    // PM.load_plugin(&client, LIB_PATH, &"printer");
    PLUGIN_MANAGER.lock().unwrap().load_plugin(&client, "target/debug/libalive_plugin.so", &"status");
    reactor.register_client_with_handler(client, |client, message| {
        PLUGIN_MANAGER.lock().unwrap().handle_message(client, &message);
        Ok(())
    });
    thread::spawn(move || {
        term();
    });
    reactor.run().unwrap();
}
