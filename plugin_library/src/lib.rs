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
pub fn initialize(_client: &IrcClient) {
    println!("Alive Plugin initialized!")
}

#[no_mangle]
pub fn finalize() {
    println!("Alive Plugin finalized")
}
