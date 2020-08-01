use std::sync::Weak;
use serde::{Deserialize, Serialize};
use flutter_engine::{
    channel::{MethodCallHandler, MethodChannel},
    codec::{value::Value as CodecValue, STANDARD_CODEC},
    plugins::Plugin,
    FlutterEngine,
};
use flutter_engine::channel::MethodCall;
use std::process::Command;

pub const PLUGIN_NAME: &str = module_path!();
pub const CHANNEL_NAME: &str = "plugins.flutter.io/url_launcher";

pub struct UrlLauncherPlugin {
    channel: Weak<MethodChannel>,
}

impl Default for UrlLauncherPlugin {
    fn default() -> Self {
        Self {
            channel: Weak::new(),
        }
    }
}

impl Plugin for UrlLauncherPlugin {
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

impl UrlLauncherPlugin {

}

struct Handler;

impl MethodCallHandler for Handler {
    fn on_method_call(&mut self, call: MethodCall) {
        println!("on_method_call {:?}", call.method().as_str());
        match call.method().as_str() {
            "canLaunch" => {
                let args: CanLaunchParams = call.args();
                println!("args is {:?}", args);
                call.success(true)
            },
            "launch" => {
                let args: LaunchParams = call.args();
                println!("args is {:?}", args);

                Command::new("open")
                    .arg(args.url)
                    .output()
                    .expect("failed to execute process");

                call.success(true)
            }
            _ => {
                call.not_implemented()
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CanLaunchParams {
    pub url: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct LaunchParams {
    pub universalLinksOnly: bool,
    pub useWebView: bool,
    pub url: String,
    pub useSafariVC: bool,
    pub enableJavaScript: bool,
    pub headers: CodecValue,
    pub enableDomStorage: bool,
}
