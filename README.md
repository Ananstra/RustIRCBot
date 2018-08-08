# RustIRCBot

This is an irc bot in rust that loads its message responses dynamically from libraries at runtime.

It expects libraries to be in a folder called plugins in the working directory from which the binary is called.

Plugins are expected to be platform dynamic library files (.dll, .so, etc) which contain a function with signature


> handle_message(&IrcClient, &Message) {...}


where the client and message types are structs from the Rust IRC crate.
