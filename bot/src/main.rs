extern crate irc;
extern crate regex;
extern crate dynamic_reload;
#[macro_use]
extern crate lazy_static;
mod plugin;

use irc::client::prelude::*;
use plugin::Plugins;
use dynamic_reload::{DynamicReload,PlatformName, Search};
use std::sync::{Arc,Mutex};

lazy_static! {
    static ref plugins: Arc<Mutex<Plugins>> = Arc::new(Mutex::new(Plugins::new()));
    static ref reload_handler: Arc<Mutex<DynamicReload<'static>>> = Arc::new(Mutex::new(DynamicReload::new(Some(vec!["target/debug"]), Some("target/debug"), Search::Backwards)));
}

/// Main Loop
fn main() {
    // Load Configuration
    let config = Config::load("config.toml").unwrap();
    // Initialize IRC client
    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();
    match reload_handler.lock().unwrap().add_library(&"plugin", PlatformName::Yes) {
        Ok(lib) => plugins.lock().unwrap().add_plugin(&lib),
        Err(e) => {
            println!("Unable to load dynamic library: {:?}", e);
            return;
        }
    }
    // Register Handler
    reactor.register_client_with_handler(client, |client, message| {
        println!("{:?}", message);
        if let Some(nick) = message.source_nickname() {
            if let Command::PRIVMSG(ref _chan, ref msg) = message.command {
                if msg == "!reload" && nick == "kimani"{
                    println!("Triggering reload.");
                    reload_handler.lock().unwrap().update(Plugins::reload_callback, &mut plugins.lock().unwrap());
                }
            }
        }
        plugins.lock().unwrap().handle_message(client, &message);
        Ok(())
    });

    // Kick off IRC Client
    reactor.run().unwrap();
}
