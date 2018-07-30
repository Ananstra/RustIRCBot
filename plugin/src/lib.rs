extern crate irc;

use irc::client::prelude::*;

#[no_mangle]
pub fn handle_message(client: &IrcClient, message: &str) {
    println!("{}", message);
}
