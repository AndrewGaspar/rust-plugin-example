struct PluginB;

impl core::Plugin for PluginB {
    fn callback1(&self) {
        println!("PluginB::callback1")
    }

    fn callback2(&self) {
        println!("PluginB::callback2")
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(PluginB));
}
