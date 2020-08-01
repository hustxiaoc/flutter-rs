use std::sync::Weak;
use serde::{Deserialize, Serialize};
use flutter_engine::{
    channel::{MethodCallHandler, MethodChannel},
    codec::{STANDARD_CODEC},
    plugins::Plugin,
    FlutterEngine,
};
use flutter_engine::channel::MethodCall;

pub const PLUGIN_NAME: &str = module_path!();
pub const CHANNEL_NAME: &str = "flutter/mousecursor";

pub struct MouseCursorPlugin {
    channel: Weak<MethodChannel>,
}

impl Default for MouseCursorPlugin {
    fn default() -> Self {
        Self {
            channel: Weak::new(),
        }
    }
}

impl Plugin for MouseCursorPlugin {
    fn plugin_name() -> &'static str {
        PLUGIN_NAME
    }

    fn init(&mut self, engine: &FlutterEngine) {
        self.channel = engine.register_channel(MethodChannel::new(
            CHANNEL_NAME,
            Handler,
            &STANDARD_CODEC,
        ));
    }
}

impl MouseCursorPlugin {

}

struct Handler;

impl MethodCallHandler for Handler {
    fn on_method_call(&mut self, call: MethodCall) {
        match call.method().as_str() {
            "activateSystemCursor" => {
                let args: ActivateSystemCursorParams = call.args();
                // todo
                println!("args {:?}", args);
                call.success(true)
            }
            _ => {
                call.not_implemented()
            },
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ActivateSystemCursorParams {
    pub kind: String,
    pub device: i64,
}
