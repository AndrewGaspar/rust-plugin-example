# rust-plugin-example
This code demonstrates how you can implement a dynamic loading plugin system in
Rust.

## Running the code
```
$ cargo build --all && cargo run -- ./target/debug/libplugin_a.so ./target/debug/libplugin_b.so
PluginA::callback1
PluginA::callback2
[src/main.rs:35] plugin.callback2(7) = 8
PluginB::callback1
PluginB::callback2
[src/main.rs:35] plugin.callback2(7) = 6
```

## Code Structure
Your application will need to break itself apart into at least two crates: the
application or library crate and a separate "core" crate. This "core" crate will
provide the shared types and routines with your plugins.

This particular example has the application, which is the top level crate, and
the `core` crate. This [`core`](core) crate merely provides traits, and is
currently implemented as a static library. If you provide a large amount of
actual code, or have some sort of global state in it, then you may want to
consider making it a "dylib" as well. For now, it's a static lib because all it
provides are thin trait definitions.

This crate also defines two plugins: [`plugin_a`](plugin_a) and [`plugin_b`](
plugin_b). Each of those plugins then link the `core` crate, as well.
Importantly, the application does not directly link `plugin_a` and `plugin_b`:
they will be loaded at runtime.

### The `core` Crate
The `core` crate in this example is very simple, but in practice would likely
be more complicated for a real application:
```rust
/// The `PluginRegistrar` is defined by the application and passed to `plugin_entry`. It's used
/// for a plugin module to register itself with the application.
pub trait PluginRegistrar {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>);
}

/// `Plugin` is implemented by a plugin library for one or more types. As you need additional
/// callbacks, they can be defined here. These are first class Rust trait objects, so you have the
/// full flexibility of that system. The main thing you'll lose access to is generics, but that's
/// expected with a plugin system
pub trait Plugin {
    /// This is a callback routine implemented by the plugin.
    fn callback1(&self);
    /// Callbacks can take arguments and return values
    fn callback2(&self, i: i32) -> i32;
}
```

### Implementing a Plugin Module
Each plugin must be a dynamic library. This can be done by specifying the
following in your `Cargo.toml`:
```toml
[lib]
crate-type = ["dylib"]
```

A plugin must export a known, linkable name. In our case, we're using the name
`plugin_entry`, but you may want a more specific name for your application.
Critically, you must use `#[no_mangle]` to make sure Rust produces a predictable
name.

Here's an example `plugin_entry`:
```rust
#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(PluginA));
}
```

Here, you can see we register our `Plugin` implementation with the registrar via
the callback mechanism on the registrar. The actual meat of the plugin will be
implemented by `PluginA`.

### Loading and Using Plugins
The precise code can be found in [src/main.rs](src/main.rs), but the gist of it
is:
1. Implement and instantiate `PluginRegistrar`
2. Find your plugins via some mechanism (we use command line arguments)
3. Load your plugin libraries. I recommend the [`libloading`](
   https://lib.rs/crates/libloading) crate.
   - (Optional) Manage the lifetime of the libraries. We sidestep this by
     never unloading our plugins.
4. Find `plugin_entry` in each library.
5. Call `plugin_entry` with your registrar for each library.
6. Use the plugins received in `register_plugin` in your `PluginRegistrar`
   implementation.