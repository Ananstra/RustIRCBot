extern crate irc;

use irc::client::prelude::*;

#[no_mangle]
pub fn handle_message(client: &IrcClient, message: &Message) {
    if let Command::PRIVMSG(ref channel, ref msg) = message.command {
        if msg.contains("!status") {
            client.send_privmsg(&channel, "^_^");
        }
    }
}

#[no_mangle]
pub fn initialize(_client: &IrcClient) {
    println!("Printer initialized!")
}

#[no_mangle]
pub fn finalize() {
    println!("Printer finalized")
}
