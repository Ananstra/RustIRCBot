extern crate irc;

use irc::client::prelude::*;

/// handle_message is called every time the bot sees a message. This is where the plugin can respond to messages however it wishes.
#[no_mangle]
pub fn handle_message(client: &IrcClient, message: &Message) {
    if let Command::PRIVMSG(ref channel, ref msg) = message.command {
        if msg.contains("!status") {
            client
                .send_privmsg(&channel, "<3")
                .unwrap_or_else(|e| println!("send_privmsg failed {:?}", e));
        }
    }
}

/// initialize is called when a plugin is loaded from disk. This is where a plugin should do initial setup.
#[no_mangle]
pub fn initialize(client: &IrcClient) {
    for channel in client.list_channels().unwrap() {
        client
            .send_privmsg(&channel, "RustBot example plugin online!")
            .unwrap_or_else(|e| println!("send_privmsg failed {:?}", e));
    }
}

/// finalize is called before a plugin is unloaded or reloaded. This is where a plugin should do any final tasks.
#[no_mangle]
pub fn finalize() {
    println!("RustBot example plugin finalizing.");
}

/// print_description is where the plugin should write to the provided channel a brief description of itself.
#[no_mangle]
pub fn print_description(client: &IrcClient, channel: &str) {
    client
        .send_privmsg(&channel, "plugin: A simple example plugin")
        .unwrap_or_else(|e| {
            println!{"send_privmsg failed {:?}", e}
        });
}

/// print_help is where the plugin should write a help message to the provided channel.
#[no_mangle]
pub fn print_help(client: &IrcClient, channel: &str) {
    client
        .send_privmsg(&channel, "!status - proves that this bot is alive!")
        .unwrap_or_else(|e| {
            println!{"send_privmsg failed {:?}", e}
        });
}
