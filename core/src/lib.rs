pub trait PluginRegistrar {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>);
}

pub trait Plugin {
    fn callback1(&self);
    fn callback2(&self);
}
