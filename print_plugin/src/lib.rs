extern crate irc;

use irc::client::prelude::*;

#[no_mangle]
pub fn handle_message(client: &IrcClient, message: &Message) {
    println!("{}", message);
}

#[no_mangle]
pub fn initialize(_client: &IrcClient) {
    println!("Printer initialized!")
}

#[no_mangle]
pub fn finalize() {
    println!("Printer finalized")
}
