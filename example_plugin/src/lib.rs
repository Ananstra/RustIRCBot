extern crate irc;

use irc::client::prelude::*;

#[no_mangle]
pub fn handle_message(client: &IrcClient, message: &Message) {
    if let Command::PRIVMSG(ref channel, ref msg) = message.command {
        if msg.contains("!status") {
            client.send_privmsg(&channel, "<3");
        }
    }
}

#[no_mangle]
pub fn initialize(client: &IrcClient) {
    for channel in client.list_channels().unwrap() {
        client.send_privmsg(&channel, "RustBot example plugin online!");
    }
}

#[no_mangle]
pub fn finalize() {
    println!("RustBot example plugin finalizing.");
}
