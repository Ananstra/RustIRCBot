extern crate irc;

use irc::client::prelude::*;

use std::ffi:OsStr;
use std::any::Any;
use libloading::{Library, Symbol};

use errors::*;

/// IRC Bot Plugin Trait
pub trait Plugin: Any + Send + Sync {
    /// Get Plugin Name
    fn name(&self) -> &'static str;
    /// Plugin Load Function
    /// Allows for a plugin to do initialization
    fn on_plugin_load(&self) {}
    /// Plugin Unload Function
    /// Allows for a plugin to do finalization
    fn on_plugin_unload(&self) {}
    /// IRC Message handler.
    fn on_message(&self, IrcClient, &str) {}
}
/// Declare a plugin type and its constructor.
///
/// # Notes
///
/// This works by automatically generating an `extern "C"` function with a
/// pre-defined signature and symbol name. Therefore you will only be able to
/// declare one plugin per library.
///
/// Borrowed from https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut $crate::Plugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<$crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
/// Plugin Manager - Allows for loading of plugins at runtime.
pub struct PluginManager {
    plugins: Vec<Box<Plugin>>,
    loaded_libraries: Vec<Library>,
}
impl PluginManager {
    /// Plugin Manager 'constructor'
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }
    /// Load plugin at runtime.
    /// Again borrowed from https://micheal-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
    pub unsafe fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<()> {
        type PluginCreate = unsafe fn() -> *mut Plugin;

        let lib = Library::new(filename.as_ref()).chain_err(|| "Unable to load the plugin")?;

        // We need to keep the library around otherwise our plugin's vtable will
        // point to garbage. We do this little dance to make sure the library
        // doesn't end up getting moved.
        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();

        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")
            .chain_err(|| "The `_plugin_create` symbol wasn't found.")?;
        let boxed_raw = constructor();

        let plugin = Box::from_raw(boxed_raw);
        debug!("Loaded plugin: {}", plugin.name());
        plugin.on_plugin_load();
        self.plugins.push(plugin);


        Ok(())
    }
    /// Pass on a message/client to each plugins
    pub fn on_message(&mut self, client: IrcClient, message: &str) {
        debug!("Distributing message {}", message);
        for plugin in &mut self.plugins {
            trace!("Sending message to {:?}", plugin.name());
            plugin.on_message(client, message);
        }
    }

    /// Unload all plugins and loaded plugin libraries, making sure to fire
    /// their `on_plugin_unload()` methods so they can do any necessary cleanup.
    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for plugin in self.plugins.drain(..) {
            trace!("Firing on_plugin_unload for {:?}", plugin.name());
            plugin.on_plugin_unload();
        }

        for lib in self.loaded_libraries.drain(..) {
            drop(lib);
        }
    }
}
impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
            self.unload();
        }
    }
}
