# RustIRCBot

This is an irc bot in rust that loads its message responses dynamically from libraries at runtime.

It expects libraries to be in a folder called plugins in the working directory from which the binary is called.

Plugins should be named according with plaftorm standard convention, i.e. libpluginname.so on Linux.

The bot will translate a load request for 'pluginname' to the appropriate filename and load it.

Plugins are expected to be platform dynamic library files (.dll, .so, etc) which contain the following function signatures, where the client and message types are structs from the irc crate:

> handle_message(client: &IrcClient, message: &Message) {...}

This is the function which will be called whenever the bot sees a message, and is the primary way for plugins to respond to commands.

> initialize(client: &IrcClient) {...}

This function will be called when the plugin is loaded from disk, and is intended to provide for initial setup.

> finalize() {...}

This function is called before a plugin is unloaded/reloaded, and is intended to provide for any necessary finalization.

> print_desciption(client: &IrcClient, channel: &str) {...}

This function is meant to print a brief description of the plugin to the specified channel. 

> print_help(client: &IrcClient, channel: &str) {...}

This function is meant to print a detailed help message to the specified channel.

A simple example of a plugin can be found in the example_plugin directory.


More plugins for this bot can be found [here](https://github.com/Ananstra/RustIRCBotPlugins/).
