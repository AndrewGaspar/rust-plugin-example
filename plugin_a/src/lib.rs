struct PluginA;

impl core::Plugin for PluginA {
    fn callback1(&self) {
        println!("PluginA::callback1")
    }

    fn callback2(&self, i: i32) -> i32 {
        println!("PluginA::callback2");
        i + 1
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(PluginA));
}
