extern crate irc;
extern crate regex;
extern crate dynamic_reload;
extern crate ctrlc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;
mod plugin;

use irc::client::prelude::*;
use plugin::Plugins;
use dynamic_reload::{DynamicReload,PlatformName, Search};
use std::sync::{Arc,Mutex};
use regex::Regex;

/// Global plugin state is defined here.
/// Also define static reference Regexes for improved performance.
lazy_static! {
    static ref PLUGINS: Arc<Mutex<Plugins>> = Arc::new(Mutex::new(Plugins::new()));
    static ref RELOAD_HANDLER: Arc<Mutex<DynamicReload<'static>>> = Arc::new(Mutex::new(DynamicReload::new(Some(vec!["plugins"]), Some("plugins"), Search::Backwards)));
    static ref LOAD_REGEX: Regex = Regex::new(r"!load (.*)").unwrap();
}

/// This function cleanly exits the program by finalizing all plugins before exiting.
fn quit() {
    PLUGINS.lock().unwrap().finalize_all();
    info!("Plugins finalized, exiting.");
    std::process::exit(0);
}

/// Entry Point
fn main() {
    env_logger::init();
    // Load Configuration
    let config = match Config::load("config.toml") {
        Ok(c) => c,
        Err(e) => {
            error!("Unable to load config.toml for IRC configuration with error {:?}, exiting!", e);
            std::process::exit(1);
        }
    };
    // Initialize IRC client
    let mut reactor = match IrcReactor::new() {
        Ok(r) => r,
        Err(e) => {
            error!("Unable to initialize IrcReactor with error {:?}, exiting!", e);
            std::process::exit(1);
        }
    };
    let client = match reactor.prepare_client_and_connect(&config) {
        Ok(c) => c,
        Err(e) => {
            error!("Unable to initialize IrcClient with error {:?}, exiting!", e);
            std::process::exit(1);
        }
    };
    // Identify to server
    match client.identify() {
        Ok(_) => info!("Sent identify."),
        Err(e) => {
            warn!("Identify call failed with error {:?}", e);
        }
    };
    // Register Handler
    reactor.register_client_with_handler(client, move |client, message| {
        // Print all messages to console for debugging/monitoring
        debug!("{:?}", message);
        // Bot owner plugin commands, must occur here as need access to plugin globals
        if let Some(nick) = message.source_nickname() {
            if let Command::PRIVMSG(ref chan, ref msg) = message.command {
                // Reload plugins
                if msg == "!reload" && config.is_owner(nick) {
                    info!("Reloading plugins.");
                    PLUGINS.lock().unwrap().finalize_all();
                    RELOAD_HANDLER.lock().unwrap().update(Plugins::reload_callback, &mut PLUGINS.lock().unwrap());
                    PLUGINS.lock().unwrap().initialize_all(client);
                    client.send_privmsg(&chan, "Reloaded plugins successfully.").unwrap_or_else(|e| {warn!("send_privmsg failed for plugin reload with error {:?}.", e);});
                }
                if msg == "!listplugins" && config.is_owner(nick) {
                    debug!("Printing plugin descriptions.");
                    PLUGINS.lock().unwrap().print_descriptions(client, &chan);
                }
                if msg == "!goodbye" && config.is_owner(nick) {
                    client.send_privmsg(&chan, "Goodbye.").unwrap_or_else(|e| {warn!("send_privmsg failed for goodbye with error {:?}", e);});
                    quit();
                }
                // Load plugin
                if let Some(caps) = LOAD_REGEX.captures(msg) {
                    if let Some(name) = caps.get(1) {
                        let name = name.as_str();
                        match RELOAD_HANDLER.lock().unwrap().add_library(&name, PlatformName::Yes) {
                            Ok(lib) => {
                                println!("Loading plugin {}", name);
                                PLUGINS.lock().unwrap().add_plugin(&lib);
                                PLUGINS.lock().unwrap().initialize_plugin(&lib,client);
                                client.send_privmsg(&chan, &format!("Successfully loaded {}", name)).unwrap_or_else(|e| {warn!("send_privmsg failed for plugin load notice with error {:?}", e);});
                            }
                            Err(_) => {
                                client.send_privmsg(&chan, &format!("Unable to load {}", name)).unwrap_or_else(|e| {warn!("send_privmsg failed for plugin load failure notice with error {:?}", e);});
                            }
                        }
                    }
                }
            }
        }
        // Pass message on to plugins
        PLUGINS.lock().unwrap().handle_message(client, &message);
        Ok(())
    });

    // Setup exit handler
    ctrlc::set_handler(|| {
        quit();
    }).unwrap_or_else(|e| {
        error!("Failed to register SIGINT/SIGTERM handler with error {:?}", e);
    });

    // Kick off IRC Client
    reactor.run().unwrap_or_else(|e| {
        error!("Failed to start reactor run with error {:?}", e);
        std::process::exit(1);
    });
}
