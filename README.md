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

# Using this bot

This repository includes two crates.

The bot crate build the binary IRCBot. This binary can be run out of whatever directory you desire.

The example_plugin crate build a simple example plugin library. It, and all other libraries, should be placed in the `plugins` directory of the folder out of which the bot binary is run.

The bot is configured via the rust IRC crate standard configuration, config.toml, which should be placed in the same directory in which the bot binary is run. An example configuration file is available in this repo at config.toml.example. For a more detailed description of this file, please see the IRC crate documentations available on [docs.rs](https://docs.rs/irc/0.13.5/irc/).

Once connected, any bot owner (as specified in the above-mentioned config) can issue plugin management commands to the bot via private message or any channel in which the bot resides.

>!load \<plugin_name>

This command will make the bot load libplugin_name.<platform specific extenstion> from the plugins directory.

>!reload

This command will make the bot reload all plugins, picking up any changes made to the libraries loaded from the plugins directory.

>!goodbye

This command will make the bot gracefully exit, calling all finalize methods.

The example plugin implements a command.

>!status

Which the bot will respond to, verifying that it is alive and seeing messages.

# CS 510 Stuff

This was a project for CS 510: Rust Topics at Portland State University. The below relates mostly to this class.

## Comments

The primary difficulty making this project work was figuring out how to dynamically load libraries in a useful way - being able to trigger load and reload actions via IRC being the goal. I spent some time attempting to build my own system for this, but ultimately settled on the dymanic_reload crate, as it had already solved a good deal of the problem I was tackling. From there, I simply had to build out an interface for plugin management and event passthrough.

The framework has a decent amount of error handling, which should ideally prevent most issues stemming from loading 'bad' libraries, however, loading unknown libraries at runtime IS an inherently unsafe operation, and should be treated as such. This bot is intended much more as a proof of concept and an opportunity for me to explore dynamic loading in Rust than as a real, production-ready IRC Bot core.

Features I would like to implement should I continue work on this project include plugin unloading (I was unable to find a simple method for handling this, it would likely require a name-> library lookup table), reloading specific plugins (the same difficulties as unloading apply here), more robust event passthrough, exposing more triggers and options to plugins, and better error handling and library safety (insomuch as this is possible with dynamic library loading.)
