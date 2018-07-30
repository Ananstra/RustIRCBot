extern crate irc;
extern crate libloading;

use libloading::Library;
use irc::client::prelude::*;

const LIB_PATH: &'static str = "target/debug/libplugin.so"; 

struct Plugin(Library);
impl Plugin {
    fn handle_message(&self, client: &IrcClient, message: &str) {
        unsafe {
            let f = self.0.get::<fn(&IrcClient, &str)> (
                b"handle_message\0"
            ).unwrap();
            f(client, message);
        }
    }
}

fn main() {
    let config = Config::load("config.toml").unwrap();

    let p = Plugin(Library::new(LIB_PATH).unwrap_or_else(|error| panic!("{}", error)));

    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();
    reactor.register_client_with_handler(client, move |client, message| {
        p.handle_message(client, &message.to_string());
        // And here we can do whatever we want with the messages.
        Ok(())
    });

    reactor.run().unwrap();
}
